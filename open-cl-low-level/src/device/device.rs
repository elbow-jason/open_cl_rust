use std::fmt;
use std::fmt::Debug;

use crate::{ErrorT, Output};

use super::functions;

use crate::cl::{
    cl_device_affinity_domain, cl_device_exec_capabilities, cl_device_local_mem_type,
    cl_device_mem_cache_type, cl_device_type,
};
use crate::cl::{cl_device_id, ClObject, ObjectWrapper};
use crate::cl::{
    DeviceAffinityDomain, DeviceExecCapabilities, DeviceInfo, DeviceLocalMemType,
    DeviceMemCacheType, DeviceType,
};

/// An error related to a Device.
#[derive(ErrorT, Debug, PartialEq, Eq, Clone)]
pub enum DeviceError {
    #[error("The given platform had no default device")]
    NoDefaultDevice,

    #[error("The given device had no parent device")]
    NoParentDevice,

    #[error("Invalid device info value")]
    InvalidInfoValue,
}

pub type Device = ObjectWrapper<cl_device_id>;

pub trait DevicePtr {
    unsafe fn device_ptr(&self) -> cl_device_id;

    fn address(&self) -> String {
        unsafe { self.device_ptr() }.address()
    }
}

impl DevicePtr for Device {
    unsafe fn device_ptr(&self) -> cl_device_id {
        self.cl_object()
    }
}

macro_rules! info_fn {
    ($name:ident, $flag:ident, String) => {
        fn $name(&self) -> Output<String> {
            unsafe {
                functions::get_device_info_string(self.device_ptr(), DeviceInfo::$flag.into())
            }
        }
    };

    ($name:ident, $flag:ident, bool) => {
        fn $name(&self) -> Output<bool> {
            unsafe { functions::get_device_info_bool(self.device_ptr(), DeviceInfo::$flag.into()) }
        }
    };

    ($name:ident, $flag:ident, $cl_type:ty, Vec<usize>) => {
        fn $name(&self) -> Output<Vec<usize>> {
            unsafe {
                functions::get_device_info_vec_usize(self.device_ptr(), DeviceInfo::$flag.into())
            }
        }
    };

    ($name:ident, $flag:ident, u32) => {
        fn $name(&self) -> Output<u32> {
            unsafe { functions::get_device_info_u32(self.device_ptr(), DeviceInfo::$flag.into()) }
        }
    };

    ($name:ident, $flag:ident, u64) => {
        fn $name(&self) -> Output<u64> {
            unsafe { functions::get_device_info_u64(self.device_ptr(), DeviceInfo::$flag.into()) }
        }
    };
    ($name:ident, $flag:ident, usize) => {
        fn $name(&self) -> Output<usize> {
            unsafe { functions::get_device_info_usize(self.device_ptr(), DeviceInfo::$flag.into()) }
        }
    };
}

