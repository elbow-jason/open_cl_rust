use std::fmt::Debug;

use crate::ffi::{clCreateKernel, clSetKernelArg, cl_kernel, cl_uint};

use crate::cl::ClObject;
use crate::error::Output;
use crate::program::{Program, ProgramPtr};
use crate::utils::strings;
use crate::utils::StatusCode;

use super::kernel_arg::{KernelArg, KernelArgSizeAndPointer};
use super::{Kernel, KernelError};

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
    let c_name = strings::to_c_string(name)
        .ok_or_else(|| KernelError::CStringInvalidKernelName(name.to_string()))?;
    let kernel = unsafe { clCreateKernel(program.program_ptr(), c_name.as_ptr(), &mut err_code) };
    StatusCode::build_output(err_code, ())?;
    unsafe { Kernel::new(kernel) }
}
