use crate::object::*;

#[derive(Debug)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Defer(*const RawObject),
    Name(String),
    Number(u64),
    Map(Vec<Primitive>),
    Pair(String, Box<Primitive>),
}

impl Into<Vec<Primitive>> for Primitive {
    fn into(self) -> Vec<Primitive> {
        vec![self]
    }
}

impl Into<Primitive> for u64 {
    fn into(self) -> Primitive {
        Primitive::Number(self)
    }
}

impl Into<Primitive> for &str {
    fn into(self) -> Primitive {
        Primitive::Name(self.to_string())
    }
}

impl Into<Primitive> for String {
    fn into(self) -> Primitive {
        Primitive::Name(self)
    }
}

impl Primitive {
    pub fn iter_mut(&mut self) -> PrimitiveIterMut {
        PrimitiveIterMut {
            stack: vec![self]
        }
    }
}

pub struct PrimitiveIterMut<'a> {
    stack: Vec<&'a mut Primitive>,
}

impl<'a> Iterator for PrimitiveIterMut<'a> {
    type Item = &'a mut Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[macro_export]
macro_rules! array {
    () => {
        Primitive::Array(Vec::new())
    };
    ($($elm:expr),*) => {
        Primitive::Array(vec![
            $(
                $elm.into()
            ),*
        ])
    };
    (@vec $vec:expr) => {
        Primitive::Array($vec)
    };
}

#[macro_export]
macro_rules! map {
    () => {
        Primitive::Map(Vec::new())
    };
    ($(($key:expr, $value:expr)),*) => {
        Primitive::Map(vec![
            $(
                Primitive::Pair(
                    $key.into(),
                    Box::new($value.into())
                )
            ),*
        ])
    };
}