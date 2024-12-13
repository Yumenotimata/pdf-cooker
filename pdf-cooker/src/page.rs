use std::collections::{HashMap, HashSet};

use crate::document::*;
use crate::object::*;
use crate::prim::*;


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
        let pages = Object::new(Primitive::Map(
            vec![
                Pair::new(Primitive::name("Type"), Primitive::name("Page")),
                Pair::new(Primitive::name("Parent"), Primitive::ParentRef),
                Pair::new(Primitive::name("Resource"), Primitive::Defer(resource.as_ref())),
                Pair::new(Primitive::name("MediaBox"), self.mediabox.into()),
            ]
        ));
        
        vec![resource, pages]
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
            Primitive::Map(
                self.fonts.into_iter().map(Into::into).collect()
            )]
        )
    }
}

impl Into<Pair> for Font {
    fn into(self) -> Pair {
        Pair::new(
            Primitive::name(self.identifier), 
            Primitive::Map(vec![
                Pair::new(Primitive::name("Type"), Primitive::name("Font")),
                Pair::new(Primitive::name("BaseFont"), Primitive::name(self.base)),
                Pair::new(Primitive::name("SubType"), Primitive::name("Type1"))
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
                MediaBox::A4 => [0, 0, 595, 842]
            }.into_iter().map(Primitive::Number).collect()
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