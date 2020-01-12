// use crate::error::{Error, Output};
// /// This module implements DeviceInfo methods for Device.
// /// It was too much boilerplate for the Device module.
// use crate::ffi::*;

// use crate::cl::{cl_get_info5, ClObjectError, ClPointer};

// use crate::device::flags::{
//     DeviceAffinityDomain, DeviceExecCapabilities, DeviceFpConfig, DeviceLocalMemType,
//     DeviceMemCacheType, DevicePartitionProperty,
// };
// use crate::device::{Device, DeviceError, DeviceType, DevicePtr}; //, NO_PARENT_DEVICE_ERROR};

// use crate::platform::Platform;



// macro_rules! __impl_device_info_one {
//     ($name:ident, $flag:ident, String) => {
//         impl Device {
//             pub fn $name(&self) -> Output<String> {
//                 self.get_info(DeviceInfo::$flag)
//                     .map(|ret| unsafe { ret.into_string() })
//             }
//         }
//     };

//     ($name:ident, $flag:ident, bool) => {
//         impl Device {
//             pub fn $name(&self) -> Output<bool> {
//                 use crate::ffi::cl_bool;
//                 self.get_info::<cl_bool>(DeviceInfo::$flag)
//                     .map(From::from)
//             }
//         }
//     };

//     ($name:ident, $flag:ident, Vec<$output_t:ty>) => {
//         impl Device {
//             pub fn $name(&self) -> Output<Vec<$output_t>> {
//                 self.get_info(DeviceInfo::$flag)
//                     .map(|ret| unsafe { ret.into_vec() })
//             }
//         }
//     };
//     ($name:ident, $flag:ident, $output_t:ty) => {
//         impl Device {
//             pub fn $name(&self) -> Output<$output_t> {
//                 self.get_info(DeviceInfo::$flag)
//                     .map(|ret| unsafe { ret.into_one() })
//             }
//         }
//     };
//     ($name:ident, $flag:ident, $cl_type:ty, $output_t:ty) => {
//         impl Device {
//             pub fn $name(&self) -> Output<$output_t> {
//                 self.get_info::<$cl_type>(DeviceInfo::$flag)
//                     .map(|ret| unsafe { ret.into_one() })
//             }
//         }
//     };
// }

// unsafe fn cast_device_partition_properties(
//     ret: ClPointer<cl_device_partition_property>,
// ) -> Vec<DevicePartitionProperty> {
//     let props: Vec<cl_device_partition_property> = ret.into_vec();
//     let mut output = Vec::new();
//     for p in props {
//         // Zero here is an indication that the list of  device partition properties is
//         // at an end. So we immediately return.
//         if p == 0 {
//             return output;
//         }
//         output.push(DevicePartitionProperty::from(p))
//     }
//     output
// }

// impl Device {
//     pub fn built_in_kernels(&self) -> Output<Vec<String>> {
//         self.get_info(DeviceInfo::BuiltInKernels).map(|ret| {
//             let kernel_names: String = unsafe { ret.into_string() };
//             kernel_names.split(';').map(|s| s.to_string()).collect()
//         })
//     }

//     pub fn extensions(&self) -> Output<Vec<String>> {
//         self.get_info(DeviceInfo::Extensions).map(|ret| {
//             let kernels: String = unsafe { ret.into_string() };
//             kernels.split(' ').map(|s| s.to_string()).collect()
//         })
//     }

//     pub fn partition_properties(&self) -> Output<Vec<DevicePartitionProperty>> {
//         self.get_info(DeviceInfo::PartitionProperties)
//             .map(|ret| unsafe { cast_device_partition_properties(ret) })
//     }

//     pub fn partition_type(&self) -> Output<Vec<DevicePartitionProperty>> {
//         self.get_info(DeviceInfo::PartitionType)
//             .map(|ret| unsafe { cast_device_partition_properties(ret) })
//     }

