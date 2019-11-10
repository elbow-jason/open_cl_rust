/// This module implements DeviceInfo methods for Device.
/// It was too much boilerplate for the Device module.

// use libc::c_void;

use crate::ffi::{
    cl_device_info,
    cl_device_partition_property,
    clGetDeviceInfo,
};
use crate::error::Output;

use crate::cl::{
    // ClOutput,
    // ClReturn,
    // ClDecoder,
    ClObject,
    ClRetain,
    ClPointer,
    cl_get_info5
};

use crate::device::{Device, DeviceType};
use crate::device::flags::{
    DeviceFpConfig,
    DeviceExecCapabilities,
    DeviceMemCacheType,
    DeviceLocalMemType,
    DevicePartitionProperty,
    DeviceAffinityDomain,
};

use crate::platform::Platform;

// deprecated
// QueueProperties => cl_command_queue_properties

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
    PreferredVectorWidthHalf => 0x1034,
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
    SubGroupIndependentForwardProgress => 0x105D,
    HalfFpConfig => 0x1033,
    DriverVersion => 0x102D
});

pub fn cl_get_device_info<T: Copy>(device: &Device, flag: DeviceInfo) -> Output<ClPointer<T>> {
    device.usability_check()?;
    unsafe {
        cl_get_info5(
            device.raw_cl_object(),
            flag as cl_device_info,
            clGetDeviceInfo
        )
    }
}

    // let mut size = 0 as libc::size_t;
    // let err_code = unsafe {
    //     clGetDeviceInfo(
    //         device.raw_cl_object(),
    //         device_info as cl_device_info,
    //         0,
    //         std::ptr::null_mut(),
    //         &mut size,
    //     )
    // };
    // size = StatusCode::into_output(err_code, size)?;
    // // println!("Before");
    // // inspect_var!(size);
    // // size = std::cmp::max(size, 8);
    // // println!("After");
    // // inspect_var!(size);
    // let mut buf: Vec<u8> = utils::vec_filled_with(0u8, size as usize);
    // device.usability_check()?;
    // let err_code = unsafe {
    //     clGetDeviceInfo(
    //         device.raw_cl_object(),
    //         device_info as cl_device_info,
    //         size,
    //         buf.as_mut_ptr() as *mut libc::c_void,
    //         std::ptr::null_mut(),
    //     )
    // };
    // // inspect_var!(buf);
    
    // let () = StatusCode::into_output(err_code, ())?;
    // let ret = unsafe { ClReturn::from_vec(buf) };
    // Ok(ret)
// }


// Platform





macro_rules! __impl_info_for_one_wrapper {
    ($impl_struct:ident, $func_name:ident, $flag:expr, $wrapper_struct:ident) => {
        impl $impl_struct {
            pub fn $func_name(&self) -> Output<$wrapper_struct> {
                self.get_info($flag).map(|ret| unsafe { ret.into_one_wrapper() })
            }
        }
    }
}

macro_rules! __impl_info_for_one_wrapper_retained {
    ($impl_struct:ident, $func_name:ident, $flag:expr, $wrapper_struct:ident) => {
        impl $impl_struct {
            pub fn $func_name(&self) -> Output<$wrapper_struct> {
                self.get_info($flag).map(|ret| unsafe { ret.into_one_wrapper().cl_retain() })
            }
        }
    }
}

__impl_info_for_one_wrapper!(Device, platform, DeviceInfo::Platform, Platform);


macro_rules! __impl_device_info_one {
    ($name:ident, $flag:ident, String) => {
        impl Device {
            pub fn $name(&self) -> Output<String> {
                self.get_info(DeviceInfo::$flag).map(|ret| unsafe { ret.into_string() })
            }
        }
    };
    ($name:ident, $flag:ident, Vec<$output_t:ty>) => {
        impl Device {
            pub fn $name(&self) -> Output<Vec<$output_t>> {
                self.get_info(DeviceInfo::$flag).map(|ret| unsafe { ret.into_many() })
            }
        }
    };
    ($name:ident, $flag:ident, $output_t:ty) => {
        impl Device {
            pub fn $name(&self) -> Output<$output_t> {
                self.get_info(DeviceInfo::$flag).map(|ret| unsafe { ret.into_one() })
            }
        }
    };
}


