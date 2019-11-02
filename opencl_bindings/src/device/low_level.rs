use crate::ffi::{
    cl_uint,
    cl_device_id,
    cl_device_type,
    clGetDeviceIDs,
};
use crate::utils::{ClObject, StatusCode};
use crate::platform::Platform;
use crate::device::Device;
use crate::error::Output;
use crate::utils;


#[cfg(feature = "opencl_version_1_2_0")]
__release_retain!(device_id, Device);

// NOTE: fix cl_device_type
pub fn cl_get_device_count(platform: &Platform, device_type: cl_device_type) -> Output<u32> {
    let mut num_devices = 0;
    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type,
            0,
            std::ptr::null_mut(),
            &mut num_devices,
        )
    };
    StatusCode::into_output(err_code, num_devices)
}

pub fn cl_get_device_ids(
    platform: &Platform,
    device_type: cl_device_type,
) -> Output<Vec<Device>> {
    let mut num_devices: u32 = cl_get_device_count(platform, device_type)?;
    let mut ids: Vec<cl_device_id> = utils::vec_filled_with(0 as cl_device_id, num_devices as usize);
    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type,
            ids.len() as cl_uint,
            ids.as_mut_ptr(),
            &mut num_devices,
        )
    };
    let checked_ids = StatusCode::into_output(err_code, ids)?;
    let devices = checked_ids
        .into_iter()
        .map(|id| {
            unsafe { Device::new(id) }
        })
        .collect();
    Ok(devices)
}



// #[cfg(feature = "opencl_version_1_2_0")]
// pub fn cl_create_sub_devices(
//     in_device: cl_device_id,
//     properties: *const cl_device_partition_property,
//     num_devices: cl_uint,
//     out_devices: *mut cl_device_id,
//     num_devices_ret: *mut cl_uint) -> Vec<Device> {
//         notimplemented!();
//     }