//     pub fn double_fp_config(&self) -> Output<DeviceFpConfig> {
//         self.get_info(DeviceInfo::DoubleFpConfig).map(|ret| {
//             let cfg: DeviceFpConfig = unsafe { ret.into_one() };
//             cfg
//         })
//     }
//     pub fn half_fp_config(&self) -> Output<DeviceFpConfig> {
//         self.get_info(DeviceInfo::HalfFpConfig).map(|ret| {
//             let cfg: DeviceFpConfig = unsafe { ret.into_one() };
//             cfg
//         })
//     }
//     pub fn single_fp_config(&self) -> Output<DeviceFpConfig> {
//         self.get_info(DeviceInfo::SingleFpConfig).map(|ret| {
//             let cfg: DeviceFpConfig = unsafe { ret.into_one() };
//             cfg
//         })
//     }

//     // Docs says cl_uint, but API returns u64?
//     pub fn reference_count(&self) -> Output<u64> {
//         self.get_info::<cl_ulong>(DeviceInfo::SingleFpConfig)
//             .map(|ret| unsafe { ret.into_one() })
//     }

//     pub fn parent_device(&self) -> Output<Device> {
//         self.get_info::<cl_device_id>(DeviceInfo::ParentDevice)
//             .and_then(|ret| unsafe {
//                 let device_id: cl_device_id = ret.into_one();
//                 if device_id.is_null() {
//                     return Err(NO_PARENT_DEVICE_ERROR);
//                 }
//                 Device::from_unretained_object(device_id)
//             })
//             .map_err(|e| match e {
//                 Error::ClObjectError(ClObjectError::ClObjectCannotBeNull(..)) => {
//                     DeviceError::NoParentDevice.into()
//                 }
//                 _ => e,
//             })
//     }
//     pub fn platform(&self) -> Output<Platform> {
//         self.get_info(DeviceInfo::Platform)
//             .and_then(|ret| unsafe { Ok(Platform::new(ret.into_one())) })
//     }
// }

// // cl_uint
// __impl_device_info_one!(
//     global_mem_cacheline_size,
//     GlobalMemCachelineSize,
//     cl_uint,
//     u32
// );
// __impl_device_info_one!(
//     native_vector_width_double,
//     NativeVectorWidthDouble,
//     cl_uint,
//     u32
// );
// __impl_device_info_one!(
//     native_vector_width_half,
//     NativeVectorWidthHalf,
//     cl_uint,
//     u32
// );
// __impl_device_info_one!(address_bits, AddressBits, u32);
// __impl_device_info_one!(max_clock_frequency, MaxClockFrequency, u32);
// __impl_device_info_one!(max_compute_units, MaxComputeUnits, u32);
// __impl_device_info_one!(max_constant_args, MaxConstantArgs, u32);
// __impl_device_info_one!(max_read_image_args, MaxReadImageArgs, u32);
// __impl_device_info_one!(max_samplers, MaxSamplers, u32);
// __impl_device_info_one!(max_work_item_dimensions, MaxWorkItemDimensions, u32);
// __impl_device_info_one!(max_write_image_args, MaxWriteImageArgs, u32);
// __impl_device_info_one!(mem_base_addr_align, MemBaseAddrAlign, u32);
// __impl_device_info_one!(min_data_type_align_size, MinDataTypeAlignSize, u32);
// __impl_device_info_one!(native_vector_width_char, NativeVectorWidthChar, u32);
// __impl_device_info_one!(native_vector_width_short, NativeVectorWidthShort, u32);
// __impl_device_info_one!(native_vector_width_int, NativeVectorWidthInt, u32);
// __impl_device_info_one!(native_vector_width_long, NativeVectorWidthLong, u32);
// __impl_device_info_one!(native_vector_width_float, NativeVectorWidthFloat, u32);
// __impl_device_info_one!(
//     partition_max_sub_devices,
//     PartitionMaxSubDevices,
//     cl_uint,
//     u32
// );
// __impl_device_info_one!(preferred_vector_width_char, PreferredVectorWidthChar, u32);
// __impl_device_info_one!(preferred_vector_width_short, PreferredVectorWidthShort, u32);
// __impl_device_info_one!(preferred_vector_width_int, PreferredVectorWidthInt, u32);
// __impl_device_info_one!(preferred_vector_width_long, PreferredVectorWidthLong, u32);
// __impl_device_info_one!(preferred_vector_width_float, PreferredVectorWidthFloat, u32);
// __impl_device_info_one!(
//     preferred_vector_width_double,
//     PreferredVectorWidthDouble,
//     u32
// );
// __impl_device_info_one!(preferred_vector_width_half, PreferredVectorWidthHalf, u32);
// __impl_device_info_one!(vendor_id, VendorId, u32);