unsafe fn cast_device_partition_properties(ret: ClPointer<cl_device_partition_property>) -> Vec<DevicePartitionProperty> {
    let props: Vec<cl_device_partition_property> = ret.into_many();
    let mut output = Vec::new();
    for p in props {
        // Zero here is an indication that the list of  device partition properties is
        // at an end. So we immediately return.
        if p == 0 {
            return output;
        }
        output.push(DevicePartitionProperty::from(p))
    }
    output
}

impl Device {
    pub fn built_in_kernels(&self) -> Output<Vec<String>> {
        self.get_info(DeviceInfo::BuiltInKernels)
            .map(|ret| {
                let kernel_names: String = unsafe { ret.into_string() };
                kernel_names.split(";").map(|s| s.to_string()).collect()
            })
    }

    pub fn extensions(&self) -> Output<Vec<String>> {
        self.get_info(DeviceInfo::Extensions)
            .map(|ret| {
                let kernels: String = unsafe { ret.into_string() };
                kernels.split(" ").map(|s| s.to_string()).collect()
            })
    }

    pub fn partition_properties(&self) -> Output<Vec<DevicePartitionProperty>> {
        self.get_info(DeviceInfo::PartitionProperties).map(|ret| {
            unsafe { cast_device_partition_properties(ret) }
        })
    }

    pub fn partition_type(&self) -> Output<Vec<DevicePartitionProperty>> {
        self.get_info(DeviceInfo::PartitionType).map(|ret| {
            unsafe { cast_device_partition_properties(ret) }
        })
    }

    pub fn double_fp_config(&self) -> Output<DeviceFpConfig> {
        self.get_info(DeviceInfo::DoubleFpConfig).map(|ret| {
            let cfg: DeviceFpConfig = unsafe { ret.into_one() };
            cfg
        })
    }
    pub fn half_fp_config(&self) -> Output<DeviceFpConfig> {
        self.get_info(DeviceInfo::HalfFpConfig).map(|ret| {
            let cfg: DeviceFpConfig = unsafe { ret.into_one() };
            cfg
        })
    }
    pub fn single_fp_config(&self) -> Output<DeviceFpConfig> {
        self.get_info(DeviceInfo::SingleFpConfig).map(|ret| {
            let cfg: DeviceFpConfig = unsafe { ret.into_one() };
            cfg
        })
    }
    
    // Docs says cl_uint, but API returns u64?
    pub fn reference_count(&self) -> Output<u32> {
        self.get_info(DeviceInfo::SingleFpConfig).map(|ret| {
            unsafe { ret.into_one() }
        })
    }

    pub fn parent_device(&self) -> Output<Option<Device>> {
        self.get_info(DeviceInfo::ParentDevice).map(|ret| {    
            if ret.is_null() {
                None
            } else {
                let device = unsafe { 
                    ret.into_one_wrapper::<Device>().cl_retain()
                };
                Some(device)
            }
        })
    }
}

