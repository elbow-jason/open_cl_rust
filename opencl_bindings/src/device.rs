use crate::ffi::{
    cl_device_exec_capabilities,
    cl_device_fp_config,
    cl_device_id,
    cl_device_info,
    cl_device_local_mem_type,
    cl_device_mem_cache_type,
    cl_device_type,
    // cl_device_partition_property,
    // cl_device_affinity_domain,
    // cl_device_svm,
};
use crate::open_cl::{
    cl_create_context, cl_get_device_count, cl_get_device_ids, cl_get_device_info, ClObject,
};

use crate::{Context, Error, Output, Platform};

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

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct Device {
    inner: cl_device_id,
    _unconstructable: (),
}

impl From<cl_device_id> for Device {
    fn from(inner: cl_device_id) -> Device {
        Device::new(inner)
    }
}

impl ClObject<cl_device_id> for Device {
    unsafe fn raw_cl_object(&self) -> cl_device_id {
        self.inner
    }
}

use DeviceInfo::*;

impl Device {
    pub fn new(inner: cl_device_id) -> Device {
        Device {
            inner,
            _unconstructable: (),
        }
    }

    pub fn is_usable(&self) -> bool {
        self.inner != UNUSABLE_DEVICE_ID
    }

    pub fn usability_check(&self) -> Output<()> {
        if self.is_usable() {
            Ok(())
        } else {
            Err(Error::DeviceError(DeviceError::UnusableDevice))
        }
    }

    pub fn create_context(&self) -> Output<Context> {
        let context_id = cl_create_context(self)?;
        Ok(Context::new(context_id))
    }

    pub fn all(platform: &Platform) -> Output<Vec<Device>> {
        Device::all_by_type(platform, DeviceType::ALL)
    }

    pub fn count_by_type(platform: &Platform, device_type: DeviceType) -> Output<u32> {
        cl_get_device_count(platform, device_type.bits())
    }

    pub fn all_by_type(platform: &Platform, device_type: DeviceType) -> Output<Vec<Device>> {
        let ids = cl_get_device_ids(platform, device_type.bits())?;
        Ok(ids.into_iter().map(|id| Device::new(id)).collect())
    }

    pub fn get_info(&self, info: DeviceInfo) -> Output<String> {
        cl_get_device_info(self, info as cl_device_info)
    }