// // cl_bool
// __impl_device_info_one!(available, Available, bool);
// __impl_device_info_one!(compiler_available, CompilerAvailable, bool);
// __impl_device_info_one!(endian_little, EndianLittle, bool);
// __impl_device_info_one!(error_correction_support, ErrorCorrectionSupport, bool);
// __impl_device_info_one!(host_unified_memory, HostUnifiedMemory, bool);
// __impl_device_info_one!(image_support, ImageSupport, bool);
// __impl_device_info_one!(linker_available, LinkerAvailable, bool);
// __impl_device_info_one!(preferred_interop_user_sync, PreferredInteropUserSync, bool);

// // char[]
// __impl_device_info_one!(name, Name, String);
// __impl_device_info_one!(opencl_c_version, OpenclCVersion, String);
// __impl_device_info_one!(profile, Profile, String);
// __impl_device_info_one!(vendor, Vendor, String);
// __impl_device_info_one!(version, Version, String);
// __impl_device_info_one!(driver_version, DriverVersion, String);

// // ulong as u64
// __impl_device_info_one!(global_mem_cache_size, GlobalMemCacheSize, cl_ulong, u64);
// __impl_device_info_one!(global_mem_size, GlobalMemSize, u64);
// __impl_device_info_one!(local_mem_size, LocalMemSize, u64);
// __impl_device_info_one!(max_constant_buffer_size, MaxConstantBufferSize, u64);
// __impl_device_info_one!(max_mem_alloc_size, MaxMemAllocSize, u64);

// // size_t as usize
// __impl_device_info_one!(image2d_max_width, Image2DMaxWidth, usize);
// __impl_device_info_one!(image2d_max_height, Image2DMaxHeight, usize);
// __impl_device_info_one!(image3d_max_width, Image3DMaxWidth, usize);
// __impl_device_info_one!(image3d_max_height, Image3DMaxHeight, usize);
// __impl_device_info_one!(image3d_max_depth, Image3DMaxDepth, usize);
// __impl_device_info_one!(image_max_buffer_size, ImageMaxBufferSize, usize);
// __impl_device_info_one!(image_max_array_size, ImageMaxArraySize, usize);
// __impl_device_info_one!(max_parameter_size, MaxParameterSize, usize);
// __impl_device_info_one!(max_work_group_size, MaxWorkGroupSize, usize);
// __impl_device_info_one!(printf_buffer_size, PrintfBufferSize, usize);
// __impl_device_info_one!(profiling_timer_resolution, ProfilingTimerResolution, usize);

// // size_t[]
// __impl_device_info_one!(max_work_item_sizes, MaxWorkItemSizes, Vec<usize>);

// // cl_device_local_mem_type
// __impl_device_info_one!(local_mem_type, LocalMemType, DeviceLocalMemType);

// // ExecutionCapabilities
// __impl_device_info_one!(
//     execution_capabilities,
//     ExecutionCapabilities,
//     DeviceExecCapabilities
// );

// //  CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
// __impl_device_info_one!(
//     global_mem_cache_type,
//     GlobalMemCacheType,
//     DeviceMemCacheType
// );

// // cl_device_affinity_domain
// __impl_device_info_one!(
//     partition_affinity_domain,
//     PartitionAffinityDomain,
//     DeviceAffinityDomain
// );

