use crate::ffi::{
    // cl_uint,
    cl_device_id,
    cl_device_type,
    cl_platform_id,
    clGetDeviceIDs,
};

// use libc::c_void;

use crate::utils::StatusCode;
use crate::platform::Platform;
use crate::error::Output;
// use crate::utils;
use crate::cl::{
    // ClReturn,
    // ClDecoder,
    ClObject,
    ClPointer,
    // cl_get_info,
    // cl_get_info_count,
    cl_get_object_count,
    cl_get_object,
};
use super::flags::DeviceType;

#[cfg(feature = "opencl_version_1_2_0")]
__release_retain!(device_id, Device);

// NOTE: fix cl_device_type
pub fn cl_get_device_count(platform: &Platform, device_type: DeviceType) -> Output<u32> {
    let num_devices: u32 = unsafe {
        cl_get_object_count::<cl_platform_id, cl_device_type, cl_device_id>(
            platform.raw_cl_object(),
            device_type.bits(),
            clGetDeviceIDs
        )
    }?;
    Ok(num_devices)
}

pub fn cl_get_device_ids(
    platform: &Platform,
    device_type: DeviceType,
) -> Output<ClPointer<cl_device_id>> {
    
    unsafe {
        cl_get_object::<cl_platform_id, cl_device_type, cl_device_id>(
            platform.raw_cl_object(),
            device_type.bits(),
            clGetDeviceIDs
        )
    }
    // let mut n_devices: u32 = cl_get_device_count(platform, device_type)?;


    // let mut ids: Vec<cl_device_id> = utils::vec_filled_with(0 as cl_device_id, n_devices as usize);
    // let err_code = unsafe {
    //     clGetDeviceIDs(
    //         platform.raw_cl_object(),
    //         device_type.bits(),
    //         ids.len() as cl_uint,
    //         ids.as_mut_ptr(),
    //         &mut n_devices,
    //     )
    // };
    // let () = StatusCode::into_output(err_code, ())?;
    // let bytes: Vec<u8> = unsafe { std::mem::transmute(ids) };
    // let cl_ret = unsafe { ClReturn::new(bytes) };
    // Ok(cl_ret)
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
