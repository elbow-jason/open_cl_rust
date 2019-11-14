
use std::fmt::Debug;
use libc::{c_void, size_t};

use crate::ffi::{cl_mem};

use crate::device_mem::DeviceMem;

pub type KernelArgSizeAndPointer = (size_t, *const c_void);
pub trait KernelArg {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer;
}

impl<T> KernelArg for DeviceMem<T>
where
    T: Debug,
{
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        (
            std::mem::size_of::<cl_mem>() as size_t,
            self.ptr_to_cl_object() as *const c_void,
        )
    }
}

macro_rules! sized_scalar_kernel_arg {
    ($scalar:ty) => {
        impl KernelArg for $scalar {
            unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
                (
                    std::mem::size_of::<$scalar>() as size_t,
                    (self as *const $scalar) as *const c_void,
                )
            }
        }
    };
}

sized_scalar_kernel_arg!(isize);
sized_scalar_kernel_arg!(i8);
sized_scalar_kernel_arg!(i16);
sized_scalar_kernel_arg!(i32);
sized_scalar_kernel_arg!(i64);

sized_scalar_kernel_arg!(usize);
sized_scalar_kernel_arg!(u8);
sized_scalar_kernel_arg!(u16);
sized_scalar_kernel_arg!(u32);
sized_scalar_kernel_arg!(u64);

sized_scalar_kernel_arg!(f32);
sized_scalar_kernel_arg!(f64);
