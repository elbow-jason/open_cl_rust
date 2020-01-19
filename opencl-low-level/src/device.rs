use std::fmt;

use crate::ffi::*;

use crate::{
    Output, DeviceInfo, DeviceExecCapabilities, DeviceLocalMemType,
    DeviceMemCacheType, DeviceType, DeviceAffinityDomain,
    ClPointer, PlatformPtr, utils, Error, ClPlatformID,
    StatusCodeError
};

use crate::cl_helpers::{cl_get_object, cl_get_object_count, cl_get_info5};

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

#[cfg(feature = "opencl_version_1_2_0")]
__release_retain!(device_id, Device);

// NOTE: fix cl_device_type
pub fn cl_get_device_count(platform: cl_platform_id, device_type: cl_device_type) -> Output<u32> {
    unsafe { 
        cl_get_object_count::<cl_platform_id, cl_device_type, cl_device_id>(
            platform,
            device_type,
            clGetDeviceIDs,
        )
    }
}

pub fn list_devices_by_type(
    platform: &ClPlatformID,
    device_type: DeviceType,
) -> Output<Vec<ClDeviceID>> {
    unsafe {
        match cl_get_object(platform.platform_ptr(), device_type.into(), clGetDeviceIDs) {
            Ok(cl_ptr) => {
                let devices: Vec<ClDeviceID> = cl_ptr
                    .into_vec()
                    .into_iter()
                    .map(|d| ClDeviceID::from_retained(d))
                    .filter_map(Result::ok)
                    .collect();
                Ok(devices)
            },
            Err(Error::StatusCodeError(StatusCodeError{status_code: -1})) => Ok(vec![]),
            Err(Error::StatusCodeError(StatusCodeError{status_code: -30})) => Ok(vec![]),
            Err(e) => Err(e),
        }
        
    }
}


