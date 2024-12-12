use either::*;
use std::ops::{Deref, DerefMut};

use crate::utils::*;
use crate::prim::*;

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

impl Deref for RawObject {
    type Target = Vec<Primitive>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RawObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug)]
pub struct Object {
    pub inner: Either<RawObject, Fix<RawObject>>,
}

impl Object {
    pub fn new<V: Into<Vec<Primitive>>>(inner: V) -> Object {
        Object {
            inner: Left(RawObject::new(inner)),
        }
    }

    pub fn fmap<R>(&mut self, f: impl FnOnce(&mut Vec<Primitive>, &mut Option<u64>) -> R) -> R {
        match self.inner {
            Left(ref mut raw) => f(&mut raw.inner, &mut raw.number),
            Right(ref mut fix) => fix.fmap(|proj| f(&mut proj.inner.inner, &mut proj.inner.number))
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self) -> &mut RawObject {
        match self.inner {
            Left(ref mut raw) => raw,
            Right(ref mut fix) => {
                return unsafe { fix.as_mut().get_unchecked_mut() };
            } 
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

impl Deref for Object {
    type Target = Either<RawObject, Fix<RawObject>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}