// // DeviceType
// __impl_device_info_one!(device_type, Type, DeviceType);

// // v2.0+
// // __impl_device_info!(queue_on_host_properties, QueueOnHostProperties, String);
// // __impl_device_info!(image_pitch_alignment, ImagePitchAlignment, String);
// // __impl_device_info!(image_base_address_alignment, ImageBaseAddressAlignment, String);
// // __impl_device_info!(max_read_write_image_args, MaxReadWriteImageArgs, String);
// // __impl_device_info!(max_global_variable_size, MaxGlobalVariableSize, String);
// // __impl_device_info!(queue_on_device_properties, QueueOnDeviceProperties, String);
// // __impl_device_info!(queue_on_device_preferred_size, QueueOnDevicePreferredSize, String);
// // __impl_device_info!(queue_on_device_max_size, QueueOnDeviceMaxSize, String);
// // __impl_device_info!(max_on_device_queues, MaxOnDeviceQueues, String);
// // __impl_device_info!(max_on_device_events, MaxOnDeviceEvents, String);
// // __impl_device_info!(svm_capabilities, SvmCapabilities, String);
// // __impl_device_info!(global_variable_preferred_total_size, GlobalVariablePreferredTotalSize, String);
// // __impl_device_info!(max_pipe_args, MaxPipeArgs, String);
// // __impl_device_info!(pipe_max_active_reservations, PipeMaxActiveReservations, String);
// // __impl_device_info!(pipe_max_packet_size, PipeMaxPacketSize, String);
// // __impl_device_info!(preferred_platform_atomic_alignment, PreferredPlatformAtomicAlignment, String);
// // __impl_device_info!(preferred_global_atomic_alignment, PreferredGlobalAtomicAlignment, String);
// // __impl_device_info!(preferred_local_atomic_alignment, PreferredLocalAtomicAlignment, String);
// // __impl_device_info!(il_version, IlVersion, String);
// // __impl_device_info!(max_num_sub_groups, MaxNumSubGroups, String);
// // __impl_device_info!(sub_group_independent_forward_progress, SubGroupIndependentForwardProgress, String);

// #[cfg(test)]
// mod tests {
//     use super::{
//         Device,
//         DeviceAffinityDomain,
//         DeviceExecCapabilities,
//         DeviceLocalMemType,
//         DeviceMemCacheType,
//         DevicePartitionProperty,
//         // DeviceInfo,
//         DeviceType,
//         DevicePtr,
//     };

//     use crate::device::flags::DeviceFpConfig;
//     use crate::device::DeviceError;
//     use crate::error::Error;
//     use crate::platform::Platform;

//     #[test]
//     fn device_method_type_works() {
//         let device = Device::default();
//         let device_type: DeviceType = device
//             .device_type()
//             .expect("Device method test for device_type failed");
//         assert!(device_type == DeviceType::CPU || device_type == DeviceType::GPU);
//     }

//     #[test]
//     fn device_method_vendor_id_works() {
//         let device = Device::default();
//         let vendor_id = device
//             .vendor_id()
//             .expect("Device method test for vendor_id failed");

//         assert!(vendor_id != 0);
//     }

//     #[test]
//     fn device_method_max_compute_units_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_compute_units()
//             .expect("Device method test for max_compute_units failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_work_item_dimensions_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_work_item_dimensions()
//             .expect("Device method test for max_work_item_dimensions failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_work_group_size_works() {
//         let device = Device::default();
//         let info: usize = device
//             .max_work_group_size()
//             .expect("Device method test for max_work_group_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_work_item_sizes_works() {
//         let device = Device::default();
//         let item_sizes: Vec<usize> = device
//             .max_work_item_sizes()
//             .expect("Device method test for max_work_item_sizes failed");
//         assert!(item_sizes.len() > 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_char_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .preferred_vector_width_char()
//             .expect("Device method test for preferred_vector_width_char failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_short_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .preferred_vector_width_short()
//             .expect("Device method test for preferred_vector_width_short failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_int_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .preferred_vector_width_int()
//             .expect("Device method test for preferred_vector_width_int failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_long_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .preferred_vector_width_long()
//             .expect("Device method test for preferred_vector_width_long failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_float_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .preferred_vector_width_float()
//             .expect("Device method test for preferred_vector_width_float failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_preferred_vector_width_double_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .preferred_vector_width_double()
//             .expect("Device method test for preferred_vector_width_double failed");
//     }

