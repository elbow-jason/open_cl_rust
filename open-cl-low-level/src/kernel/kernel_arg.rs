use crate::cl::{cl_mem, ClObject};
use crate::numbers::{AsPtr, Number};
use crate::{Mem, MemPtr, NumberType, NumberTyped, NumberTypedT};
use libc::c_void;
use std::marker::PhantomData;

pub unsafe trait KernelArgPtr: Sized {
    /// size_of<T> or size_of<T> * len
    fn kernel_arg_size(&self) -> usize;
    fn kernel_arg_number_type(&self) -> NumberType;
    unsafe fn kernel_arg_ptr(&self) -> *const c_void;
    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void;
}

unsafe impl<T> KernelArgPtr for T
where
    T: AsPtr<T> + Number + NumberTypedT,
{
    fn kernel_arg_size(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn kernel_arg_number_type(&self) -> NumberType {
        T::number_type()
    }

    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _ as *const c_void
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *const _ as *mut c_void
    }
}

unsafe impl KernelArgPtr for Mem {
    fn kernel_arg_size(&self) -> usize {
        std::mem::size_of::<cl_mem>()
    }

    fn kernel_arg_number_type(&self) -> NumberType {
        self.number_type()
    }

    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self.mem_ptr_ref() as *const _ as *const c_void
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self.mem_ptr_ref() as *const _ as *mut c_void
    }
}

#[derive(Debug)]
pub struct KernelArg<'a> {
    _t: NumberType,
    _ptr: *const c_void,
    _phantom: PhantomData<&'a c_void>,
    _size: usize,
}

impl<'a> KernelArg<'a> {
    pub fn new<A: KernelArgPtr>(arg: &A) -> KernelArg<'a> {
        unsafe {
            KernelArg::from_raw_parts(
                arg.kernel_arg_number_type(),
                arg.kernel_arg_ptr(),
                arg.kernel_arg_size(),
            )
        }
    }

    pub fn from_num<T: NumberTypedT + AsPtr<T> + KernelArgPtr + Copy>(num: T) -> KernelArg<'a> {
        let t = T::number_type();
        unsafe {
            KernelArg::from_raw_parts(t, num.as_ptr() as *const c_void, t.number_type_size_of())
        }
    }

    pub fn from_mem(mem: &Mem) -> KernelArg {
        unsafe {
            KernelArg::from_raw_parts(
                mem.number_type(),
                mem.mem_ptr().as_ptr(),
                std::mem::size_of::<cl_mem>(),
            )
        }
    }

    pub unsafe fn from_raw_parts(t: NumberType, ptr: *const c_void, size: usize) -> KernelArg<'a> {
        KernelArg {
            _t: t,
            _size: size,
            _ptr: ptr,
            _phantom: PhantomData,
        }
    }
}

// impl<'a> NumberTyped for NumArg<'a> {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

unsafe impl<'a> KernelArgPtr for KernelArg<'a> {
    fn kernel_arg_size(&self) -> usize {
        self._size
    }

    fn kernel_arg_number_type(&self) -> NumberType {
        self._t
    }

    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self._ptr
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self._ptr as *mut c_void
    }
}
