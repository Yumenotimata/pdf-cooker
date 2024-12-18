use crate::object::*;

#[derive(Debug)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Defer(*const RawObject),
    Name(String),
    Number(u64),
    Map(Vec<Primitive>),
    Pair(String, Box<Primitive>),
    Ref(u64),
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

impl Into<Primitive> for i32 {
    fn into(self) -> Primitive {
        Primitive::Number(self as u64)
    }
}

impl Into<Primitive> for usize {
    fn into(self) -> Primitive {
        Primitive::Number(self as u64)
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

impl<T> From<Vec<T>> for Primitive where T: Into<Primitive> {
    fn from(target: Vec<T>) -> Primitive {
        Primitive::Array(target.into_iter().map(|t| t.into()).collect())
    }
}

macro_rules! indent {
    ($num:expr) => {
        if ($num >= 0) {
            "  ".repeat($num as usize)
        } else {
            "".to_string()
        }
    };
}

impl Primitive {
    pub fn iter_mut(&mut self) -> PrimitiveIterMut {
        PrimitiveIterMut {
            stack: vec![self]
        }
    }

    pub fn is_type(&self, ty: &str) -> bool {
        match self {
            Primitive::Array(array) => array.iter().any(|elm| Primitive::is_type(elm, ty)),
            Primitive::Map(pairs) => pairs.iter().any(|pair| pair.is_type(ty)),
            Primitive::Pair(key, ref value) => {
                if key == &String::from("Type") {
                    if let Primitive::Name(ref value) = **value {
                        if value == &String::from(ty) {
                            return true;
                        }
                    }
                }

                return false;
            },
            _ => false,
        }
    }

    pub fn encode(&self, indent: usize, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Primitive::Array(array) => write!(f, "[{}]", array.iter().map(|elm| format!("{}", elm)).collect::<Vec<_>>().join(" "))?,
            Primitive::Map(pairs) => {
                write!(f, "<<\n")?;
                for pair in pairs.iter() {
                    pair.encode(indent + 1, f)?;
                }
                write!(f, "{}>>", indent!(indent as i32 - 1))?;
            },
            Primitive::Pair(key, value) => {
                write!(f, "{}/{} ", indent!(indent), key)?;
                value.encode(indent + 1, f)?;
                write!(f, "\n")?;
            },
            Primitive::Name(name) => write!(f, "/{}", name)?,
            Primitive::Number(num) => write!(f, "{}", num)?,
            Primitive::Ref(number) => write!(f, "{} 0 R", number)?,
            _ => {},
        }

        Ok(())
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // match self {
        //     Primitive::Array(array) => write!(f, "[{}]", array.iter().map(|elm| format!("{}", elm)).collect::<Vec<_>>().join(" ")),
        //     Primitive::Map(pairs) => write!(f, "<<{}\n>>", pairs.iter().map(|pair| format!("{}", pair)).collect::<Vec<_>>().join(" ")),
        //     Primitive::Pair(key, value) => write!(f, "\n/{} {}", key, value),
        //     Primitive::Name(name) => write!(f, "/{}", name),
        //     Primitive::Number(num) => write!(f, "{}", num),
        //     Primitive::Ref(number) => write!(f, "{} 0 R", number),
        //     _ => Ok(()),
        // }
        self.encode(0, f)
    }
}

pub struct PrimitiveIterMut<'a> {
    stack: Vec<&'a mut Primitive>,
}

impl<'a> Iterator for PrimitiveIterMut<'a> {
    type Item = &'a mut Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            match current {
                Primitive::Array(array) => {self.stack.extend(array.iter_mut()); continue},
                Primitive::Map(pairs) => {self.stack.extend(pairs.iter_mut()); continue},
                Primitive::Pair(_, value) => {self.stack.push(value); continue;},
                _ => return Some(current)
            }
        }
        None
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
    (@vec $vec:expr) => {
        Primitive::Map($vec)
    };
}