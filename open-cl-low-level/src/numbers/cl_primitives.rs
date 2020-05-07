// The only entrypoint into the entire app for cl_* primitives.

use libc::size_t;
use num_traits::cast::{NumCast, ToPrimitive};
use num_traits::{One, Zero};
use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::*;

pub use cl_sys::{
    cl_char, cl_double, cl_float, cl_int, cl_long, cl_short, cl_uchar, cl_uint, cl_ulong, cl_ushort,
};

pub trait ClPrimitiveNumber:
    Copy
    + Clone
    + Default
    + Sized
    + PartialEq
    + PartialEq<Self>
    + PartialOrd
    + Send
    + Sync
    + 'static
    + Debug
    + Display
    + Zero<Output = Self>
    + One<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    + Sum<Self>
    + Product<Self>
    + ToPrimitive
    + NumCast
{
}

pub trait ClSignedPrimitive:
    ClPrimitiveNumber
    + Neg<Output = Self>
    + Shr<Self, Output = Self>
    + Shl<Self, Output = Self>
    + Not<Output = Self>
{
}

pub trait ClUnsignedPrimitive:
    ClPrimitiveNumber + Shr<Self, Output = Self> + Shl<Self, Output = Self> + Not<Output = Self>
{
}

pub trait ClFloatPrimitive: ClPrimitiveNumber {}

impl ClPrimitiveNumber for cl_uchar {}
impl ClPrimitiveNumber for cl_char {}
impl ClPrimitiveNumber for cl_ushort {}
impl ClPrimitiveNumber for cl_short {}
impl ClPrimitiveNumber for cl_uint {}
impl ClPrimitiveNumber for cl_int {}
impl ClPrimitiveNumber for cl_ulong {}
impl ClPrimitiveNumber for cl_long {}
impl ClPrimitiveNumber for cl_float {}
impl ClPrimitiveNumber for cl_double {}
impl ClPrimitiveNumber for size_t {}

impl ClSignedPrimitive for cl_char {}
impl ClSignedPrimitive for cl_short {}
impl ClSignedPrimitive for cl_int {}
impl ClSignedPrimitive for cl_long {}

impl ClUnsignedPrimitive for cl_uchar {}
impl ClUnsignedPrimitive for cl_ushort {}
impl ClUnsignedPrimitive for cl_uint {}
impl ClUnsignedPrimitive for cl_ulong {}
impl ClUnsignedPrimitive for size_t {}

impl ClFloatPrimitive for cl_float {}
impl ClFloatPrimitive for cl_double {}