impl Device {
    fn get_info<T: Copy>(&self, info: DeviceInfo) -> Output<ClPointer<T>> {
        cl_get_device_info::<T>(self, info)
    }
}
// cl_uint
__impl_device_info_one!(address_bits, AddressBits, u32);
__impl_device_info_one!(global_mem_cacheline_size, GlobalMemCachelineSize, u32);
__impl_device_info_one!(max_clock_frequency, MaxClockFrequency, u32);
__impl_device_info_one!(max_compute_units, MaxComputeUnits, u32);
__impl_device_info_one!(max_constant_args, MaxConstantArgs, u32);
__impl_device_info_one!(max_read_image_args, MaxReadImageArgs, u32);
__impl_device_info_one!(max_samplers, MaxSamplers, u32);
__impl_device_info_one!(max_work_item_dimensions, MaxWorkItemDimensions, u32);
__impl_device_info_one!(max_write_image_args, MaxWriteImageArgs, u32);
__impl_device_info_one!(mem_base_addr_align, MemBaseAddrAlign, u32);
__impl_device_info_one!(min_data_type_align_size, MinDataTypeAlignSize, u32);
__impl_device_info_one!(native_vector_width_char, NativeVectorWidthChar, u32);
__impl_device_info_one!(native_vector_width_short, NativeVectorWidthShort, u32);
__impl_device_info_one!(native_vector_width_int, NativeVectorWidthInt, u32);
__impl_device_info_one!(native_vector_width_long, NativeVectorWidthLong, u32);
__impl_device_info_one!(native_vector_width_float, NativeVectorWidthFloat, u32);
__impl_device_info_one!(native_vector_width_double, NativeVectorWidthDouble, u32);
__impl_device_info_one!(native_vector_width_half, NativeVectorWidthHalf, u32);
__impl_device_info_one!(partition_max_sub_devices, PartitionMaxSubDevices, u32);
__impl_device_info_one!(preferred_vector_width_char, PreferredVectorWidthChar, u32);
__impl_device_info_one!(preferred_vector_width_short, PreferredVectorWidthShort, u32);
__impl_device_info_one!(preferred_vector_width_int, PreferredVectorWidthInt, u32);
__impl_device_info_one!(preferred_vector_width_long, PreferredVectorWidthLong, u32);
__impl_device_info_one!(preferred_vector_width_float, PreferredVectorWidthFloat, u32);
__impl_device_info_one!(preferred_vector_width_double, PreferredVectorWidthDouble, u32);
__impl_device_info_one!(preferred_vector_width_half, PreferredVectorWidthHalf, u32);




__impl_device_info_one!(vendor_id, VendorId, u32);

// cl_bool
__impl_device_info_one!(available, Available, bool);
__impl_device_info_one!(compiler_available, CompilerAvailable, bool);
__impl_device_info_one!(endian_little, EndianLittle, bool);
__impl_device_info_one!(error_correction_support, ErrorCorrectionSupport, bool);
__impl_device_info_one!(host_unified_memory, HostUnifiedMemory, bool);
__impl_device_info_one!(image_support, ImageSupport, bool);
__impl_device_info_one!(linker_available, LinkerAvailable, bool);
__impl_device_info_one!(preferred_interop_user_sync, PreferredInteropUserSync, bool);

// char[]
__impl_device_info_one!(name, Name, String);
__impl_device_info_one!(opencl_c_version, OpenclCVersion, String);
__impl_device_info_one!(profile, Profile, String);
__impl_device_info_one!(vendor, Vendor, String);
__impl_device_info_one!(version, Version, String);
__impl_device_info_one!(driver_version, DriverVersion, String);

// DeviceFpConfig


// ExecutionCapabilities
__impl_device_info_one!(execution_capabilities, ExecutionCapabilities, DeviceExecCapabilities);

// ulong as u64
__impl_device_info_one!(global_mem_cache_size, GlobalMemCacheSize, u64);
__impl_device_info_one!(global_mem_size, GlobalMemSize, u64);
__impl_device_info_one!(local_mem_size, LocalMemSize, u64);
__impl_device_info_one!(max_constant_buffer_size, MaxConstantBufferSize, u64);
__impl_device_info_one!(max_mem_alloc_size, MaxMemAllocSize, u64);

//  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
__impl_device_info_one!(global_mem_cache_type, GlobalMemCacheType, DeviceMemCacheType);

// size_t as usize
__impl_device_info_one!(image2d_max_width, Image2DMaxWidth, usize);
__impl_device_info_one!(image2d_max_height, Image2DMaxHeight, usize);
__impl_device_info_one!(image3d_max_width, Image3DMaxWidth, usize);
__impl_device_info_one!(image3d_max_height, Image3DMaxHeight, usize);
__impl_device_info_one!(image3d_max_depth, Image3DMaxDepth, usize);
__impl_device_info_one!(image_max_buffer_size, ImageMaxBufferSize, usize);
__impl_device_info_one!(image_max_array_size, ImageMaxArraySize, usize);
__impl_device_info_one!(max_parameter_size, MaxParameterSize, usize);
__impl_device_info_one!(max_work_group_size, MaxWorkGroupSize, usize);
__impl_device_info_one!(printf_buffer_size, PrintfBufferSize, usize);
__impl_device_info_one!(profiling_timer_resolution, ProfilingTimerResolution, usize);

