/// Entrypoint for FFI types and functions.
///
/// cl_* object pointers are defined in cl_objects module and are not imported from cl_sys.
/*


*/
// FFI data types and info flags
pub use cl_sys::{
    cl_addressing_mode, cl_buffer_create_type, cl_build_status, cl_channel_order, cl_channel_type,
    cl_command_queue_info, cl_command_queue_properties, cl_command_type, cl_context_info,
    cl_context_properties, cl_device_affinity_domain, cl_device_exec_capabilities, cl_device_info,
    cl_device_local_mem_type, cl_device_mem_cache_type, cl_device_partition_property,
    cl_device_type, cl_event_info, cl_filter_mode, cl_half, cl_image_info, cl_int,
    cl_kernel_arg_access_qualifier, cl_kernel_arg_address_qualifier, cl_kernel_arg_info,
    cl_kernel_arg_type_qualifier, cl_kernel_info, cl_kernel_work_group_info, cl_map_flags,
    cl_mem_flags, cl_mem_info, cl_mem_migration_flags, cl_mem_object_type, cl_platform_info,
    cl_profiling_info, cl_program_binary_type, cl_program_build_info, cl_program_info,
    cl_sampler_info, cl_uint,
};

#[allow(non_camel_case_types)]
pub type cl_command_execution_status = cl_int;

#[allow(non_camel_case_types)]
pub type cl_bool = cl_uint;

// FFI functions
pub use cl_sys::{
    clBuildProgram, clCreateBuffer, clCreateCommandQueue, clCreateContext, clCreateKernel,
    clCreateProgramWithBinary, clCreateProgramWithSource, clEnqueueNDRangeKernel,
    clEnqueueReadBuffer, clEnqueueWriteBuffer, clFinish, clGetCommandQueueInfo, clGetContextInfo,
    clGetDeviceIDs, clGetDeviceInfo, clGetEventInfo, clGetEventProfilingInfo, clGetKernelInfo,
    clGetMemObjectInfo, clGetPlatformIDs, clGetPlatformInfo, clGetProgramBuildInfo,
    clGetProgramInfo, clReleaseCommandQueue, clReleaseContext, clReleaseDevice, clReleaseEvent,
    clReleaseKernel, clReleaseMemObject, clReleaseProgram, clRetainCommandQueue, clRetainContext,
    clRetainDevice, clRetainEvent, clRetainKernel, clRetainMemObject, clRetainProgram,
    clSetKernelArg, clWaitForEvents,
};
