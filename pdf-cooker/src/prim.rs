use crate::object::*;

#[derive(Debug)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Map(Vec<Primitive>),
    Number(u64),
    Name(String),
    ParentRef,
    Pair(Box<Primitive>, Box<Primitive>),
    Defer(*const RawObject),
    Ref(u64),
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

pub struct PrimitiveMutIterator<'a> {
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
                    Primitive::Map(ref mut dictionary) => {
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