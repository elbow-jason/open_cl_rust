/* cl_channel_order */

use crate::open_cl::ffi::{cl_channel_order, cl_channel_type};

crate::__codes_enum!(ChannelOrder, cl_channel_order, {
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

/* cl_channel_type */
crate::__codes_enum!(ChannelType, cl_channel_type, {
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
