pub trait ClObject<T> {
    // For internal access only
    // NOTE: Does not increase the cl_object's reference count.
    unsafe fn raw_cl_object(&self) -> T;
}

pub trait CopyClObject<T>: ClObject<T> {
    // Calls the object's clRetain<object_name> function thereby increasing
    // the reference count of that object
    unsafe fn copy_cl_object_ref(&self) -> T;
}

pub trait MutClObject<T> {
    unsafe fn raw_mut_cl_object(&mut self) -> T;
}
