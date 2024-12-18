use crate::fix::*;
use crate::prim::*;

use std::ops::{Deref, DerefMut};

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
pub enum Object {
    Ind(RawObject),
    Ref(Fix<RawObject>)
}

impl Object {
    pub fn new(prim: impl Into<Vec<Primitive>>) -> Object {
        Object::Ind(RawObject::new(prim))
    }

    pub fn as_ptr(&mut self) -> *const RawObject {
        if let Object::Ind(ref mut raw) = self {
            let raw = std::mem::replace(raw, RawObject { inner: vec![], number: None });
            *self = Object::Ref(Fix::new(raw));
        }

        if let Object::Ref(ref mut fix) = self {
            return unsafe { &fix.as_mut().get_unchecked_mut().inner };
        }

        unreachable!()
    }

    pub fn fmap<R>(&mut self, f: &impl Fn(&mut Vec<Primitive>, &mut Option<u64>) -> R) -> R {
        match self {
            Object::Ind(raw) => f(&mut raw.inner, &mut raw.number),
            Object::Ref(fix) => Fix::fmap(fix, |proj| f(&mut proj.inner.inner, &mut proj.inner.number)),
        }
    }

    pub fn fmap_ref<R>(&self, f: impl for<'a, 'b> Fn(&'a Vec<Primitive>, &'b Option<u64>) -> R) -> R {
        match self {
            Object::Ind(raw) => f(&raw.inner, &raw.number),
            Object::Ref(fix) => Fix::fmap_ref(fix, |proj| f(&proj.inner.inner, &proj.inner.number)),
        }
    }
    
    pub fn project(&mut self) -> RawFixProj<RawObject> {
        match self {
            Object::Ind(raw) => RawFixProj { inner: raw },
            Object::Ref(fix) => fix.as_mut().project()
        }
    }

    pub fn project_ref(&self) -> RawFixProjRef<RawObject> {
        match self {
            Object::Ind(raw) => RawFixProjRef { inner: raw },
            Object::Ref(fix) => fix.as_ref().project_ref()
        }
    }
}

impl Into<Primitive> for &mut Object {
    fn into(self) -> Primitive {
        Primitive::Defer(self.as_ptr())
    }
}

impl<'a> From<&'a mut Object> for Vec<&'a Object> {
    fn from(obj: &'a mut Object) -> Vec<&'a Object> {
        vec![obj]
    }
}
