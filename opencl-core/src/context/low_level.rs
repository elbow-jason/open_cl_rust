use crate::device::{cl_device_id, DevicePtr, Device};
use crate::error::Output;
use crate::ffi::{clCreateContext, cl_context, clGetContextInfo, cl_context_info};
use crate::utils::StatusCode;
use crate::cl::{cl_get_info5, ClPointer};
use super::{ContextObject, ContextInfo, ContextRefCount};


#[allow(clippy::transmuting_null)]
pub unsafe fn cl_create_context<D: DevicePtr>(device_wrappers: &[D]) -> Output<ContextObject> {
    let devices: Vec<Device> = Device::clone_slice(device_wrappers)?;
    let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
    
    let mut err_code = 0;
    let context = clCreateContext(
        std::ptr::null(),
        device_ptrs.len() as u32,
        device_ptrs.as_ptr() as *const cl_device_id,
        std::mem::transmute(std::ptr::null::<fn()>()),
        std::ptr::null_mut(),
        &mut err_code,
    );
    let () = StatusCode::build_output(err_code, ())?;
    debug_assert!(!context.is_null());
    
    ContextObject::from_retained(context)
}

pub fn cl_get_context_info<T>(context: cl_context, flag: ContextInfo) -> Output<ClPointer<T>> where T: Copy {
    unsafe {
        cl_get_info5(
            context,
            flag as cl_context_info,
            clGetContextInfo,
        )
    }
}

__release_retain!(context, Context);
