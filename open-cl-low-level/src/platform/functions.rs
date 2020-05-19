/// Platform serves 3 basic purposes:
///
/// Platform is the interface for listing platforms.
///
/// Platform is the interface for getting metadata about platforms.
///
/// Platform is the interface for listing Devices.
use crate::cl::{
    clGetDeviceIDs, clGetPlatformIDs, clGetPlatformInfo, cl_device_id, cl_device_type,
    cl_platform_id, cl_platform_info, ClObject, StatusCodeError,
};
use crate::Output;
use libc::c_void;
use std::sync::Mutex;

// There are race conditions on some platforms, so hide access to platforms behind
// a single global mutex. Better than nothing.
lazy_static! {
    static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

/// Gets the cl_platform_ids of the host machine
pub unsafe fn list_platform_ids() -> Output<Vec<cl_platform_id>> {
    let platform_lock = PLATFORM_ACCESS.lock();
    // transactional access to the platform Mutex requires a lock for some OpenCL implementations.
    let mut num_platforms: u32 = 0;
    StatusCodeError::check(clGetPlatformIDs(
        0,
        std::ptr::null_mut(),
        &mut num_platforms,
    ))?;
    let mut ids: Vec<cl_platform_id> = vec![cl_platform_id::null_ptr(); num_platforms as usize];
    StatusCodeError::check(clGetPlatformIDs(
        num_platforms,
        ids.as_mut_ptr() as *mut *mut c_void,
        &mut num_platforms,
    ))?;
    std::mem::drop(platform_lock);
    Ok(ids)
}

#[inline(always)]
pub unsafe fn device_count(platform: cl_platform_id, device_type: cl_device_type) -> Output<u32> {
    _object_count!(clGetDeviceIDs, platform, device_type)
}

#[inline(always)]
pub unsafe fn list_devices(
    platform: cl_platform_id,
    device_type: cl_device_type,
) -> Output<Vec<cl_device_id>> {
    let count = device_count(platform, device_type)?;
    _get_objects!(clGetDeviceIDs, cl_device_id, platform, device_type, count)
}

/// Gets platform info for a given cl_platform_id and the given cl_platform_info flag via the
/// OpenCL FFI call to clGetPlatformInfo.
///
/// # Safety
/// Use of an invalid cl_platform_id is undefined behavior. Be careful. There be dragons.
#[inline(always)]
pub unsafe fn get_platform_info(
    platform: cl_platform_id,
    flag: cl_platform_info,
) -> Output<String> {
    cl_get_info!(One, String, clGetPlatformInfo, platform, flag)
}
