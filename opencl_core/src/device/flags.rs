use crate::ffi::{
    cl_device_exec_capabilities,
    cl_device_fp_config,
    cl_device_local_mem_type,
    cl_device_mem_cache_type,
    cl_device_type,
    cl_device_partition_property,
    cl_device_affinity_domain,
    // v2.0+ ?
    // cl_device_svm,
};




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

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L389-L401
bitflags! {
    pub struct DeviceFpConfig: cl_device_fp_config {
        const DENORM = 1;
        const INF_NAN = 2;
        const ROUND_TO_NEAREST = 4;
        const ROUND_TO_ZERO = 8;
        const ROUND_TO_INF = 16;
        const FMA = 32;
        const SOFT_FLOAT = 64;
        const CORRECTLY_ROUNDED_DIVIDE_SQRT = 128;
    }
}

crate::__codes_enum!(DevicePartitionProperty, cl_device_partition_property, {
    Equally => 0x1086,
    ByCounts => 0x1087,
    // This is not a valid property. The 0 is used as a signal that the
    // list of cl_device_partition_propertys is at an end.
    // ByCountsListEnd => 0x0,
    ByAffinityDomain => 0x1088
});

bitflags! {
    pub struct DeviceAffinityDomain: cl_device_affinity_domain {
        const NONE_SUPPORTED = 0;
        const NUMA = 1;
        const L4_CACHE = 2;
        const L3_CACHE = 4;
        const L2_CACHE = 8;
        const L1_CACHE = 16;
        const NEXT_PARTITIONABLE = 32;
    }
}

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L412-L414
// /* cl_device_exec_capabilities - bitfield */
bitflags! {
    pub struct DeviceExecCapabilities: cl_device_exec_capabilities {
        const KERNEL = 1;
        const NATIVE_KERNEL = 2;
    }
}


// NOTE: Version for cl_device_svm?
// crate::__codes_enum!(DeviceSvm, cl_device_svm, {
//     CoarseGrainBuffer => 1,
//     FineGrainBuffer => 2,
//     FineGrainSystem => 4,
//     Atomics => 8
// });