macro_rules! flag_info_fn_u64 {
    ($name:ident, $flag:ident, $cl_type:ident, $flag_out:ident) => {
        fn $name(&self) -> Output<$flag_out> {
            let flag_value: $cl_type = unsafe {
                functions::get_device_info_u64(self.device_ptr(), DeviceInfo::$flag.into())
            }?;
            match $flag_out::from_bits(flag_value) {
                Some(f) => Ok(f),
                None => Err(DeviceError::InvalidInfoValue)?,
            }
        }
    };
}
macro_rules! flag_info_fn_u32 {
    ($name:ident, $flag:ident, $cl_type:ident, $flag_out:ident) => {
        fn $name(&self) -> Output<$flag_out> {
            let flag_value: $cl_type = unsafe {
                functions::get_device_info_u32(self.device_ptr(), DeviceInfo::$flag.into())
            }?;
            Ok($flag_out::from(flag_value))
        }
    };
}
pub trait HasDeviceInfo
where
    Self: fmt::Debug + Sized + DevicePtr,
{
    info_fn!(global_mem_cacheline_size, GlobalMemCachelineSize, u32);
    info_fn!(native_vector_width_double, NativeVectorWidthDouble, u32);
    info_fn!(native_vector_width_half, NativeVectorWidthHalf, u32);
    info_fn!(address_bits, AddressBits, u32);
    info_fn!(max_clock_frequency, MaxClockFrequency, u32);
    info_fn!(max_compute_units, MaxComputeUnits, u32);
    info_fn!(max_constant_args, MaxConstantArgs, u32);
    info_fn!(max_read_image_args, MaxReadImageArgs, u32);
    info_fn!(max_samplers, MaxSamplers, u32);
    info_fn!(max_work_item_dimensions, MaxWorkItemDimensions, u32);
    info_fn!(max_write_image_args, MaxWriteImageArgs, u32);
    info_fn!(mem_base_addr_align, MemBaseAddrAlign, u32);
    info_fn!(min_data_type_align_size, MinDataTypeAlignSize, u32);
    info_fn!(native_vector_width_char, NativeVectorWidthChar, u32);
    info_fn!(native_vector_width_short, NativeVectorWidthShort, u32);
    info_fn!(native_vector_width_int, NativeVectorWidthInt, u32);
    info_fn!(native_vector_width_long, NativeVectorWidthLong, u32);
    info_fn!(native_vector_width_float, NativeVectorWidthFloat, u32);
    info_fn!(partition_max_sub_devices, PartitionMaxSubDevices, u32);
    info_fn!(preferred_vector_width_char, PreferredVectorWidthChar, u32);
    info_fn!(preferred_vector_width_short, PreferredVectorWidthShort, u32);
    info_fn!(preferred_vector_width_int, PreferredVectorWidthInt, u32);
    info_fn!(preferred_vector_width_long, PreferredVectorWidthLong, u32);
    info_fn!(preferred_vector_width_float, PreferredVectorWidthFloat, u32);
    info_fn!(
        preferred_vector_width_double,
        PreferredVectorWidthDouble,
        u32
    );
    info_fn!(preferred_vector_width_half, PreferredVectorWidthHalf, u32);
    info_fn!(vendor_id, VendorId, u32);

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
    info_fn!(global_mem_cache_size, GlobalMemCacheSize, u64);
    info_fn!(global_mem_size, GlobalMemSize, u64);
    info_fn!(local_mem_size, LocalMemSize, u64);
    info_fn!(max_constant_buffer_size, MaxConstantBufferSize, u64);
    info_fn!(max_mem_alloc_size, MaxMemAllocSize, u64);

    // size_t as usize
    info_fn!(image2d_max_width, Image2DMaxWidth, usize);
    info_fn!(image2d_max_height, Image2DMaxHeight, usize);
    info_fn!(image3d_max_width, Image3DMaxWidth, usize);
    info_fn!(image3d_max_height, Image3DMaxHeight, usize);
    info_fn!(image3d_max_depth, Image3DMaxDepth, usize);
    info_fn!(image_max_buffer_size, ImageMaxBufferSize, usize);
    info_fn!(image_max_array_size, ImageMaxArraySize, usize);
    info_fn!(max_parameter_size, MaxParameterSize, usize);
    info_fn!(max_work_group_size, MaxWorkGroupSize, usize);
    info_fn!(printf_buffer_size, PrintfBufferSize, usize);
    info_fn!(profiling_timer_resolution, ProfilingTimerResolution, usize);

    // size_t[]
    info_fn!(max_work_item_sizes, MaxWorkItemSizes, size_t, Vec<usize>);

    // cl_device_local_mem_type
    flag_info_fn_u32!(
        local_mem_type,
        LocalMemType,
        cl_device_local_mem_type,
        DeviceLocalMemType
    );

    // ExecutionCapabilities
    flag_info_fn_u64!(
        execution_capabilities,
        ExecutionCapabilities,
        cl_device_exec_capabilities,
        DeviceExecCapabilities
    );

    //  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
    flag_info_fn_u32!(
        global_mem_cache_type,
        GlobalMemCacheType,
        cl_device_mem_cache_type,
        DeviceMemCacheType
    );

    // cl_device_affinity_domain
    flag_info_fn_u64!(
        partition_affinity_domain,
        PartitionAffinityDomain,
        cl_device_affinity_domain,
        DeviceAffinityDomain
    );

    // DeviceType
    flag_info_fn_u64!(device_type, Type, cl_device_type, DeviceType);
}

impl<T> HasDeviceInfo for T where T: DevicePtr + fmt::Debug {}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

#[cfg(test)]
mod tests {
    use crate::cl::DeviceType;
    use crate::{ll_testing, Platform};

    #[test]
    fn lists_all_devices() {
        let platform = Platform::default();
        let devices = platform.list_devices().expect("Failed to list all devices");
        assert!(devices.len() > 0);
    }

