use std::rc::Rc;
use std::cell::RefCell;
use std::pin::Pin;
use std::marker::PhantomPinned;
use either::*;

pub struct SharedPin<T> {
    inner: Rc<RefCell<Pin<Box<T>>>>,
    _pinned: PhantomPinned,
}

impl<T> SharedPin<T> {
    pub fn new(inner: T) -> SharedPin<T> {
        SharedPin {
            inner: Rc::new(RefCell::new(Box::pin(inner))),
            _pinned: PhantomPinned,
        }
    }

    pub fn fmap<R>(&mut self, f: impl Fn(Pin<&mut T>) -> R) -> R {
        f(self.inner.borrow_mut().as_mut())
    }

    pub fn clone(&mut self) -> SharedPin<T> {
        SharedPin {
            inner: self.inner.clone(),
            _pinned: PhantomPinned,
        }
    }
}

enum Primitive {

}

pub struct Object {

}

impl Object {
    pub fn new<P: Into<Vec<Primitive>>>(inner: P) -> Object {
        Object {
            
        }
    }
}

pub struct Entity {
    object: Either<Object, SharedPin<Object>>,
}

impl Entity {
    pub fn new<P: Into<Vec<Primitive>>>(inner: P) -> Entity {
        Entity {
            object: Left(Object::new(inner)),
        }
    }

    pub fn fmap<R>(&mut self, f: impl Fn(Pin<&mut Object>) -> R) -> R {
        match &mut self.object {
            Left(ref mut obj) => f(Pin::new(obj)),
            Right(ref mut pin) => pin.fmap(f)
        }
    }

    // pub fn as_ref(&mut self) -> Pin<Rc<Object>> {

    // }
}