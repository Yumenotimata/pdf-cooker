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

    pub fn flatten(&mut self, f: impl Fn(&mut Primitive)) {
        self.objects.iter_mut().for_each(|object| object.fmap(|prims, _| {
            prims.iter_mut().map(|prim| prim.iter_mut()).flatten().for_each(|prim| {
                f(prim);
            });
        }));
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

        // self.objects.iter_mut().for_each(|obj| obj.fmap(|prims, _| {
        //     prims.iter_mut().for_each(|prim| {
        //         println!("{}", prim);
        //     });
        // }));

        self.flatten(|prim| {
            if let Primitive::Pair(_, ref mut value) = prim {
                let mut value = value.borrow_mut();
                if let Primitive::Defer(ptr) = *value {
                    *value = Primitive::Ref(*query.get(&ptr).unwrap());
                }
            }
        });
    }
}
