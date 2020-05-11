use crate::Number;

pub trait AsPtr<T> {
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<T> AsPtr<T> for T
where
    T: Number,
{
    fn as_ptr(&self) -> *const T {
        self
    }
    fn as_mut_ptr(&mut self) -> *mut T {
        self
    }
}
