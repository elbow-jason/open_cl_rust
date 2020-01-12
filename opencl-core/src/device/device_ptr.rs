use std::fmt;

use crate::Output;
use crate::ffi::{cl_device_id, cl_uint, cl_ulong};

use super::flags::DeviceInfo;
use super::DeviceError;
use super::low_level::cl_get_device_info;
use super::flags::{DeviceExecCapabilities, DeviceLocalMemType, DeviceMemCacheType, DeviceType, DeviceAffinityDomain};
use super::{UNUSABLE_DEVICE_ID};
macro_rules! info_fn {
    ($name:ident, $flag:ident, String) => {
        fn $name(&self) -> Output<String> {
            cl_get_device_info::<u8>(unsafe{ self.device_ptr() }, DeviceInfo::$flag)
                .map(|ret| unsafe { ret.into_string() })
        }
    };

    ($name:ident, $flag:ident, bool) => {
        fn $name(&self) -> Output<bool> {
            use crate::ffi::cl_bool;
            cl_get_device_info::<cl_bool>(unsafe{ self.device_ptr() }, DeviceInfo::$flag)
                .map(From::from)
        }
    };

    ($name:ident, $flag:ident, Vec<$output_t:ty>) => {
        fn $name(&self) -> Output<Vec<$output_t>> {
            cl_get_device_info(unsafe { self.device_ptr() }, DeviceInfo::$flag)
                .map(|ret| unsafe { ret.into_vec() })
        }
    };

    ($name:ident, $flag:ident, $output_t:ty) => {
        fn $name(&self) -> Output<$output_t> {
            cl_get_device_info(unsafe { self.device_ptr() }, DeviceInfo::$flag)
                .map(|ret| unsafe { ret.into_one() })
        }
    };
    ($name:ident, $flag:ident, $cl_type:ty, $output_t:ty) => {
        fn $name(&self) -> Output<$output_t> {
            cl_get_device_info::<$cl_type>(unsafe { self.device_ptr() }, DeviceInfo::$flag)
                .map(|ret| unsafe { ret.into_one() })
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
    info_fn!(
        partition_max_sub_devices,
        PartitionMaxSubDevices,
        cl_uint,
        u32
    );
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
    info_fn!(global_mem_cache_size, GlobalMemCacheSize, cl_ulong, u64);
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
    info_fn!(max_work_item_sizes, MaxWorkItemSizes, Vec<usize>);

    // cl_device_local_mem_type
    info_fn!(local_mem_type, LocalMemType, DeviceLocalMemType);

    // ExecutionCapabilities
    info_fn!(
        execution_capabilities,
        ExecutionCapabilities,
        DeviceExecCapabilities
    );

    //  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
    info_fn!(
        global_mem_cache_type,
        GlobalMemCacheType,
        DeviceMemCacheType
    );

    // cl_device_affinity_domain
    info_fn!(
        partition_affinity_domain,
        PartitionAffinityDomain,
        DeviceAffinityDomain
    );

    // DeviceType
    info_fn!(device_type, Type, DeviceType);
}