use std::fmt::Debug;


use crate::ffi::{
    cl_int,
    cl_mem,
    cl_mem_flags,
    cl_mem_info,
    clCreateBuffer,
    clGetMemObjectInfo,
};

use super::DeviceMem;
use super::flags::MemFlags;

use crate::context::Context;
use crate::error::Output;
use crate::utils::{StatusCode, ClObject};

__release_retain!(mem, MemObject);

unsafe fn buffer_mem_size_and_ptr<T>(buf: &[T]) -> (usize, *const libc::c_void) {
    (
        std::mem::size_of::<T>() * buf.len(),
        buf.as_ptr() as *const libc::c_void,
    )
}

pub fn cl_create_buffer_from_slice<T>(
    context: &Context,
    mem_flags: MemFlags,
    slice: &[T],
) -> Output<DeviceMem<T>> where T: Debug {
    unsafe {
        let (
            buffer_mem_size,
            buffer_ptr
        ) = buffer_mem_size_and_ptr(slice);
        
        _cl_create_buffer(
            context,
            mem_flags,
            buffer_mem_size,
            buffer_ptr as *mut libc::c_void,
        )
    }
}

pub fn cl_create_buffer_with_len<T>(
    context: &Context,
    mem_flags: MemFlags,
    len: usize,
) -> Output<DeviceMem<T>> where T: Debug {
    unsafe {
        _cl_create_buffer(context, mem_flags, len, std::ptr::null_mut())
    }
}

unsafe fn _cl_create_buffer<T>(
    context: &Context,
    mem_flags: MemFlags,
    len: usize,
    ptr: *mut libc::c_void,
) -> Output<DeviceMem<T>> where T: Debug {
    let mut err_code: cl_int = 0;
    let mut cl_mem_object: cl_mem = clCreateBuffer(
        context.raw_cl_object(),
        mem_flags.bits() as cl_mem_flags,
        (std::mem::size_of::<T>() * len) as libc::size_t,
        ptr,
        &mut err_code,
    );
    cl_mem_object = StatusCode::into_output(err_code, cl_mem_object)?;
    Ok(DeviceMem::new(cl_mem_object))
}


// NOTE: Fix this cl_mem_info arg
pub fn cl_get_mem_object_info<T>(
    device_mem: &DeviceMem<T>,
    mem_info_flag: cl_mem_info,
) -> Output<usize>
where
    T: Debug,
{
    let mut size: libc::size_t = 0;
    let err_code = unsafe {
        clGetMemObjectInfo(
            device_mem.raw_cl_object(),
            mem_info_flag,
            std::mem::size_of::<libc::size_t>() as libc::size_t,
            (&mut size as *mut libc::size_t) as *mut libc::c_void,
            std::ptr::null_mut(),
        )
    };
    StatusCode::into_output(err_code, size as usize)
}