//     #[test]
//     fn device_method_max_clock_frequency_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_clock_frequency()
//             .expect("Device method test for max_clock_frequency failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_address_bits_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .address_bits()
//             .expect("Device method test for address_bits failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_read_image_args_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_read_image_args()
//             .expect("Device method test for max_read_image_args failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_write_image_args_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_write_image_args()
//             .expect("Device method test for max_write_image_args failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_mem_alloc_size_works() {
//         let device = Device::default();
//         let info: u64 = device
//             .max_mem_alloc_size()
//             .expect("Device method test for max_mem_alloc_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image2d_max_width_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image2d_max_width()
//             .expect("Device method test for image2_d_max_width failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image2d_max_height_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image2d_max_height()
//             .expect("Device method test for image2_d_max_height failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image3d_max_width_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image3d_max_width()
//             .expect("Device method test for image3_d_max_width failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image3d_max_height_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image3d_max_height()
//             .expect("Device method test for image3_d_max_height failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image3_d_max_depth_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image3d_max_depth()
//             .expect("Device method test for image3_d_max_depth failed");

//         assert!(info > 0);
//     }

//     #[test]
//     fn device_method_image_support_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .image_support()
//             .expect("Device method test for image_support failed");
//     }

//     #[test]
//     fn device_method_max_parameter_size_works() {
//         let device = Device::default();
//         let info: usize = device
//             .max_parameter_size()
//             .expect("Device method test for max_parameter_size failed");

//         assert!(info > 0);
//     }

//     #[test]
//     fn device_method_max_samplers_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_samplers()
//             .expect("Device method test for max_samplers failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_mem_base_addr_align_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .mem_base_addr_align()
//             .expect("Device method test for mem_base_addr_align failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_min_data_type_align_size_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .min_data_type_align_size()
//             .expect("Device method test for min_data_type_align_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_single_fp_config_works() {
//         let device = Device::default();
//         let _info: DeviceFpConfig = device
//             .single_fp_config()
//             .expect("Device method test for single_fp_config failed");
//     }

//     #[test]
//     fn device_method_global_mem_cache_type_works() {
//         let device = Device::default();
//         let info: DeviceMemCacheType = device
//             .global_mem_cache_type()
//             .expect("Device method test for global_mem_cache_type failed");

//         assert!(
//             info == DeviceMemCacheType::NoneType
//                 || info == DeviceMemCacheType::ReadOnlyCache
//                 || info == DeviceMemCacheType::ReadWriteCache
//         );
//     }

//     #[test]
//     fn device_method_global_mem_cacheline_size_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .global_mem_cacheline_size()
//             .expect("Device method test for global_mem_cacheline_size failed");
//         // NOTE: Needs meaningful test for correct behavior.
//     }

//     #[test]
//     fn device_method_global_mem_cache_size_works() {
//         let device = Device::default();
//         let _info: u64 = device
//             .global_mem_cache_size()
//             .expect("Device method test for global_mem_cache_size failed");
//         // Note: It returns a u64. Not sure how to make this test meaningful though...
//     }

//     #[test]
//     fn device_method_global_mem_size_works() {
//         let device = Device::default();
//         let info: u64 = device
//             .global_mem_size()
//             .expect("Device method test for global_mem_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_constant_buffer_size_works() {
//         let device = Device::default();
//         let info: u64 = device
//             .max_constant_buffer_size()
//             .expect("Device method test for max_constant_buffer_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_max_constant_args_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .max_constant_args()
//             .expect("Device method test for max_constant_args failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_local_mem_type_works() {
//         let device = Device::default();
//         let info: DeviceLocalMemType = device
//             .local_mem_type()
//             .expect("Device method test for local_mem_type failed");

