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

impl<'a> Into<Vec<Object>> for Page {
    fn into(self) -> Vec<Object> {
        let mut resource: Object = self.resource.into();

        let pages = Object::new(map![
            ("Type", "Page"),
            ("Resource", &mut resource),
            ("MediaBox", self.mediabox)
        ]);
        
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
        if !self.fonts.contains(&Font { base: base.clone(), identifier: String::from("") }) {
            self.fonts.insert(Font { base: base.clone(), identifier: format!("F{}", self.fonts.len()) });
        }
    }
}

impl<'a> Into<Object> for Resource {
    fn into(self) -> Object {
        Object::new(
            self.fonts.into_iter().map(Into::into).collect::<Vec<Primitive>>()
        )
    }
}

impl<'a> Into<Primitive> for Font {
    fn into(self) -> Primitive {
        map![(self.identifier, map![
            ("Type", "Font"),
            ("BaseFont", self.base),
            ("SubType", "Type1")
        ])]
    }
}

#[derive(Debug)]
pub enum MediaBox {
    A4
}

impl Into<Primitive> for MediaBox {
    fn into(self) -> Primitive {
        match self {
            MediaBox::A4 => array![0, 0, 595, 842]
        }
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