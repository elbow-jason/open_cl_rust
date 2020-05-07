
// use super::cl_primitives::{
//   ClPrimitiveNumber, cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float, cl_double};
// use std::fmt::Debug;

// pub trait PrimitiveArray<Num: ClPrimitiveNumber>: Sized {}

// macro_rules! def_primitive_array {
//   ($num:ty, $len:expr) => {
//     impl PrimitiveArray<$num> for [$num; $len] {}
//   }
// }

// macro_rules! primitive_arrays {
//   ( $( $t:ty ),*) => {
//     $(
//       def_primitive_array!($t, 1);
//       def_primitive_array!($t, 2);
//       def_primitive_array!($t, 3);
//       def_primitive_array!($t, 4);
//       def_primitive_array!($t, 8);
//       def_primitive_array!($t, 16);
//     )*
//   }
// }

// primitive_arrays!(cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float, cl_double);

// pub trait ClNumber: Sized + Copy + Clone + Debug {
//   type Primitive: ClPrimitiveNumber;
//   type Inner: Sized + PrimitiveArray<Self::Primitive>;
//   type Outer = Self::Inner;

//   fn from_outer(val: Self::Outer) -> Self;
//   fn from_inner(val: Self::Inner) -> Self;
//   fn inner_len() -> usize;
//   fn outer_len() -> usize;
//   fn into_inner(self) -> Self::Inner;
//   fn inner_ref(&self) -> &Self::Inner;
//   fn inner_mut_ref(&mut self) -> &mut Self::Inner;
//   fn inner_slice(&self) -> &[Self::Inner];
//   fn inner_slice_mut(&self) -> &mut [Self::Inner];

//   fn into_outer(self) -> Self::Outer;
//   fn from_bytes(bytes: &[u8]) -> Self;
// }

// #[derive(Debug, Clone, Copy, )]
// #[repr(transparent)]
// pub struct Uchar()