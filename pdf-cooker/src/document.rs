use std::collections::{HashMap, HashSet};
use either::*;

use crate::object::*;
use crate::prim::*;

#[derive(Debug)]
pub struct Document {
    objects: Vec<Object>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            objects: vec![]
        }
    }

    pub fn append(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn appendix<V: Into<Vec<Object>>>(&mut self, objects: V) {
        self.objects.extend(objects.into());
    }

    pub fn resolve(&mut self) {
        type Number = u64;
        let mut query: HashMap<*const RawObject, Number> = HashMap::new(); 
        for (number, object) in self.objects.iter_mut().enumerate() {
            object.fmap(|_, opt| *opt = Some(number as u64));
            if matches!(object.inner, Right(_)) {
                query.insert(object.as_ref(), number as u64);
            }
        }

        self.objects.iter_mut().map(|object| unsafe { object.get_unchecked_mut().iter_mut() }).flatten().for_each(|prim| {
            if let Primitive::Pair(_, ref mut value) = prim {
                if let Primitive::Ref(ptr) = value.as_ref() {
                    **value = Primitive::Solved(*query.get(&ptr).unwrap());
                }
            }
        });

        // self.objects.iter_mut().for_each(|object| object.fmap(|prims, _| {
        //     prims.iter_mut().map(|prim| prim.iter_mut()).flatten().for_each(|prim| {
        //         if let Primitive::Pair(_, ref mut value) = prim {
        //             if let Primitive::Ref(ptr) = value.as_ref() {
        //                 **value = Primitive::Solved(*query.get(&ptr).unwrap());
        //             }
        //         }
        //     });
        // }));
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
pub enum MediaBox {
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
pub struct Page {
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

impl Into<Vec<Object>> for Page {
    fn into(self) -> Vec<Object> {
        let mut resource: Object = self.resource.into();
        let pages = Object::new(Primitive::Dictionary(
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