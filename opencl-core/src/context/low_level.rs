use crate::cl::ClObject;
use crate::device::Device;
use crate::error::Output;
use crate::ffi::{clCreateContext, cl_context, cl_device_id};
use crate::utils::StatusCode;

use super::Context;

#[allow(clippy::transmuting_null)]
pub fn cl_create_context(device: &Device) -> Output<Context> {
    device.usability_check()?;
    let mut err_code = 0;
    let context = unsafe {
        clCreateContext(
            std::ptr::null(),
            1,
            &device.raw_cl_object() as *const cl_device_id,
            std::mem::transmute(std::ptr::null::<fn()>()),
            std::ptr::null_mut(),
            &mut err_code,
        )
    };
    let checked_context = StatusCode::build_output(err_code, context)?;
    debug_assert!(!checked_context.is_null());
    unsafe { Context::new(checked_context) }
}

__release_retain!(context, Context);
