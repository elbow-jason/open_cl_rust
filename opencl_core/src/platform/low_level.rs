use std::sync::Mutex;

use crate::ffi::{
    cl_platform_id,
    cl_platform_info,
    clGetPlatformIDs,
    clGetPlatformInfo,
    cl_uint,
};

use crate::utils::{ClObject, StatusCode};
use crate::error::Output;
use crate::utils;

use super::Platform;
use super::flags::PlatformInfo;

lazy_static! {
    pub static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

fn with_platform_access<T, F: FnOnce() -> T>(op: F) -> T {
    let platform_lock = PLATFORM_ACCESS.lock();
    let output = op();
    std::mem::drop(platform_lock);
    output
}

pub fn cl_get_platforms_count() -> Output<u32> {
    let mut num_platforms: cl_uint = 0;
    let err_code = with_platform_access(|| unsafe {
        clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms)
    });
    StatusCode::into_output(err_code, num_platforms)
}

pub fn cl_get_platforms() -> Output<Vec<Platform>> {    
    // transactional access to the platform Mutex
    let result: Output<Vec<cl_platform_id>> = with_platform_access(|| {
        let mut num_platforms: cl_uint = 0;
        let e1 = unsafe { clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms) };
        let mut ids: Vec<cl_platform_id> = utils::vec_filled_with(0 as cl_platform_id, num_platforms as usize);
        let () = StatusCode::into_output(e1, ())?;
        let e2 = unsafe { clGetPlatformIDs(num_platforms, ids.as_mut_ptr(), &mut num_platforms) };
        let () = StatusCode::into_output(e2, ())?;
        Ok(ids)
    });

    result.map(|ids| {
        ids.into_iter().map(|platform_id| unsafe { Platform::new(platform_id) }).collect()
    })
}

pub fn cl_get_platform_info(
    platform: &Platform,
    platform_info: PlatformInfo,
) -> Output<String> {
    let mut size = 0 as libc::size_t;
    let mut err_code = unsafe {
        clGetPlatformInfo(
            platform.raw_cl_object(),
            platform_info as cl_platform_info,
            0,
            std::ptr::null_mut(),
            &mut size,
        )
    };
    size = StatusCode::into_output(err_code, size)?;
    let mut buf: Vec<u8> = utils::vec_filled_with(0u8, size);
    err_code = unsafe {
        clGetPlatformInfo(
            platform.raw_cl_object(),
            platform_info as cl_platform_info,
            size,
            buf.as_mut_ptr() as *mut libc::c_void,
            std::ptr::null_mut(),
        )
    };
    buf = StatusCode::into_output(err_code, buf)?;
    let info = unsafe { String::from_utf8_unchecked(buf) };
    Ok(info)
}

#[test]
fn test_cl_get_platforms_count() {
    let count = cl_get_platforms_count()
        .map_err(|e| panic!("get_platform_count failed with {:?}", e))
        .unwrap();
    assert!(count > 0);
}

#[test]
fn test_cl_get_platforms() {
    let platforms_result: Output<Vec<Platform>> = cl_get_platforms();
    assert!(platforms_result.is_ok());
    let platforms = platforms_result.unwrap();
    assert!(platforms.len() > 0);
}
