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
        // let pages = Object::new(Prim::map(
        //     vec![
        //         Prim::pair("Type", "Page"),
        //         // Prim::pair("Parent", Prim::ParentRef),
        //         Prim::pair("Resource", Prim::defer(resource.as_ref())),
        //         Prim::pair("MediaBox", self.mediabox),
        //     ]
        // ));
        
        // vec![resource, pages]
        todo!()
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
            Prim::map(
                self.fonts.into_iter().map(|f| f.into()).collect::<Vec<Pair>>()
            )
        ])
    }
}

impl Into<Pair> for Font {
    fn into(self) -> Pair {
        // Prim::pair(
        //     self.identifier, 
        //     Prim::map(vec![
        //         Prim::pair("Type", "Font"),
        //         Prim::pair("BaseFont", self.base),
        //         Prim::pair("SubType", "Type1")
        //     ])
        // )
        todo!()
    }
}

#[derive(Debug)]
pub enum MediaBox {
    A4
}

impl Into<Prim> for MediaBox {
    fn into(self) -> Prim {
        // Prim::array(
        //     match self {
        //         MediaBox::A4 => [0, 0, 595, 842]
        //     }.into_iter().map(Prim::number).collect::<Vec<Prim>>()
        // )
        todo!()
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