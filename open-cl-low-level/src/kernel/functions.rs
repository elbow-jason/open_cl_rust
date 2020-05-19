use super::KernelArg;
use crate::cl::{
    clCreateKernel, clGetKernelInfo, clSetKernelArg, cl_context, cl_kernel, cl_kernel_info,
    cl_program, cl_uint, ClObject, StatusCodeError,
};
use crate::Output;
use libc::c_void;
use std::ffi::CString;

pub unsafe fn set_kernel_arg<T: KernelArg>(
    kernel: cl_kernel,
    arg_index: usize,
    arg: &T,
) -> Output<()> {
    cl_set_kernel_arg_raw(
        kernel,
        arg_index as cl_uint,
        arg.kernel_arg_size(),
        arg.kernel_arg_ptr(),
    )
}

unsafe fn cl_set_kernel_arg_raw(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_size: usize,
    arg_ptr: *const c_void,
) -> Output<()> {
    let status_code = clSetKernelArg(
        kernel.as_ptr() as *mut c_void,
        arg_index as cl_uint,
        arg_size,
        arg_ptr,
    );
    StatusCodeError::check(status_code)?;
    Ok(())
}

pub unsafe fn create_kernel(program: cl_program, c_name: CString) -> Output<cl_kernel> {
    let mut status_code = 0;
    let raw_kernel: *mut c_void = clCreateKernel(
        program.as_ptr() as *mut c_void,
        c_name.as_ptr(),
        &mut status_code,
    );
    StatusCodeError::check(status_code)?;
    cl_kernel::new(raw_kernel)
}

/// Low level helper function for the FFI call to clGetKernelInfo with String expected
///
/// # Safety
/// Calling this function with a cl_kernel that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_string(kernel: cl_kernel, flag: cl_kernel_info) -> Output<String> {
    cl_get_info!(
        One,
        String,
        clGetKernelInfo,
        kernel,
        Into::<cl_kernel_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetKernelInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_kernel that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_u32(kernel: cl_kernel, flag: cl_kernel_info) -> Output<u32> {
    cl_get_info!(
        One,
        u32,
        clGetKernelInfo,
        kernel,
        Into::<cl_kernel_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetKernelInfo with cl_context expected
///
/// # Safety
/// Calling this function with a cl_kernel that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_context(kernel: cl_kernel, flag: cl_kernel_info) -> Output<cl_context> {
    cl_get_info!(
        One,
        cl_context,
        clGetKernelInfo,
        kernel,
        Into::<cl_kernel_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetKernelInfo with cl_program expected
///
/// # Safety
/// Calling this function with a cl_kernel that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_program(kernel: cl_kernel, flag: cl_kernel_info) -> Output<cl_program> {
    cl_get_info!(
        One,
        cl_program,
        clGetKernelInfo,
        kernel,
        Into::<cl_kernel_info>::into(flag)
    )
}
