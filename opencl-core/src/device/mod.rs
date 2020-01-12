use std::default::Default;
use std::mem::ManuallyDrop;
use std::fmt;

// pub mod device_info;
pub mod device_ptr;
pub mod flags;
pub mod low_level;

pub use flags::{DeviceType, DeviceInfo};
pub use device_ptr::DevicePtr;

// use low_level::{cl_release_device_id, cl_retain_device_id};

use crate::error::{Error, Output};
use crate::platform::Platform;
use crate::utils;

pub use crate::ffi::cl_device_id;

/// NOTE: UNUSABLE_DEVICE_ID might be osx specific? or OpenCL
/// implementation specific?
/// UNUSABLE_DEVICE_ID was the cl_device_id encountered on my Macbook
/// Pro for a Radeon graphics card that becomes unavailable when
/// powersaving mode enables. Apparently the OpenCL platform can still
/// see the device, instead of a "legit" cl_device_id the inactive
/// device's cl_device_id is listed as 0xFFFF_FFFF.
pub const UNUSABLE_DEVICE_ID: cl_device_id = 0xFFFF_FFFF as *mut usize as cl_device_id;

pub const UNUSABLE_DEVICE_ERROR: Error = Error::DeviceError(DeviceError::UnusableDevice);

pub const NO_PARENT_DEVICE_ERROR: Error = Error::DeviceError(DeviceError::NoParentDevice);

pub fn device_usability_check(device_id: cl_device_id) -> Result<(), Error> {
    if device_id == UNUSABLE_DEVICE_ID {
        Err(UNUSABLE_DEVICE_ERROR)
    } else {
        Ok(())
    }    
}

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

pub trait DeviceRefCount: DevicePtr + fmt::Debug {
    unsafe fn from_retained(device: cl_device_id) -> Output<Self>;
    unsafe fn from_unretained(device: cl_device_id) -> Output<Self>;
}

unsafe fn release_device(device_id: cl_device_id) {
    low_level::cl_release_device_id(device_id).unwrap_or_else(|e| {
        panic!("Failed to release cl_device_id {:?} due to {:?} ", device_id, e);
    });
}

