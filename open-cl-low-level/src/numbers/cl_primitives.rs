// The only entrypoint into the entire app for cl_* primitives.

use crate::numbers::{NumCast, NumCastFrom, NumberOps};
use libc::size_t;
use std::ops::*;

pub use cl_sys::{
    cl_char, cl_double, cl_float, cl_int, cl_long, cl_short, cl_uchar, cl_uint, cl_ulong, cl_ushort,
};

impl NumberOps for cl_char {}
impl NumberOps for cl_double {}
impl NumberOps for cl_float {}
impl NumberOps for cl_int {}
impl NumberOps for cl_long {}
impl NumberOps for cl_short {}
impl NumberOps for cl_uchar {}
impl NumberOps for cl_uint {}
impl NumberOps for cl_ulong {}
impl NumberOps for cl_ushort {}
impl NumberOps for size_t {}

pub trait ClPrimitiveNumber: NumberOps + PartialOrd {}

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

macro_rules! impl_cast_via_num_cast {
    ($name:ident, $target:ident) => {
        impl NumCastFrom<$target> for $name {
            #[inline(always)]
            fn num_cast_from(val: $target) -> Option<$name> {
                NumCast::from(val)
            }
        }
    };
}

macro_rules! impl_cast_many {
    ([ $( $t:ident ),+ ]) => {
        impl_cast_types!([$( $t ),+], [$( $t ),+]);
    };
}

macro_rules! impl_cast_types {
    ([ $( $t:ident ),+ ], $types:tt) => {
        $(
            impl_cast_one_to_many!($t => $types);
        )+
    }
}

macro_rules! impl_cast_one_to_many {
    ($t1:ident => [ $( $t2:ident ),+ ]) => {
        $(
            impl_cast_via_num_cast!($t1, $t2);
        )+
    };
}

impl_cast_many!([
    cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float,
    cl_double, size_t
]);
