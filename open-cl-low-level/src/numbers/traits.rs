use crate::NumberTypedT;
use num_traits::cast::{FromPrimitive, NumCast, ToPrimitive};
use std::fmt::Debug;

pub trait Number: Sized + Clone + Copy + Send + Sync + 'static + Zeroed + NumberTypedT {}

pub trait ClNum: Number {}

pub trait ClNewNum: Number {}

pub trait ClRustNum: Number {}

pub trait ClRustPrimitiveNum: Number {}

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

pub trait ClPrimitive:
    NumCast + ToPrimitive + FromPrimitive + Copy + NumberTypedT + ClNum + Debug
{
}

// pub trait ClVector<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector2<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector3<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector4<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector8<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector16<T: ClPrimitive>: Copy + ClNum {}

pub trait InnerMutRef<T> {
    fn inner_mut_ref(&mut self) -> &mut T;
}
