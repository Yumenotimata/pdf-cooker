use std::pin::Pin;
use pin_project::{pin_project, pinned_drop};
use either::*;
use crate::utils::*;

#[derive(Debug)]
pub enum Primitive {

}

#[derive(Debug)]
pub struct RawObject {
    inner: Vec<Primitive>,
    number: Option<u64>,    
}

impl RawObject {
    pub fn new<V: Into<Vec<Primitive>>>(inner: V) -> RawObject {
        RawObject {
            inner: inner.into(),
            number: None,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    inner: Either<RawObject, Fix<RawObject>>,
}

impl Object {
    pub fn new<V: Into<Vec<Primitive>>>(inner: V) -> Object {
        Object {
            inner: Left(RawObject::new(inner)),
        }
    }

    pub fn fmap<R>(&mut self, f: impl Fn(&mut Option<u64>) -> R) -> R {
        match self.inner {
            Left(ref mut raw) => f(&mut raw.number),
            Right(ref mut fix) => fix.fmap(|proj| f(&mut proj.inner.number))
        }
    }

    pub fn as_ref(&mut self) -> *const RawObject {
        if let Left(ref mut raw) = self.inner {
            let raw = std::mem::replace(raw, RawObject::new(vec![]));
            self.inner = Right(Fix::new(raw))
        }
        
        if let Right(ref mut fix) = self.inner {
            let this = fix.as_mut();
            return unsafe { &this.get_unchecked_mut().inner };
        }

        unreachable!()
    }
}