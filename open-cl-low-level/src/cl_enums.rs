use crate::ffi::*;

#[doc(hidden)]
macro_rules! __cl_enum {
    ($enum_name:ident, $cl_type:ident, $body:tt) => {
        __enum_define!($enum_name, $body);
        __enum_two_way_from!($enum_name, $cl_type, $body);
        __test_enum_converter!($enum_name, $cl_type, $body);
    };
}

#[doc(hidden)]
macro_rules! __enum_two_way_from {
    ($source_type:ident, $dest_type:ident, { $($source_value:ident => $dest_value:expr),* }) => {
        impl From<$source_type> for $dest_type {
            fn from(source_value: $source_type) -> $dest_type {
                (source_value as $dest_type)
            }
        }

        impl From<&$source_type> for $dest_type {
            fn from(source_value: &$source_type) -> $dest_type {
                $source_type::from(*source_value) as $dest_type
            }
        }

        impl From<$dest_type> for $source_type {
            fn from(dest_value: $dest_type) -> $source_type {
                // when this low level API is a little more mature,
                // we can add a config flag to remove this check and simply
                // mem::transmute. Better off checking for now.
                // TODO: Investigate if's vs HashMap vs other KV performance.
                $(
                    if dest_value == $dest_value as $dest_type {
                        return $source_type::$source_value
                    }
                )*

                // Note: replace this with a TryFrom some day....
                panic!(
                    "From failed for {:?} to {:?} for value {:?}",
                    stringify!($right_type),
                    stringify!($source_type),
                    dest_value
                );
            }
        }
    };

    ($source_type:ident, $dest_type:ident, $source_value:expr, $dest_value:expr) => {
        two_way_from!($source_type, $dest_type, $source_value, $dest_value)
    };
}

#[doc(hidden)]
macro_rules! __enum_define {
    ($name:ident, { $($field:ident => $value:expr),* }) => {
        #[allow(non_camel_case_types)]

        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
        pub enum $name {
            $(
                $field = $value as isize,
            )*
        }
    };
}

#[doc(hidden)]
macro_rules! __test_enum_converter {
    ($enum_type:ident, $other_type:ty ,{ $($enum_value:expr => $other_value:expr),* }) => {
        paste::item! {
            $(
                #[allow(non_snake_case)]
                #[test]
                fn [<type_ $enum_type _ $enum_value _converts_to_and_from_ $other_type>]() {
                    assert_eq!($enum_type::from($other_value), $enum_type::$enum_value);
                    assert_eq!($other_type::from($enum_type::$enum_value), $other_value);
                }
            )*
        }
    };
}

__cl_enum!(AddressingMode, cl_addressing_mode, {
    NoneType => 0x1130,
    ClampToEdge => 0x1131,
    Clamp => 0x1132,
    Repeat => 0x1133,
    MirroredRepeat => 0x1134
});

__cl_enum!(ChannelOrder, cl_channel_order, {
    R => 0x10B0,
    A => 0x10B1,
    RG => 0x10B2,
    RA => 0x10B3,
    RGB => 0x10B4,
    RGBA => 0x10B5,
    BGRA => 0x10B6,
    ARGB => 0x10B7,
    Intensity => 0x10B8,
    Luminance => 0x10B9,
    Rx => 0x10BA,
    RGx => 0x10BB,
    RGBx => 0x10BC,
    Depth => 0x10BD,
    DepthStencil => 0x10BE,
    sRGB => 0x10BF,
    sRGBx => 0x10C0,
    sRGBA => 0x10C1,
    sBGRA => 0x10C2,
    ABGR => 0x10C3
});

__cl_enum!(ChannelType, cl_channel_type, {
    SnormInt8 => 0x10D0,
    SnormInt16 => 0x10D1,
    UnormInt8 => 0x10D2,
    UnormInt16 => 0x10D3,
    UnormShort_565 => 0x10D4,
    UnormShort_555 => 0x10D5,
    UnormInt_101010 => 0x10D6,
    SignedInt8 => 0x10D7,
    SignedInt16 => 0x10D8,
    SignedInt32 => 0x10D9,
    UnsignedInt8 => 0x10DA,
    UnsignedInt16 => 0x10DB,
    UnsignedInt32 => 0x10DC,
    HalfFloat => 0x10DD,
    Float => 0x10DE,
    UnormInt24 => 0x10DF,
    UnormInt_101010_2 => 0x10E0
});

__cl_enum!(FilterMode, cl_filter_mode, {
    Nearest => 0x1140,
    Linear => 0x1141
});

__cl_enum!(ImageInfo, cl_image_info, {
    Format => 0x1110,
    ElementSize => 0x1111,
    RowPitch => 0x1112,
    SlicePitch => 0x1113,
    Width => 0x1114,
    Height => 0x1115,
    Depth => 0x1116,
    ArraySize => 0x1117,
    Buffer => 0x1118,
    NumMipLevels => 0x1119,
    NumSamples => 0x111A
});

// NOTE: Version for cl_pipe_info?
// crate::__codes_enum!(PipeInfo, cl_pipe_info, {
//     PacketSize => 0x1120,
//     MaxPackets => 0x1121
// });

