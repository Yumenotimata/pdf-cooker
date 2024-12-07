use std::collections::{HashSet, HashMap};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Primitive {
    Dictionary(Vec<Primitive>),
    Pair(Box<Primitive>, Box<Primitive>),
    Name(String),
    Array(Vec<Primitive>),
    Ref(*const Object),
    Reference(u64),
    Stream(String),
    Number(u64),
    ParentRef,
}

impl Primitive {
    pub fn dictionary(pairs: Vec<Primitive>) -> Self {
        Primitive::Dictionary(pairs)
    }

    pub fn name<S: Into<String>>(name: S) -> Self {
        Primitive::Name(name.into())
    }

    pub fn array(array: Vec<Primitive>) -> Self {
        Primitive::Array(array)
    }

    pub fn reference(reference: &Object) -> Self {
        Primitive::Ref(reference)
    }

    pub fn pair(key: Primitive, value: Primitive) -> Self {
        Primitive::Pair(Box::new(key), Box::new(value))
    }

    pub fn iter(&self) -> PrimitiveIterator {
        PrimitiveIterator {
            stack: vec![self]
        }
    }

    pub fn iter_mut(&mut self) -> PrimitiveMutIterator {
        PrimitiveMutIterator {
            stack: vec![self]
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut Primitive {
        // match self {
        //     Primitive::Array(array) => (*array).as_mut_ptr(),
        //     Primitive::Dictionary(dictionary) => dictionary.as_mut_ptr(),
        //     Primitive::Name(ref mut name) => name.as_mut_ptr() as *mut Primitive,
        //     Primitive::Ref(inner) => Box::into_raw(Box::new(Primitive::Ref(*inner))) as *mut Primitive,
        //     _ => std::ptr::null_mut()
        // }
        self as *mut Primitive
    } 

    pub fn is_type<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        self.iter().any(|object| {
            matches!(
                object, 
                Primitive::Pair(key, value)
                    if matches!(
                        (key.as_ref(), value.as_ref()),
                        (Primitive::Name(key), Primitive::Name(value))
                            if key == "Type" && value == &name
                    )
            )
        })
    }
}

struct PrimitiveIterator<'a> {
    stack: Vec<&'a Primitive>
}

impl<'a> Iterator for PrimitiveIterator<'a> {
    type Item = &'a Primitive;

    // TODO:
    // comprehensive?
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            match current {
                Primitive::Array(array) => self.stack.extend(array),
                Primitive::Dictionary(dictionary) => self.stack.extend(dictionary), 
                _ => {}
            }

            return Some(current);
        }

        None
    }
}

struct PrimitiveMutIterator<'a> {
    stack: Vec<&'a mut Primitive>,
}

impl<'a> Iterator for PrimitiveMutIterator<'a> {
    type Item = &'a mut Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            unsafe {
                match *current.as_mut_ptr() {
                    Primitive::Array(ref mut array) => {
                        let array: Vec<&'a mut Primitive> = array.iter_mut().map(|a| &mut *a.as_mut_ptr()).collect();
                        self.stack.extend(array);
                    }
                    Primitive::Dictionary(ref mut dictionary) => {
                        self.stack.extend(dictionary.iter_mut());
                    }
                    _ => {}
                }
            }
            unsafe {
                return Some(&mut *current.as_mut_ptr());
            }
        }

        None
    }
}

impl Into<Vec<Primitive>> for Primitive {
    fn into(self) -> Vec<Primitive> {
        vec![self]
    }
}

// impl std::fmt::Display for Primitive {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             Primitive::Dictionary(news) => {
//                 news.iter()
//                     .map(|new| write!(f, "<<{} {}>>", new.key, new.value))
//                     .collect()
//             },
//             otherwise => {
//                 Ok(())
//             }
//         }
//     }
// }

#[derive(Debug)]
struct Object {
    inner: Vec<Primitive>,
    number: Option<u64>,
}

impl Object {
    pub fn new<V: Into<Vec<Primitive>>>(inner: V) -> Self {
        Object {
            inner: inner.into(),
            number: None,
        }
    }
}

struct Document {
    objects: Vec<Object>
}

impl Document {
    pub fn new() -> Document {
        Document {
            objects: vec![],
        }
    }

    pub fn append<O: Into<Object>>(&mut self, object: O) {
        self.objects.push(object.into());
    }

    pub fn appendix<V: Into<Vec<Object>>>(&mut self, objects: V) {
        self.objects.extend(objects.into());
    }

    pub fn resolve(&mut self) {
        let mut number: u64 = 1;
        let mut pending: HashSet<*const Object> = HashSet::new();
        let mut resolved: HashMap<*const Object, u64> = HashMap::new();

        self.objects.iter_mut().for_each(|object: &mut Object| {
            object.number = Some(number);
            println!("locking object {:?}", object as *const Object);
            resolved.insert(object as *const Object, number);
            number += 1;

            object.inner.iter_mut().for_each(|prim| {
                prim.iter_mut().for_each(|elm| 
                    match elm {
                    Primitive::Pair(_, value) => {
                        match value.as_ref() {
                            Primitive::Ref(reference) => {
                                if resolved.contains_key(reference) {
                                    *elm = Primitive::Reference(*resolved.get(reference).unwrap());
                                    println!("resolved");
                                } else {
                                    println!("inquire {:?}", reference);
                                    println!("{:#?}", resolved);
                                    pending.insert(reference.clone());
                                }
                                println!("kokokoko");
                            },
                            _ => {}
                        }

                    },
                    _ => { 
                        
                    } 
                });
            });

        });

        println!("{:#?}", resolved);

            // object.inner.iter_mut().for_each(|prim| match prim {
            //     // Primitive::Pair(_, value) => {
            //     //     match value.as_ref() {
            //     //         Primitive::Ref(reference) => {
            //     //             if resolved.contains_key(reference) {
            //     //                 *prim = Primitive::Reference(*resolved.get(reference).unwrap());
            //     //             } else {
            //     //                 pending.insert(reference.clone());
            //     //             }
            //     //         },
            //     //         _ => {}
            //     //     }

            //     // },
            //     _ => { println!("koko");} 
            // });
        // });
    }

