

use super::{Zeroed, NumberTypedT};

// u8
pub use crate::ffi::{cl_uchar, cl_uchar16, cl_uchar2, cl_uchar3, cl_uchar4, cl_uchar8};

// i8
pub use crate::ffi::{cl_char, cl_char16, cl_char2, cl_char3, cl_char4, cl_char8};

// u16
pub use crate::ffi::{cl_ushort, cl_ushort16, cl_ushort2, cl_ushort3, cl_ushort4, cl_ushort8};

// i16
pub use crate::ffi::{cl_short, cl_short16, cl_short2, cl_short3, cl_short4, cl_short8};

// u32
pub use crate::ffi::{cl_uint, cl_uint16, cl_uint2, cl_uint3, cl_uint4, cl_uint8};

// i32
pub use crate::ffi::{cl_int, cl_int16, cl_int2, cl_int3, cl_int4, cl_int8};

// u64
pub use crate::ffi::{cl_ulong, cl_ulong16, cl_ulong2, cl_ulong3, cl_ulong4, cl_ulong8};

// i64
pub use crate::ffi::{cl_long, cl_long16, cl_long2, cl_long3, cl_long4, cl_long8};

// f32
pub use crate::ffi::{cl_float, cl_float16, cl_float2, cl_float3, cl_float4, cl_float8};

// primitive types
pub use crate::ffi::{
    cl_bool, cl_double, cl_half
};

// NOTE: f32 does not implement Eq so it's not here. WHYEEEEE...
pub trait FFINumber:
    Sized + Clone + Copy + Send + Sync + 'static + Zeroed + NumberTypedT
{
}

macro_rules! ffi_number {
    ($( $t:ty ),*) => {
        $( 
            impl FFINumber for $t {}
        )*
    }
}

ffi_number!(cl_char, cl_char16, cl_char2, cl_char4, cl_char8);
ffi_number!(cl_float, cl_float16, cl_float2, cl_float4, cl_float8);
ffi_number!(cl_int, cl_int16, cl_int2, cl_int4, cl_int8);
ffi_number!(cl_long, cl_long16, cl_long2, cl_long4, cl_long8);
ffi_number!(cl_short, cl_short16, cl_short2, cl_short4, cl_short8);
ffi_number!(cl_uchar, cl_uchar16, cl_uchar2, cl_uchar4, cl_uchar8);
ffi_number!(cl_uint, cl_uint16, cl_uint2,  cl_uint4, cl_uint8);
ffi_number!(cl_ulong, cl_ulong16, cl_ulong2,  cl_ulong4, cl_ulong8);
ffi_number!(cl_ushort, cl_ushort16, cl_ushort2, cl_ushort4, cl_ushort8);