unsafe fn retain_device(device_id: cl_device_id) {
    low_level::cl_retain_device_id(device_id).unwrap_or_else(|e| {
        panic!("Failed to retain cl_device_id {:?} due to {:?}", device_id, e);
    });
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DeviceObject {
    object: cl_device_id,
    _unconstructable: (),
}

impl DeviceObject {
    pub unsafe fn unchecked_build(object: cl_device_id) -> DeviceObject {
        DeviceObject{
            object,
            _unconstructable: (),
        }
    }
}

impl DevicePtr for DeviceObject {
    unsafe fn device_ptr(&self) -> cl_device_id {
        self.object
    }
}

impl DeviceRefCount for DeviceObject {
    unsafe fn from_retained(device: cl_device_id) -> Output<DeviceObject> {
        utils::null_check(device, "DeviceObject::from_retained")?;
        device_usability_check(device)?;
        Ok(DeviceObject::unchecked_build(device))
    }

    unsafe fn from_unretained(device: cl_device_id) -> Output<DeviceObject> {
        utils::null_check(device, "DeviceObject::from_unretained")?;
        device_usability_check(device)?;
        retain_device(device);
        Ok(DeviceObject::unchecked_build(device))
    }
}

impl Drop for DeviceObject {
    fn drop(&mut self) {
        unsafe { release_device(self.device_ptr()) };
    }
}

impl Clone for DeviceObject {
    fn clone(&self) -> DeviceObject {
        unsafe {
            let device_id = self.device_ptr();
            retain_device(device_id);
            DeviceObject::unchecked_build(device_id)
        }
    }
}

#[derive(Hash)]
pub struct Device {
    inner: ManuallyDrop<DeviceObject>,
    _unconstructable: ()
}

impl Device {
    pub fn from_device_object(device_object: DeviceObject) -> Device {
        Device {
            inner: ManuallyDrop::new(device_object),
            _unconstructable: ()
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
            _unconstructable: ()
        }
    }
}

impl DeviceRefCount for Device {
    unsafe fn from_retained(device_id: cl_device_id) -> Output<Device> {
        let device_obj = DeviceObject::from_retained(device_id)?;
        Ok(Device::from_device_object(device_obj))
    }

    unsafe fn from_unretained(device_id: cl_device_id) -> Output<Device> {
        let device_obj = DeviceObject::from_unretained(device_id)?;
        Ok(Device::from_device_object(device_obj))
    }
}




impl Device {
    pub fn clone_slice<D: DevicePtr>(devices: &[D]) -> Output<Vec<Device>> {
        devices
            .iter()
            .map(|d| {
                // TODO: This approach is not sound. FIX ME.
                // This is super dangerous. A Device is contstructed and droppable
                // before the ptr is reference counted. Device needs a `from_unretained`.
                unsafe { 
                    Device::new(d.device_ptr())
                    .map(|dangerously_constructed_device| {
                        dangerously_constructed_device.clone()
                    })
                }
            })
            .collect()
    }
    
    pub unsafe fn new(device_id: cl_device_id) -> Output<Device> {
        let device_object = DeviceObject::from_retained(device_id)?;
        Ok(Device {
            inner: ManuallyDrop::new(device_object),
            _unconstructable: ()
        })
    }

    pub fn count_by_type(platform: &Platform, device_type: DeviceType) -> Output<u32> {
        low_level::cl_get_device_count(platform, device_type)
    }

    pub fn all_by_type(platform: &Platform, device_type: DeviceType) -> Output<Vec<Device>> {
        low_level::cl_get_device_ids(platform, device_type)
            .map(|cl_ptr| unsafe {
                cl_ptr
                    .into_vec()
                    .into_iter()
                    .map(|d| Device::new(d))
                    .filter_map(Result::ok)
                    .collect()
            })
    }

    pub fn default_devices(platform: &Platform) -> Output<Vec<Device>> {
        let ret = low_level::cl_get_device_ids(platform, DeviceType::DEFAULT)?;
        let devices: Vec<Device> = unsafe {
            ret.into_vec()
                .into_iter()
                .map(|d| Device::new(d))
                .filter_map(Result::ok)
                .collect()
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

impl DevicePtr for Device {
    unsafe fn device_ptr(&self) -> cl_device_id {
        (*self.inner).object
    }
}

impl DevicePtr for &Device {
    unsafe fn device_ptr(&self) -> cl_device_id {
        (*self.inner).object
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Default for Device {
    fn default() -> Device {
        let device = Platform::default()
            .default_device()
            .unwrap_or_else(|e| panic!("Failed to find default device {:?}", e));

        device.usability_check().unwrap();
        device
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(self.device_ptr(), other.device_ptr()) }
    }
}

impl Eq for Device {}


 impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name().unwrap();
        let ptr = unsafe { self.device_ptr() };
        write!(f, "Device{{ptr: {:?}, name: {}}}", ptr, name)
    }
}

#[cfg(test)]
mod tests {
    use super::Device as Device;
    use super::{DeviceType, UNUSABLE_DEVICE_ERROR, DevicePtr};
    use crate::ffi::cl_device_id;
    use crate::platform::Platform;

    fn get_device() -> Device {
        let platform = Platform::default();
        let mut devices: Vec<Device> = Device::all_by_type(&platform, DeviceType::ALL).expect("Failed to list all devices");
        assert!(devices.len() > 0);
        devices.remove(0)
    }

    #[test]
    fn unusable_device_id_results_in_an_unusable_device_error() {
        let unusable_device_id = 0xFFFF_FFFF as cl_device_id;
        let error =
            unsafe { Device::new(unusable_device_id) };
        assert_eq!(error, Err(UNUSABLE_DEVICE_ERROR));
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

    #[test]
    fn device_fmt_works() {
        let device = get_device();
        let formatted = format!("{:?}", device);
        assert!(formatted.starts_with("Device{ptr: 0x")); //== "".contains("Device{"));
    }

    #[test]
    fn device_name_works() {
        let device = get_device();
        let name: String = device.name().unwrap();
        assert!(name.len() > 0);
    }
}
