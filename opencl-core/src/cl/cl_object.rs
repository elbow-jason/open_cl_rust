use crate::error::Output;

/// An error related to an arbitrary ClObject.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ClObjectError {
    #[fail(
        display = "The ClObject wrapper does not allow null pointers to be wrapped: {:?}",
        _0
    )]
    ClObjectCannotBeNull(String),

    #[fail(display = "Failed to release cl object: {:?}", _0)]
    FailedToReleaseClObject(String),

    #[fail(display = "Failed to retain cl object: {:?}", _0)]
    FailedToRetainClObject(String),
}

/// For internal access only
pub trait ClObject<T: Sized> where Self: Sized {
    unsafe fn raw_cl_object(&self) -> T;

    // NOTE: ::new does not (better not) increase the cl_object's reference count.
    unsafe fn new(t: T) -> Output<Self>;
    
    // NOTE: ::new_retained increases the cl_object's reference count while also wrapping the
    // the cl_object ensuring there is no chance to wrap the object then forget to increase retain the wrapper.
    unsafe fn new_retained(t: T) -> Output<Self>;
    // unsafe fn cl_retain(&mut self) -> Output<()>;
}


pub trait CopyClObject<T>: ClObject<T> {
    // Calls the object's clRetain<object_name> function thereby increasing
    // the reference count of that object
    unsafe fn copy_cl_object_ref(&self) -> T;
}

