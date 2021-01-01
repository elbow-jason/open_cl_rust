use super::BufferBuilder;
use crate::cl::{
    clCreateBuffer, clGetMemObjectInfo, cl_context, cl_int, cl_mem, cl_mem_flags, cl_mem_info,
    ClObject, MemInfo, StatusCodeError,
};
use crate::{Number, Output};
use libc::c_void;

/// Low-level helper for creating a cl_mem buffer from a context, mem flags, and a buffer creator.
///
/// # Safety
/// Use of a invalid cl_context in this function call is undefined behavior.
pub unsafe fn create_buffer<T: Number, B: BufferBuilder>(
    context: cl_context,
    mem_flags: cl_mem_flags,
    builder: B,
) -> Output<cl_mem> {
    cl_create_buffer(
        context,
        mem_flags,
        builder.buffer_len() * std::mem::size_of::<T>(),
        builder.buffer_ptr(),
    )
}

/// Low level helper functin for creating cl_mem buffer.
///
/// # Safety
/// Calling this function with an invalid context, or an incorrect size in bytes,
/// or an invalid host pointer is undefined behavior.
unsafe fn cl_create_buffer(
    context: cl_context,
    mem_flags: cl_mem_flags,
    size_in_bytes: usize,
    ptr: *mut c_void,
) -> Output<cl_mem> {
    let mut err_code: cl_int = 0;
    let device_mem_ptr: *mut c_void = clCreateBuffer(
        context.as_ptr() as *mut c_void,
        mem_flags,
        size_in_bytes,
        ptr,
        &mut err_code,
    );
    StatusCodeError::check(err_code)?;
    cl_mem::new(device_mem_ptr)
}

// pub fn cl_get_mem_object_info<T>(device_mem: cl_mem, flag: cl_mem_info) -> Output<ClPointer<T>>
// where
//     T: Copy,
// {
//     unsafe { cl_get_info5(device_mem, flag, clGetMemObjectInfo) }
// }

// pub fn cl_get_mem_object_info<T>(device_mem: cl_mem, flag: cl_mem_info) -> Output<ClPointer<T>>
// where
//     T: Copy,
// {
//     unsafe { cl_get_info5(device_mem, flag, clGetMemObjectInfo) }
// }

/// Low level helper function for the FFI call to clGetMemObjectInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_mem that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_u32(mem_object: cl_mem, flag: cl_mem_info) -> Output<u32> {
    cl_get_info!(
        One,
        u32,
        clGetMemObjectInfo,
        mem_object,
        Into::<cl_mem_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetMemObjectInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_mem that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_usize(mem_object: cl_mem, flag: cl_mem_info) -> Output<usize> {
    cl_get_info!(
        One,
        usize,
        clGetMemObjectInfo,
        mem_object,
        Into::<cl_mem_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetMemObjectInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_mem that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_context(mem_object: cl_mem) -> Output<cl_context> {
    cl_get_info!(
        One,
        cl_context,
        clGetMemObjectInfo,
        mem_object,
        Into::<cl_mem_info>::into(MemInfo::Context)
    )
}

/// Low level helper function for the FFI call to clGetMemObjectInfo with cl_mem_flags expected
///
/// # Safety
/// Calling this function with a cl_mem that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_info_flags(mem_object: cl_mem) -> Output<cl_mem_flags> {
    cl_get_info!(
        One,
        cl_mem_flags,
        clGetMemObjectInfo,
        mem_object,
        Into::<cl_mem_info>::into(MemInfo::Flags)
    )
}