    pub fn encode(&mut self) -> Result<(), ()> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice("%PDF-1.7\n".as_bytes());
        buf.extend_from_slice(&[0xe2, 0xe3, 0xcf, 0xd3, '\n' as u8]);
        // buf.extend_from_slice(
        //     format!("{}", Primitive::Dictionary(
        //         vec![Pair{key: Primitive::Number, value: Primitive::Number}]
        //     ))
        //     .as_bytes()
        // );

        let pages: Vec<&Object> = self.objects
            .iter()
            .filter(|obj| obj.inner.iter().any(|prim| prim.is_type("Page")))
            .collect();

        // println!("{:#?}", pages);

        let page_object_refs: Vec<Primitive> = pages
            .iter()
            .map(|page: &&Object| Primitive::Ref(*page))
            .collect();

        // let pages = Object::new(Primitive::Dictionary(vec![
        //     Primitive::pair(Primitive::name("Type"), Primitive::name("Pages")),
        //     Primitive::pair(Primitive::name("Count"), Primitive::Number(page_object_refs.len() as u64)),
        //     Primitive::pair(Primitive::name("Kids"), Primitive::Array(page_object_refs)),
        // ]));

        // let catalog = Object::new(Primitive::Dictionary(vec![
        //     Primitive::pair(Primitive::name("Type"), Primitive::name("Catalog")),
        //     Primitive::pair(Primitive::name("Pages"), Primitive::Ref(&pages)),
        // ]));
        
        // self.objects.push(catalog);
        // self.objects.push(pages);
        
        self.resolve();
        println!("{:#?}", self.objects);

        let mut obj = Primitive::Array(vec![]);
        unsafe {
            println!("{:#?}", obj.as_mut_ptr());
            println!("{:#?}", &obj as *const Primitive);
        }

        let out: String = buf
            .iter()
            .map(|&c| if c.is_ascii() { c as char } else {'?'} )
            .collect();

        // println!("{}", out);

        Ok(())
    }
}

#[derive(Eq, Hash, Debug)]
struct Font {
    base: String,
    identifier: String,
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

#[derive(Debug)]
struct Resource {
    fonts: HashSet<Font>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            fonts: HashSet::new()
        }
    }

    pub fn add_font<S: Into<String>>(&mut self, base: S) {
        let base = base.into();
        if !self.fonts.contains(&Font {base: base.clone(), identifier: String::from("")}) {
            self.fonts.insert(Font{base: base.clone(), identifier: format!("F{}", self.fonts.len())});
        }
    }
}

impl Into<Object> for Resource {
    fn into(self) -> Object {
        Object::new(vec![
            Primitive::dictionary(
                self.fonts.into_iter().map(Into::into).collect()
            )]
        )
    }
}

impl Into<Primitive> for Font {
    fn into(self) -> Primitive {
        Primitive::pair(
            Primitive::name(self.identifier), 
            Primitive::dictionary(vec![
                Primitive::pair(Primitive::name("Type"), Primitive::name("Font")),
                Primitive::pair(Primitive::name("BaseFont"), Primitive::name(self.base)),
                Primitive::pair(Primitive::name("SubType"), Primitive::name("Type1"))
            ])
        )
    }
}

enum MediaBox {
    A4
}

impl Into<Primitive> for MediaBox {
    fn into(self) -> Primitive {
        Primitive::Array(
            match self {
                MediaBox::A4 => [0, 0, 595, 842].into_iter().map(Primitive::Number).collect()
            }
        )
    }
}

struct Contents {

}

impl Contents {
    pub fn new() -> Contents {
        Contents {

        } 
    }
}

struct Page {
    resource: Resource,
    mediabox: MediaBox,
    contents: Contents,
}

impl Page {
    pub fn new(mediabox: MediaBox) -> Page {
        Page {
            resource: Resource::new(),
            contents: Contents::new(),
            mediabox,
        }
    }
}

impl Into<Vec<Object>> for Page {
    fn into(self) -> Vec<Object> {
        let resource: Object = self.resource.into();
        let pages = Object::new(Primitive::Dictionary(
            vec![
                Primitive::pair(Primitive::name("Type"), Primitive::name("Page")),
                Primitive::pair(Primitive::name("Parent"), Primitive::ParentRef),
                Primitive::pair(Primitive::name("Resource"), Primitive::Ref(&resource)),
                Primitive::pair(Primitive::name("MediaBox"), self.mediabox.into()),
            ]
        ));


        println!("resource ref {:?}", &resource as *const Object);

        vec![resource, pages]
    }
}

fn main() {
    let mut doc: Document = Document::new();

    let mut page = Page::new(MediaBox::A4);
    let mut page: Vec<Object> = page.into();

    doc.appendix(page);

    let mut page = Page::new(MediaBox::A4);
    let mut page: Vec<Object> = page.into();

    doc.appendix(page);

    doc.encode();
}