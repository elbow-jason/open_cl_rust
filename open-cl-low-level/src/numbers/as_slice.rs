use super::cl_number::*;
use libc::size_t;

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

macro_rules! self_as_slice {
    ($($t:ty),*) => {
        $(
            impl AsSlice<$t> for $t {
                fn as_slice(&self) -> &[$t] {
                    unsafe { std::slice::from_raw_parts(self, 1) }
                }

                fn as_mut_slice(&mut self) -> &mut [$t] {
                    unsafe { std::slice::from_raw_parts_mut(self, 1) }
                }
            }
        )*
    }
}

self_as_slice!(
    cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float,
    cl_double, size_t
);

macro_rules! vector_as_slice {
    ($t:ty, $( $vector_t:ty ),*) => {
        $(
            impl AsSlice<$t> for $vector_t {
                fn as_slice(&self) -> &[$t] {
                    unsafe { &self.s[..] }
                }

                fn as_mut_slice(&mut self) -> &mut [$t] {
                    unsafe { &mut self.s[..] }
                }
            }
        )*
    }
}

vector_as_slice!(cl_uchar, cl_uchar2, cl_uchar4, cl_uchar8, cl_uchar16);
vector_as_slice!(cl_char, cl_char2, cl_char4, cl_char8, cl_char16);
vector_as_slice!(cl_ushort, cl_ushort2, cl_ushort4, cl_ushort8, cl_ushort16);
vector_as_slice!(cl_short, cl_short2, cl_short4, cl_short8, cl_short16);
vector_as_slice!(cl_uint, cl_uint2, cl_uint4, cl_uint8, cl_uint16);
vector_as_slice!(cl_int, cl_int2, cl_int4, cl_int8, cl_int16);
vector_as_slice!(cl_ulong, cl_ulong2, cl_ulong4, cl_ulong8, cl_ulong16);
vector_as_slice!(cl_long, cl_long2, cl_long4, cl_long8, cl_long16);
vector_as_slice!(cl_float, cl_float2, cl_float4, cl_float8, cl_float16);
