use super::ffi_types::*;

pub trait AsPtr {
    fn as_ptr(&self) -> *const Self;
    fn as_mut_ptr(&mut self) -> *mut Self;
}

macro_rules! as_ptr {
    ($($t:ty),*) => {
        $(
            impl AsPtr for $t {
                fn as_ptr(&self) -> *const $t {
                    self as *const $t
                }

                fn as_mut_ptr(&mut self) -> *mut $t {
                    self as *mut $t
                }
            }
        )*
    }
}

as_ptr!(cl_double);
as_ptr!(cl_char, cl_char16, cl_char2, cl_char4, cl_char8);
as_ptr!(cl_float, cl_float16, cl_float2, cl_float4, cl_float8);
as_ptr!(cl_int, cl_int16, cl_int2, cl_int4, cl_int8);
as_ptr!(cl_long, cl_long16, cl_long2, cl_long4, cl_long8);
as_ptr!(cl_short, cl_short16, cl_short2, cl_short4, cl_short8);
as_ptr!(cl_uchar, cl_uchar16, cl_uchar2, cl_uchar4, cl_uchar8);
as_ptr!(cl_uint, cl_uint16, cl_uint2,  cl_uint4, cl_uint8);
as_ptr!(cl_ulong, cl_ulong16, cl_ulong2,  cl_ulong4, cl_ulong8);
as_ptr!(cl_ushort, cl_ushort16, cl_ushort2, cl_ushort4, cl_ushort8);