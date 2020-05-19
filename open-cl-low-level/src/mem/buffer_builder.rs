use crate::Number;
use libc::c_void;

use super::MemConfig;

pub trait BufferBuilder: Sized {
    /// The "size-and-ptr" of a buffer creation arg.
    ///
    /// Currently the only 2 types that implement BufferCreator are
    /// `usize` representiing length/size and &[T] (or mut slice) for Numberber
    /// T representing data.
    fn buffer_len(&self) -> usize;
    fn buffer_ptr(&self) -> *mut c_void;
    fn mem_config(&self) -> MemConfig;
}

impl<T> BufferBuilder for &[T]
where
    T: Number,
{
    fn buffer_len(&self) -> usize {
        self.len()
    }

    fn buffer_ptr(&self) -> *mut c_void {
        self.as_ptr() as *const _ as *mut c_void
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}

impl<T> BufferBuilder for &mut [T]
where
    T: Number,
{
    fn buffer_len(&self) -> usize {
        self.len()
    }

    fn buffer_ptr(&self) -> *mut c_void {
        self.as_ptr() as *const _ as *mut c_void
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}

impl BufferBuilder for usize {
    fn buffer_len(&self) -> usize {
        *self
    }

    fn buffer_ptr(&self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_size()
    }
}
