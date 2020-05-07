use thiserror::Error;
use std::fmt;
use num_traits::{Zero, One};
use std::ops::*;

use crate::ffi::cl_uint;
use crate::Output;

#[allow(non_camel_case_types)]
pub type cl_bool = cl_uint;

#[derive(Error, Debug)]
pub enum BoolError {
    #[error("Invalid cl_bool value (expected 0 or 1, got {0}")]
    InvalidValue(cl_bool),
}

use BoolError::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Bool(cl_bool);

pub const TRUE: Bool = Bool(1);
pub const FALSE: Bool = Bool(0);

impl Bool {
    pub fn try_from_u32(num: u32) -> Output<Bool> {
        match num {
            0 => Ok(Bool(0)),
            1 => Ok(Bool(1)),
            v => Err(InvalidValue(v))?,
        }
    }

    pub fn to_bool(self) -> bool {
        match self.0 {
            0 => false,
            1 => true,
            v => panic!(InvalidValue(v))
        }
    }

    pub fn from_bool(val: bool) -> Bool {
        match val {
            false => Bool(0),
            true => Bool(1),
        }
    }
}

impl Add for Bool {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self.0, other.0) {
            (0, 0) => Bool(0),
            (1, 0) => Bool(1),
            (0, 1) => Bool(1),
            (1, 1) => Bool(0),
            (_, _) => unreachable!()
        }
    }
}

impl Mul for Bool {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self.0 * other.0 {
            0 => Bool(0),
            1 => Bool(1),
            _ => unreachable!()
        }
    }
}

impl Zero for Bool {
    fn zero() -> Self {
        Bool(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for Bool {
    fn one() -> Self {
        Bool(1)
    }
}


impl From<cl_bool> for Bool {
    fn from(val: cl_bool) -> Bool {
        Bool::try_from_u32(val).unwrap()
    }
}

impl From<Bool> for cl_bool {
    fn from(val: Bool) -> cl_bool {
        val.0
    }
}

impl From<bool> for Bool {
    fn from(val: bool) -> Bool {
        match val {
            false => Bool(0),
            true => Bool(1),
        }
    }
}

impl From<Bool> for bool {
    fn from(val: Bool) -> bool {
        val.to_bool()
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.to_bool() {
            write!(f, "Bool(1 as true)")
        } else {
            write!(f, "Bool(0 as false)")
        }
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.to_bool() {
            write!(f, "true")
        } else {
            write!(f, "false")
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{Bool, TRUE, FALSE};
    use num_traits::{Zero, One};

    #[test]
    fn test_debug_works() {
        assert_eq!(format!("{:?}", TRUE), "Bool(1 as true)");
        assert_eq!(format!("{:?}", FALSE), "Bool(0 as false)");
        assert_eq!(format!("{:?}", Bool(1)), "Bool(1 as true)");
        assert_eq!(format!("{:?}", Bool(0)), "Bool(0 as false)");
    }

    #[test]
    fn test_zero_impl() {
        let zero: Bool = Bool::zero();
        assert_eq!(zero, Bool(0));
    }

    #[test]
    fn test_one_impl() {
        let one: Bool = Bool::one();
        assert_eq!(one, Bool(1));
    }


    #[test]
    fn try_from_u32_works_with_1() {
        let val: Bool = Bool::try_from_u32(1).unwrap();
        assert_eq!(Bool(1), val);
    }


    #[test]
    fn try_from_u32_works_with_0() {
        let val: Bool = Bool::try_from_u32(0).unwrap();
        assert_eq!(Bool(0), val);
    }

    #[test]
    fn try_from_u32_works_with_u32_greater_than_1() {
        match Bool::try_from_u32(2) {
            Err(_) => (),
            Ok(Bool(_)) => panic!("Bool::try_from_u32(2) did not return an Err as expected")
        }
    }

    // #[test]
    // fn try_from_fails_with_invalid_values() {
    //     let big_float = 100_000.0f32;
    //     let result: Result<Half, Error> = ClTryFrom::<f32>::try_from(big_float);
    //     assert!(result.is_err());
    // }


    // fn Half_min_is_expected_value() {}
}
