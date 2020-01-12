use crate::device::{cl_device_id, device_usability_check};
use crate::error::Output;
use crate::ffi::{clCreateContext, cl_context};
use crate::utils::StatusCode;

use super::Context;

#[allow(clippy::transmuting_null)]
pub fn cl_create_context(devices: &[cl_device_id]) -> Output<Context> {
    for d in devices {
        let () = device_usability_check(*d)?;
    }
    
    let mut err_code = 0;
    let context = unsafe {
        clCreateContext(
            std::ptr::null(),
            1,
            devices.as_ptr() as *const cl_device_id,
            std::mem::transmute(std::ptr::null::<fn()>()),
            std::ptr::null_mut(),
            &mut err_code,
        )
    };
    let () = StatusCode::build_output(err_code, ())?;
    debug_assert!(!context.is_null());

    // devices.clone()

    Ok(unsafe { Context::new(context) })
}

__release_retain!(context, Context);
