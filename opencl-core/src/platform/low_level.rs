use std::sync::Mutex;

use crate::ffi::{
    cl_platform_id,
    cl_platform_info,
    clGetPlatformIDs,
    clGetPlatformInfo,
    cl_uint,
};


use crate::utils::StatusCode;
use crate::error::Output;
use crate::utils;
use crate::cl::{
    ClPointer,
    // ClReturn,
    ClObject,
    cl_get_info5
};

use super::Platform;
use super::flags::PlatformInfo;

lazy_static! {
    pub static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

pub fn cl_get_platforms() -> Output<ClPointer<cl_platform_id>> {
    let platform_lock = PLATFORM_ACCESS.lock();
    // transactional access to the platform Mutex requires one lock.
    let mut num_platforms: cl_uint = 0;
    let e1 = unsafe {
        clGetPlatformIDs(
            0,
            std::ptr::null_mut(),
            &mut num_platforms
        )
    };
    let mut ids: Vec<cl_platform_id> = utils::vec_filled_with(0 as cl_platform_id, num_platforms as usize);
    let () = StatusCode::into_output(e1, ())?;
    let e2 = unsafe {
        clGetPlatformIDs(
            num_platforms,
            ids.as_mut_ptr(),
            &mut num_platforms
        )
    };
    let () = StatusCode::into_output(e2, ())?;
    std::mem::drop(platform_lock);
    Ok(unsafe { ClPointer::from_vec(ids) })
}

pub fn cl_get_platform_info<T: Copy>(
    platform: &Platform,
    info_flag: PlatformInfo,
) -> Output<ClPointer<T>> {
      unsafe {
        cl_get_info5(
            platform.raw_cl_object(),
            info_flag as cl_platform_info,
            clGetPlatformInfo
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::cl::ClPointer;
    use super::*;

    #[test]
    fn test_cl_get_platforms() {
        let cl_pointer: ClPointer<cl_platform_id> = cl_get_platforms().unwrap_or_else(|e| {
            panic!("cl_get_platforms failed with {:?}", e)
        });
        let platforms: Vec<cl_platform_id> = unsafe { cl_pointer.into_many() };
        assert!(platforms.len() > 0);

        for p in platforms {
            assert!(p.is_null() == false);
        }
    }
}