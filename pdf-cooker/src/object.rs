use crate::fix::*;
use crate::prim::*;

#[derive(Debug)]
pub struct RawObject {
    pub inner: Vec<Primitive>,
    pub number: Option<u64>,
}

impl RawObject {
    pub fn new(prim: impl Into<Vec<Primitive>>) -> RawObject {
        RawObject {
            inner: prim.into(),
            number: None,
        }
    }
}

#[derive(Debug)]
pub enum Object {
    Ind(RawObject),
    Ref(Fix<RawObject>)
}

impl Object {
    pub fn new(prim: impl Into<Vec<Primitive>>) -> Object {
        Object::Ind(RawObject::new(prim))
    }

    fn as_ptr(&mut self) -> *const RawObject {
        if let Object::Ind(ref mut raw) = self {
            let raw = std::mem::replace(raw, RawObject { inner: vec![], number: None });
            *self = Object::Ref(Fix::new(raw));
        }

        if let Object::Ref(ref mut fix) = self {
            return unsafe { &fix.as_mut().get_unchecked_mut().inner };
        }

        unreachable!()
    }
}

impl Into<Primitive> for &mut Object {
    fn into(self) -> Primitive {
        Primitive::Defer(self.as_ptr())
    }
}