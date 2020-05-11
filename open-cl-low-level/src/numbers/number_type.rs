use crate::numbers::Number;

use std::any;
use std::cmp;
use std::fmt;

use thiserror::Error;

#[derive(Clone, Copy, Hash)]
pub struct NumberType {
    type_id: any::TypeId,
    type_name: &'static str,
    size_of: usize,
}

impl NumberType {
    pub fn new<T: any::Any + Number>() -> NumberType {
        NumberType {
            type_id: any::TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            size_of: std::mem::size_of::<T>(),
        }
    }
}

impl NumberTyped for NumberType {
    fn number_type(&self) -> NumberType {
        (*self).clone()
    }
}

impl cmp::PartialEq for NumberType {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl cmp::Eq for NumberType {}

impl fmt::Debug for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NumberType({}, {:?})", self.type_name, self.type_id)
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum NumberTypeError {
    #[error("Number Type Mismatch - {0:?} vs {1:?}")]
    Mismatch(NumberType, NumberType),
}

pub trait NumberTypedT {
    fn number_type() -> NumberType;

    fn matches(other: NumberType) -> bool {
        Self::number_type() == other
    }
    fn match_or_panic(other: NumberType) {
        _match_or_panic(Self::number_type(), other);
    }

    fn type_name() -> &'static str {
        Self::number_type().type_name
    }

    fn size_of() -> usize {
        Self::number_type().size_of
    }
}

pub trait NumberTyped {
    fn number_type(&self) -> NumberType;

    fn matches(&self, other: NumberType) -> bool {
        self.number_type() == other
    }

    fn match_or_panic(&self, other: NumberType) {
        _match_or_panic(self.number_type(), other);
    }

    fn type_name(&self) -> &'static str {
        self.number_type().type_name
    }

    fn size_of(&self) -> usize {
        self.number_type().size_of
    }
}

#[inline]
fn _match_or_panic(t1: NumberType, t2: NumberType) {
    if t1 != t2 {
        let err = NumberTypeError::Mismatch(t1, t2);
        panic!("{:?}", err);
    }
}
