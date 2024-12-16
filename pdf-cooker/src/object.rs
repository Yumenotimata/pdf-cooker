use crate::utils::*;
use crate::prim::*;

use std::ops::{Deref, DerefMut};
use either::*;

#[derive(Debug)]
pub struct RawObject {
    inner: Vec<Prim>,
    number: Option<u64>,
}

impl RawObject {
    fn new<V: Into<Vec<Prim>>>(inner: V) -> RawObject {
        RawObject {
            inner: inner.into(),
            number: None
        }
    }
}

#[derive(Debug)]
pub enum Object {
    Ind(RawObject),
    Ref(Fix<RawObject>),
}

impl Object {
    pub fn new<V: Into<Vec<Prim>>>(inner: V) -> Object {
        Object::Ind(RawObject::new(inner))
    }

    pub fn fmap<R>(&mut self, f: impl FnOnce(&mut Vec<Prim>, &mut Option<u64>) -> R) -> R {
        match self {
            Object::Ind(ref mut raw) => f(&mut raw.inner, &mut raw.number),
            Object::Ref(ref mut fix) => fix.fmap(|proj| f(&mut proj.inner.inner, &mut proj.inner.number))
        }
    }

    pub fn as_ref(&mut self) -> *const RawObject {
        if let Object::Ind(ref mut raw) = self {
            let raw = std::mem::replace(raw, RawObject::new(vec![]));
            *self = Object::Ref(Fix::new(raw));
        }

        if let Object::Ref(ref mut fix) = self {
            let this = fix.as_mut();
            return unsafe {&this.get_unchecked_mut().inner};
        }

        unreachable!()
    }
}