//         assert!(info == DeviceLocalMemType::Local || info == DeviceLocalMemType::Global);
//     }

//     #[test]
//     fn device_method_local_mem_size_works() {
//         let device = Device::default();
//         let info: u64 = device
//             .local_mem_size()
//             .expect("Device method test for local_mem_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_error_correction_support_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .error_correction_support()
//             .expect("Device method test for error_correction_support failed");
//     }

//     #[test]
//     fn device_method_profiling_timer_resolution_works() {
//         let device = Device::default();
//         let info: usize = device
//             .profiling_timer_resolution()
//             .expect("Device method test for profiling_timer_resolution failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_endian_little_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .endian_little()
//             .expect("Device method test for endian_little failed");
//     }

//     #[test]
//     fn device_method_available_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .available()
//             .expect("Device method test for available failed");
//     }

//     #[test]
//     fn device_method_compiler_available_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .compiler_available()
//             .expect("Device method test for compiler_available failed");
//     }

//     #[test]
//     fn device_method_execution_capabilities_works() {
//         // use DeviceExecCapabilities as C;
//         let device = Device::default();
//         let _info: DeviceExecCapabilities = device
//             .execution_capabilities()
//             .expect("Device method test for execution_capabilities failed");

//         // assert!(
//         //     info == C::Kernel || info == C::NativeKernel
//         // );
//     }

//     #[test]
//     fn device_method_name_works() {
//         let device = Device::default();
//         let name: String = device.name().expect("Device method test for name failed");

//         assert!(name != "".to_string());
//     }

//     #[test]
//     fn device_method_vendor_works() {
//         let device = Device::default();
//         let vendor: String = device
//             .vendor()
//             .expect("Device method test for vendor failed");

//         assert!(vendor != "".to_string());
//     }

//     #[test]
//     fn device_method_profile_works() {
//         let device = Device::default();
//         let profile: String = device
//             .profile()
//             .expect("Device method test for profile failed");

//         assert!(profile != "".to_string());
//     }

//     #[test]
//     fn device_method_version_works() {
//         let device = Device::default();
//         let version: String = device
//             .version()
//             .expect("Device method test for version failed");

//         assert!(version != "".to_string());
//     }

//     #[test]
//     fn device_method_extensions_works() {
//         let device = Device::default();
//         let _extensions: Vec<String> = device
//             .extensions()
//             .expect("Device method test for extensions failed");
//     }

//     #[test]
//     fn device_method_platform_works() {
//         let device = Device::default();
//         let _info: Platform = device
//             .platform()
//             .expect("Device method test for platform failed");
//     }

//     #[test]
//     fn device_method_double_fp_config_works() {
//         let device = Device::default();
//         let _info: DeviceFpConfig = device
//             .double_fp_config()
//             .expect("Device method test for double_fp_config failed");
//     }

//     #[test]
//     fn device_method_preferred_vector_width_half_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .preferred_vector_width_half()
//             .expect("Device method test for preferred_vector_width_half failed");
//     }

//     #[test]
//     fn device_method_host_unified_memory_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .host_unified_memory()
//             .expect("Device method test for host_unified_memory failed");
//     }

//     #[test]
//     fn device_method_native_vector_width_char_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .native_vector_width_char()
//             .expect("Device method test for native_vector_width_char failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_native_vector_width_short_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .native_vector_width_short()
//             .expect("Device method test for native_vector_width_short failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_native_vector_width_int_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .native_vector_width_int()
//             .expect("Device method test for native_vector_width_int failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_native_vector_width_long_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .native_vector_width_long()
//             .expect("Device method test for native_vector_width_long failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_native_vector_width_float_works() {
//         let device = Device::default();
//         let info: u32 = device
//             .native_vector_width_float()
//             .expect("Device method test for native_vector_width_float failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_native_vector_width_double_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .native_vector_width_double()
//             .expect("Device method test for native_vector_width_double failed");
//         // NOTE: Needs meaningful test. My MacBook's integrated GPU has this as 0,
//         // but that will not be the case on other devices.
//         // For details see the notes about CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE
//         // at https://www.khronos.org/registry/OpenCL/sdk/1.2/docs/man/xhtml/clGetDeviceInfo.html
//     }

