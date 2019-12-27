use std::default::Default;

pub mod device_info;
pub mod flags;
pub mod low_level;

pub use flags::DeviceType;

use low_level::{cl_release_device_id, cl_retain_device_id};

use crate::error::{Error, Output};
use crate::ffi::cl_device_id;
use crate::platform::Platform;

/// NOTE: UNUSABLE_DEVICE_ID might be osx specific? or OpenCL
/// implementation specific?
/// UNUSABLE_DEVICE_ID was the cl_device_id encountered on my Macbook
/// Pro for a Radeon graphics card that becomes unavailable when
/// powersaving mode enables. Apparently the OpenCL platform can still
/// see the device, instead of a "legit" cl_device_id the inactive
/// device's cl_device_id is listed as 0xFFFF_FFFF.
const UNUSABLE_DEVICE_ID: cl_device_id = 0xFFFF_FFFF as *mut usize as cl_device_id;

/// An error related to a Device.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum DeviceError {
    #[fail(display = "Device is not in a usable state")]
    UnusableDevice,

    #[fail(display = "The given platform had no default device")]
    NoDefaultDevice,

    #[fail(display = "The given device had no parent device")]
    NoParentDevice,
}

impl From<DeviceError> for Error {
    fn from(err: DeviceError) -> Error {
        Error::DeviceError(err)
    }
}

__impl_unconstructable_cl_wrapper!(Device, cl_device_id);
__impl_default_debug_for!(Device);
__impl_cl_object_for_wrapper!(
    Device,
    cl_device_id,
    cl_retain_device_id,
    cl_release_device_id
);
__impl_clone_for_cl_object_wrapper!(Device, cl_retain_device_id);
__impl_drop_for_cl_object_wrapper!(Device, cl_release_device_id);

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub fn is_usable(&self) -> bool {
        self.inner != UNUSABLE_DEVICE_ID
    }

    pub fn usability_check(&self) -> Output<()> {
        if self.is_usable() {
            Ok(())
        } else {
            Err(DeviceError::UnusableDevice.into())
        }
    }

    pub fn count_by_type(platform: &Platform, device_type: DeviceType) -> Output<u32> {
        low_level::cl_get_device_count(platform, device_type)
    }

    pub fn all_by_type(platform: &Platform, device_type: DeviceType) -> Output<Vec<Device>> {
        low_level::cl_get_device_ids(platform, device_type)
            .and_then(|ret| unsafe { ret.into_many_retained_wrappers() })
    }

    pub fn default_devices(platform: &Platform) -> Output<Vec<Device>> {
        let ret = low_level::cl_get_device_ids(platform, DeviceType::DEFAULT)?;
        let devices: Vec<Device> = unsafe {
            ret.into_many_retained_wrappers().unwrap_or_else(|e| {
                panic!("Failed to get default devices due to {:?}", e);
            })
        };
        match devices.len() {
            0 => Err(DeviceError::NoDefaultDevice.into()),
            _ => Ok(devices),
        }
    }

    pub fn all(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::ALL)
    }

    pub fn cpu_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::CPU)
    }

    pub fn gpu_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::GPU)
    }

    pub fn accelerator_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::ACCELERATOR)
    }

    pub fn custom_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::CUSTOM)
    }
}

impl Default for Device {
    fn default() -> Device {
        Platform::default()
            .default_device()
            .expect("Failed to find default device")
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, DeviceError, DeviceType};
    use crate::cl::ClObject;
    use crate::error::Error;
    use crate::ffi::cl_device_id;
    use crate::platform::Platform;

    #[test]
    fn unusable_device_id_is_unusable() {
        let unusable_device_id = 0xFFFF_FFFF as cl_device_id;
        let device =
            unsafe { Device::new(unusable_device_id).expect("Failed to create new device!") };
        assert_eq!(device.is_usable(), false);
    }

    #[test]
    fn unusable_device_check_errors_for_unusable_device_id() {
        let unusable_device_id = 0xFFFF_FFFF as cl_device_id;
        let device =
            unsafe { Device::new(unusable_device_id).expect("Failed to create new device!") };
        assert_eq!(
            device.usability_check(),
            Err(Error::DeviceError(DeviceError::UnusableDevice))
        );
    }

    #[test]
    fn device_all_lists_all_devices() {
        let platform = Platform::default();
        let devices = Device::all(&platform).expect("Failed to list all devices");
        assert!(devices.len() > 0);
    }

    #[test]
    fn device_has_a_default_that_is_usable() {
        let device = Device::default();
        assert!(device.is_usable() == true);
        let _name = device.name().expect("Failed to get name of device");
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform() {
        let platform = Platform::default();
        let _ = Device::default_devices(&platform);
        let _ = Device::cpu_devices(&platform);
        let _ = Device::gpu_devices(&platform);
        let _ = Device::accelerator_devices(&platform);
        let _ = Device::custom_devices(&platform);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform_via_flags() {
        let platform = Platform::default();
        let _ = Device::all_by_type(&platform, DeviceType::ALL);
        let _ = Device::all_by_type(&platform, DeviceType::CPU);
        let _ = Device::all_by_type(&platform, DeviceType::GPU);
        let _ = Device::all_by_type(&platform, DeviceType::ACCELERATOR);
        let _ = Device::all_by_type(&platform, DeviceType::CUSTOM);
    }
}
