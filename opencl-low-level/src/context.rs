use crate::{utils, build_output, Output, ClPointer, DevicePtr}; // ClDeviceID,
use crate::ffi::{cl_device_id, clCreateContext, cl_context, clGetContextInfo, cl_context_info};

use crate::cl_helpers::{cl_get_info5};
// use super::{ContextObject, ContextInfo, ContextRefCount};

#[allow(clippy::transmuting_null)]
pub unsafe fn cl_create_context(device_ids: &[cl_device_id]) -> Output<cl_context> {
    let mut err_code = 0;
    let context = clCreateContext(
        std::ptr::null(),
        device_ids.len() as u32,
        device_ids.as_ptr() as *const cl_device_id,
        std::mem::transmute(std::ptr::null::<fn()>()),
        std::ptr::null_mut(),
        &mut err_code,
    );
    build_output(context, err_code)
}

pub fn cl_get_context_info<T>(context: cl_context, flag: cl_context_info) -> Output<ClPointer<T>> where T: Copy {
    utils::null_check(context)?;
    unsafe {
        cl_get_info5(
            context,
            flag as cl_context_info,
            clGetContextInfo,
        )
    }
}

__release_retain!(context, Context);

pub trait ContextPtr: Sized {
    unsafe fn context_ptr(&self) -> cl_context;
}

unsafe fn release_context(context: cl_context) {
    cl_release_context(context).unwrap_or_else(|e| {
        panic!("Failed to release cl_context {:?}", e);
    });
}

unsafe fn retain_context(context: cl_context) {
    cl_retain_context(context).unwrap_or_else(|e| {
        panic!("Failed to retain cl_context {:?}", e);
    });
}

pub struct ClContext {
    object: cl_context,
    _unconstructable: (),
}

impl ClContext {
    pub unsafe fn unchecked_new(object: cl_context) -> ClContext {
        ClContext {
            object,
            _unconstructable: (),
        }
    }
    pub unsafe fn new(object: cl_context) -> Output<ClContext> {
        utils::null_check(object)?;
        Ok(ClContext::unchecked_new(object))
    }

    pub unsafe fn create<D>(devices: &[D]) -> Output<ClContext> where D: DevicePtr {
        let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
        let object = cl_create_context(&device_ptrs[..])?;
        ClContext::new(object)
    }
}

impl ContextPtr for ClContext {
    unsafe fn context_ptr(&self) -> cl_context {
        self.object
    }
}

impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe{
            release_context(self.context_ptr())
        }
    }
}

impl Clone for ClContext {
    fn clone(&self) -> ClContext {
        unsafe { 
            let context = self.context_ptr();
            retain_context(context);
            ClContext::unchecked_new(context)
        }
    }
}
