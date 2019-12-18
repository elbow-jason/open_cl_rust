use std::fmt::Debug;

use crate::ffi::{clCreateBuffer, clGetMemObjectInfo, cl_int, cl_mem, cl_mem_flags, cl_mem_info};

use super::flags::{MemFlags, MemInfo};
use super::DeviceMem;

use crate::cl::ClPointer;
use crate::cl::{cl_get_info5, ClObject};
use crate::context::Context;
use crate::error::Output;
use crate::utils::StatusCode;

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
) -> Output<DeviceMem<T>> where T: Debug + Sync + Send {
    unsafe {
        let (buffer_mem_size, buffer_ptr) = buffer_mem_size_and_ptr(slice);
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
) -> Output<DeviceMem<T>>  where T: Debug + Sync + Send {
    unsafe {
        _cl_create_buffer(
            context,
            mem_flags,
            (std::mem::size_of::<T>() * len) as libc::size_t,
            std::ptr::null_mut(),
        )
    }
}

unsafe fn _cl_create_buffer<T>(
    context: &Context,
    mem_flags: MemFlags,
    size_in_bytes: usize,
    ptr: *mut libc::c_void,
) -> Output<DeviceMem<T>> where T: Debug + Sync + Send {
    let mut err_code: cl_int = 0;
    let mut cl_mem_object: cl_mem = clCreateBuffer(
        context.raw_cl_object(),
        mem_flags.bits() as cl_mem_flags,
        size_in_bytes,
        ptr,
        &mut err_code,
    );
    cl_mem_object = StatusCode::build_output(err_code, cl_mem_object)?;
    DeviceMem::new(cl_mem_object)
}

// NOTE: Fix this cl_mem_info arg
pub fn cl_get_mem_object_info<T, P>(
    device_mem: &DeviceMem<T>,
    flag: MemInfo,
) -> Output<ClPointer<P>>
where
    T: Debug + Sync + Send,
    P: Copy,
{
    unsafe {
        cl_get_info5(
            device_mem.raw_cl_object(),
            flag as cl_mem_info,
            clGetMemObjectInfo,
        )
    }
}
