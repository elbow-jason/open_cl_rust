use std::sync::Mutex;

use libc::c_void;

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
use crate::utils::cl_value::{
    ClReturn,
    ClOutput,
    
    // VecClPlatform,
};

use super::Platform;
use super::flags::PlatformInfo;

lazy_static! {
    pub static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

// fn with_platform_access<T, F: FnOnce() -> T>(op: F) -> T {
//     let platform_lock = PLATFORM_ACCESS.lock();
//     let output = op();
//     std::mem::drop(platform_lock);
//     output
// }

pub fn cl_get_platforms_count() -> ClOutput {
    let platform_lock = PLATFORM_ACCESS.lock();
    let mut num_platforms: cl_uint = 0;
    let err_code = unsafe {
        clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms)
    };
    std::mem::drop(platform_lock);
    let () = StatusCode::into_output(err_code, ())?;
    Ok(unsafe { ClReturn::new_sized::<u32>(num_platforms as *mut c_void) })
}

pub fn cl_get_platforms() -> Output<ClReturn> {
    let platform_lock = PLATFORM_ACCESS.lock();
    // transactional access to the platform Mutex
    let mut num_platforms: cl_uint = 0;
    let e1 = unsafe { clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms) };
    let mut ids: Vec<cl_platform_id> = utils::vec_filled_with(0 as cl_platform_id, num_platforms as usize);
    let () = StatusCode::into_output(e1, ())?;
    let e2 = unsafe { clGetPlatformIDs(num_platforms, ids.as_mut_ptr(), &mut num_platforms) };
    let () = StatusCode::into_output(e2, ())?;
    std::mem::drop(platform_lock);
    Ok(unsafe { ClReturn::from_vec(ids) })
}

pub fn cl_get_platform_info(
    platform: &Platform,
    platform_info: PlatformInfo,
) -> Output<ClReturn> {
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
    let () = StatusCode::into_output(err_code, ())?;
    Ok(unsafe { ClReturn::from_vec(buf) })
}

#[cfg(test)]
mod tests {
    use crate::utils::cl_value::ClDecoder;
    use super::*;

    #[test]
    fn test_cl_get_platforms_count() {
        let count: usize = unsafe { 
            cl_get_platforms_count()
                .unwrap_or_else(|e| panic!("cl_get_platform_count failed with {:?}", e))
                .cl_decode()
        };
        assert!(count > 0);
    }

    #[test]
    fn test_cl_get_platforms() {
        let platforms: Vec<Platform> = unsafe { 
            cl_get_platforms()
                .unwrap_or_else(|e| panic!("cl_get_platforms failed with {:?}", e))
                .cl_decode()
        };
        // inspect_var!(platforms);
        assert!(platforms.len() > 0);
        let count: usize = unsafe { 
            cl_get_platforms_count()
                .unwrap_or_else(|e| panic!("cl_get_platform_count failed with {:?}", e))
                .cl_decode()
        };
        assert!(count == platforms.len());
    }
}