    pub fn type_info(&self) -> Output<String> {
        self.get_info(Type)
    }
    pub fn vendor_id_info(&self) -> Output<String> {
        self.get_info(VendorId)
    }
    pub fn max_compute_units_info(&self) -> Output<String> {
        self.get_info(MaxComputeUnits)
    }
    pub fn max_work_item_dimensions_info(&self) -> Output<String> {
        self.get_info(MaxWorkItemDimensions)
    }
    pub fn max_work_group_size_info(&self) -> Output<String> {
        self.get_info(MaxWorkGroupSize)
    }
    pub fn max_work_item_sizes_info(&self) -> Output<String> {
        self.get_info(MaxWorkItemSizes)
    }
    pub fn preferred_vector_width_char_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthChar)
    }
    pub fn preferred_vector_width_short_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthShort)
    }
    pub fn preferred_vector_width_int_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthInt)
    }
    pub fn preferred_vector_width_long_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthLong)
    }
    pub fn preferred_vector_width_float_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthFloat)
    }
    pub fn preferred_vector_width_double_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthDouble)
    }
    pub fn max_clock_frequency_info(&self) -> Output<String> {
        self.get_info(MaxClockFrequency)
    }
    pub fn address_bits_info(&self) -> Output<String> {
        self.get_info(AddressBits)
    }
    pub fn max_read_image_args_info(&self) -> Output<String> {
        self.get_info(MaxReadImageArgs)
    }
    pub fn max_write_image_args_info(&self) -> Output<String> {
        self.get_info(MaxWriteImageArgs)
    }
    pub fn max_mem_alloc_size_info(&self) -> Output<String> {
        self.get_info(MaxMemAllocSize)
    }
    pub fn image2_d_max_width_info(&self) -> Output<String> {
        self.get_info(Image2DMaxWidth)
    }
    pub fn image2_d_max_height_info(&self) -> Output<String> {
        self.get_info(Image2DMaxHeight)
    }
    pub fn image3_d_max_width_info(&self) -> Output<String> {
        self.get_info(Image3DMaxWidth)
    }
    pub fn image3_d_max_height_info(&self) -> Output<String> {
        self.get_info(Image3DMaxHeight)
    }
    pub fn image3_d_max_depth_info(&self) -> Output<String> {
        self.get_info(Image3DMaxDepth)
    }
    pub fn image_support_info(&self) -> Output<String> {
        self.get_info(ImageSupport)
    }
    pub fn max_parameter_size_info(&self) -> Output<String> {
        self.get_info(MaxParameterSize)
    }
    pub fn max_samplers_info(&self) -> Output<String> {
        self.get_info(MaxSamplers)
    }
    pub fn mem_base_addr_align_info(&self) -> Output<String> {
        self.get_info(MemBaseAddrAlign)
    }
    pub fn min_data_type_align_size_info(&self) -> Output<String> {
        self.get_info(MinDataTypeAlignSize)
    }
    pub fn single_fp_config_info(&self) -> Output<String> {
        self.get_info(SingleFpConfig)
    }
    pub fn global_mem_cache_type_info(&self) -> Output<String> {
        self.get_info(GlobalMemCacheType)
    }
    pub fn global_mem_cacheline_size_info(&self) -> Output<String> {
        self.get_info(GlobalMemCachelineSize)
    }
    pub fn global_mem_cache_size_info(&self) -> Output<String> {
        self.get_info(GlobalMemCacheSize)
    }
    pub fn global_mem_size_info(&self) -> Output<String> {
        self.get_info(GlobalMemSize)
    }
    pub fn max_constant_buffer_size_info(&self) -> Output<String> {
        self.get_info(MaxConstantBufferSize)
    }
    pub fn max_constant_args_info(&self) -> Output<String> {
        self.get_info(MaxConstantArgs)
    }
    pub fn local_mem_type_info(&self) -> Output<String> {
        self.get_info(LocalMemType)
    }
    pub fn local_mem_size_info(&self) -> Output<String> {
        self.get_info(LocalMemSize)
    }
    pub fn error_correction_support_info(&self) -> Output<String> {
        self.get_info(ErrorCorrectionSupport)
    }
    pub fn profiling_timer_resolution_info(&self) -> Output<String> {
        self.get_info(ProfilingTimerResolution)
    }
    pub fn endian_little_info(&self) -> Output<String> {
        self.get_info(EndianLittle)
    }
    pub fn available_info(&self) -> Output<String> {
        self.get_info(Available)
    }
    pub fn compiler_available_info(&self) -> Output<String> {
        self.get_info(CompilerAvailable)
    }
    pub fn execution_capabilities_info(&self) -> Output<String> {
        self.get_info(ExecutionCapabilities)
    }
    pub fn queue_on_host_properties_info(&self) -> Output<String> {
        self.get_info(QueueOnHostProperties)
    }
    pub fn name_info(&self) -> Output<String> {
        self.get_info(Name)
    }
    pub fn vendor_info(&self) -> Output<String> {
        self.get_info(Vendor)
    }
    pub fn profile_info(&self) -> Output<String> {
        self.get_info(Profile)
    }
    pub fn version_info(&self) -> Output<String> {
        self.get_info(Version)
    }
    pub fn extensions_info(&self) -> Output<String> {
        self.get_info(Extensions)
    }
    pub fn platform_info(&self) -> Output<String> {
        // Platform was a struct in scope so DeviceInfo must be fully qualified.
        self.get_info(DeviceInfo::Platform)
    }
    pub fn double_fp_config_info(&self) -> Output<String> {
        self.get_info(DoubleFpConfig)
    }
    pub fn preferred_vector_width_half_info(&self) -> Output<String> {
        self.get_info(PreferredVectorWidthHalf)
    }
    pub fn host_unified_memory_info(&self) -> Output<String> {
        self.get_info(HostUnifiedMemory)
    }
    pub fn native_vector_width_char_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthChar)
    }
    pub fn native_vector_width_short_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthShort)
    }
    pub fn native_vector_width_int_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthInt)
    }
    pub fn native_vector_width_long_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthLong)
    }
    pub fn native_vector_width_float_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthFloat)
    }
    pub fn native_vector_width_double_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthDouble)
    }
    pub fn native_vector_width_half_info(&self) -> Output<String> {
        self.get_info(NativeVectorWidthHalf)
    }
    pub fn opencl_c_version_info(&self) -> Output<String> {
        self.get_info(OpenclCVersion)
    }
    pub fn linker_available_info(&self) -> Output<String> {
        self.get_info(LinkerAvailable)
    }
    pub fn built_in_kernels_info(&self) -> Output<String> {
        self.get_info(BuiltInKernels)
    }
    pub fn image_max_buffer_size_info(&self) -> Output<String> {
        self.get_info(ImageMaxBufferSize)
    }
    pub fn image_max_array_size_info(&self) -> Output<String> {
        self.get_info(ImageMaxArraySize)
    }
    pub fn parent_device_info(&self) -> Output<String> {
        self.get_info(ParentDevice)
    }
    pub fn partition_max_sub_devices_info(&self) -> Output<String> {
        self.get_info(PartitionMaxSubDevices)
    }
    pub fn partition_properties_info(&self) -> Output<String> {
        self.get_info(PartitionProperties)
    }
    pub fn partition_affinity_domain_info(&self) -> Output<String> {
        self.get_info(PartitionAffinityDomain)
    }
    pub fn partition_type_info(&self) -> Output<String> {
        self.get_info(PartitionType)
    }
    pub fn reference_count_info(&self) -> Output<String> {
        self.get_info(ReferenceCount)
    }
    pub fn preferred_interop_user_sync_info(&self) -> Output<String> {
        self.get_info(PreferredInteropUserSync)
    }
    pub fn printf_buffer_size_info(&self) -> Output<String> {
        self.get_info(PrintfBufferSize)
    }
    pub fn image_pitch_alignment_info(&self) -> Output<String> {
        self.get_info(ImagePitchAlignment)
    }
    pub fn image_base_address_alignment_info(&self) -> Output<String> {
        self.get_info(ImageBaseAddressAlignment)
    }
    pub fn max_read_write_image_args_info(&self) -> Output<String> {
        self.get_info(MaxReadWriteImageArgs)
    }
    pub fn max_global_variable_size_info(&self) -> Output<String> {
        self.get_info(MaxGlobalVariableSize)
    }
    pub fn queue_on_device_properties_info(&self) -> Output<String> {
        self.get_info(QueueOnDeviceProperties)
    }
    pub fn queue_on_device_preferred_size_info(&self) -> Output<String> {
        self.get_info(QueueOnDevicePreferredSize)
    }
    pub fn queue_on_device_max_size_info(&self) -> Output<String> {
        self.get_info(QueueOnDeviceMaxSize)
    }
    pub fn max_on_device_queues_info(&self) -> Output<String> {
        self.get_info(MaxOnDeviceQueues)
    }
    pub fn max_on_device_events_info(&self) -> Output<String> {
        self.get_info(MaxOnDeviceEvents)
    }
    pub fn svm_capabilities_info(&self) -> Output<String> {
        self.get_info(SvmCapabilities)
    }
    pub fn global_variable_preferred_total_size_info(&self) -> Output<String> {
        self.get_info(GlobalVariablePreferredTotalSize)
    }
    pub fn max_pipe_args_info(&self) -> Output<String> {
        self.get_info(MaxPipeArgs)
    }
    pub fn pipe_max_active_reservations_info(&self) -> Output<String> {
        self.get_info(PipeMaxActiveReservations)
    }
    pub fn pipe_max_packet_size_info(&self) -> Output<String> {
        self.get_info(PipeMaxPacketSize)
    }
    pub fn preferred_platform_atomic_alignment_info(&self) -> Output<String> {
        self.get_info(PreferredPlatformAtomicAlignment)
    }
    pub fn preferred_global_atomic_alignment_info(&self) -> Output<String> {
        self.get_info(PreferredGlobalAtomicAlignment)
    }
    pub fn preferred_local_atomic_alignment_info(&self) -> Output<String> {
        self.get_info(PreferredLocalAtomicAlignment)
    }
    pub fn il_version_info(&self) -> Output<String> {
        self.get_info(IlVersion)
    }
    pub fn max_num_sub_groups_info(&self) -> Output<String> {
        self.get_info(MaxNumSubGroups)
    }
    pub fn sub_group_independent_forward_progress_info(&self) -> Output<String> {
        self.get_info(SubGroupIndependentForwardProgress)
    }
}

