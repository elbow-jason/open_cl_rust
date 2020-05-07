mod traits;
pub use traits::*;

mod scalars;
pub use scalars::*;

// mod vectors;
// pub use vectors::*;
// The only entrypoint into the entire app for cl_* primitives.

use num_traits::cast::{NumCast, ToPrimitive};
use num_traits::{One, Zero};
use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::*;

pub trait Scalar:
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
    + NumCast
{
}

pub trait SignedScalar:
    Scalar
    + Neg<Output = Self>
    + Shr<Self, Output = Self>
    + Shl<Self, Output = Self>
    + Not<Output = Self>
{
}

pub trait UnsignedScalar:
    Scalar + Shr<Self, Output = Self> + Shl<Self, Output = Self> + Not<Output = Self>
{
}

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

// impl SignedScalar for Char {}
// impl SignedScalar for Short {}
// impl SignedScalar for Int {}
// impl SignedScalar for Long {}

// impl UnsignedScalar for Uchar {}
// impl UnsignedScalar for Ushort {}
// impl UnsignedScalar for Uint {}
// impl UnsignedScalar for Ulong {}
// impl UnsignedScalar for SizeT {}

// impl FloatScalar for Float {}
// impl FloatScalar for Double {}

#[cfg(test)]
mod tests;

// use std::iter::{Sum, Product};
// use std::hash::{Hash, Hasher};
// use std::cmp::{Eq, Ord, Ordering};
// use num_traits::{Zero, One};
