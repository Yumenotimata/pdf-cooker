use std::ops::{Deref, DerefMut};

use crate::object::RawObject;

#[derive(Debug)]
pub enum RawPrim {
    Array(Vec<RawPrim>),
    Map(Vec<Pair>),
    PairElm(String, Box<RawPrim>),
    Name(String),
    Number(u64),
    Ref(u64),
    Stream(String),
    Defer(*const RawObject),
}

impl RawPrim {
    pub fn as_mut_ptr(&mut self) -> *mut RawPrim {
        self as *mut RawPrim
    }

    pub fn concat(&mut self, f: &impl Fn(&mut RawPrim) -> bool) -> Vec<&mut RawPrim> {
        match self {
            RawPrim::Map(pairs) => pairs.iter_mut().map(|p| p.0.concat(f)).flatten().collect(),
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub struct Pair(RawPrim);

impl Pair {
    fn new(key: impl Into<String>, value: impl Into<RawPrim>) -> Pair {
        Pair(RawPrim::PairElm(key.into(), Box::new(value.into())))
    }
}

impl Deref for Pair {
    type Target = RawPrim;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<Vec<Pair>> for Pair {
    fn into(self) -> Vec<Pair> {
        vec![self]
    }
}

impl AsMut<RawPrim> for Pair {
    fn as_mut(&mut self) -> &mut RawPrim {
        &mut self.0
    }
}

impl<S, T> Into<Pair> for (S, T) where T: Into<Prim>, S: Into<String> {
    fn into(self) -> Pair {
        Prim::pair(self.0.into(), self.1.into())
    }
}

#[macro_export]
macro_rules! array {
    ($($elm:expr),*) => {
        {
            let mut v: Vec<RawPrim> = Vec::new();
            $(
                v.push($elm.into());
            )*
            Prim::array(v)
        }
    };
}

#[macro_export]
macro_rules! map {
    ($(($key:expr, $value:expr)),* $(,)?) => {
        {
            let mut pairs = Vec::new();
            $(
                pairs.push(Prim::pair($key, $value));
            )*
            Prim::map(pairs)
        }
    };
}

impl Into<RawPrim> for Prim {
    fn into(self) -> RawPrim {
        self.0
    }
}

// TODO:
// prevent temporary pub
#[derive(Debug)]
pub struct Prim(pub RawPrim);

impl Prim {
    pub fn array(elms: impl Into<Vec<RawPrim>>) -> Prim {
        Prim(RawPrim::Array(elms.into()))
    }

    pub fn map(pairs: impl Into<Vec<Pair>>) -> Prim {
        Prim(RawPrim::Map(pairs.into()))
    }

    pub fn number(num: u64) -> Prim {
        Prim(RawPrim::Number(num))
    }

    pub fn name(name: impl Into<String>) -> Prim {
        Prim(RawPrim::Name(name.into()))
    }

    pub fn pair(key: impl Into<String>, value: impl Into<RawPrim>) -> Pair {
        Pair::new(key, value.into())
    }

    pub fn defer(reference: *const RawObject) -> Prim {
        Prim(RawPrim::Defer(reference))
    }

    pub fn iter_mut(&mut self) -> ObjectIterMut {
        ObjectIterMut { stack: vec![self.as_mut()] }
    }

    pub fn concat(&mut self, f: &impl Fn(&mut RawPrim) -> bool) -> Vec<&mut RawPrim> {
        self.iter_mut().map(|elm| match elm {
            RawPrim::Array(array) =>  array.iter_mut().map(|a| a.concat(f)).flatten().collect(),
            RawPrim::Map(pairs) => pairs.iter_mut().map(|pair| pair.0.concat(f)).flatten().collect(),
            RawPrim::PairElm(_, value) => value.concat(f),
            _ => if f(elm) { vec![elm] } else { vec![] }
        }).flatten().collect()
    }
}

impl Into<Vec<Prim>> for Prim {
    fn into(self) -> Vec<Prim> {
        vec![self]
    }
}

impl AsMut<RawPrim> for Prim {
    fn as_mut(&mut self) -> &mut RawPrim {
        &mut self.0
    }
}

pub struct ObjectIterMut<'a> {
    stack: Vec<&'a mut RawPrim>,
}

impl<'a> Iterator for ObjectIterMut<'a> {
    type Item = &'a mut RawPrim;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ref mut current) = self.stack.pop() {
            unsafe {
                match *current.as_mut_ptr() {
                    RawPrim::Map(ref mut pairs) => {
                        for pair in pairs.iter_mut() {
                            self.stack.push(pair.as_mut())
                        }
                        continue;
                    },
                    RawPrim::Array(ref mut elms) => {
                        for elm in elms.iter_mut() {
                            self.stack.push(elm)
                        }
                        continue
                    }
                    _ => {}
                }

                return Some(&mut *current.as_mut_ptr())
            }
        }

        None
    }
}

impl Into<Prim> for String {
    fn into(self) -> Prim {
        Prim::name(self)
    }
}

impl Into<Prim> for &str {
    fn into(self) -> Prim {
        Prim::name(self)
    }
}

impl Into<Prim> for u64 {
    fn into(self) -> Prim {
        Prim::number(self)
    }
}

impl Into<RawPrim> for String {
    fn into(self) -> RawPrim {
        RawPrim::Name(self)
    }
}

impl Into<RawPrim> for &str {
    fn into(self) -> RawPrim {
        RawPrim::Name(self.to_string())
    }
}

impl Into<RawPrim> for u64 {
    fn into(self) -> RawPrim {
        RawPrim::Number(self)
    }
}
// impl<T> Into<Prim> for Vec<T> where T: Into<Prim> {
//     fn into(self) -> Prim {
//         Prim::array(self.into_iter().map(|v| v.into()).collect::<Vec<Prim>>())
//     }
// }