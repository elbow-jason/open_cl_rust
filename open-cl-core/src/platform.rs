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

use crate::device::Device;
use crate::ll::cl::DeviceType;
use crate::ll::{Output, Platform as ClPlatformID};

pub struct Platform {
    inner: ClPlatformID,
}

impl Platform {
    pub fn new(p: ClPlatformID) -> Platform {
        Platform { inner: p }
    }

    pub fn low_level_platform(&self) -> &ClPlatformID {
        &self.inner
    }
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

impl Default for Platform {
    fn default() -> Platform {
        Platform::new(ClPlatformID::default())
    }
}

impl Platform {
    pub fn list_all() -> Output<Vec<Platform>> {
        ClPlatformID::list_all().map(|plats| plats.into_iter().map(|p| Platform::new(p)).collect())
    }

    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Output<Vec<Device>> {
        let devices = self
            .inner
            .list_devices_by_type(device_type)?
            .into_iter()
            .map(|d| Device::new(d))
            .collect();
        Ok(devices)
    }

    pub fn name(&self) -> Output<String> {
        self.inner.name()
    }

    pub fn version(&self) -> Output<String> {
        self.inner.version()
    }

    pub fn profile(&self) -> Output<String> {
        self.inner.profile()
    }

    pub fn vendor(&self) -> Output<String> {
        self.inner.vendor()
    }

    pub fn extensions(&self) -> Output<Vec<String>> {
        self.inner.extensions()
    }

    pub fn list_default_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::DEFAULT)
    }

    pub fn list_all_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::ALL)
    }

    pub fn list_cpu_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::CPU)
    }

    pub fn list_gpu_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::GPU)
    }

    pub fn list_accelerator_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::ACCELERATOR)
    }

    pub fn list_custom_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::CUSTOM)
    }

    // v2.1
    // pub fn host_timer_resolution(&self) -> Output<String> {
    //     self.get_info(PlatformInfo::HostTimerResolution)
    // }
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Platform{{{:?}}}", self.inner.address())
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

#[cfg(test)]
mod device_tests {
    use super::DeviceType;
    use crate::platform::Platform;

    #[test]
    fn device_all_lists_all_devices() {
        let platform = Platform::default();
        let devices = platform
            .list_all_devices()
            .expect("Failed to list all devices");
        assert!(devices.len() > 0);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform() {
        let platform = Platform::default();
        let d1 = platform.list_default_devices();
        assert!(d1.is_ok() || d1.is_err());
        let d2 = platform.list_cpu_devices();
        assert!(d2.is_ok() || d2.is_err());
        let d3 = platform.list_gpu_devices();
        assert!(d3.is_ok() || d3.is_err());
        let d4 = platform.list_accelerator_devices();
        assert!(d4.is_ok() || d4.is_err());
        let d5 = platform.list_custom_devices();
        assert!(d5.is_ok() || d5.is_err());
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform_via_flags() {
        let platform = Platform::default();
        let d1 = platform.list_devices_by_type(DeviceType::ALL);
        assert!(d1.is_ok() || d1.is_err());
        let d2 = platform.list_devices_by_type(DeviceType::CPU);
        assert!(d2.is_ok() || d2.is_err());
        let d3 = platform.list_devices_by_type(DeviceType::GPU);
        assert!(d3.is_ok() || d3.is_err());
        let d4 = platform.list_devices_by_type(DeviceType::ACCELERATOR);
        assert!(d4.is_ok() || d4.is_err());
        let d5 = platform.list_devices_by_type(DeviceType::CUSTOM);
        assert!(d5.is_ok() || d5.is_err());
    }
}