    #[test]
    fn devices_of_many_types_can_be_listed_for_a_platform() {
        let platform = Platform::default();
        let _ = platform.list_devices_by_type(DeviceType::DEFAULT);
        // .expect("Failed to list DEFAULT devices");
        let _ = platform.list_devices_by_type(DeviceType::CPU);
        // .expect("Failed to list CPU devices");
        let _ = platform.list_devices_by_type(DeviceType::GPU);
        // .expect("Failed to list GPU devices");
        let _ = platform.list_devices_by_type(DeviceType::ACCELERATOR);
        // .expect("Failed to list ACCELERATOR devices");
        let _ = platform.list_devices_by_type(DeviceType::CUSTOM);
        // .expect("Failed to list CUSTOM devices");
        let _ = platform.list_devices_by_type(DeviceType::ALL);
        // .expect("Failed to list ALL devices");
    }

    #[test]
    fn device_fmt_debug_works() {
        ll_testing::with_each_device(|device| {
            let formatted = format!("{:?}", device);
            expect_method!(formatted, contains, device.address());
            expect_method!(formatted, contains, "cl_device_id");
        })
    }
}
#[cfg(test)]
mod device_ptr_tests {
    use crate::*;

    #[test]
    fn device_name_works() {
        ll_testing::with_each_device(|device| {
            let name: String = device.name().unwrap();
            println!("name: {:?}", name);
            assert!(name.len() > 0);
        })
    }

    macro_rules! test_method {
        ($method:ident) => {
            paste::item! {
                #[test]
                fn [<$method _works>]() {
                    ll_testing::with_each_device(|device| {
                        let _result = device.$method().unwrap();
                    })
                }
            }
        };
    }

    // u32
    test_method!(global_mem_cacheline_size);
    test_method!(native_vector_width_double);
    test_method!(native_vector_width_half);
    test_method!(address_bits);
    test_method!(max_clock_frequency);
    test_method!(max_compute_units);
    test_method!(max_constant_args);
    test_method!(max_read_image_args);
    test_method!(max_samplers);
    test_method!(max_work_item_dimensions);
    test_method!(max_write_image_args);
    test_method!(mem_base_addr_align);
    test_method!(min_data_type_align_size);
    test_method!(native_vector_width_char);
    test_method!(native_vector_width_short);
    test_method!(native_vector_width_int);
    test_method!(native_vector_width_long);
    test_method!(native_vector_width_float);
    test_method!(partition_max_sub_devices);
    test_method!(preferred_vector_width_char);
    test_method!(preferred_vector_width_short);
    test_method!(preferred_vector_width_int);
    test_method!(preferred_vector_width_long);
    test_method!(preferred_vector_width_float);
    test_method!(preferred_vector_width_double);
    test_method!(preferred_vector_width_half);
    test_method!(vendor_id);

    // bool
    test_method!(available);
    test_method!(compiler_available);
    test_method!(endian_little);
    test_method!(error_correction_support);
    test_method!(host_unified_memory);
    test_method!(image_support);
    test_method!(linker_available);
    test_method!(preferred_interop_user_sync);

    // String
    test_method!(name);
    test_method!(opencl_c_version);
    test_method!(profile);
    test_method!(vendor);
    test_method!(version);
    test_method!(driver_version);

    // u64
    test_method!(global_mem_cache_size);
    test_method!(global_mem_size);
    test_method!(local_mem_size);
    test_method!(max_constant_buffer_size);
    test_method!(max_mem_alloc_size);

    // usize
    test_method!(image2d_max_width);
    test_method!(image2d_max_height);
    test_method!(image3d_max_width);
    test_method!(image3d_max_height);
    test_method!(image3d_max_depth);
    test_method!(image_max_buffer_size);
    test_method!(image_max_array_size);
    test_method!(max_parameter_size);
    test_method!(max_work_group_size);
    test_method!(printf_buffer_size);
    test_method!(profiling_timer_resolution);

    // Vec<usize>
    test_method!(max_work_item_sizes);

    // cl_device_local_mem_type
    test_method!(local_mem_type);

    // ExecutionCapabilities
    test_method!(execution_capabilities);
    //  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
    test_method!(global_mem_cache_type);

    // cl_device_affinity_domain
    test_method!(partition_affinity_domain);

    // DeviceType
    test_method!(device_type);
}
