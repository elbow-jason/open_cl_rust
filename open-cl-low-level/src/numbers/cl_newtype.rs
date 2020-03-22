// use std::ops::Deref;
// use libc::size_t;

// use super::ffi_number::*;
// use super::{NumberTypedT, Zeroed};
// use crate::{ClType, ToFFIType, FFINumber, RustNumber, Number};
// use crate::{ToRustType, ToFFIType};

// macro_rules! define_newtype_and_vectors {
//     (cl_float, $new_t:ident, $rust_t:ty) => {
//         paste::item! {
//             #[derive(Debug, Clone, Copy)]
//             pub struct $new_t(pub cl_float);
//             /// Vector containing 2 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t 2>](pub [<cl_float 2>]);

//             /// Vector containing 3 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t 3>](pub [<cl_float 3>]);
//             /// Vector containing 4 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t 4>](pub [<cl_float 4>]);

//             /// Vector containing 8 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t 8>](pub [<cl_float 8>]);

//             /// Vector containing 16 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t 16>](pub [<cl_float 16>]);
//         }

//     };

//     ($t:ident, $new_t:ident, $rust_t:ty) => {
//         paste::item! {
//             #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//             pub struct $new_t(pub $t);

//             /// Vector containing 2 $rust_t
//             define_vector_newtype!($t, $new_t, $rust_t, 2);
//             define_vector_newtype!($t, $new_t, $rust_t, 3);
//             define_vector_newtype!($t, $new_t, $rust_t, 4);
//             define_vector_newtype!($t, $new_t, $rust_t, 8);
//             define_vector_newtype!($t, $new_t, $rust_t, 16);
//         }
//     };
// }

// macro_rules! impl_to_rust_type {
//     ($t:ident, $new_t:ident, $rust_t:ty, 3) => {
//         paste::item! {
//             impl ToRustType<[$rust_t; 3]> for [<$new_t 3>] {
//                 fn to_rust_type(self) -> [$rust_t; 3] {
//                     let inner = self.0.s;
//                     [inner[0], inner[1], inner[2]]
//                 }
//             }
//         }
//     };

//     ($t:ident, $new_t:ident, $rust_t:ty, $count:expr) => {
//         paste::item! {
//             impl ToRustType<[$rust_t; $count]> for [<$new_t $count>] {
//                 fn to_rust_type(self) -> [$rust_t; $count] {
//                     self.0.s
//                 }
//             }
//         }
//     };
// }

// macro_rules! define_vector_newtype {
//     ($t:ident, $new_t:ident, $rust_t:ty, $count:expr) => {
//         paste::item! {
//             /// Vector containing 2 $rust_t
//             #[derive(Clone, Copy)]
//             pub struct [<$new_t $count>](pub [<$t $count>]);

//             impl_to_rust_type!($t, $new_t, $rust_t, $count);

//             impl ToFFIType<[<$t $count>]> for [<$new_t $count>] {
//                 fn to_ffi_type(self) -> [<$t $count>] {
//                     self.0
//                 }
//             }
//             impl std::fmt::Debug for [<$new_t $count>] {
//                 fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                     let rusty_val: [$rust_t; $count] = self.to_rust_type();
//                     write!(f, "{}({:?})", stringify!([<$new_t $count>]), rusty_val)
//                 }
//             }
//         }
//     }
// }

// define_newtype_and_vectors!(cl_char, ClChar, i8);
// define_newtype_and_vectors!(cl_uchar, ClUchar, u8);
// define_newtype_and_vectors!(cl_short, ClShort, i16);
// define_newtype_and_vectors!(cl_ushort, ClUshort, u16);
// define_newtype_and_vectors!(cl_int, ClInt, i32);
// define_newtype_and_vectors!(cl_uint, ClUint, u32);
// define_newtype_and_vectors!(cl_long, ClLong, i64);
// define_newtype_and_vectors!(cl_ulong, ClUlong, u64);
// define_newtype_and_vectors!(cl_float, ClFloat, f32);d

