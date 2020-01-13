use crate::error::Output;

/// An error related to an arbitrary ClObject.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ClObjectError {
    #[fail(
        display = "OpenCL object cannot be null. Found during: {:?}",
        _0
    )]
    CannotBeNull(String),
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
