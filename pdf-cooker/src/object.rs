use std::rc::Rc;
use std::cell::RefCell;
use std::pin::{Pin, pin};
use std::marker::PhantomPinned;
use either::*;
use std::ops::Deref;use std::ops::DerefMut;

#[derive(Debug)]
pub struct RawPin<T> {
    inner: T,
    _pinned: PhantomPinned,
}

impl<T> RawPin<T> {
    pub fn new(inner: T) -> RawPin<T> {
        RawPin {
            inner,
            _pinned: PhantomPinned,
        }
    }

    pub fn pin(inner: T) -> Rc<RefCell<Pin<Box<RawPin<T>>>>> {
        let obj = RawPin::new(inner);

        let boxed = Box::pin(obj);
        return Rc::new(RefCell::new(boxed));
    }

    pub fn fmap<R>(this: &mut Rc<RefCell<Pin<Box<Self>>>>, f: impl Fn(Pin<&mut RawPin<&mut T>>) -> R) -> R {
        let mut this = this.borrow_mut();
        let this = this.as_mut();
        unsafe {
            let mut this = this.get_unchecked_mut();
            let pin: Pin<&mut RawPin<&mut T>> = pin!(RawPin::new(&mut this));
            f(pin)
        }
    }
}

#[derive(Debug)]
pub struct SharedPin<T> {
    pin: Rc<RefCell<Pin<Box<RawPin<T>>>>> 
}

impl<T> SharedPin<T> {
    pub fn new(inner: T) -> SharedPin<T> {
        SharedPin {
            pin: RawPin::pin(inner)
        }
    }

    pub fn fmap<R>(&mut self, f: impl Fn(Pin<&mut RawPin<&mut T>>) -> R) -> R {
        RawPin::fmap(&mut self.pin, f)
    }

    pub fn clone(&mut self) -> SharedPin<T> {
        SharedPin {
            pin: self.pin.clone()
        }
    }
}

impl<T> Deref for RawPin<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RawPin<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug)]
pub enum Primitive {

}

#[derive(Debug)]
pub struct RawObject {
    pub number: Option<u64>,
}

impl RawObject {
    pub fn new<P: Into<Vec<Primitive>>>(inner: P) -> RawObject {
        RawObject {
            number: None,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    raw: Either<RawObject, SharedPin<RawObject>>,
}

impl Object {
    pub fn new<P: Into<Vec<Primitive>>>(inner: P) -> Object {
        Object {
            raw: Left(RawObject::new(inner)),
        }
    }

    pub fn fmap<R>(&mut self, f: impl Fn(Pin<&mut RawPin<&mut RawObject>>) -> R) -> R {
        match &mut self.raw {
            Left(ref mut obj) => {
                f(pin!(RawPin::new(obj)))
            },
            Right(ref mut pin) => pin.fmap(f)
        }
    }

    pub fn as_ref(&mut self) -> SharedPin<RawObject> {
        if let Left(ref mut object) = self.raw {
            let raw = std::mem::replace(object, RawObject { number: Some(999)});
            self.raw = Right(SharedPin::new(raw));
        }

        if let Right(ref mut pin) = self.raw {
            return pin.clone();
        }

        unreachable!();
    }
}