/// Platform serves 3 basic purposes:
///
/// Platform is the interface for listing platforms.
///
/// Platform is the interface for getting metadata about platforms.
///
/// Platform is the interface for listing Devices.
///
/// NOTE: ClPlatformID is tested!
use std::default::Default;

use crate::cl::{cl_platform_id, ClObject, ObjectWrapper};
use crate::cl::{cl_platform_info, DeviceType, PlatformInfo};

use crate::{Device, Error, Output};

use super::functions;

/// An error related to Platform.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PlatformError {
    #[error("No platforms found!")]
    NoPlatforms,

    #[error("The platform has no useable devices!")]
    NoUsableDevices,

    #[error("The given platform had no default device!")]
    NoDefaultDevice,
}

pub type Platform = ObjectWrapper<cl_platform_id>;

impl Platform {
    pub fn list_all() -> Output<Vec<Platform>> {
        unsafe {
            let platform_ids = functions::list_platform_ids()?;
            let mut plats = Vec::with_capacity(platform_ids.len());
            for id in platform_ids.into_iter() {
                // this program should not continue if the FFI returns null pointers.
                id.check().unwrap();
                plats.push(Platform::new(id));
            }
            Ok(plats)
        }
    }

    pub fn list_devices(&self) -> Output<Vec<Device>> {
        self.list_devices_by_type(DeviceType::ALL)
    }

    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Output<Vec<Device>> {
        let device_ids = unsafe { functions::list_devices(self.cl_object(), device_type.bits()) }?;
        let mut devices = Vec::with_capacity(device_ids.len());
        for device_id in device_ids.into_iter() {
            match device_id.check() {
                Ok(()) => devices.push(unsafe { Device::new(device_id) }),
                Err(_) => (),
            }
        }
        Ok(devices)
    }

    pub fn info<I: Into<cl_platform_info>>(&self, info_code: I) -> Output<String> {
        unsafe { functions::get_platform_info(self.cl_object(), info_code.into()) }
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
}

// v2.1
// pub fn host_timer_resolution(&self) -> Output<String> {
//     self.get_info(PlatformInfo::HostTimerResolution)
// }

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

fn _default_platform() -> Output<Platform> {
    let mut platforms = Platform::list_all()?;

    if platforms.is_empty() {
        return Err(PlatformError::NoPlatforms)?;
    }
    Ok(platforms.remove(0))
}

impl Default for Platform {
    fn default() -> Platform {
        _default_platform().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cl_get_platforms() {
        let all_platforms: Vec<Platform> = Platform::list_all().unwrap();
        assert!(all_platforms.len() > 0);
        for platform in all_platforms.into_iter() {
            let name = platform.name().unwrap();
            assert!(name.len() > 0);
        }
    }

    #[test]
    fn platform_func_default_works() {
        let _platform: Platform = Platform::default();
    }

    #[test]
    fn platform_func_all_works() {
        let platforms: Vec<Platform> = Platform::list_all().expect("list_platforms() failed");
        assert!(platforms.len() > 0);
    }

    #[test]
    fn platform_has_functions_getting_for_info() {
        let platform = Platform::default();
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
