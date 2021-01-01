use super::{ClObject, RetainRelease};
use std::fmt;

#[repr(transparent)]
pub struct ObjectWrapper<T: ClObject + RetainRelease> {
    object: T,
}

impl<T: ClObject + RetainRelease> ObjectWrapper<T> {
    pub unsafe fn new(object: T) -> Self {
        ObjectWrapper { object }
    }

    pub unsafe fn cl_object(&self) -> T {
        self.object
    }

    pub unsafe fn cl_object_ref(&self) -> &T {
        &self.object
    }

    pub unsafe fn cl_object_mut_ref(&mut self) -> &mut T {
        &mut self.object
    }

    pub unsafe fn retain_new(object: T) -> Self {
        object.retain();
        Self::new(object)
    }

    pub fn address(&self) -> String {
        self.object.address()
    }

    // pub fn type_name() -> &'static str {
    //     T::type_name()
    // }
}

impl<T: RetainRelease> Drop for ObjectWrapper<T> {
    fn drop(&mut self) {
        unsafe {
            self.cl_object().release();
        }
    }
}

impl<T: RetainRelease> Clone for ObjectWrapper<T> {
    fn clone(&self) -> ObjectWrapper<T> {
        let object = self.object;
        unsafe { object.retain() };
        ObjectWrapper { object }
    }
}

impl<T: ClObject + RetainRelease> PartialEq for ObjectWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.cl_object() == other.cl_object() }
    }
}

impl<T: ClObject + RetainRelease> Eq for ObjectWrapper<T> {}

impl<T: ClObject + RetainRelease> fmt::Debug for ObjectWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", T::type_name(), self.address())
    }
}