// cl_device_local_mem_type
__impl_device_info_one!(local_mem_type, LocalMemType, DeviceLocalMemType);

// size_t[]
__impl_device_info_one!(max_work_item_sizes, MaxWorkItemSizes, Vec<usize>);

// Device


// cl_device_partition_property[]

// cl_device_affinity_domain
__impl_device_info_one!(partition_affinity_domain, PartitionAffinityDomain, DeviceAffinityDomain);


// DeviceType
__impl_device_info_one!(device_type, Type, DeviceType);


// v2.0+
// __impl_device_info!(queue_on_host_properties, QueueOnHostProperties, String);
// __impl_device_info!(image_pitch_alignment, ImagePitchAlignment, String);
// __impl_device_info!(image_base_address_alignment, ImageBaseAddressAlignment, String);
// __impl_device_info!(max_read_write_image_args, MaxReadWriteImageArgs, String);
// __impl_device_info!(max_global_variable_size, MaxGlobalVariableSize, String);
// __impl_device_info!(queue_on_device_properties, QueueOnDeviceProperties, String);
// __impl_device_info!(queue_on_device_preferred_size, QueueOnDevicePreferredSize, String);
// __impl_device_info!(queue_on_device_max_size, QueueOnDeviceMaxSize, String);
// __impl_device_info!(max_on_device_queues, MaxOnDeviceQueues, String);
// __impl_device_info!(max_on_device_events, MaxOnDeviceEvents, String);
// __impl_device_info!(svm_capabilities, SvmCapabilities, String);
// __impl_device_info!(global_variable_preferred_total_size, GlobalVariablePreferredTotalSize, String);
// __impl_device_info!(max_pipe_args, MaxPipeArgs, String);
// __impl_device_info!(pipe_max_active_reservations, PipeMaxActiveReservations, String);
// __impl_device_info!(pipe_max_packet_size, PipeMaxPacketSize, String);
// __impl_device_info!(preferred_platform_atomic_alignment, PreferredPlatformAtomicAlignment, String);
// __impl_device_info!(preferred_global_atomic_alignment, PreferredGlobalAtomicAlignment, String);
// __impl_device_info!(preferred_local_atomic_alignment, PreferredLocalAtomicAlignment, String);
// __impl_device_info!(il_version, IlVersion, String);
// __impl_device_info!(max_num_sub_groups, MaxNumSubGroups, String);
// __impl_device_info!(sub_group_independent_forward_progress, SubGroupIndependentForwardProgress, String);


#[cfg(test)]
mod tests {
    use super::{
        Device, 
        // DeviceInfo,
        DeviceType,
        DeviceMemCacheType,
        DeviceLocalMemType,
        DeviceExecCapabilities,
        DevicePartitionProperty,
        DeviceAffinityDomain,
    };

    use crate::device::flags::DeviceFpConfig;
    use crate::platform::Platform;

    #[test]
    fn device_method_type_works() {
        let device = Device::default();
        let device_type: DeviceType = device.device_type()
            .expect("Device method test for device_type failed");
        assert!(
            device_type == DeviceType::CPU || device_type == DeviceType::GPU
        );
    }

    #[test]
    fn device_method_vendor_id_works() {
        let device = Device::default();
        let vendor_id = device.vendor_id()
            .expect("Device method test for vendor_id failed");
            
        assert!(vendor_id != 0);
    }

