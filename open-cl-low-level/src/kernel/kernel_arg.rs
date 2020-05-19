use crate::cl::cl_mem;
use crate::numbers::{AsPtr, Number};
use crate::{Mem, MemPtr};
use libc::c_void;

pub unsafe trait KernelArg {
    /// size_of<T> or size_of<T> * len
    fn kernel_arg_size(&self) -> usize;
    unsafe fn kernel_arg_ptr(&self) -> *const c_void;
    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void;
}

unsafe impl<T> KernelArg for T
where
    T: AsPtr<T> + Number,
{
    fn kernel_arg_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _ as *const c_void
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *const _ as *mut c_void
    }
}

unsafe impl KernelArg for Mem {
    fn kernel_arg_size(&self) -> usize {
        std::mem::size_of::<cl_mem>()
    }
    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self.mem_ptr_ref() as *const _ as *const c_void
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self.mem_ptr_ref() as *const _ as *mut c_void
    }
}
