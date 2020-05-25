use crate::{Number, Output};
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

    pub fn number_type_id(&self) -> any::TypeId {
        self.type_id
    }

    pub fn number_type_name(&self) -> &'static str {
        self.type_name
    }

    pub fn number_type_size_of(&self) -> usize {
        self.size_of
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

    fn type_check(other: &NumberType) -> Output<()> {
        if Self::number_type() != *other {
            Err(NumberTypeError::Mismatch(Self::number_type(), *other))?
        } else {
            Ok(())
        }
    }

    fn match_or_panic(other: &NumberType) {
        Self::type_check(other).unwrap()
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

    fn type_check(&self, other: &NumberType) -> Output<()> {
        if self.number_type() != *other {
            Err(NumberTypeError::Mismatch(self.number_type(), *other))?
        } else {
            Ok(())
        }
    }

    fn match_or_panic(&self, other: &NumberType) {
        self.type_check(other).unwrap()
    }

    fn type_name(&self) -> &'static str {
        self.number_type().type_name
    }

    fn size_of(&self) -> usize {
        self.number_type().size_of
    }
}
