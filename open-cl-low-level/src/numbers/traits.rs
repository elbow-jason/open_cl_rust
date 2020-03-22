use crate::NumberTypedT;
use num_traits::cast::{FromPrimitive, NumCast, ToPrimitive};
use std::fmt::Debug;

pub trait Number: Sized + Clone + Copy + Send + Sync + 'static + Zeroed + NumberTypedT {}

pub trait ClNum: Number {}

pub trait ClNewNum: Number {}

pub trait ClRustNum: Number {}

pub trait NumChange: Number {
    type ClNum: ClNum;
    type NewNum: ClNewNum;
    type RustNum: ClRustNum;

    fn to_cl_num(self) -> Self::ClNum;
    fn to_new_num(self) -> Self::NewNum;
    fn to_rust_num(self) -> Self::RustNum;
}

pub trait Zeroed {
    fn zeroed() -> Self;
}

// pub trait ClPrimitive {}
pub trait ClPrimitive:
    NumCast + ToPrimitive + FromPrimitive + Copy + NumberTypedT + ClNum + Debug
{
}
// pub trait IsClVector: Copy + FFINumber {}
pub trait ClVector<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector2 {}
pub trait ClVector3 {}
pub trait ClVector4 {}
pub trait ClVector8 {}
pub trait ClVector16 {}