pub unsafe fn cl_get_device_info<T>(device: cl_device_id, flag: DeviceInfo) -> Output<ClPointer<T>>
where
    T: Copy,
{
    device_usability_check(device)?;
    cl_get_info5(device.device_ptr(), flag.into(), clGetDeviceInfo)
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

pub trait DeviceRefCount: DevicePtr + fmt::Debug {
    unsafe fn from_retained(device: cl_device_id) -> Output<Self>;
    unsafe fn from_unretained(device: cl_device_id) -> Output<Self>;
}

unsafe fn release_device(device_id: cl_device_id) {
    cl_release_device_id(device_id).unwrap_or_else(|e| {
        panic!("Failed to release cl_device_id {:?} due to {:?} ", device_id, e);
    });
}

unsafe fn retain_device(device_id: cl_device_id) {
    cl_retain_device_id(device_id).unwrap_or_else(|e| {
        panic!("Failed to retain cl_device_id {:?} due to {:?}", device_id, e);
    });
}

#[derive(Hash)]
pub struct ClDeviceID {
    object: cl_device_id,
    _unconstructable: (),
}

impl ClDeviceID {
    pub unsafe fn unchecked_build(object: cl_device_id) -> ClDeviceID {
        ClDeviceID{
            object,
            _unconstructable: (),
        }
    }
}

impl DevicePtr for ClDeviceID {
    unsafe fn device_ptr(&self) -> cl_device_id {
        self.object
    }
}

impl DevicePtr for &ClDeviceID {
    unsafe fn device_ptr(&self) -> cl_device_id {
        self.object
    }
}

impl DevicePtr for cl_device_id {
    unsafe fn device_ptr(&self) -> cl_device_id {
        *self
    }
}



impl DeviceRefCount for ClDeviceID {
    unsafe fn from_retained(device: cl_device_id) -> Output<ClDeviceID> {
        utils::null_check(device)?;
        device_usability_check(device)?;
        Ok(ClDeviceID::unchecked_build(device))
    }

    unsafe fn from_unretained(device: cl_device_id) -> Output<ClDeviceID> {
        utils::null_check(device)?;
        device_usability_check(device)?;
        retain_device(device);
        Ok(ClDeviceID::unchecked_build(device))
    }
}

impl Drop for ClDeviceID {
    fn drop(&mut self) {
        unsafe { release_device(self.device_ptr()) };
    }
}

impl Clone for ClDeviceID {
    fn clone(&self) -> ClDeviceID {
        unsafe {
            let device_id = self.device_ptr();
            retain_device(device_id);
            ClDeviceID::unchecked_build(device_id)
        }
    }
}
 
// pub unsafe fn list_devices_by_type<P, D>(platform: P, device_type: D) -> Output<Vec<ClDeviceID>> where P: PlatformPtr, D: Into<cl_device_type> {
//     cl_get_device_ids(platform, device_type)
// }

// pub unsafe fn list_default_devices(platform: &ClPlatformID) -> Output<Vec<ClDeviceID>> {
//     let devices: Vec<ClDeviceID> = list_devices_by_type(platform, DeviceType::DEFAULT)?;
//     match devices.len() {
//         0 => Err(DeviceError::NoDefaultDevice.into()),
//         _ => Ok(devices),
//     }
// }

macro_rules! info_fn {
    ($name:ident, $flag:ident, String) => {
        paste::item! {
            fn [<device_ $name>](&self) -> Output<String> {
                unsafe{ 
                    cl_get_device_info(self.device_ptr(), DeviceInfo::$flag)
                        .map(|ret| ret.into_string() )
                }
            }
        }
    };

    ($name:ident, $flag:ident, bool) => {
        paste::item! {
            fn [<device_ $name>](&self) -> Output<bool> {
                use crate::ffi::cl_bool;
                unsafe {
                    cl_get_device_info::<cl_bool>(self.device_ptr(), DeviceInfo::$flag).map(From::from)
                }
            }
        }
    };

    ($name:ident, $flag:ident, $cl_type:ty, Vec<$output_t:ty>) => {
        paste::item! {
            fn [<device_ $name>](&self) -> Output<Vec<$output_t>> {
                unsafe { 
                    cl_get_device_info(self.device_ptr(), DeviceInfo::$flag).map(|ret| ret.into_vec())
                }
            }
        }
    };

    ($name:ident, $flag:ident, $output_t:ty) => {
        paste::item! {
            fn [<device_ $name>](&self) -> Output<$output_t> {
                unsafe {
                    cl_get_device_info(self.device_ptr(), DeviceInfo::$flag).map(|ret| ret.into_one())
                }
            }
        }
    };

    ($name:ident, $flag:ident, $cl_type:ty, $output_t:ty) => {
        paste::item! {
            fn [<device_ $name>](&self) -> Output<$output_t> {
                unsafe {
                    cl_get_device_info(self.device_ptr(), DeviceInfo::$flag)
                        .map(|ret| ret.into_one())
                }
            }
        }
    };
}

pub trait DevicePtr where Self: fmt::Debug + Sized {
    unsafe fn device_ptr(&self) -> cl_device_id;

    fn is_usable(&self) -> bool {
        unsafe { self.device_ptr() != UNUSABLE_DEVICE_ID }
    }

    fn usability_check(&self) -> Output<()> {
        if self.is_usable() {
            Ok(())
        } else {
            Err(DeviceError::UnusableDevice.into())
        }
    }

    info_fn!(
        global_mem_cacheline_size,
        GlobalMemCachelineSize,
        cl_uint,
        u32
    );
    info_fn!(
        native_vector_width_double,
        NativeVectorWidthDouble,
        cl_uint,
        u32
    );
    info_fn!(
        native_vector_width_half,
        NativeVectorWidthHalf,
        cl_uint,
        u32
    );
    info_fn!(address_bits, AddressBits, cl_uint, u32);
    info_fn!(max_clock_frequency, MaxClockFrequency, cl_uint, u32);
    info_fn!(max_compute_units, MaxComputeUnits, cl_uint, u32);
    info_fn!(max_constant_args, MaxConstantArgs, cl_uint, u32);
    info_fn!(max_read_image_args, MaxReadImageArgs, cl_uint, u32);
    info_fn!(max_samplers, MaxSamplers, cl_uint, u32);
    info_fn!(max_work_item_dimensions, MaxWorkItemDimensions, cl_uint, u32);
    info_fn!(max_write_image_args, MaxWriteImageArgs, cl_uint, u32);
    info_fn!(mem_base_addr_align, MemBaseAddrAlign, cl_uint, u32);
    info_fn!(min_data_type_align_size, MinDataTypeAlignSize, cl_uint, u32);
    info_fn!(native_vector_width_char, NativeVectorWidthChar, cl_uint, u32);
    info_fn!(native_vector_width_short, NativeVectorWidthShort, cl_uint, u32);
    info_fn!(native_vector_width_int, NativeVectorWidthInt, cl_uint, u32);
    info_fn!(native_vector_width_long, NativeVectorWidthLong, cl_uint, u32);
    info_fn!(native_vector_width_float, NativeVectorWidthFloat, cl_uint, u32);
    info_fn!(
        partition_max_sub_devices,
        PartitionMaxSubDevices,
        cl_uint,
        u32
    );
    info_fn!(preferred_vector_width_char, PreferredVectorWidthChar, cl_uint, u32);
    info_fn!(preferred_vector_width_short, PreferredVectorWidthShort, cl_uint, u32);
    info_fn!(preferred_vector_width_int, PreferredVectorWidthInt, cl_uint, u32);
    info_fn!(preferred_vector_width_long, PreferredVectorWidthLong, cl_uint, u32);
    info_fn!(preferred_vector_width_float, PreferredVectorWidthFloat, cl_uint, u32);
    info_fn!(
        preferred_vector_width_double,
        PreferredVectorWidthDouble,
        cl_uint,
        u32
    );
    info_fn!(preferred_vector_width_half, PreferredVectorWidthHalf, cl_uint, u32);
    info_fn!(vendor_id, VendorId, cl_uint, u32);

    // cl_bool
    info_fn!(available, Available, bool);
    info_fn!(compiler_available, CompilerAvailable, bool);
    info_fn!(endian_little, EndianLittle, bool);
    info_fn!(error_correction_support, ErrorCorrectionSupport, bool);
    info_fn!(host_unified_memory, HostUnifiedMemory, bool);
    info_fn!(image_support, ImageSupport, bool);
    info_fn!(linker_available, LinkerAvailable, bool);
    info_fn!(preferred_interop_user_sync, PreferredInteropUserSync, bool);

    // char[]
    info_fn!(name, Name, String);
    info_fn!(opencl_c_version, OpenclCVersion, String);
    info_fn!(profile, Profile, String);
    info_fn!(vendor, Vendor, String);
    info_fn!(version, Version, String);
    info_fn!(driver_version, DriverVersion, String);

    // ulong as u64
    info_fn!(global_mem_cache_size, GlobalMemCacheSize, cl_ulong, u64);
    info_fn!(global_mem_size, GlobalMemSize, cl_ulong, u64);
    info_fn!(local_mem_size, LocalMemSize, cl_ulong, u64);
    info_fn!(max_constant_buffer_size, MaxConstantBufferSize, cl_ulong, u64);
    info_fn!(max_mem_alloc_size, MaxMemAllocSize, cl_ulong, u64);

    // size_t as usize
    info_fn!(image2d_max_width, Image2DMaxWidth, size_t, usize);
    info_fn!(image2d_max_height, Image2DMaxHeight, size_t, usize);
    info_fn!(image3d_max_width, Image3DMaxWidth, size_t, usize);
    info_fn!(image3d_max_height, Image3DMaxHeight, size_t, usize);
    info_fn!(image3d_max_depth, Image3DMaxDepth, size_t, usize);
    info_fn!(image_max_buffer_size, ImageMaxBufferSize, size_t, usize);
    info_fn!(image_max_array_size, ImageMaxArraySize, size_t, usize);
    info_fn!(max_parameter_size, MaxParameterSize, size_t, usize);
    info_fn!(max_work_group_size, MaxWorkGroupSize, size_t, usize);
    info_fn!(printf_buffer_size, PrintfBufferSize, size_t, usize);
    info_fn!(profiling_timer_resolution, ProfilingTimerResolution, size_t, usize);

    // size_t[]
    info_fn!(max_work_item_sizes, MaxWorkItemSizes, size_t, Vec<usize>);

    // cl_device_local_mem_type
    info_fn!(local_mem_type, LocalMemType, cl_device_local_mem_type, DeviceLocalMemType);

    // ExecutionCapabilities
    info_fn!(
        execution_capabilities,
        ExecutionCapabilities,
        cl_device_exec_capabilities,
        DeviceExecCapabilities
    );

    //  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
    info_fn!(
        global_mem_cache_type,
        GlobalMemCacheType,
        cl_device_mem_cache_type,
        DeviceMemCacheType
    );

    // cl_device_affinity_domain
    info_fn!(
        partition_affinity_domain,
        PartitionAffinityDomain,
        cl_device_affinity_domain,
        DeviceAffinityDomain
    );

    // DeviceType
    info_fn!(device_type, Type, cl_device_type, DeviceType);
}

// pub fn default_device(platform: PlatformPtr) -> Device {
//     Device {

//     }
// }

unsafe impl Send for ClDeviceID {}
unsafe impl Sync for ClDeviceID {}

// impl Default for Device {
//     fn default() -> Device {
//         let platform = Platform::default();
//         let device = default_device(platform)
//             .unwrap_or_else(|e| panic!("Failed to find default device {:?}", e));

//         device.usability_check().unwrap();
//         device
//     }
// }

impl PartialEq for ClDeviceID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(self.device_ptr(), other.device_ptr()) }
    }
}

