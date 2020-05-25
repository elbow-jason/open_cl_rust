use crate::numbers::{NumCastInto, Scalar};
pub use num_traits::{NumCast, One, ToPrimitive, Zero};
use std::fmt::{Debug, Display};

pub trait NumberOps:
    Copy
    + Clone
    + Default
    + Sized
    + PartialEq<Self>
    + Send
    + Sync
    + 'static
    + Debug
    + Display
    + Zero<Output = Self>
    + One<Output = Self>
{
}

pub trait Number: NumberOps {
    type Scalar: Scalar;
    type Outer: Sized + NumCastInto<Self>;

    fn new(val: Self::Outer) -> Self;
    fn into_outer(self) -> Self::Outer;
}
