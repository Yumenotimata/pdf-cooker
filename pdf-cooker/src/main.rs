use std::collections::{HashSet, HashMap};
use std::marker::PhantomPinned;
use std::pin::Pin;
use pin_project::pin_project;
use std::mem;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Dictionary(Vec<Primitive>),
    Number(u64),
    Name(String),
    ParentRef,
    Pair(Box<Primitive>, Box<Primitive>),
    Ref(Rc<RefCell<Pin<Box<Object>>>>),
    Solved(u64),
    Stream(String),
}

impl Primitive {
    pub fn name<S: Into<String>>(name: S) -> Self {
        Primitive::Name(name.into())
    }

    pub fn pair(key: Primitive, value: Primitive) -> Self {
        Primitive::Pair(Box::new(key), Box::new(value))
    }

    pub fn iter_mut(&mut self) -> PrimitiveMutIterator {
        PrimitiveMutIterator {
            stack: vec![self]
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut Primitive {
        self as *mut Primitive
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

struct SharedPin<T> {
    inner: Rc<RefCell<Pin<Box<T>>>>,
}

impl<T> SharedPin<T> {
    pub fn new(inner: T) -> Self {
        SharedPin {
            inner: Rc::new(RefCell::new(Box::pin(inner)))
        }
    }

    pub fn fmap<F: Fn(&mut T)>(&mut self, f: F) {
        
    }
}

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

#[derive(Debug)]
enum ObjectProj {
    Pinned(Rc<RefCell<Pin<Box<Object>>>>),
    Unpinned(Object)
}

impl ObjectProj {
    pub fn map<R>(&mut self, f: impl Fn(&mut Pin<&mut Object>) -> R) -> R {
        match self {
            ObjectProj::Pinned(pinned) => {
                let mut this = pinned.borrow_mut();
                let mut this = this.as_mut();
                f(&mut this)
            },
            ObjectProj::Unpinned(unpinned) => {
                f(&mut Pin::new(unpinned))
            }
        }
    }
}

#[derive(Debug)]
struct Entity {
    inner: ObjectProj,
    _pinned: PhantomPinned,
}

impl Entity {
    pub fn new<V: Into<Vec<Primitive>>>(inner: V) -> Self {
        Entity {
            inner: ObjectProj::Unpinned(Object::new(inner)),
            _pinned: PhantomPinned,
        }
    }

    pub fn as_ref(&mut self) -> Rc<RefCell<Pin<Box<Object>>>> {
        if let ObjectProj::Unpinned(ref mut x) = self.inner {
            let obj = std::mem::replace(x, Object { inner: vec![], number: None });
            self.inner = ObjectProj::Pinned(Rc::new(RefCell::new(Box::pin(obj))));
        }
    
        if let ObjectProj::Pinned(ref mut pinned) = self.inner {
            return pinned.clone();
        }
    
        unreachable!();
    }
}

struct Document {
    entity: Vec<Entity>,
}

impl IntoIterator for Primitive {
    type Item = Primitive;
    type IntoIter = PrimitiveIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Primitive::Array(array) => PrimitiveIntoIterator::Array(array.into_iter()),
            Primitive::Dictionary(dictionary) => PrimitiveIntoIterator::Dictionary(dictionary.into_iter()),
            _ => PrimitiveIntoIterator::Empty,
        }
    }
}

pub enum PrimitiveIntoIterator {
    Array(std::vec::IntoIter<Primitive>),
    Dictionary(std::vec::IntoIter<Primitive>),
    Empty,
}

impl Iterator for PrimitiveIntoIterator {
    type Item = Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PrimitiveIntoIterator::Array(iter) => iter.next(),
            PrimitiveIntoIterator::Dictionary(iter) => iter.next(),
            PrimitiveIntoIterator::Empty => None,
        }
    }
}

impl Document {
    pub fn new() -> Self {
        Document {
            entity: vec![],
        }
    }
    pub fn append<E: Into<Entity>>(&mut self, entity: E) {
        self.entity.push(entity.into());
    }

    pub fn appendix<E: Into<Vec<Entity>>>(&mut self, entities: E) {
        self.entity.extend(entities.into());
    }

