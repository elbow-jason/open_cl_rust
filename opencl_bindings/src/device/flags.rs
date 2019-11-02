use crate::ffi::{
    cl_device_exec_capabilities,
    cl_device_fp_config,
    cl_device_local_mem_type,
    cl_device_mem_cache_type,
    cl_device_type,
    // v2.0+ ?
    // cl_device_partition_property,
    // cl_device_affinity_domain,
    // cl_device_svm,
};

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