//     #[test]
//     fn device_method_native_vector_width_half_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .native_vector_width_half()
//             .expect("Device method test for native_vector_width_half failed");
//         // NOTE: Needs meaningful test. My MacBook's integrated GPU has this as 0,
//         // but that will not be the case on other devices.
//         // For details see the notes about CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALFq
//         // at https://www.khronos.org/registry/OpenCL/sdk/1.2/docs/man/xhtml/clGetDeviceInfo.html
//     }

//     #[test]
//     fn device_method_opencl_c_version_works() {
//         let device = Device::default();
//         let info: String = device
//             .opencl_c_version()
//             .expect("Device method test for opencl_c_version failed");

//         assert!(info != "".to_string());
//     }

//     #[test]
//     fn device_method_linker_available_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .linker_available()
//             .expect("Device method test for linker_available failed");
//     }

//     #[test]
//     fn device_method_built_in_kernels_works() {
//         let device = Device::default();
//         let _info: Vec<String> = device
//             .built_in_kernels()
//             .expect("Device method test for built_in_kernels failed");
//     }

//     #[test]
//     fn device_method_image_max_buffer_size_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image_max_buffer_size()
//             .expect("Device method test for image_max_buffer_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_image_max_array_size_works() {
//         let device = Device::default();
//         let info: usize = device
//             .image_max_array_size()
//             .expect("Device method test for image_max_array_size failed");

//         assert!(info != 0);
//     }

//     #[test]
//     fn device_method_parent_device_works() {
//         let device = Device::default();
//         match device.parent_device() {
//             Ok(parent_device) => {
//                 unsafe {
//                     assert!(!std::ptr::eq(device.device_ptr(),  parent_device.device_ptr()));
//                 }
//                 let name = device.name().expect("Failed to get device name");
//                 let p_name = parent_device
//                     .name()
//                     .expect("Failed to get parent_device name");
//                 assert!(name != p_name);
//             }
//             Err(Error::DeviceError(DeviceError::NoParentDevice)) => (),
//             Err(other) => panic!(
//                 "Unexpected return error from device.parent_device() {:?}",
//                 other
//             ),
//         }
//     }

//     #[test]
//     fn device_method_partition_max_sub_devices_works() {
//         let device = Device::default();
//         let _info: u32 = device
//             .partition_max_sub_devices()
//             .expect("Device method test for partition_max_sub_devices failed");
//     }

//     #[test]
//     fn device_method_partition_properties_works() {
//         let device = Device::default();
//         let _info: Vec<DevicePartitionProperty> = device
//             .partition_properties()
//             .expect("Device method test for partition_properties failed");
//     }

//     #[test]
//     fn device_method_partition_affinity_domain_works() {
//         let device = Device::default();
//         let _info: DeviceAffinityDomain = device
//             .partition_affinity_domain()
//             .expect("Device method test for partition_affinity_domain failed");
//     }

//     #[test]
//     fn device_method_partition_type_works() {
//         use DevicePartitionProperty as P;
//         let device = Device::default();
//         let infos: Vec<DevicePartitionProperty> = device
//             .partition_type()
//             .expect("Device method test for partition_type failed");
//         for info in infos {
//             assert!(info == P::Equally || info == P::ByCounts || info == P::ByAffinityDomain);
//         }
//     }

//     #[test]
//     fn device_method_reference_count_works() {
//         let device = Device::default();
//         let count: u64 = device
//             .reference_count()
//             .expect("Device method test for reference_count failed");
//         // NOTE: I have no idea why 191 is here...
//         assert_eq!(count, 191);
//     }

