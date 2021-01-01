// Module for defining scalar newtypes
pub use crate::numbers::Number;
use crate::numbers::{
    Bool, Char, Double, Float, Half, Int, Long, NumberOps, Short, SizeT, Uchar, Uint, Ulong, Ushort,
};
use std::ops::*;

pub trait Scalar: NumberOps {}

pub trait IntScalar: Scalar {}
pub trait SignedScalar: Scalar + IntScalar + Neg<Output = Self> {}
pub trait UnsignedScalar: Scalar + IntScalar {}
pub trait FloatScalar: Scalar {}

impl Scalar for Uchar {}
impl Scalar for Char {}
impl Scalar for Ushort {}
impl Scalar for Short {}
impl Scalar for Uint {}
impl Scalar for Int {}
impl Scalar for Ulong {}
impl Scalar for Long {}
impl Scalar for Float {}
impl Scalar for Double {}
impl Scalar for SizeT {}
impl Scalar for Half {}
impl Scalar for Bool {}

impl Scalar for u8 {}
impl Scalar for i8 {}
impl Scalar for u16 {}
impl Scalar for i16 {}
impl Scalar for u32 {}
impl Scalar for i32 {}
impl Scalar for f32 {}
impl Scalar for u64 {}
impl Scalar for i64 {}
impl Scalar for f64 {}
impl Scalar for usize {}
impl Scalar for isize {}

// signed
impl IntScalar for Char {}
impl IntScalar for Short {}
impl IntScalar for Int {}
impl IntScalar for Long {}
// unsigned
impl IntScalar for Uchar {}
impl IntScalar for Ushort {}
impl IntScalar for Uint {}
impl IntScalar for Ulong {}
impl IntScalar for SizeT {}
impl IntScalar for Bool {}

impl SignedScalar for Char {}
impl SignedScalar for Short {}
impl SignedScalar for Int {}
impl SignedScalar for Long {}

impl UnsignedScalar for Uchar {}
impl UnsignedScalar for Ushort {}
impl UnsignedScalar for Uint {}
impl UnsignedScalar for Ulong {}
impl UnsignedScalar for SizeT {}
impl UnsignedScalar for Bool {}

impl FloatScalar for Half {}
impl FloatScalar for Float {}
impl FloatScalar for Double {}
