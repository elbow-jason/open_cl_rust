use crate::error::{Output, Error};



/// An error related to an arbitrary ClObject.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ClObjectError {
    #[fail(display = "The ClObject wrapper does not allow null pointers to be wrapped: {:?}", _0)]
    ClObjectCannotBeNull(String),

    #[fail(display = "Faile to release cl object: {:?}", _0)]
    FailedToReleaseClObject(String),
    
    #[fail(display = "Faile to retain cl object: {:?}", _0)]
    FailedToRetainClObject(String),

}

impl From<ClObjectError> for Error {
    fn from(err: ClObjectError) -> Error {
        Error::ClObjectError(err)
    }
}

/// For internal access only
pub trait ClObject<T: Sized> {

    
    unsafe fn raw_cl_object(&self) -> T;

    /// NOTE: ::new does not (better not) increase the cl_object's reference count.
    unsafe fn new(t: T) -> Output<Self> where Self: Sized;

    /// NOTE: ::new_retained increases the cl_object's reference count while also atomically wrapping the 
    /// the cl_object ensuring there is no chance to wrap the object then forget to increase retain the wrapper.
    unsafe fn new_retained(t: T) -> Output<Self> where Self: Sized;
    // unsafe fn cl_retain(&mut self) -> Output<()>;
}

pub trait CopyClObject<T>: ClObject<T> {
    // Calls the object's clRetain<object_name> function thereby increasing
    // the reference count of that object
    unsafe fn copy_cl_object_ref(&self) -> T;
}

pub trait MutClObject<T> {
    unsafe fn raw_mut_cl_object(&mut self) -> T;
}
