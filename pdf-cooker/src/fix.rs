use std::pin::Pin;
use std::ops::{Deref, DerefMut};
use pin_project::{pin_project};

#[derive(Debug)]
#[pin_project(project = RawFixProj, project_ref = RawFixProjRef)]
pub struct RawFix<T> {
    pub inner: T
}

impl<T> Deref for RawFixProj<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RawFixProj<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Deref for RawFixProjRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> RawFix<T> {
    pub fn new(inner: T) -> Pin<Box<RawFix<T>>> {
        Box::pin(RawFix{inner})
    }

    pub fn fmap<R>(self: Pin<&mut Self>, f: impl FnOnce(RawFixProj<T>) -> R) -> R {
        f(self.project())
    }

    pub fn fmap_ref<R>(self: Pin<&Self>, f: impl FnOnce(RawFixProjRef<T>) -> R) -> R {
        f(self.project_ref())
    }
}

impl<T> Deref for RawFix<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RawFix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug)]
pub struct Fix<T> {
    pub inner: Pin<Box<RawFix<T>>>
}

impl<T> Fix<T> {
    pub fn new(inner: T) -> Fix<T> {
        Fix {
            inner: RawFix::new(inner),
        }
    }

    pub fn fmap<R>(&mut self, f: impl FnOnce(RawFixProj<T>) -> R) -> R {
        self.as_mut().fmap(f)
    }

    pub fn fmap_ref<R>(&self, f: impl FnOnce(RawFixProjRef<T>) -> R) -> R {
        self.as_ref().fmap_ref(f)
    }
}

impl<T> Deref for Fix<T> {
    type Target = Pin<Box<RawFix<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Fix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}