impl Eq for ClDeviceID {}


impl fmt::Debug for ClDeviceID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.device_name().unwrap();
        let ptr = unsafe { self.device_ptr() };
        write!(f, "ClDeviceID{{ptr: {:?}, name: {}}}", ptr, name)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::ffi::*;

    #[test]
    fn unusable_device_id_results_in_an_unusable_device_error() {
        let unusable_device_id = 0xFFFF_FFFF as cl_device_id;
        let error =
            unsafe { ClDeviceID::from_retained(unusable_device_id) };
        assert_eq!(error, Err(UNUSABLE_DEVICE_ERROR));
    }
    // use super::ClDeviceID;
    // use super::{DeviceType, UNUSABLE_DEVICE_ERROR, DevicePtr};
    // use crate::ffi::cl_device_id;
    // use crate::ClPlatformID;
    // // use crate::testing;

    #[test]
    fn lists_all_devices() {
        let platform = ClPlatformID::default();
        let devices = list_devices_by_type(&platform, DeviceType::ALL).expect("Failed to list all devices");
        assert!(devices.len() > 0);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform() {
        let platform = ClPlatformID::default();
        let _ = list_devices_by_type(&platform, DeviceType::DEFAULT).expect("Failed to list DEFAULT devices");
        let _ = list_devices_by_type(&platform, DeviceType::CPU).expect("Failed to list CPU devices");
        let _ = list_devices_by_type(&platform, DeviceType::GPU).expect("Failed to list GPU devices");
        let _ = list_devices_by_type(&platform, DeviceType::ACCELERATOR).expect("Failed to list ACCELERATOR devices");
        let _ = list_devices_by_type(&platform, DeviceType::CUSTOM).expect("Failed to list CUSTOM devices");
        let _ = list_devices_by_type(&platform, DeviceType::ALL).expect("Failed to list ALL devices");
    }

    // #[test]
    // fn devices_of_many_types_can_be_listed_for_a_platform_via_flags() {
    //     let platform = Platform::default();
    //     let _ = Device::all_by_type(&platform, DeviceType::ALL);
    //     let _ = Device::all_by_type(&platform, DeviceType::CPU);
    //     let _ = Device::all_by_type(&platform, DeviceType::GPU);
    //     let _ = Device::all_by_type(&platform, DeviceType::ACCELERATOR);
    //     let _ = Device::all_by_type(&platform, DeviceType::CUSTOM);
    // }

    // #[test]
    // fn device_fmt_works() {
    //     let device = testing::get_device();
    //     let formatted = format!("{:?}", device);
    //     assert!(formatted.starts_with("Device{ptr: 0x")); //== "".contains("Device{"));
    // }

    // #[test]
    // fn device_name_works() {
    //     let device = testing::get_device();
    //     let name: String = device.name().unwrap();
    //     assert!(name.len() > 0);
    // }
}



