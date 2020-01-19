use std::fmt::Debug;

use libc::{c_void, size_t};

use crate::ffi::{clCreateKernel, clSetKernelArg, cl_kernel, cl_uint, cl_program};
use crate::{Output, strings, build_output};

pub type KernelArgSizeAndPointer = (size_t, *const c_void);
pub trait KernelArg {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer;
}

// impl<T> KernelArg for DeviceMem<T> where T: Debug + Sync + Send {
//     unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
//         (
//             std::mem::size_of::<cl_mem>() as size_t,
//             self.ptr_to_cl_object() as *const c_void,
//         )
//     }
// }

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


// pub use kernel_arg::{KernelArg, KernelArgSizeAndPointer};
// use super::kernel_arg::{KernelArg, KernelArgSizeAndPointer};
// use super::{Kernel, KernelError, KernelLock, KernelPtr, KernelRefCountWithProgram};

__release_retain!(kernel, Kernel);

pub unsafe fn cl_set_kernel_arg<T>(kernel: cl_kernel, arg_index: usize, arg: &T) -> Output<()>
where
    T: KernelArg + Debug
{
    let (arg_size, arg_ptr): KernelArgSizeAndPointer = arg.as_kernel_arg();

    let err_code = clSetKernelArg(
        kernel,
        arg_index as cl_uint,
        arg_size,
        arg_ptr,
    );

    build_output((), err_code)
}

pub unsafe fn cl_create_kernel(program: cl_program, name: &str) -> Output<cl_kernel> {
    let c_name = strings::to_c_string(name)
        .ok_or_else(|| KernelError::CStringInvalidKernelName(name.to_string()))?;
    let mut err_code = 0;
    let kernel = clCreateKernel(program, c_name.as_ptr(), &mut err_code);
    build_output(kernel, err_code)
}



/// An error related to a `Kernel`.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[fail(
        display = "The kernel name '{}' could not be represented as a CString.",
        _0
    )]
    CStringInvalidKernelName(String),
}


unsafe fn release_kernel(kernel: cl_kernel) {
    cl_release_kernel(kernel).unwrap_or_else(|e| {
        panic!("Failed to release cl_kernel {:?} due to {:?} ", kernel, e);
    });
}

unsafe fn retain_kernel(kernel: cl_kernel) {
    cl_retain_kernel(kernel).unwrap_or_else(|e| {
        panic!("Failed to retain cl_kernel {:?} due to {:?}", kernel, e);
    });
}

pub trait KernelPtr: Sized {
    unsafe fn kernel_ptr(&self) -> cl_kernel;
}

pub struct ClKernel {
    object: cl_kernel,
    _unconstructable: ()
}

impl ClKernel {
    pub unsafe fn unchecked_new(object: cl_kernel) -> ClKernel {
        ClKernel { object, _unconstructable: () }
    }
}

impl KernelPtr for ClKernel {
    unsafe fn kernel_ptr(&self) -> cl_kernel {
        self.object
    }
}

impl Drop for ClKernel {
    fn drop(&mut self) {
        unsafe {
            release_kernel(self.object);
        }
    }
}

impl Clone for ClKernel {
    fn clone(&self) -> ClKernel {
        unsafe {
            let kernel = self.object;
            retain_kernel(kernel);
            ClKernel::unchecked_new(kernel)
        }
        
    }
}
