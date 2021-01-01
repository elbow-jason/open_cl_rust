use std::fmt;
use std::mem::ManuallyDrop;

use crate::Platform;

use crate::ll::cl::{cl_device_id, ClObject, DeviceType};
use crate::ll::{Device as ClDeviceID, DevicePtr, Output};

pub struct Device {
    inner: ManuallyDrop<ClDeviceID>,
    _unconstructable: (),
}

impl Device {
    pub fn new(device_id: ClDeviceID) -> Device {
        Device {
            inner: ManuallyDrop::new(device_id),
            _unconstructable: (),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
        }
    }
}

impl Clone for Device {
    fn clone(&self) -> Device {
        Device {
            inner: ManuallyDrop::new((*self.inner).clone()),
            _unconstructable: (),
        }
    }
}

impl DevicePtr for Device {
    unsafe fn device_ptr(&self) -> cl_device_id {
        self.inner.device_ptr()
    }
}

impl Device {
    pub fn low_level_device(&self) -> &ClDeviceID {
        &*self.inner
    }

    pub fn list_devices_by_type(
        platform: &Platform,
        device_type: DeviceType,
    ) -> Output<Vec<Device>> {
        platform.list_devices_by_type(device_type)
    }

    pub fn list_default_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::DEFAULT)
    }

    pub fn list_all_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::ALL)
    }

    pub fn list_cpu_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::CPU)
    }

    pub fn list_gpu_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::GPU)
    }

    pub fn list_accelerator_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::ACCELERATOR)
    }

    pub fn list_custom_devices(platform: &Platform) -> Output<Vec<Device>> {
        Device::list_devices_by_type(platform, DeviceType::CUSTOM)
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.low_level_device() == other.low_level_device()
    }
}

impl Eq for Device {}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let name = self.device_name().unwrap();
        let ptr = unsafe { self.low_level_device().device_ptr() };
        write!(f, "Device{{ptr: {}}}", ptr.address())
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use super::DeviceType;
    use crate::ll::cl::*;
    use crate::ll::*;
    use crate::platform::Platform;
    use crate::testing;

    #[test]
    fn device_all_lists_all_devices() {
        let platform = Platform::default();
        let devices = Device::list_all_devices(&platform).expect("Failed to list all devices");
        assert!(devices.len() > 0);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform() {
        let platform = Platform::default();
        let _ = Device::list_default_devices(&platform);
        let _ = Device::list_cpu_devices(&platform);
        let _ = Device::list_gpu_devices(&platform);
        let _ = Device::list_accelerator_devices(&platform);
        let _ = Device::list_custom_devices(&platform);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform_via_flags() {
        let platform = Platform::default();
        let _ = Device::list_devices_by_type(&platform, DeviceType::ALL);
        let _ = Device::list_devices_by_type(&platform, DeviceType::CPU);
        let _ = Device::list_devices_by_type(&platform, DeviceType::GPU);
        let _ = Device::list_devices_by_type(&platform, DeviceType::ACCELERATOR);
        let _ = Device::list_devices_by_type(&platform, DeviceType::CUSTOM);
    }

    #[test]
    fn device_fmt_works() {
        let device = testing::get_device();
        let formatted = format!("{:?}", device);
        assert!(formatted.starts_with("Device{ptr: 0x")); //== "".contains("Device{"));
    }

    #[test]
    fn device_implements_device_ptr() {
        let device = testing::get_device();
        let device_id: cl_device_id = unsafe { device.device_ptr() };
        let device_ptr = unsafe { device_id.as_ptr() };
        assert!(!device_ptr.is_null());
        assert_ne!(device_ptr, 0xFFFF_FFFF as *mut libc::c_void);
    }
}
