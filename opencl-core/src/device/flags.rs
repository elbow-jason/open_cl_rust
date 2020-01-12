use crate::ffi::{
    cl_device_info,
    cl_device_affinity_domain,
    cl_device_exec_capabilities,
    cl_device_fp_config,
    cl_device_local_mem_type,
    cl_device_mem_cache_type,
    cl_device_partition_property,
    cl_device_type,
    
    // v2.0+ ?
    // cl_device_svm,
};


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
    // This is not a valid variant. The 0 is used as a signal that the
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



// NOTE: Version for cl_device_svm? 2.0?
// crate::__codes_enum!(DeviceSvm, cl_device_svm, {
//     CoarseGrainBuffer => 1,
//     FineGrainBuffer => 2,
//     FineGrainSystem => 4,
//     Atomics => 8
// });