//     #[test]
//     fn device_method_preferred_interop_user_sync_works() {
//         let device = Device::default();
//         let _info: bool = device
//             .preferred_interop_user_sync()
//             .expect("Device method test for preferred_interop_user_sync failed");
//     }

//     #[test]
//     fn device_method_printf_buffer_size_works() {
//         let device = Device::default();
//         let info: usize = device
//             .printf_buffer_size()
//             .expect("Device method test for printf_buffer_size failed");

//         assert!(info != 0);
//     }

//     // v2.0+

//     // #[test]
//     // fn device_method_queue_on_host_properties_works() {
//     //     let device = Device::default();
//     //     let info = device.queue_on_host_properties()
//     //         .expect("Device method test for queue_on_host_properties failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_image_pitch_alignment_works() {
//     //     let device = Device::default();
//     //     let info = device.image_pitch_alignment()
//     //         .expect("Device method test for image_pitch_alignment failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_image_base_address_alignment_works() {
//     //     let device = Device::default();
//     //     let info = device.image_base_address_alignment()
//     //         .expect("Device method test for image_base_address_alignment failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_max_read_write_image_args_works() {
//     //     let device = Device::default();
//     //     let info = device.max_read_write_image_args()
//     //         .expect("Device method test for max_read_write_image_args failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_max_global_variable_size_works() {
//     //     let device = Device::default();
//     //     let info = device.max_global_variable_size()
//     //         .expect("Device method test for max_global_variable_size failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_queue_on_device_properties_works() {
//     //     let device = Device::default();
//     //     let info = device.queue_on_device_properties()
//     //         .expect("Device method test for queue_on_device_properties failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_queue_on_device_preferred_size_works() {
//     //     let device = Device::default();
//     //     let info = device.queue_on_device_preferred_size()
//     //         .expect("Device method test for queue_on_device_preferred_size failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_queue_on_device_max_size_works() {
//     //     let device = Device::default();
//     //     let info = device.queue_on_device_max_size()
//     //         .expect("Device method test for queue_on_device_max_size failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_max_on_device_queues_works() {
//     //     let device = Device::default();
//     //     let info = device.max_on_device_queues()
//     //         .expect("Device method test for max_on_device_queues failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_max_on_device_events_works() {
//     //     let device = Device::default();
//     //     let info = device.max_on_device_events()
//     //         .expect("Device method test for max_on_device_events failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_svm_capabilities_works() {
//     //     let device = Device::default();
//     //     let info = device.svm_capabilities()
//     //         .expect("Device method test for svm_capabilities failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_global_variable_preferred_total_size_works() {
//     //     let device = Device::default();
//     //     let info = device.global_variable_preferred_total_size()
//     //         .expect("Device method test for global_variable_preferred_total_size failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_max_pipe_args_works() {
//     //     let device = Device::default();
//     //     let info = device.max_pipe_args()
//     //         .expect("Device method test for max_pipe_args failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_pipe_max_active_reservations_works() {
//     //     let device = Device::default();
//     //     let info = device.pipe_max_active_reservations()
//     //         .expect("Device method test for pipe_max_active_reservations failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_pipe_max_packet_size_works() {
//     //     let device = Device::default();
//     //     let info = device.pipe_max_packet_size()
//     //         .expect("Device method test for pipe_max_packet_size failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_preferred_platform_atomic_alignment_works() {
//     //     let device = Device::default();
//     //     let info = device.preferred_platform_atomic_alignment()
//     //         .expect("Device method test for preferred_platform_atomic_alignment failed");

//     //     assert!(info != "".to_string());
//     // }

//     // #[test]
//     // fn device_method_preferred_global_atomic_alignment_works() {
//     //     let device = Device::default();
//     //     let info = device.preferred_global_atomic_alignment()
//     //         .expect("Device method test for preferred_global_atomic_alignment failed");

//     //     assert!(info != "".to_string());
//     // }

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
// }