    pub fn resolve(&mut self) {
        for (number, entity) in self.entity.iter_mut().enumerate() {
            entity.inner.map(|obj| {
                obj.number = Some(number as u64)
            });
        }

        for (number, entity) in self.entity.iter_mut().enumerate() {
            entity.inner.map(|pin| {
                pin.inner.iter_mut().map(|x| x.iter_mut()).flatten().for_each(|prim| {
                    match prim {
                        Primitive::Pair(_, ref mut value) => {
                            if let Primitive::Ref(ref reference) = value.as_ref() {
                                let number: u64;
                                {
                                    let mut this = reference.borrow_mut();
                                    let this = this.as_mut();
                                    unsafe {
                                        let this = this.get_unchecked_mut();
                                        number = this.number.unwrap();
                                    }
                                }
                            *prim = Primitive::Solved(number);
                            }
                        },
                        _ => {}
                    }
                });
                // }
            });
            // match &mut entity.inner {
            //     ObjectProj::Pinned(pinned) => {
            //         let mut this = pinned.borrow_mut();
            //         let mut this = this.as_mut();
            //         unsafe {
            //             let mut this = this.get_unchecked_mut();
            //             this.inner.iter_mut().map(|x| x.iter_mut()).flatten().for_each(|prim| {
            //                 match prim {
            //                     Primitive::Pair(_, ref mut value) => {
            //                         if let Primitive::Ref(ref reference) = value.as_ref() {
            //                             let number: u64;
            //                             {
            //                                 let mut this = reference.borrow_mut();
            //                                 let mut this = this.as_mut();
            //                                 // unsafe {
            //                                     let this = this.get_unchecked_mut();
            //                                     number = this.number.unwrap();
            //                                 // }
            //                             }
            //                             *prim = Primitive::Solved(number);
            //                         }
            //                     }
            //                     _ => {},
            //                 }
            //             });
            //         }
            //     },
            //     ObjectProj::Unpinned(unpinned) => {
            //         unpinned.inner.iter_mut().map(|x| x.iter_mut()).flatten().for_each(|prim| {
            //             match prim {
            //                 Primitive::Pair(_, ref mut value) => {
            //                     if let Primitive::Ref(ref reference) = value.as_ref() {
            //                         let number: u64;
            //                         {
            //                             let mut this = reference.borrow_mut();
            //                             let mut this = this.as_mut();
            //                             unsafe {
            //                                 let this = this.get_unchecked_mut();
            //                                 number = this.number.unwrap();
            //                             }
            //                         }
            //                         *prim = Primitive::Solved(number);
            //                     }
            //                 }
            //                 _ => {},
            //             }
            //         });
            //     }
            // }
        }

        // for (number, entity) in self.entity.iter_mut().enumerate() {
        //     match &mut entity.inner {
        //         ObjectProj::Pinned(pinned) => {

        //         },
        //         ObjectProj::Unpinned(unpinned) => {
        //             // unpinned.inner.iter_mut().for_each(|f| {
        //             //     f.flatten();
        //             // });
        //             // let m: Vec<Primitive> = unpinned.inner.into_iter().flat_map(|x| x).collect();
        //             // println!("{:#?}", m);
        //             unpinned.inner.iter_mut().map(|iter| iter.iter_mut()).flatten().for_each(|obj| {
        //                 println!("-> {:#?}", obj);
        //             });
        //             // unpinned.number = Some(number as u64);
        //             // unpinned.inner.iter_mut().for_each(|prim: &mut Primitive| {
        //             //     prim.iter_mut().for_each(|atm: &mut Primitive| {
        //             //         println!("unpined {:#?}", atm);
        //             //         match atm {
        //             //             Primitive::Pair(ref mut key, ref mut value) => {
        //             //                 if let Primitive::Ref(ref reference) = value.as_ref() {
        //             //                     let reference: &Rc<RefCell<Pin<Box<Object>>>> = reference;
        //             //                     let number: u64;
        //             //                     {
        //             //                         let mut this = reference.borrow_mut();
        //             //                         let mut this = this.as_mut();
        //             //                         unsafe {
        //             //                             let mut this = this.get_unchecked_mut();
        //             //                             number = this.number.unwrap();
        //             //                         }
        //             //                     }
        //             //                     // let mut this = this.as_mut();
        //             //                     // unsafe {
        //             //                     //     let mut this = this.get_unchecked_mut();
        //             //                     //     // *value = Box::new(Primitive::Number(this.number.unwrap()));
        //             //                     //     // std::mem::replace(&mut value, &mut Box::new(Primitive::Number(this.number.unwrap())));
        //             //                     // }
        //             //                     *atm = Primitive::Number(number);
        //             //                 }
        //             //             },
        //             //             _ => {}
        //             //         }
        //             //     });
        //             // });
        //         }
        //     }
        // }
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

impl Into<Entity> for Resource {
    fn into(self) -> Entity {
        Entity::new(vec![
            Primitive::Dictionary(
                self.fonts.into_iter().map(Into::into).collect()
            )]
        )
    }
}

impl Into<Primitive> for Font {
    fn into(self) -> Primitive {
        Primitive::pair(
            Primitive::name(self.identifier), 
            Primitive::Dictionary(vec![
                Primitive::pair(Primitive::name("Type"), Primitive::name("Font")),
                Primitive::pair(Primitive::name("BaseFont"), Primitive::name(self.base)),
                Primitive::pair(Primitive::name("SubType"), Primitive::name("Type1"))
            ])
        )
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Contents {

}

impl Contents {
    pub fn new() -> Contents {
        Contents {

        } 
    }
}

#[derive(Debug)]
struct Page {
    resource: Resource,
    mediabox: MediaBox,
    contents: Contents,
}

impl Page {
    pub fn new(mediabox: MediaBox) -> Self {
        Page {
            resource: Resource::new(),
            contents: Contents::new(),
            mediabox,
        }
    }
}

impl Into<Vec<Entity>> for Page {
    fn into(self) -> Vec<Entity> {
        let mut resource: Entity = self.resource.into();
        let pages = Entity::new(Primitive::Dictionary(
            vec![
                Primitive::pair(Primitive::name("Type"), Primitive::name("Page")),
                Primitive::pair(Primitive::name("Parent"), Primitive::ParentRef),
                Primitive::pair(Primitive::name("Resource"), Primitive::Ref(resource.as_ref())),
                Primitive::pair(Primitive::name("MediaBox"), self.mediabox.into()),
            ]
        ));
        
        vec![resource, pages]
    }
}

fn main() {
    let mut doc = Document::new();
    let mut page = Page::new(MediaBox::A4);
    doc.appendix(page);
    let mut page = Page::new(MediaBox::A4);
    doc.appendix(page);

    doc.resolve();

    println!("{:#?}", doc.entity);
}