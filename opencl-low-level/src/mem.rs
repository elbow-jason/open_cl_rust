use std::fmt::Debug;

use crate::ffi::{clCreateBuffer, clGetMemObjectInfo, cl_int, cl_mem, cl_mem_flags, cl_mem_info, cl_context};

use crate::{ClPointer, Output, build_output};
use crate::cl_helpers::cl_get_info5;

__release_retain!(mem, MemObject);

unsafe fn buffer_mem_size_and_ptr<T>(buf: &[T]) -> (usize, *const libc::c_void) {
    (
        std::mem::size_of::<T>() * buf.len(),
        buf.as_ptr() as *const libc::c_void,
    )
}

pub unsafe fn cl_create_buffer_from_slice<T>(
    context: cl_context,
    mem_flags: cl_mem_flags,
    slice: &[T],
) -> Output<cl_mem> where T: Debug + Sync + Send + Sized {
    let (buffer_mem_size, buffer_ptr) = buffer_mem_size_and_ptr(slice);
    _cl_create_buffer(
        context,
        mem_flags,
        buffer_mem_size,
        buffer_ptr as *mut libc::c_void,
    )
}

pub fn cl_create_buffer_with_len<T>(
    context: cl_context,
    mem_flags: cl_mem_flags,
    len: usize,
) -> Output<cl_mem> {
    unsafe {
        _cl_create_buffer(
            context,
            mem_flags,
            (std::mem::size_of::<T>() * len) as libc::size_t,
            std::ptr::null_mut(),
        )
    }
}

unsafe fn _cl_create_buffer(
    context: cl_context,
    mem_flags: cl_mem_flags,
    size_in_bytes: usize,
    ptr: *mut libc::c_void,
) -> Output<cl_mem> {
    let mut err_code: cl_int = 0;
    let cl_mem_object: cl_mem = clCreateBuffer(
        context,
        mem_flags,
        size_in_bytes,
        ptr,
        &mut err_code,
    );
    build_output(cl_mem_object, err_code)
}

// NOTE: Fix this cl_mem_info arg
pub fn cl_get_mem_object_info<T>(
    device_mem: cl_mem,
    flag: cl_mem_info,
) -> Output<ClPointer<T>>
where
    
    T: Copy,
{
    unsafe {
        cl_get_info5(
            device_mem,
            flag,
            clGetMemObjectInfo,
        )
    }
}