// impl fmt::Display for Device {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Device({:?})", self.name_info().unwrap())
//     }
// }

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L412-L414
// /* cl_device_exec_capabilities - bitfield */
crate::__codes_enum!(DeviceExecCapabilities, cl_device_exec_capabilities, {
    Kernel => 1,
    NativeKernel => 2
});

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L389-L401
crate::__codes_enum!(DeviceFPConfig, cl_device_fp_config, {
    Denorm => 1,
    InfNan => 2,
    RoundToNearest => 4,
    RoundToZero => 8,
    RoundToInf => 16,
    Fma => 32,
    SoftFloat => 64,
    CorrectlyRoundedDivideSqrt => 128
});

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L280-L387
//
// Note: removed due to deprecation
// pub const CL_DEVICE_QUEUE_PROPERTIES: cl_uint = 0x102A /* deprecated */;
//
// Note: What is this? a duplicate def in OpenCL....
// pub const CL_DRIVER_VERSION: cl_uint = 0x102D;
// CL_DEVICE_VERSION has two values.
// I am keeping the bottom one on the bet that a def is mutable in C.
// Rust did not like duplicate tags on the same enum.
crate::__codes_enum!(DeviceInfo, cl_device_info, {
    Type => 0x1000,
    VendorId => 0x1001,
    MaxComputeUnits => 0x1002,
    MaxWorkItemDimensions => 0x1003,
    MaxWorkGroupSize => 0x1004,
    MaxWorkItemSizes => 0x1005,
    PreferredVectorWidthChar => 0x1006,
    PreferredVectorWidthShort => 0x1007,
    PreferredVectorWidthInt => 0x1008,
    PreferredVectorWidthLong => 0x1009,
    PreferredVectorWidthFloat => 0x100A,
    PreferredVectorWidthDouble => 0x100B,
    MaxClockFrequency => 0x100C,
    AddressBits => 0x100D,
    MaxReadImageArgs => 0x100E,
    MaxWriteImageArgs => 0x100F,
    MaxMemAllocSize => 0x1010,
    Image2DMaxWidth => 0x1011,
    Image2DMaxHeight => 0x1012,
    Image3DMaxWidth => 0x1013,
    Image3DMaxHeight => 0x1014,
    Image3DMaxDepth => 0x1015,
    ImageSupport => 0x1016,
    MaxParameterSize => 0x1017,
    MaxSamplers => 0x1018,
    MemBaseAddrAlign => 0x1019,
    MinDataTypeAlignSize => 0x101A,
    SingleFpConfig => 0x101B,
    GlobalMemCacheType => 0x101C,
    GlobalMemCachelineSize => 0x101D,
    GlobalMemCacheSize => 0x101E,
    GlobalMemSize => 0x101F,
    MaxConstantBufferSize => 0x1020,
    MaxConstantArgs => 0x1021,
    LocalMemType => 0x1022,
    LocalMemSize => 0x1023,
    ErrorCorrectionSupport => 0x1024,
    ProfilingTimerResolution => 0x1025,
    EndianLittle => 0x1026,
    Available => 0x1027,
    CompilerAvailable => 0x1028,
    ExecutionCapabilities => 0x1029,
    QueueOnHostProperties => 0x102A,
    Name => 0x102B,
    Vendor => 0x102C,
    Profile => 0x102E,
    Version => 0x102F,
    Extensions => 0x1030,
    Platform => 0x1031,
    DoubleFpConfig => 0x1032,
    PreferredVectorWidthHalf => 0x1034,
    HostUnifiedMemory => 0x1035,   /* deprecated */
    NativeVectorWidthChar => 0x1036,
    NativeVectorWidthShort => 0x1037,
    NativeVectorWidthInt => 0x1038,
    NativeVectorWidthLong => 0x1039,
    NativeVectorWidthFloat => 0x103A,
    NativeVectorWidthDouble => 0x103B,
    NativeVectorWidthHalf => 0x103C,
    OpenclCVersion => 0x103D,
    LinkerAvailable => 0x103E,
    BuiltInKernels => 0x103F,
    ImageMaxBufferSize => 0x1040,
    ImageMaxArraySize => 0x1041,
    ParentDevice => 0x1042,
    PartitionMaxSubDevices => 0x1043,
    PartitionProperties => 0x1044,
    PartitionAffinityDomain => 0x1045,
    PartitionType => 0x1046,
    ReferenceCount => 0x1047,
    PreferredInteropUserSync => 0x1048,
    PrintfBufferSize => 0x1049,
    ImagePitchAlignment => 0x104A,
    ImageBaseAddressAlignment => 0x104B,
    MaxReadWriteImageArgs => 0x104C,
    MaxGlobalVariableSize => 0x104D,
    QueueOnDeviceProperties => 0x104E,
    QueueOnDevicePreferredSize => 0x104F,
    QueueOnDeviceMaxSize => 0x1050,
    MaxOnDeviceQueues => 0x1051,
    MaxOnDeviceEvents => 0x1052,
    SvmCapabilities => 0x1053,
    GlobalVariablePreferredTotalSize => 0x1054,
    MaxPipeArgs => 0x1055,
    PipeMaxActiveReservations => 0x1056,
    PipeMaxPacketSize => 0x1057,
    PreferredPlatformAtomicAlignment => 0x1058,
    PreferredGlobalAtomicAlignment => 0x1059,
    PreferredLocalAtomicAlignment => 0x105A,
    IlVersion => 0x105B,
    MaxNumSubGroups => 0x105C,
    SubGroupIndependentForwardProgress => 0x105D
});

