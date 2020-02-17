// /// Platform has 3 basic functions (other than holding a cl object handle).
// ///
// /// Platform is the interface for listing platforms.
// ///
// /// Platform is the interface for getting metadata about platforms.
// ///
// /// Platform is the interface for listing Devices.
// ///
// /// NOTE: Platform is tested!

use std::default::Default;
use std::fmt;

use crate::ll;
use crate::ll::{ClPlatformID, Output, PlatformInfo, PlatformPtr};

use ffi::cl_platform_id;

pub struct Platform {
    inner: ClPlatformID,
    _unconstructable: (),
}

impl Platform {
    pub fn new(p: ClPlatformID) -> Platform {
        Platform {
            inner: p,
            _unconstructable: (),
        }
    }

    pub fn low_level_platform(&self) -> &ClPlatformID {
        &self.inner
    }
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

impl PlatformPtr for Platform {
    fn platform_ptr(&self) -> cl_platform_id {
        self.inner.platform_ptr()
    }
}

impl PlatformPtr for &Platform {
    fn platform_ptr(&self) -> cl_platform_id {
        self.inner.platform_ptr()
    }
}

impl Default for Platform {
    fn default() -> Platform {
        Platform::new(ClPlatformID::default())
    }
}

impl Platform {
    pub fn list_all() -> Output<Vec<Platform>> {
        ll::list_platforms().map(|plats| plats.into_iter().map(|p| Platform::new(p)).collect())
    }

    fn info(&self, info_code: PlatformInfo) -> Output<String> {
        ll::platform_info(self, info_code)
    }

    pub fn name(&self) -> Output<String> {
        self.info(PlatformInfo::Name)
    }

    pub fn version(&self) -> Output<String> {
        self.info(PlatformInfo::Version)
    }

    pub fn profile(&self) -> Output<String> {
        self.info(PlatformInfo::Profile)
    }

    pub fn vendor(&self) -> Output<String> {
        self.info(PlatformInfo::Vendor)
    }

    pub fn extensions(&self) -> Output<Vec<String>> {
        self.info(PlatformInfo::Extensions)
            .map(|exts| exts.split(' ').map(|ext| ext.to_string()).collect())
    }

    // v2.1
    // pub fn host_timer_resolution(&self) -> Output<String> {
    //     self.get_info(PlatformInfo::HostTimerResolution)
    // }
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Platform{{{:?}}}", self.platform_ptr())
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing, Platform};
    // use crate::device::{Device, DeviceType, DevicePtr};

    #[test]
    fn platform_func_default_works() {
        let _platform: Platform = Platform::default();
    }

    #[test]
    fn platform_func_all_works() {
        let platforms: Vec<Platform> = Platform::list_all().expect("Platform::list_all failed");
        assert!(platforms.len() > 0);
    }

    #[test]
    fn platform_has_methods_for_info() {
        let platforms = testing::get_platforms();
        assert_ne!(platforms.len(), 0);
        for platform in platforms.into_iter() {
            let empty_string = "".to_string();

            let name = platform
                .name()
                .expect("failed to get platform info for name");

            assert!(name != empty_string);

            let version = platform
                .version()
                .expect("failed to get platform info for version");

            assert!(version != empty_string);

            let profile = platform
                .profile()
                .expect("failed to get platform info for profile");

            assert!(profile != empty_string);

            let vendor = platform
                .vendor()
                .expect("failed to get platform info for vendor");

            assert!(vendor != empty_string);

            let extensions = platform
                .extensions()
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
}