// /// A Boolean condition: true (1) or false (0)
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ClBool {
//     True,
//     False,
// }
// /// Signed two’s complement 8-bit integer
// /// 64-bit floating-point value.
// /// May not be available on some platforms.
// /// Check availability with FFI call to `clGetDeviceInfo` or `device.extensions()`
// /// Enable in kernels with `#pragma OPENCL EXTENSION cl_khr_fp64 : enable`
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct ClDouble(pub cl_double);

// /// Unsigned integer produced by the size of operator
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct SizeT(pub size_t);

// /// 16-bit floating-point value, IEEE-754-2008 conformant
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct ClHalf(pub cl_half);

// macro_rules! impl_deref {
//     ($new_t:ty => $t:ty) => {
//         impl Deref for $new_t {
//             type Target = $t;

//             fn deref(&self) -> &$t {
//                 &self.0
//             }
//         }
//     }
// }

// macro_rules! impl_deref_for_vectors {
//     ($new_t:ident => $t:ident) => {
//         paste::item! {
//             impl_deref!($new_t => $t);
//             impl_deref!([<$new_t 2>] => [<$t 2>]);
//             impl_deref!([<$new_t 3>] => [<$t 3>]);
//             impl_deref!([<$new_t 4>] => [<$t 4>]);
//             impl_deref!([<$new_t 8>] => [<$t 8>]);
//             impl_deref!([<$new_t 16>] => [<$t 16>]);
//         }
//     }
// }

// impl_deref!(SizeT => size_t);
// impl_deref!(ClDouble => cl_double);
// impl_deref!(ClHalf => cl_half);

// const CL_BOOL_FALSE: cl_bool = 0;
// const CL_BOOL_TRUE: cl_bool = 1;

// impl Deref for ClBool {
//     type Target = cl_bool;

//     fn deref(&self) -> &cl_bool {
//         match self {
//             ClBool::True => &CL_BOOL_TRUE,
//             ClBool::False => &CL_BOOL_FALSE,
//         }
//     }
// }

// impl_deref_for_vectors!(ClChar => cl_char);
// impl_deref_for_vectors!(ClUchar => cl_uchar);
// impl_deref_for_vectors!(ClShort => cl_short);
// impl_deref_for_vectors!(ClUshort => cl_ushort);
// impl_deref_for_vectors!(ClInt => cl_int);
// impl_deref_for_vectors!(ClUint => cl_uint);
// impl_deref_for_vectors!(ClLong => cl_long);
// impl_deref_for_vectors!(ClUlong => cl_ulong);
// impl_deref_for_vectors!(ClFloat => cl_float);

// // NOTE: f32 does not implement Eq so it's not here. WHYEEEEE...
// pub trait ClNewtype: Number {
//     type FFINum: FFINumber;
//     type RustNum: RustNumber;
//     fn to_ffi_number(self) -> Self::FFINum;
//     fn to_rust_number(self) -> Self::RustNum;
// }

// // impl<T> ClType for T where T: ClNumber {}

// // impl<T, U> ToFFIType<U> for T where T: ClNumber<Inner=U>, U: FFINumber {
// //     fn to_ffi_type(self) -> T::Inner {
// //         self.inner()
// //     }
// // }

// macro_rules! impl_cl_number {
//     ( $( $new_t:ident => $ffi_t:ident ),* ) => {
//         $(
//             impl ClNumber for $new_t {
//                 type FFINum = $ffi_t;
//                 fn to_ffi_number(&self) -> Self::FFINum {
//                     self.0
//                 }
//             }
//         )*
//     };
// }

// impl ClNumber for ClBool {
//     type FFINum = cl_bool;
//     fn to_ffi_number(self) -> Self::FFINum {
//         match self {
//             ClBool::True => 1,
//             ClBool::False => 0,
//         }
//     }
// }

// impl_cl_number!(
//     SizeT => size_t,
//     ClHalf => cl_half,
//     ClDouble => cl_double,
//     ClFloat => cl_float,
//     ClChar => cl_char,
//     ClUchar => cl_uchar,
//     ClInt => cl_int,
//     ClUint => cl_uint,
//     ClLong => cl_long,
//     ClUlong => cl_ulong,
//     ClChar16 => cl_char16,
//     ClChar2 => cl_char2,
//     ClChar3 => cl_char3,
//     ClChar4 => cl_char4,
//     ClChar8 => cl_char8,
//     ClFloat16 => cl_float16,
//     ClFloat2 => cl_float2,
//     ClFloat3 => cl_float3,
//     ClFloat4 => cl_float4,
//     ClFloat8 => cl_float8,
//     ClInt16 => cl_int16,
//     ClInt2 => cl_int2,
//     ClInt3 => cl_int3,
//     ClInt4 => cl_int4,
//     ClInt8 => cl_int8,
//     ClLong16 => cl_long16,
//     ClLong2 => cl_long2,
//     ClLong3 => cl_long3,
//     ClLong4 => cl_long4,
//     ClLong8 => cl_long8,
//     ClShort => cl_short,
//     ClShort16 => cl_short16,
//     ClShort2 => cl_short2,
//     ClShort3 => cl_short3,
//     ClShort4 => cl_short4,
//     ClShort8 => cl_short8,
//     ClUchar16 => cl_uchar16,
//     ClUchar2 => cl_uchar2,
//     ClUchar3 => cl_uchar3,
//     ClUchar4 => cl_uchar4,
//     ClUchar8 => cl_uchar8,
//     ClUint16 => cl_uint16,
//     ClUint2 => cl_uint2,
//     ClUint3 => cl_uint3,
//     ClUint4 => cl_uint4,
//     ClUint8 => cl_uint8,
//     ClUlong16 => cl_ulong16,
//     ClUlong2 => cl_ulong2,
//     ClUlong3 => cl_ulong3,
//     ClUlong4 => cl_ulong4,
//     ClUlong8 => cl_ulong8,
//     ClUshort => cl_ushort,
//     ClUshort16 => cl_ushort16,
//     ClUshort2 => cl_ushort2,
//     ClUshort3 => cl_ushort3,
//     ClUshort4 => cl_ushort4,
//     ClUshort8 => cl_ushort8
// );

// // impl PartialEq for [<$new_t $count>] {
// //     fn eq(&self, other: &Self) -> bool {
// //         let left: [$rust_t; $count] = self.convert_to();
// //         let right: [$rust_t; $count] = other.convert_to();
// //         left == right
// //     }
// // }