crate::__codes_enum!(DeviceLocalMemType,  cl_device_local_mem_type, {
    Local => 0x1,
    Global => 0x2
});

crate::__codes_enum!(DeviceMemCacheType, cl_device_mem_cache_type, {
    NoneType => 0x0,
    ReadOnlyCache => 0x1,
    ReadWriteCache => 0x2
});

bitflags! {
    pub struct DeviceType: cl_device_type {
        const DEFAULT = 1;
        const CPU = 2;
        const GPU = 4;
        const ACCELERATOR = 8;
        const CUSTOM = 16;
        const ALL = 0xFFFF_FFFF;
    }
}

// NOTE: Version for cl_device_partition_property?
// crate::__codes_enum!(DevicePartitionPropertery, cl_device_partition_property, {
//     Equally => 0x1086,
//     ByCounts => 0x1087,
//     ByCountsListEnd => 0x0,
//     ByAffinityDomain => 0x1088
// });

// NOTE: Version for cl_device_svm?
// crate::__codes_enum!(DeviceSvm, cl_device_svm, {
//     CoarseGrainBuffer => 1,
//     FineGrainBuffer => 2,
//     FineGrainSystem => 4,
//     Atomics => 8
// });

// NOTE: Version for cl_device_affinity_domain?
// crate::__codes_enum!(DeviceAffinityDomain, cl_device_affinity_domain, {
//     Numa => 1,
//     L4Cache => 2,
//     L3Cache => 4,
//     L2Cache => 8,
//     L1Cache => 16,
//     NextPartitionable => 32
// });
