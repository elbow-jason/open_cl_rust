pub use libc::size_t;

pub use half::f16;
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
pub use crate::ffi::{cl_bool, cl_double, cl_half};

// macro_rules! cl_vector {
//     ($primitive:ty, $( $t:ty ),*) => {
//         $(
//             impl ClVector<$primitive> for $t {}
//         )*
//     }
// }

// macro_rules! impl_trait {
//     ($the_trait:ident, $( $t:ty ),*) => {
//         $(
//             impl $the_trait for $t {}
//         )*
//     }
// }

// pub trait FFINumber {}

// macro_rules! ffi_number {
//     ($( $t:ty ),*) => {
//         $(
//             impl FFINumber for $t {}
//         )*
//     }
// }

// // impl<T> ClType for T where T: FFINumber {}

// ffi_number!(cl_double, size_t);
// ffi_number!(cl_uchar, cl_uchar16, cl_uchar2, cl_uchar4, cl_uchar8);
// ffi_number!(cl_char, cl_char16, cl_char2, cl_char4, cl_char8);
// ffi_number!(cl_ushort, cl_ushort16, cl_ushort2, cl_ushort4, cl_ushort8);
// ffi_number!(cl_short, cl_short16, cl_short2, cl_short4, cl_short8);
// ffi_number!(cl_uint, cl_uint16, cl_uint2, cl_uint4, cl_uint8);
// ffi_number!(cl_int, cl_int16, cl_int2, cl_int4, cl_int8);
// ffi_number!(cl_ulong, cl_ulong16, cl_ulong2, cl_ulong4, cl_ulong8);
// ffi_number!(cl_long, cl_long16, cl_long2, cl_long4, cl_long8);
// ffi_number!(cl_float, cl_float16, cl_float2, cl_float4, cl_float8);

// impl_trait!(
//     ClPrimitive,
//     cl_double,
//     size_t,
//     cl_uchar,
//     cl_char,
//     cl_ushort,
//     cl_short,
//     cl_uint,
//     cl_int,
//     cl_ulong,
//     cl_long,
//     cl_float
// );

// cl_vector!(cl_uchar, cl_uchar16, cl_uchar2, cl_uchar4, cl_uchar8);
// cl_vector!(cl_char, cl_char16, cl_char2, cl_char4, cl_char8);
// cl_vector!(cl_ushort, cl_ushort16, cl_ushort2, cl_ushort4, cl_ushort8);
// cl_vector!(cl_short, cl_short16, cl_short2, cl_short4, cl_short8);
// cl_vector!(cl_uint, cl_uint16, cl_uint2, cl_uint4, cl_uint8);
// cl_vector!(cl_int, cl_int16, cl_int2, cl_int4, cl_int8);
// cl_vector!(cl_ulong, cl_ulong16, cl_ulong2, cl_ulong4, cl_ulong8);
// cl_vector!(cl_long, cl_long16, cl_long2, cl_long4, cl_long8);
// cl_vector!(cl_float, cl_float16, cl_float2, cl_float4, cl_float8);

// impl_trait!(
//     ClVector2, cl_uchar2, cl_char2, cl_ushort2, cl_short2, cl_uint2, cl_int2, cl_ulong2, cl_long2,
//     cl_float2
// );
// impl_trait!(
//     ClVector4, cl_uchar4, cl_char4, cl_ushort4, cl_short4, cl_uint4, cl_int4, cl_ulong4, cl_long4,
//     cl_float4
// );
// impl_trait!(
//     ClVector8, cl_uchar8, cl_char8, cl_ushort8, cl_short8, cl_uint8, cl_int8, cl_ulong8, cl_long8,
//     cl_float8
// );
// impl_trait!(
//     ClVector16,
//     cl_uchar16,
//     cl_char16,
//     cl_ushort16,
//     cl_short16,
//     cl_uint16,
//     cl_int16,
//     cl_ulong16,
//     cl_long16,
//     cl_float16
// );

// newtype_primitive_and_newtype_vectors!(cl_char, ClChar, i8);
// newtype_primitive_and_newtype_vectors!(cl_uchar, ClUchar, u8);
// newtype_primitive_and_newtype_vectors!(cl_short, ClShort, i16);
// newtype_primitive_and_newtype_vectors!(cl_ushort, ClUshort, u16);
// newtype_primitive_and_newtype_vectors!(cl_int, ClInt, i32);dd
// newtype_primitive_and_newtype_vectors!(cl_uint, ClUint, u32);
// newtype_primitive_and_newtype_vectors!(cl_long, ClLong, i64);
// newtype_primitive_and_newtype_vectors!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(size_t, SizeT, usize);
// newtype_primitive_and_newtype_vectors!(cl_float, ClFloat, f32);
// impl_primitive_conversion!(cl_double, ClDouble, f64);

// pub trait TypeName {
//     fn type_name(&self) -> String;
// }

// macro_rules! impl_type_name {
//     ($t:ident) => {
//         paste::item! {
//             const [<TYPE_NAME_ $t>]: &'static str = stringify!($t);
//             impl TypeName for $t {
//                 fn type_name(&self) -> &'static str {
//                     [<TYPE_NAME_ $t>]
//                 }
//             }
//         }
//     }
// }

