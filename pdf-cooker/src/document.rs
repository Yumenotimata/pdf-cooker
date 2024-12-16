use std::collections::{HashMap, HashSet};
use either::*;

use crate::object::*;
use crate::prim::*;
use crate::page::*;

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
}