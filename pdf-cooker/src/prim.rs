use std::cell::RefCell;
use std::rc::Rc;

use crate::object::*;

#[derive(Debug, Clone)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Map(Vec<Pair>),
    Number(u64),
    Name(String),
    ParentRef,
    Pair(Box<Primitive>, Box<Primitive>),
    Defer(*const RawObject),
    Ref(u64),
    Stream(String),
}

#[derive(Debug, Clone)]
pub struct Pair(Primitive);
pub struct Name(Primitive);

impl Name {
    pub fn new<N: Into<String>>(name: N) -> Name {
        Name(Primitive::Name(name.into())) 
    }
}

impl Into<Primitive> for Name {
    fn into(self) -> Primitive {
        self.0
    }
}

impl Pair {
    pub fn new<P: Into<Primitive>>(key: Name, value: P) -> Pair {
        Pair(Primitive::Pair(Box::new(key.0), Box::new(value.into())))
    }
}

impl AsMut<Primitive> for Pair {
    fn as_mut(&mut self) -> &mut Primitive {
        &mut self.0
    }
}

impl Primitive {
    pub fn name<S: Into<String>>(name: S) -> Self {
        Primitive::Name(name.into())
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

impl AsMut<Primitive> for Primitive {
    fn as_mut(&mut self) -> &mut Primitive {
        self
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Primitive::Array(array) => write!(f, "[{}]", array.iter().map(|elm| format!("{}", elm)).collect::<Vec<_>>().join(" ")),
            // Primitive::Map(map) => write!(f, "<<{}\n>>", map.iter().map(|pair| format!("{}", pair)).collect::<String>()),
            Primitive::Number(number) => write!(f, "{}", number),
            Primitive::Pair(key, value) => write!(f, "\npair"),
            _ => Ok(())
        }
    }
}

pub struct PrimitiveMutIterator<'a> {
    stack: Vec<&'a mut (dyn AsMut<Primitive> + 'a)>,
}

impl<'a> Iterator for PrimitiveMutIterator<'a> {
    type Item = &'a mut Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ref mut current) = self.stack.pop() {
            unsafe {
                match *current.as_mut().as_mut_ptr() {
                    Primitive::Map(ref mut pairs) => {
                        pairs.iter_mut().for_each(|pair| {
                            self.stack.push(pair);
                        });
                    },
                    // self.stack.extend(pairs.iter_mut().map(Primitive::Pair)),
                    Primitive::Array(ref mut array) => {
                        array.iter_mut().for_each(|elm| {
                            self.stack.push(elm);
                        });
                    },
                    // self.stack.extend(array.iter_mut()),
                    _ => {}
                }

                return Some(&mut *current.as_mut().as_mut_ptr());
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