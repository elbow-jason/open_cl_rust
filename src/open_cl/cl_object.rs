pub trait ClObject<T> {
    fn raw_cl_object(&self) -> T;
}

pub trait MutClObject<T> {
    fn raw_mut_cl_object(&mut self) -> T;
}