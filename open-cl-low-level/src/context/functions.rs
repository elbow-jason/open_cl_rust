use crate::cl::{
    clCreateContext, clGetContextInfo, cl_context, cl_context_info, cl_device_id, ClObject,
    ContextInfo, StatusCodeError,
};
use crate::Output;

#[allow(clippy::transmuting_null)]
pub unsafe fn create_context(device_ids: &[cl_device_id]) -> Output<cl_context> {
    let mut err_code = 0;
    let context_ptr = clCreateContext(
        std::ptr::null(),
        device_ids.len() as u32,
        device_ids.as_ptr() as *const *mut libc::c_void,
        std::mem::transmute(std::ptr::null::<fn()>()),
        std::ptr::null_mut(),
        &mut err_code,
    );
    StatusCodeError::check(err_code)?;
    cl_context::new(context_ptr)
}

#[inline(always)]
pub unsafe fn get_context_info_devices(context: cl_context) -> Output<Vec<cl_device_id>> {
    let flag: cl_context_info = ContextInfo::Devices.into();
    cl_get_info!(Many, cl_device_id, clGetContextInfo, context, flag)
}

#[inline(always)]
pub unsafe fn get_context_info_vec_u64(
    context: cl_context,
    flag: cl_context_info,
) -> Output<Vec<u64>> {
    cl_get_info!(Many, u64, clGetContextInfo, context, flag)
}

#[inline(always)]
pub unsafe fn get_context_info_u32(context: cl_context, flag: cl_context_info) -> Output<u32> {
    cl_get_info!(One, u32, clGetContextInfo, context, flag)
}
