pub mod device_info;
pub mod low_level;
pub mod flags;

use low_level::{cl_retain_device_id, cl_release_device_id};
use flags::DeviceType;

use crate::ffi::cl_device_id;
use crate::error::{Error, Output};
use crate::platform::Platform;


/// NOTE: UNUSABLE_DEVICE_ID might be osx specific? or OpenCL
/// implementation specific?
/// UNUSABLE_DEVICE_ID was the cl_device_id encountered on my Macbook
/// Pro for a Radeon graphics card that becomes unavailable when
/// powersaving mode enables. Apparently the OpenCL platform can still
/// see the device, instead of a "legit" cl_device_id the inactive
/// device's cl_device_id is listed as 0xFFFF_FFFF.
const UNUSABLE_DEVICE_ID: cl_device_id = 0xFFFF_FFFF as *mut usize as cl_device_id;

/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum DeviceError {
    #[fail(display = "Device is not in a usable state")]
    UnusableDevice,
}

impl From<DeviceError> for Error {
    fn from(err: DeviceError) -> Error {
        Error::DeviceError(err)
    }
}

__impl_unconstructable_cl_wrapper!(Device, cl_device_id);
__impl_cl_object_for_wrapper!(Device, cl_device_id);
#[cfg(feature = "opencl_version_1_2_0")]
__impl_clone_for_cl_object_wrapper!(Device, cl_retain_device_id);
#[cfg(feature = "opencl_version_1_2_0")]
__impl_drop_for_cl_object_wrapper!(Device, cl_release_device_id);


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

    pub fn all(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::ALL)
    }

    pub fn count_by_type(platform: &Platform, device_type: DeviceType) -> Output<u32> {
        low_level::cl_get_device_count(platform, device_type.bits())
    }

    pub fn all_by_type(platform: &Platform, device_type: DeviceType) -> Output<Vec<Device>> {
        low_level::cl_get_device_ids(platform, device_type.bits())
    }
}



// #[cfg(feature = "opencl_version_1_2_0")]
// clCreateSubDevices clRetainDevice
