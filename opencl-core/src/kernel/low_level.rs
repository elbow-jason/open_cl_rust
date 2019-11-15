use std::fmt::Debug;

use crate::ffi::{
    cl_uint,
    cl_kernel,
    clSetKernelArg,
    clCreateKernel,
};

use crate::error::Output;
use crate::utils::StatusCode;
use crate::cl::ClObject;
use crate::program::Program;
use crate::utils::strings;

use super::{Kernel, KernelError};
use super::kernel_arg::{KernelArg, KernelArgSizeAndPointer};


__release_retain!(kernel, Kernel);

pub fn cl_set_kernel_arg<T>(kernel: &Kernel, arg_index: usize, arg: &T) -> Output<()>
where
    T: KernelArg + Debug,
{
    let err_code = unsafe {
        let (arg_size, arg_ptr): KernelArgSizeAndPointer = arg.as_kernel_arg();

        debug_assert!(!kernel.raw_cl_object().is_null());

        clSetKernelArg(
            kernel.raw_cl_object(),
            arg_index as cl_uint,
            arg_size,
            arg_ptr,
        )
    };
    StatusCode::build_output(err_code, ())
}

pub fn cl_create_kernel(program: &Program, name: &str) -> Output<Kernel> {
    let mut err_code = 0;
    let c_name = strings::to_c_string(name).ok_or_else(|| {
        KernelError::CStringInvalidKernelName(name.to_string())
    })?;
    let kernel = unsafe { clCreateKernel(program.raw_cl_object(), c_name.as_ptr(), &mut err_code) };
    StatusCode::build_output(err_code, ())?;
    unsafe { Kernel::new(kernel) }
}
