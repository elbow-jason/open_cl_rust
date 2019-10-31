use crate::ffi::{
    cl_image_info,
    cl_sampler_info,
    // cl_pipe_info,
};

/* cl_image_info */
crate::__codes_enum!(ImageInfo, cl_image_info, {
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

// crate::__codes_enum!(PipeInfo, cl_pipe_info, {
//     PacketSize => 0x1120,
//     MaxPackets => 0x1121
// });

// /* cl_sampler_info */
// /* These enumerants are for the cl_khr_mipmap_image extension.
//    They have since been added to cl_ext.h with an appropriate
//    KHR suffix, but are left here for backwards compatibility. */
crate::__codes_enum!(SamplerInfo, cl_sampler_info, {
    ReferenceCount => 0x1150,
    Context => 0x1151,
    NormalizedCoords => 0x1152,
    AddressingMode => 0x1153,
    FilterMode => 0x1154,
    MipFilterMode => 0x1155,
    LodMin => 0x1156,
    LodMax => 0x1157
});