// impl_type_name!(cl_char);
// impl_type_name!(cl_uchar);
// impl_type_name!(cl_short);
// impl_type_name!(cl_ushort);
// impl_type_name!(cl_int);
// impl_type_name!(cl_uint);
// impl_type_name!(cl_long);
// impl_type_name!(cl_ulong);
// impl_type_name!(cl_half);
// impl_type_name!(cl_float);
// impl_type_name!(cl_double);
// impl_type_name!(size_t);

// #[macro_use]
// macro_rules! impl_type_name_vector {
//     ($t:ident) => {
//         paste::item! {
//             impl_type_name!([<$t 2>]);
//             // impl_type_name!([<$t 3>]);
//             impl_type_name!([<$t 4>]);
//             impl_type_name!([<$t 8>]);
//             impl_type_name!([<$t 16>]);
//         }
//     }
// }

// impl_type_name_vector!(cl_char);
// impl_type_name_vector!(cl_uchar);
// impl_type_name_vector!(cl_short);
// impl_type_name_vector!(cl_ushort);
// impl_type_name_vector!(cl_int);
// impl_type_name_vector!(cl_uint);
// impl_type_name_vector!(cl_long);
// impl_type_name_vector!(cl_ulong);
// impl_type_name_vector!(cl_float);

// impl_type_name_vector!(cl_half);
// impl_type_name_vector!(cl_double);
// impl_type_name_vector!(size_t);

// impl ClNumber for u8 {}
// impl ClNumber for i8 {}
// impl ClNumber for u16 {}
// impl ClNumber for i16 {}
// impl ClNumber for u32 {}
// impl ClNumber for i32 {}
// impl ClNumber for f32 {}
// impl ClNumber for u64 {}
// impl ClNumber for i64 {}
// impl ClNumber for f64 {}

// pub trait FromClNumber<T: ClNumber> {
//     fn from_cl_number(value: Self) -> T;
// }

// pub trait ClNumberInto<T: ClNumber> where T: ClNumber, Self: FromClNumber<T> {
//     fn cl_number_into(val: T) -> Self {
//         Self::from_cl_number(val)
//     }
// }

// pub trait ClNumberFrom<T, N: ClNumber> {
//     fn cl_number_from(val: T) -> N;
// }

// pub trait IntoClNumber<T, N: ClNumber> where N: ClNumber, Self: ClNumberFrom<T> {
//     fn cl_number_into(val: T) -> Self {
//         Self::from_cl_number(val)
//     }
// }

// impl<T: ClNumber, R> ClNumberInto<T> for R where R: FromClNumber<T> {}

// impl ClNumber for $new_t {}
// impl ClNumber for [<$new_t 2>] {}
// impl ClNumber for [<$new_t 3>] {}
// impl ClNumber for [<$new_t 4>] {}
// impl ClNumber for [<$new_t 8>] {}
// impl ClNumber for [<$new_t 16>] {}

// macro_rules! newtype_primitive_and_newtype_vectors {
//     ($t:ident, $new_t:ident, $rust_t:ident) => {
//         paste::item! {

//             // define_newtypes!($t, $new_t, $rust_t);

//             // // unsafe impl ClNumber for $t {}
//             // impl ClNumber for [<$t 2>] {}
//             // impl ClNumber for [<$t 4>] {}
//             // // unsafe impl ClNumber for [<$t 3>] {}
//             // impl ClNumber for [<$t 8>] {}
//             // impl ClNumber for [<$t 16>] {}

//             // impl_type_name!([<$t 2>]);
//             // impl_type_name!([<$t 4>]);
//             // impl_type_name!([<$t 8>]);
//             // impl_type_name!([<$t 16>]);

//             // // from_cl_number_inner_s!($t, $new_t, $rust_t);
//             // from_cl_number_inner_s!([<$t 2>], [<$new_t 2>], [$rust_t; 2]);
//             // from_cl_number_inner_s!([<$t 4>], [<$new_t 4>], [$rust_t; 4]);
//             // from_cl_number_inner_s!([<$t 8>], [<$new_t 8>], [$rust_t; 8]);
//             // from_cl_number_inner_s!([<$t 16>], [<$new_t 16>], [$rust_t; 16]);
//             // from_cl_number_inner_s3!($t, $new_t, $rust_t);
//         }
//     }
// }

// newtype_primitive_and_newtype_vectors!(cl_char, ClChar, i8);
// newtype_primitive_and_newtype_vectors!(cl_uchar, ClUchar, u8);
// newtype_primitive_and_newtype_vectors!(cl_short, ClShort, i16);
// newtype_primitive_and_newtype_vectors!(cl_ushort, ClUshort, u16);
// newtype_primitive_and_newtype_vectors!(cl_int, ClInt, i32);
// newtype_primitive_and_newtype_vectors!(cl_uint, ClUint, u32);
// newtype_primitive_and_newtype_vectors!(cl_long, ClLong, i64);
// newtype_primitive_and_newtype_vectors!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(size_t, SizeT, usize);
// newtype_primitive_and_newtype_vectors!(cl_float, ClFloat, f32);
// impl_primitive_conversion!(cl_double, ClDouble, f64);