// /* cl_sampler_info */
// /* These enumerants are for the cl_khr_mipmap_image extension.
//    They have since been added to cl_ext.h with an appropriate
//    KHR suffix, but are left here for backwards compatibility. */
__cl_enum!(SamplerInfo, cl_sampler_info, {
    ReferenceCount => 0x1150,
    Context => 0x1151,
    NormalizedCoords => 0x1152,
    AddressingMode => 0x1153,
    FilterMode => 0x1154,
    MipFilterMode => 0x1155,
    LodMin => 0x1156,
    LodMax => 0x1157
});

__cl_enum!(MapFlags, cl_map_flags, {
    Read => 1,
    Write => 2,
    WriteInvalidateRegion => 4
});

// All return these flags return chars except perhaps HostTimerResolution.
__cl_enum!(PlatformInfo, cl_platform_info, {
    Profile => 0x0900,
    Version => 0x0901,
    Name => 0x0902,
    Vendor => 0x0903,
    Extensions => 0x0904
    // v2.1
    // HostTimerResolution => 0x0905
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
__cl_enum!(DeviceInfo, cl_device_info, {
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

__cl_enum!(DeviceLocalMemType,  cl_device_local_mem_type, {
    Local => 0x1,
    Global => 0x2
});

__cl_enum!(DeviceMemCacheType, cl_device_mem_cache_type, {
    NoneType => 0x0,
    ReadOnlyCache => 0x1,
    ReadWriteCache => 0x2
});

__cl_enum!(DevicePartitionProperty, cl_device_partition_property, {
    Equally => 0x1086,
    ByCounts => 0x1087,
    // This is not a valid variant. The 0 is used as a signal that the
    // list of cl_device_partition_propertys is at an end.
    // ByCountsListEnd => 0x0,
    ByAffinityDomain => 0x1088
});

__cl_enum!(ContextInfo, cl_context_info, {
    ReferenceCount => 0x1080,
    Devices => 0x1081,
    Properties => 0x1082,
    NumDevices => 0x1083
});

__cl_enum!(ContextProperties, cl_context_properties, {
    Platform => 0x1084,
    InteropUserSync => 0x1085
});

__cl_enum!(ProgramBuildInfo, cl_program_build_info, {
    Status => 0x1181,
    Options => 0x1182,
    Log => 0x1183,
    // NOTE: Version for BinaryType?
    // BinaryType => 0x1184,
    GlobalVariableTotalSize => 0x1185
});

__cl_enum!(ProgramInfo, cl_program_info, {
    ReferenceCount => 0x1160,
    Context => 0x1161,
    NumDevices => 0x1162,
    Devices => 0x1163,
    Source => 0x1164,
    BinarySizes => 0x1165,
    Binaries => 0x1166,
    NumKernels => 0x1167,
    KernelNames => 0x1168
    // v2.0+
    // Il => 0x1169,
    // ScopeGlobalCtorsPresent => 0x116A,
    // ScopeGlobalDtorsPresent => 0x116B
});

/* cl_build_status */
__cl_enum!(BuildStatus, cl_build_status, {
    Success => 0,
    NoneType => -1,
    Error => -2,
    InProgress => -3
});

// NOTE: Version for cl_program_binary_type?
__cl_enum!(ProgramBinaryType, cl_program_binary_type, {
    NoneType => 0x0,
    CompiledObject => 0x1,
    Library => 0x2,
    Executable => 0x4
});

__cl_enum!(MemMigrationFlags, cl_mem_migration_flags, {
    Host => 1,
    ContentUndefined => (1 << 1)
});

__cl_enum!(MemObjectType, cl_mem_object_type, {
    Buffer => 0x10F0,
    Image2D => 0x10F1,
    Image3D => 0x10F2,
    Image2DArray => 0x10F3,
    Image1D => 0x10F4,
    Image1DArray => 0x10F5,
    Image1DBuffer => 0x10F6,
    Pipe => 0x10F7
});

__cl_enum!(MemInfo, cl_mem_info, {
    Type => 0x1100,
    Flags => 0x1101,
    Size => 0x1102,
    HostPtr => 0x1103,
    MapCount => 0x1104,
    ReferenceCount => 0x1105,
    Context => 0x1106,
    AssociatedMemobject => 0x1107,
    Offset => 0x1108

    // v2.0
    // UsesSvmPointer => 0x1109
});

__cl_enum!(BufferCreateType, cl_buffer_create_type, {
    /* cl_buffer_create_type */
    CreateTypeRegion => 0x1220
});

__cl_enum!(KernelInfo, cl_kernel_info, {
    FunctionName => 0x1190,
    NumArgs => 0x1191,
    ReferenceCount => 0x1192,
    Context => 0x1193,
    Program => 0x1194,
    Attributes => 0x1195
    // OpenCL v2.0
    // MaxNumSubGroups => 0x11B9,
    // CompileNumSubGroups => 0x11BA
});

__cl_enum!(KernelWorkGroupInfo, cl_kernel_work_group_info, {
    WorkGroupSize => 0x11B0,
    CompileWorkGroupSize => 0x11B1,
    LocalMemSize => 0x11B2,
    PreferredWorkGroupSizeMultiple => 0x11B3,
    PrivateMemSize => 0x11B4,
    GlobalWorkSize => 0x11B5
});

__cl_enum!(KernelArgAccessQualifier, cl_kernel_arg_access_qualifier, {
    ReadOnly => 0x11A0,
    WriteOnly => 0x11A1,
    ReadWrite => 0x11A2,
    NoneType => 0x11A3
});

__cl_enum!(KernelArgTypeQualifier, cl_kernel_arg_type_qualifier, {
    NoneType => 0,
    Const => 1,
    Restrict => 2,
    Volatile => 4,
    Pipe => 8
});

__cl_enum!(KernelArgAddressQualifier, cl_kernel_arg_address_qualifier, {
    Global => 0x119B,
    Local => 0x119C,
    Constant => 0x119D,
    Private => 0x119E
});

__cl_enum!(KernelArgInfo, cl_kernel_arg_info, {
    AddressQualifier => 0x1196,
    AccessQualifier => 0x1197,
    TypeName => 0x1198,
    TypeQualifier => 0x1199,
    Name => 0x119A
});

__cl_enum!(CommandQueueInfo, cl_command_queue_info, {
    Context => 0x1090,
    Device => 0x1091,
    ReferenceCount => 0x1092,
    Properties => 0x1093

    // v2.0
    // Size => 0x1094,

    // v2.1
    // DeviceDefault => 0x1095
});

__cl_enum!(CommandExecutionStatus, cl_int, {
    Complete => 0x0,
    Running => 0x1,
    Submitted => 0x2,
    Queued => 0x3
});

// The cl_command_type is the return type of clGetEventInfo.
__cl_enum!(CommandType, cl_command_type, {
    NdrangeKernel => 0x11F0,
    Task => 0x11F1,
    NativeKernel => 0x11F2,
    ReadBuffer => 0x11F3,
    WriteBuffer => 0x11F4,
    CopyBuffer => 0x11F5,
    ReadImage => 0x11F6,
    WriteImage => 0x11F7,
    CopyImage => 0x11F8,
    CopyImageToBuffer => 0x11F9,
    CopyBufferToImage => 0x11FA,
    MapBuffer => 0x11FB,
    MapImage => 0x11FC,
    UnmapMemObject => 0x11FD,
    Marker => 0x11FE,
    AcquireGlObjects => 0x11FF,
    ReleaseGlObjects => 0x1200,
    ReadBufferRect => 0x1201,
    WriteBufferRect => 0x1202,
    CopyBufferRect => 0x1203,
    User => 0x1204,
    Barrier => 0x1205,
    MigrateMemObjects => 0x1206,
    FillBuffer => 0x1207,
    FillImage => 0x1208,
    SvmFree => 0x1209,
    SvmMemcpy => 0x120A,
    SvmMemfill => 0x120B,
    SvmMap => 0x120C,
    SvmUnmap => 0x120D
});

__cl_enum!(EventInfo, cl_event_info, {
    CommandQueue => 0x11D0,
    CommandType => 0x11D1,
    ReferenceCount => 0x11D2,
    CommandExecutionStatus => 0x11D3,
    Context => 0x11D4
});

__cl_enum!(ProfilingInfo, cl_profiling_info, {
    Queued => 0x1280,
    Submit => 0x1281,
    Start => 0x1282,
    End => 0x1283,
    Complete => 0x1284
});

// NOTE: Add support for d3d and khr flags
// https://www.khronos.org/registry/OpenCL/sdk/1.2/docs/man/xhtml/clCreateContext.html
// CL_CONTEXT_D3D10_DEVICE_KHR 	    ID3D10Device* 	                    default: NULL
// CL_GL_CONTEXT_KHR 	            0, OpenGL context handle 	        (available if the cl_khr_gl_sharing extension is enabled)
// CL_EGL_DISPLAY_KHR 	            EGL_NO_DISPLAY, EGLDisplay handle 	(available if the cl_khr_gl_sharing extension is enabled)
// CL_GLX_DISPLAY_KHR 	            None, X handle 	                    (available if the cl_khr_gl_sharing extension is enabled)
// CL_CGL_SHAREGROUP_KHR 	        0, CGL share group handle 	        (available if the cl_khr_gl_sharing extension is enabled)
// CL_WGL_HDC_KHR 	                0, HDC handle 	                    (available if the cl_khr_gl_sharing extension is enabled)
// CL_CONTEXT_ADAPTER_D3D9_KHR      IDirect3DDevice9 *                  (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_ADAPTER_D3D9EX_KHR    IDirect3DDeviceEx*                  (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_ADAPTER_DXVA_KHR      IDXVAHD_Device *                    (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_D3D11_DEVICE_KHR      ID3D11Device *                      default: NULL

// NOTE: Version for cl_device_svm? 2.0?
// __cl_enum!(DeviceSvm, cl_device_svm, {
//     CoarseGrainBuffer => 1,
//     FineGrainBuffer => 2,
//     FineGrainSystem => 4,
//     Atomics => 8
// });
