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

use crate::ll::{ClPlatformID, PlatformPtr, PlatformInfo, Output};
use crate::ll;

use ffi::cl_platform_id;

pub struct Platform {
    inner: ClPlatformID,
    _unconstructable: ()
}

impl Platform {
    pub fn new(p: ClPlatformID) -> Platform {
        Platform { inner: p, _unconstructable: () }
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
        ll::list_platforms().map(|plats| {
            plats.into_iter().map(|p| Platform::new(p)).collect()
        })
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

// impl Default for Platform {
//     fn default() -> Platform {
//         let mut platforms =
//             Platform::all().expect("Failed to list platforms for Platform::default()");
//         if platforms.is_empty() {
//             panic!("No platforms during Platform::default()");
//         }
//         platforms.remove(0)
//     }
// }

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Platform{{{:?}}}", self.platform_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::Platform;
    // use crate::device::{Device, DeviceType, DevicePtr};

    // fn get_platform() -> Platform {
    //     Platform::default()
    // }

    #[test]
    fn platform_func_default_works() {
        let _platform: Platform = Platform::default();
    }

    #[test]
    fn platform_func_all_works() {
        let platforms: Vec<Platform> = Platform::list_all().expect("Platform::list_all failed");
        assert!(platforms.len() > 0);
    }

    // #[test]
    // fn platform_can_list_all_devices() {
    //     let platform = get_platform();
    //     let devices = platform
    //         .all_devices()
    //         .expect("failed to list all platform devices");
    //     assert!(devices.len() > 0);
    // }

    // #[test]
    // fn platform_can_list_devices_by_type() {
    //     let platform = get_platform();
    //     let _cpus = platform.all_devices_by_type(DeviceType::CPU);
    //     let _gpus = platform.all_devices_by_type(DeviceType::GPU);
    //     let _accelerators = platform.all_devices_by_type(DeviceType::ACCELERATOR);
    //     let _custom = platform.all_devices_by_type(DeviceType::CUSTOM);
    // }

    // #[test]
    // fn platform_has_one_or_more_default_device() {
    //     let platform = get_platform();
    //     let devices: Vec<Device> = platform
    //         .default_devices()
    //         .expect("failed to find a default device for the platform");
    //     assert!(devices.len() > 0);
    // }

    // #[test]
    // fn platform_can_select_one_usable_default_device_best_effort() {
    //     let platform = get_platform();
    //     let device: Device = platform
    //         .default_device()
    //         .expect("failed to find a default device for the platform");
    //     // fetching a name means it is usable.
    //     assert!(device.is_usable() == true);
    //     let _name = device.name().expect("failed to fetch name_info on device");
    // }

    // #[test]
    // fn platform_has_methods_for_listing_devices_by_type() {
    //     let platform = get_platform();

    //     let cpus_flag = platform.all_devices_by_type(DeviceType::CPU);
    //     let cpus_method = platform.cpu_devices();
    //     assert_eq!(cpus_flag, cpus_method);

    //     let gpus_flag = platform.all_devices_by_type(DeviceType::GPU);
    //     let gpus_method = platform.gpu_devices();
    //     assert_eq!(gpus_flag, gpus_method);

    //     let accel_flag = platform.all_devices_by_type(DeviceType::ACCELERATOR);
    //     let accel_method = platform.accelerator_devices();
    //     assert_eq!(accel_flag, accel_method);

    //     let custom_flag = platform.all_devices_by_type(DeviceType::CUSTOM);
    //     let custom_method = platform.custom_devices();
    //     assert_eq!(custom_flag, custom_method);
    // }

    // #[test]
    // fn platform_can_get_the_first_default_device() {
    //     let device = Platform::get_any_default_device()
    //         .expect("Call to Platform::get_any_default_device() failed.");
    //     assert!(device.is_usable() == true);
    //     let _name = device.name().expect("Failed to fetch Device name");
    // }

    // #[test]
    // fn platform_has_methods_for_info() {
    //     let platform = get_platform();
    //     let empty_string = "".to_string();

    //     let name = platform
    //         .name()
    //         .expect("failed to get platform info for name");

    //     assert!(name != empty_string);

    //     let version = platform
    //         .version()
    //         .expect("failed to get platform info for version");

    //     assert!(version != empty_string);

    //     let profile = platform
    //         .profile()
    //         .expect("failed to get platform info for profile");

    //     assert!(profile != empty_string);

    //     let vendor = platform
    //         .vendor()
    //         .expect("failed to get platform info for vendor");

    //     assert!(vendor != empty_string);

    //     let extensions = platform
    //         .extensions()
    //         .expect("failed to get platform info for extensions");

    //     for ext in extensions.into_iter() {
    //         assert!(ext != empty_string);
    //     }
    //     // v2.1
    //     // let host_timer_resolution = platform.host_timer_resolution()
    //     //     .expect("failed to get platform info for host_timer_resolution");

    //     // assert_eq!(host_timer_resolution, "".to_string());
    // }
}