// pub fn all() -> Output<Vec<Platform>> {
//         cl_get_platforms().map(|ret| {
//             unsafe { ret.into_vec() }.into_iter().map(|p| Platform::new(p)).collect()
//         })
//     }

//     pub fn all_devices(&self) -> Output<Vec<Device>> {
//         Device::all(self)
//     }

//     pub fn all_devices_by_type(&self, device_type: DeviceType) -> Output<Vec<Device>> {
//         Device::all_by_type(self, device_type)
//     }

//     pub fn default_devices(&self) -> Output<Vec<Device>> {
//         Device::default_devices(self)
//     }

    // /// Returns first default device assuming device is usable and list default devices is
    // /// not empty. // Best effort.
    // pub fn default_device(&self) -> Output<Device> {
    //     let defaults = Device::default_devices(self)?;
    //     for d in defaults.into_iter() {
    //         if d.is_usable() {
    //             return Ok(d);
    //         }
    //     }
    //     Err(PlatformError::NoUsableDevices.into())
    // }

    // pub fn get_any_default_device() -> Output<Device> {
    //     let platform = Platform::default();
    //     let defaults = Device::default_devices(&platform)?;
    //     for d in defaults.into_iter() {
    //         if d.is_usable() {
    //             return Ok(d);
    //         }
    //     }
    //     Err(PlatformError::NoUsableDevices.into())
    // }

    // pub fn cpu_devices(&self) -> Output<Vec<Device>> {
    //     Device::cpu_devices(self)
    // }

    // pub fn gpu_devices(&self) -> Output<Vec<Device>> {
    //     Device::gpu_devices(self)
    // }

    // pub fn accelerator_devices(&self) -> Output<Vec<Device>> {
    //     Device::accelerator_devices(self)
    // }

    // pub fn custom_devices(&self) -> Output<Vec<Device>> {
    //     Device::custom_devices(self)
    // }