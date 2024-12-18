use bytes::{BufMut, BytesMut};
use std::collections::HashMap;

use crate::object::*;
use crate::prim::*;

#[derive(Debug)]
pub struct Document {
    objects: Vec<Object>,
}
use std::fmt::Write;
impl Document {
    pub fn new() -> Document {
        Document {
            objects: vec![]
        }
    }

    pub fn encode(&mut self) {
        let mut bytes = BytesMut::new();
        bytes.write_str("%PDF-1.7\n").unwrap();
        bytes.extend_from_slice(&[0xe2, 0xe3, 0xcf, 0xd3, '\n' as u8]);

        let page_ref: Vec<&mut Object> = self.objects
            .iter_mut()
            .filter(|obj| obj.project_ref().iter().any(|prim| prim.is_type("Page")))
            .collect();
        
        let mut pages = Object::new(map![
            ("Type", "Pages"),
            ("Count", page_ref.len()),
            ("Kids", page_ref)
        ]);

        let catalog = Object::new(map![
            ("Type", "Catalog"),
            ("Pages", &mut pages)
        ]);

        self.objects.push(catalog);
        self.objects.push(pages);

        Document::resolve(&mut self.objects);

        // println!("{:#?}", self.objects);
        for obj in self.objects.iter_mut() {
            println!("{} 0 obj", obj.project().number.unwrap());
            for prim in obj.project().iter_mut() {
                println!("{}", prim);
            }
        }
        
        // bytes.iter().for_each(|&c| if c.is_ascii() { print!("{}", c as char); } else { print!("{}, ", c); });
    }

    fn resolve(objects: &mut Vec<Object>) {
        let mut query: HashMap<*const RawObject, u64> = HashMap::new();

        for (number, ref mut obj) in objects.iter_mut().enumerate() {
            obj.project().number = Some(number as u64 + 1);
            if matches!(obj, Object::Ref(_)) {
                query.insert(obj.as_ptr(), number as u64 + 1);
            }
        }

        for obj in objects.iter_mut() {
            for prim in obj.project().iter_mut() {
                for p in prim.iter_mut() {
                    if let Primitive::Defer(ptr) = p {
                        *p = Primitive::Ref(*query.get(ptr).unwrap());
                    }
                }
            }
        }
    }

    pub fn append(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn appendix<V: Into<Vec<Object>>>(&mut self, objects: V) {
        self.objects.extend(objects.into());
    }
}