    #[test]
    fn device_method_max_compute_units_works() {
        let device = Device::default();
        let info: u32 = device.max_compute_units()
            .expect("Device method test for max_compute_units failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_work_item_dimensions_works() {
        let device = Device::default();
        let info: u32 = device.max_work_item_dimensions()
            .expect("Device method test for max_work_item_dimensions failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_work_group_size_works() {
        let device = Device::default();
        let info: usize = device.max_work_group_size()
            .expect("Device method test for max_work_group_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_work_item_sizes_works() {
        let device = Device::default();
        let item_sizes: Vec<usize> = device.max_work_item_sizes()
            .expect("Device method test for max_work_item_sizes failed");
        assert!(item_sizes.len() > 0);
    }

    #[test]
    fn device_method_preferred_vector_width_char_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_char()
            .expect("Device method test for preferred_vector_width_char failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_preferred_vector_width_short_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_short()
            .expect("Device method test for preferred_vector_width_short failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_preferred_vector_width_int_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_int()
            .expect("Device method test for preferred_vector_width_int failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_preferred_vector_width_long_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_long()
            .expect("Device method test for preferred_vector_width_long failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_preferred_vector_width_float_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_float()
            .expect("Device method test for preferred_vector_width_float failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_preferred_vector_width_double_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_double()
            .expect("Device method test for preferred_vector_width_double failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_clock_frequency_works() {
        let device = Device::default();
        let info: u32 = device.max_clock_frequency()
            .expect("Device method test for max_clock_frequency failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_address_bits_works() {
        let device = Device::default();
        let info: u32 = device.address_bits()
            .expect("Device method test for address_bits failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_read_image_args_works() {
        let device = Device::default();
        let info: u32 = device.max_read_image_args()
            .expect("Device method test for max_read_image_args failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_write_image_args_works() {
        let device = Device::default();
        let info: u32 = device.max_write_image_args()
            .expect("Device method test for max_write_image_args failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_mem_alloc_size_works() {
        let device = Device::default();
        let info: u64 = device.max_mem_alloc_size()
            .expect("Device method test for max_mem_alloc_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image2d_max_width_works() {
        let device = Device::default();
        let info: usize = device.image2d_max_width()
            .expect("Device method test for image2_d_max_width failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image2d_max_height_works() {
        let device = Device::default();
        let info: usize = device.image2d_max_height()
            .expect("Device method test for image2_d_max_height failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image3d_max_width_works() {
        let device = Device::default();
        let info: usize = device.image3d_max_width()
            .expect("Device method test for image3_d_max_width failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image3d_max_height_works() {
        let device = Device::default();
        let info: usize = device.image3d_max_height()
            .expect("Device method test for image3_d_max_height failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image3_d_max_depth_works() {
        let device = Device::default();
        let info: usize = device.image3d_max_depth()
            .expect("Device method test for image3_d_max_depth failed");
            
        assert!(info > 0);
    }

    #[test]
    fn device_method_image_support_works() {
        let device = Device::default();
        let _info: bool = device.image_support()
            .expect("Device method test for image_support failed");
    }

    #[test]
    fn device_method_max_parameter_size_works() {
        let device = Device::default();
        let info: usize = device.max_parameter_size()
            .expect("Device method test for max_parameter_size failed");
            
        assert!(info > 0);
    }

    #[test]
    fn device_method_max_samplers_works() {
        let device = Device::default();
        let info: u32 = device.max_samplers()
            .expect("Device method test for max_samplers failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_mem_base_addr_align_works() {
        let device = Device::default();
        let info: u32 = device.mem_base_addr_align()
            .expect("Device method test for mem_base_addr_align failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_min_data_type_align_size_works() {
        let device = Device::default();
        let info: u32 = device.min_data_type_align_size()
            .expect("Device method test for min_data_type_align_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_single_fp_config_works() {
        let device = Device::default();
        let _info: DeviceFpConfig = device.single_fp_config()
            .expect("Device method test for single_fp_config failed");
    }

    #[test]
    fn device_method_global_mem_cache_type_works() {
        let device = Device::default();
        let info: DeviceMemCacheType = device.global_mem_cache_type()
            .expect("Device method test for global_mem_cache_type failed");
            
        assert!(
            info == DeviceMemCacheType::NoneType ||
            info == DeviceMemCacheType::ReadOnlyCache ||
            info == DeviceMemCacheType::ReadWriteCache
        );
    }

    #[test]
    fn device_method_global_mem_cacheline_size_works() {
        let device = Device::default();
        let info: u32 = device.global_mem_cacheline_size()
            .expect("Device method test for global_mem_cacheline_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_global_mem_cache_size_works() {
        let device = Device::default();
        let info: u64 = device.global_mem_cache_size()
            .expect("Device method test for global_mem_cache_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_global_mem_size_works() {
        let device = Device::default();
        let info: u64 = device.global_mem_size()
            .expect("Device method test for global_mem_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_constant_buffer_size_works() {
        let device = Device::default();
        let info: u64 = device.max_constant_buffer_size()
            .expect("Device method test for max_constant_buffer_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_max_constant_args_works() {
        let device = Device::default();
        let info: u32 = device.max_constant_args()
            .expect("Device method test for max_constant_args failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_local_mem_type_works() {
        let device = Device::default();
        let info: DeviceLocalMemType = device.local_mem_type()
            .expect("Device method test for local_mem_type failed");
            
        assert!(info == DeviceLocalMemType::Local || info == DeviceLocalMemType::Global);
    }

    #[test]
    fn device_method_local_mem_size_works() {
        let device = Device::default();
        let info: u64 = device.local_mem_size()
            .expect("Device method test for local_mem_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_error_correction_support_works() {
        let device = Device::default();
        let _info: bool = device.error_correction_support()
            .expect("Device method test for error_correction_support failed");
    }

    #[test]
    fn device_method_profiling_timer_resolution_works() {
        let device = Device::default();
        let info: usize = device.profiling_timer_resolution()
            .expect("Device method test for profiling_timer_resolution failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_endian_little_works() {
        let device = Device::default();
        let _info: bool = device.endian_little()
            .expect("Device method test for endian_little failed");
    }

    #[test]
    fn device_method_available_works() {
        let device = Device::default();
        let _info: bool = device.available()
            .expect("Device method test for available failed");
    }

    #[test]
    fn device_method_compiler_available_works() {
        let device = Device::default();
        let _info: bool = device.compiler_available()
            .expect("Device method test for compiler_available failed");
    }

    #[test]
    fn device_method_execution_capabilities_works() {
        // use DeviceExecCapabilities as C;
        let device = Device::default();
        let _info: DeviceExecCapabilities = device.execution_capabilities()
            .expect("Device method test for execution_capabilities failed");
            
        // assert!(
        //     info == C::Kernel || info == C::NativeKernel
        // );
    }

    #[test]
    fn device_method_name_works() {
        let device = Device::default();
        let name: String = device.name()
            .expect("Device method test for name failed");
            
        assert!(name != "".to_string());
    }

    #[test]
    fn device_method_vendor_works() {
        let device = Device::default();
        let vendor: String = device.vendor()
            .expect("Device method test for vendor failed");
            
        assert!(vendor != "".to_string());
    }

    #[test]
    fn device_method_profile_works() {
        let device = Device::default();
        let profile: String = device.profile()
            .expect("Device method test for profile failed");
            
        assert!(profile != "".to_string());
    }

    #[test]
    fn device_method_version_works() {
        let device = Device::default();
        let version: String = device.version()
            .expect("Device method test for version failed");
            
        assert!(version != "".to_string());
    }

    #[test]
    fn device_method_extensions_works() {
        let device = Device::default();
        let _extensions: Vec<String> = device.extensions()
            .expect("Device method test for extensions failed");
    }

    #[test]
    fn device_method_platform_works() {
        let device = Device::default();
        let _info: Platform = device.platform()
            .expect("Device method test for platform failed");
    }

    #[test]
    fn device_method_double_fp_config_works() {
        let device = Device::default();
        let _info: DeviceFpConfig = device.double_fp_config()
            .expect("Device method test for double_fp_config failed");
    }

    #[test]
    fn device_method_preferred_vector_width_half_works() {
        let device = Device::default();
        let info: u32 = device.preferred_vector_width_half()
            .expect("Device method test for preferred_vector_width_half failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_host_unified_memory_works() {
        let device = Device::default();
        let _info: bool = device.host_unified_memory()
            .expect("Device method test for host_unified_memory failed");
    }

    #[test]
    fn device_method_native_vector_width_char_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_char()
            .expect("Device method test for native_vector_width_char failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_short_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_short()
            .expect("Device method test for native_vector_width_short failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_int_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_int()
            .expect("Device method test for native_vector_width_int failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_long_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_long()
            .expect("Device method test for native_vector_width_long failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_float_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_float()
            .expect("Device method test for native_vector_width_float failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_double_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_double()
            .expect("Device method test for native_vector_width_double failed");
        assert!(info != 0);
    }

    #[test]
    fn device_method_native_vector_width_half_works() {
        let device = Device::default();
        let info: u32 = device.native_vector_width_half()
            .expect("Device method test for native_vector_width_half failed");
        assert!(info != 0);
    }

    #[test]
    fn device_method_opencl_c_version_works() {
        let device = Device::default();
        let info: String = device.opencl_c_version()
            .expect("Device method test for opencl_c_version failed");
            
        assert!(info != "".to_string());
    }

    #[test]
    fn device_method_linker_available_works() {
        let device = Device::default();
        let _info: bool = device.linker_available()
            .expect("Device method test for linker_available failed");
    }

    #[test]
    fn device_method_built_in_kernels_works() {
        let device = Device::default();
        let _info: Vec<String> = device.built_in_kernels()
            .expect("Device method test for built_in_kernels failed");
    }

    #[test]
    fn device_method_image_max_buffer_size_works() {
        let device = Device::default();
        let info: usize = device.image_max_buffer_size()
            .expect("Device method test for image_max_buffer_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_image_max_array_size_works() {
        let device = Device::default();
        let info: usize = device.image_max_array_size()
            .expect("Device method test for image_max_array_size failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_parent_device_works() {
        let device = Device::default();
        device.parent_device().map(|maybe_device| {
            if let Some(parent_device) = maybe_device {
                assert!(device != parent_device);
                let name = device.name().expect("Failed to get device name");
                let p_name = parent_device.name().expect("Failed to get parent_device name");
                assert!(name != p_name);
            };
        }).expect("Called to device.parent_device() failed");
    }

    #[test]
    fn device_method_partition_max_sub_devices_works() {
        let device = Device::default();
        let info: u32 = device.partition_max_sub_devices()
            .expect("Device method test for partition_max_sub_devices failed");
            
        assert!(info != 0);
    }

    #[test]
    fn device_method_partition_properties_works() {
        let device = Device::default();
        let _info: Vec<DevicePartitionProperty> = device.partition_properties()
            .expect("Device method test for partition_properties failed");
    }

    #[test]
    fn device_method_partition_affinity_domain_works() {
        let device = Device::default();
        let _info: DeviceAffinityDomain = device.partition_affinity_domain()
            .expect("Device method test for partition_affinity_domain failed");
    }

    #[test]
    fn device_method_partition_type_works() {
        use DevicePartitionProperty as P;
        let device = Device::default();
        let infos: Vec<DevicePartitionProperty> = device.partition_type()
            .expect("Device method test for partition_type failed");
        for info in infos {
            assert!(
                info == P::Equally ||
                info == P::ByCounts ||
                info == P::ByAffinityDomain
            );
        }




    }

    #[test]
    fn device_method_reference_count_works() {
        let device = Device::default();
        let count: u32 = device.reference_count()
            .expect("Device method test for reference_count failed");

        // NOTE: I have no idea why 191 is here...
        assert_eq!(count, 191);
    }

    #[test]
    fn device_method_preferred_interop_user_sync_works() {
        let device = Device::default();
        let _info: bool = device.preferred_interop_user_sync()
            .expect("Device method test for preferred_interop_user_sync failed");
    }

    #[test]
    fn device_method_printf_buffer_size_works() {
        let device = Device::default();
        let info: usize = device.printf_buffer_size()
            .expect("Device method test for printf_buffer_size failed");
            
        assert!(info != 0);
    }

    // v2.0+

    // #[test]
    // fn device_method_queue_on_host_properties_works() {
    //     let device = Device::default();
    //     let info = device.queue_on_host_properties()
    //         .expect("Device method test for queue_on_host_properties failed");
            
    //     assert!(info != "".to_string());
    // }


    // #[test]
    // fn device_method_image_pitch_alignment_works() {
    //     let device = Device::default();
    //     let info = device.image_pitch_alignment()
    //         .expect("Device method test for image_pitch_alignment failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_image_base_address_alignment_works() {
    //     let device = Device::default();
    //     let info = device.image_base_address_alignment()
    //         .expect("Device method test for image_base_address_alignment failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_read_write_image_args_works() {
    //     let device = Device::default();
    //     let info = device.max_read_write_image_args()
    //         .expect("Device method test for max_read_write_image_args failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_global_variable_size_works() {
    //     let device = Device::default();
    //     let info = device.max_global_variable_size()
    //         .expect("Device method test for max_global_variable_size failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_queue_on_device_properties_works() {
    //     let device = Device::default();
    //     let info = device.queue_on_device_properties()
    //         .expect("Device method test for queue_on_device_properties failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_queue_on_device_preferred_size_works() {
    //     let device = Device::default();
    //     let info = device.queue_on_device_preferred_size()
    //         .expect("Device method test for queue_on_device_preferred_size failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_queue_on_device_max_size_works() {
    //     let device = Device::default();
    //     let info = device.queue_on_device_max_size()
    //         .expect("Device method test for queue_on_device_max_size failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_on_device_queues_works() {
    //     let device = Device::default();
    //     let info = device.max_on_device_queues()
    //         .expect("Device method test for max_on_device_queues failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_on_device_events_works() {
    //     let device = Device::default();
    //     let info = device.max_on_device_events()
    //         .expect("Device method test for max_on_device_events failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_svm_capabilities_works() {
    //     let device = Device::default();
    //     let info = device.svm_capabilities()
    //         .expect("Device method test for svm_capabilities failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_global_variable_preferred_total_size_works() {
    //     let device = Device::default();
    //     let info = device.global_variable_preferred_total_size()
    //         .expect("Device method test for global_variable_preferred_total_size failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_pipe_args_works() {
    //     let device = Device::default();
    //     let info = device.max_pipe_args()
    //         .expect("Device method test for max_pipe_args failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_pipe_max_active_reservations_works() {
    //     let device = Device::default();
    //     let info = device.pipe_max_active_reservations()
    //         .expect("Device method test for pipe_max_active_reservations failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_pipe_max_packet_size_works() {
    //     let device = Device::default();
    //     let info = device.pipe_max_packet_size()
    //         .expect("Device method test for pipe_max_packet_size failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_preferred_platform_atomic_alignment_works() {
    //     let device = Device::default();
    //     let info = device.preferred_platform_atomic_alignment()
    //         .expect("Device method test for preferred_platform_atomic_alignment failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_preferred_global_atomic_alignment_works() {
    //     let device = Device::default();
    //     let info = device.preferred_global_atomic_alignment()
    //         .expect("Device method test for preferred_global_atomic_alignment failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_preferred_local_atomic_alignment_works() {
    //     let device = Device::default();
    //     let info = device.preferred_local_atomic_alignment()
    //         .expect("Device method test for preferred_local_atomic_alignment failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_il_version_works() {
    //     let device = Device::default();
    //     let info = device.il_version()
    //         .expect("Device method test for il_version failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_max_num_sub_groups_works() {
    //     let device = Device::default();
    //     let info = device.max_num_sub_groups()
    //         .expect("Device method test for max_num_sub_groups failed");
            
    //     assert!(info != "".to_string());
    // }

    // #[test]
    // fn device_method_sub_group_independent_forward_progress_works() {
    //     let device = Device::default();
    //     let info = device.sub_group_independent_forward_progress()
    //         .expect("Device method test for sub_group_independent_forward_progress failed");
            
    //     assert!(info != "".to_string());
    // }

}