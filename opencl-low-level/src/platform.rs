/// Platform has 3 basic functions (other than holding a cl object handle).
///
/// Platform is the interface for listing platforms.
///
/// Platform is the interface for getting metadata about platforms.
///
/// Platform is the interface for listing Devices.
///
/// NOTE: ClPlatformID is tested!

use std::default::Default;
use std::fmt;
use std::sync::Mutex;

use crate::ffi::{clGetPlatformIDs, clGetPlatformInfo, cl_platform_id, cl_platform_info, cl_uint};
use crate::cl_helpers::cl_get_info5;
use crate::cl_enums::PlatformInfo;
use crate::{Output, utils, ClPointer, build_output, Error};

lazy_static! {
    static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

pub unsafe fn cl_get_platform_ids() -> Output<ClPointer<cl_platform_id>> {
    let platform_lock = PLATFORM_ACCESS.lock();
    // transactional access to the platform Mutex requires one lock.
    let mut num_platforms: cl_uint = 0;
    let e1 = clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms);
    let mut ids: Vec<cl_platform_id> =
        utils::vec_filled_with(0 as cl_platform_id, num_platforms as usize);
    build_output((), e1)?;
    let e2 = clGetPlatformIDs(num_platforms, ids.as_mut_ptr(), &mut num_platforms);
    build_output((), e2)?;
    std::mem::drop(platform_lock);
    Ok(ClPointer::from_vec(ids))
}


pub unsafe fn cl_get_platform_info<T: Copy>(
    platform: cl_platform_id,
    info_flag: cl_platform_info,
) -> Output<ClPointer<T>> {
    cl_get_info5(
        platform,
        info_flag,
        clGetPlatformInfo,
    )
}


/// An error related to Platform.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum PlatformError {
    #[fail(display = "No platforms found!")]
    NoPlatforms,

    #[fail(display = "The platform has no useable devices!")]
    NoUsableDevices,

    #[fail(display = "The given platform had no default device!")]
    NoDefaultDevice,
}

pub trait PlatformPtr {
    fn platform_ptr(&self) -> cl_platform_id;
}

pub struct ClPlatformID {
    object: cl_platform_id
}

impl PlatformPtr for cl_platform_id {
    fn platform_ptr(&self) -> cl_platform_id {
        *self
    }
}

impl PlatformPtr for ClPlatformID {
    fn platform_ptr(&self) -> cl_platform_id {
        self.object
    }
}

impl PlatformPtr for &ClPlatformID {
    fn platform_ptr(&self) -> cl_platform_id {
        self.object
    }
}

impl ClPlatformID {
    pub fn new(object: cl_platform_id) -> Output<ClPlatformID> {
        utils::null_check(object)?;
        Ok(ClPlatformID{object})
    }
}

pub fn list_platforms() -> Output<Vec<ClPlatformID>> {
    let mut plats = Vec::new();
    unsafe {
        let cl_ptr = cl_get_platform_ids()?;
        for object in cl_ptr.into_vec().into_iter() {
            let plat = ClPlatformID::new(object)?;
            plats.push(plat);
        }
    }
    Ok(plats)
}

pub fn default_platform() -> Output<ClPlatformID> {
    let mut platforms = list_platforms()?;
    
    if platforms.is_empty() {
        return Err(Error::from(PlatformError::NoPlatforms));
    }
    Ok(platforms.remove(0))
}


pub fn platform_info<P: PlatformPtr, I: Into<cl_platform_info>>(platform: P, info_code: I) -> Output<String> {
    unsafe {
        cl_get_platform_info(platform.platform_ptr(), info_code.into()).map(|ret| ret.into_string() )
    }
}

pub fn platform_name<P: PlatformPtr>(platform: P) -> Output<String> {
    platform_info(platform, PlatformInfo::Name)
}

pub fn platform_version<P: PlatformPtr>(platform: P) -> Output<String> {
    platform_info(platform, PlatformInfo::Version)
}

pub fn platform_profile<P: PlatformPtr>(platform: P) -> Output<String> {
    platform_info(platform, PlatformInfo::Profile)
}

pub fn platform_vendor<P: PlatformPtr>(platform: P) -> Output<String> {
    platform_info(platform, PlatformInfo::Vendor)
}

pub fn platform_extensions<P: PlatformPtr>(platform: P) -> Output<Vec<String>> {
    platform_info(platform, PlatformInfo::Extensions)
        .map(|exts| exts.split(' ').map(|ext| ext.to_string()).collect())
}

// v2.1
// pub fn host_timer_resolution(&self) -> Output<String> {
//     self.get_info(PlatformInfo::HostTimerResolution)
// }


unsafe impl Send for ClPlatformID {}
unsafe impl Sync for ClPlatformID {}

impl Default for ClPlatformID {
    fn default() -> ClPlatformID {
        default_platform().unwrap()
    }
}

impl fmt::Debug for ClPlatformID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClPlatform{{{:?}}}", self.platform_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClPointer;
    // use crate::device::{Device, DeviceType, DevicePtr};

    #[test]
    fn test_cl_get_platforms() {
        let cl_pointer: ClPointer<cl_platform_id> = unsafe {
                cl_get_platform_ids().unwrap_or_else(|e| {
                        panic!("cl_get_platforms failed with {:?}", e)
                    })
            };
        let platforms: Vec<cl_platform_id> = unsafe { cl_pointer.into_vec() };
        assert!(platforms.len() > 0);

        for p in platforms {
            assert!(p.is_null() == false);
        }
    }

    #[test]
    fn platform_func_default_works() {
        let _platform: ClPlatformID = ClPlatformID::default();
    }

    #[test]
    fn platform_func_all_works() {
        let platforms: Vec<ClPlatformID> = list_platforms().expect("list_platforms() failed");
        assert!(platforms.len() > 0);
    }

    #[test]
    fn platform_has_functions_getting_for_info() {
        let platform = ClPlatformID::default();
        let empty_string = "".to_string();

        let name = platform_name(&platform)
            .expect("failed to get platform info for name");

        assert!(name != empty_string);

        let version = platform_version(&platform)
            .expect("failed to get platform info for version");

        assert!(version != empty_string);

        let profile = platform_profile(&platform)
            .expect("failed to get platform info for profile");

        assert!(profile != empty_string);

        let vendor = platform_vendor(&platform)
            .expect("failed to get platform info for vendor");

        assert!(vendor != empty_string);

        let extensions = platform_extensions(&platform)
            .expect("failed to get platform info for extensions");

        for ext in extensions.into_iter() {
            assert!(ext != empty_string);
        }
        // v2.1
        // let host_timer_resolution = platform.host_timer_resolution()
        //     .expect("failed to get platform info for host_timer_resolution");

        // assert_eq!(host_timer_resolution, "".to_string());
    }
}
