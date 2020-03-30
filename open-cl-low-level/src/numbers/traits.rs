// use crate::NumberTypedT;
use super::number_type::NumberTypedT;
use num_traits::cast::{FromPrimitive, NumCast, ToPrimitive};
use std::fmt::Debug;

pub trait Number: Sized + Clone + Copy + Send + Sync + 'static + Zeroed {}

pub trait ClNum: Number + NumberTypedT {}

pub trait ClNewNum: Number + NumberTypedT {}

pub trait ClRustNum: Number + NumberTypedT {}

pub trait ClRustPrimitiveNum: Number {}

pub trait IntoClNum {
    type Num: ClNum;
    fn into_cl_num(self) -> Self::Num;
}

pub trait IntoNewNum {
    type Num: ClNewNum;
    fn into_new_num(self) -> Self::Num;
}

pub trait IntoRustNum {
    type Num: ClRustNum;
    fn into_rust_num(self) -> Self::Num;
}

pub trait Zeroed {
    fn zeroed() -> Self;
}

pub trait ClPrimitive: NumCast + ToPrimitive + FromPrimitive + Copy + ClNum + Debug {}

// pub trait ClVector<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector2<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector3<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector4<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector8<T: ClPrimitive>: Copy + ClNum {}
pub trait ClVector16<T: ClPrimitive>: Copy + ClNum {}

pub trait InnerMutRef<T> {
    fn inner_mut_ref(&mut self) -> &mut T;
}
