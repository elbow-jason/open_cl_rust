use crate::ffi::{
    clGetDeviceIDs, cl_device_id, cl_device_type, cl_platform_id,
    clGetDeviceInfo, cl_device_info,
};

use crate::error::Output;
use crate::platform::Platform;
use crate::utils::StatusCode;
use crate::cl::cl_get_info5;

use super::flags::{DeviceType, DeviceInfo};
use super::device_usability_check;
use crate::cl::{cl_get_object, cl_get_object_count, ClPointer};

#[cfg(feature = "opencl_version_1_2_0")]
__release_retain!(device_id, Device);

// NOTE: fix cl_device_type
pub fn cl_get_device_count(platform: &Platform, device_type: DeviceType) -> Output<u32> {
    let num_devices: u32 = unsafe {
        cl_get_object_count::<cl_platform_id, cl_device_type, cl_device_id>(
            platform.platform_ptr(),
            device_type.bits(),
            clGetDeviceIDs,
        )
    }?;
    Ok(num_devices)
}

pub fn cl_get_device_ids(
    platform: &Platform,
    device_type: DeviceType,
) -> Output<ClPointer<cl_device_id>> {
    unsafe { cl_get_object(platform.platform_ptr(), device_type.bits(), clGetDeviceIDs) }
}


pub fn cl_get_device_info<T>(device_id: cl_device_id, flag: DeviceInfo) -> Output<ClPointer<T>> where T: Copy {
    device_usability_check(device_id)?;
    unsafe {
        cl_get_info5(
            device_id,
            flag as cl_device_info,
            clGetDeviceInfo,
        )
    }
}

// NOTE: Add support for sub devices
//  // pub fn cl_create_sub_devices(
//  //  in_device: cl_device_id,
//  //  properties: *const cl_device_partition_property,
//  //  num_devices: cl_uint,
//  //  out_devices: *mut cl_device_id,
//  //  num_devices_ret: *mut cl_uint
//  // )
