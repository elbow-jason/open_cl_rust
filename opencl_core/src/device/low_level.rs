use crate::ffi::{
    cl_uint,
    cl_device_id,
    clGetDeviceIDs,
};

use libc::c_void;

use crate::utils::{ClObject, StatusCode};
use crate::platform::Platform;
use crate::error::Output;
use crate::utils;
use crate::utils::cl_value::{ ClReturn, ClDecoder };
use super::flags::DeviceType;

#[cfg(feature = "opencl_version_1_2_0")]
__release_retain!(device_id, Device);

// NOTE: fix cl_device_type
pub fn cl_get_device_count(platform: &Platform, device_type: DeviceType) -> Output<ClReturn> {
    let mut num_devices = 0;
    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type.bits(),
            0,
            std::ptr::null_mut(),
            &mut num_devices,
        )
    };
    let () = StatusCode::into_output(err_code, ())?;
    Ok(unsafe { ClReturn::new_sized::<usize>(num_devices as *mut c_void) })
}

pub fn cl_get_device_ids(
    platform: &Platform,
    device_type: DeviceType,
) -> Output<ClReturn> {
    
    let device_count_ret: ClReturn = cl_get_device_count(platform, device_type)?;
    let mut n_devices = unsafe { device_count_ret.cl_decode() };
    let mut ids: Vec<cl_device_id> = utils::vec_filled_with(0 as cl_device_id, n_devices as usize);
    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type.bits(),
            ids.len() as cl_uint,
            ids.as_mut_ptr(),
            &mut n_devices,
        )
    };
    let () = StatusCode::into_output(err_code, ())?;
    let cl_ret = unsafe { ClReturn::from_vec(ids) };
    Ok(cl_ret)
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
