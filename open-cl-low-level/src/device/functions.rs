use crate::Output;

use crate::cl::{clGetDeviceInfo, cl_device_id, cl_device_info};

#[inline(always)]
pub unsafe fn get_device_info_string(device: cl_device_id, flag: cl_device_info) -> Output<String> {
    cl_get_info!(One, String, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_bool(device: cl_device_id, flag: cl_device_info) -> Output<bool> {
    cl_get_info!(One, bool, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_vec_usize(
    device: cl_device_id,
    flag: cl_device_info,
) -> Output<Vec<usize>> {
    cl_get_info!(Many, usize, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_u32(device: cl_device_id, flag: cl_device_info) -> Output<u32> {
    cl_get_info!(One, u32, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_u64(device: cl_device_id, flag: cl_device_info) -> Output<u64> {
    cl_get_info!(One, u64, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_usize(device: cl_device_id, flag: cl_device_info) -> Output<usize> {
    cl_get_info!(One, usize, clGetDeviceInfo, device, flag)
}
