use crate::numbers::{NumCast, NumCastFrom, NumberOps, One, Zero};
use std::fmt;
use std::ops::*;
use thiserror::Error;

use crate::cl::cl_bool;
use crate::Output;

#[derive(Error, Debug)]
pub enum BoolError {
    #[error("Invalid cl_bool value (expected 0 or 1, got {0}")]
    InvalidValue(cl_bool),

    #[error("Shl op cause an overflow - {left} << {right} == {equals} - {equals} is not 1 nor 2")]
    ShlOverflow {
        left: cl_bool,
        right: cl_bool,
        equals: cl_bool,
    },
}

macro_rules! _panic_invalid {
    ($b:expr) => {
        panic!(BoolError::InvalidValue($b));
    };
}

macro_rules! _shl_overflow {
    ($left:expr, $right:expr, $equals:expr) => {
        panic!(BoolError::ShlOverflow {
            left: $left,
            right: $right,
            equals: $equals
        });
    };
}

use BoolError::*;

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, ShlAssign, ShrAssign,
};

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    ShlAssign,
    ShrAssign,
)]
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
            v => _panic_invalid!(v),
        }
    }

    pub fn from_bool(val: bool) -> Bool {
        match val {
            false => Bool(0),
            true => Bool(1),
        }
    }
}

impl NumberOps for Bool {}

impl Add for Bool {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self.0, other.0) {
            (0, 0) => Bool(0),
            (1, 0) => Bool(1),
            (0, 1) => Bool(1),
            (1, 1) => Bool(0),
            (_, _) => unreachable!(),
        }
    }
}

impl AddAssign for Bool {
    fn add_assign(&mut self, other: Bool) {
        *self = *self + other
    }
}

impl Sub for Bool {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        From::from(self.0 - other.0)
    }
}

impl SubAssign for Bool {
    fn sub_assign(&mut self, other: Bool) {
        *self = *self - other
    }
}

impl Mul for Bool {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self.0 * other.0 {
            0 => Bool(0),
            1 => Bool(1),
            v => _panic_invalid!(v),
        }
    }
}

impl MulAssign for Bool {
    fn mul_assign(&mut self, other: Bool) {
        *self = *self * other
    }
}

impl Rem for Bool {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match self.0 % other.0 {
            0 => Bool(0),
            1 => Bool(1),
            v => _panic_invalid!(v),
        }
    }
}

impl RemAssign for Bool {
    fn rem_assign(&mut self, other: Bool) {
        *self = *self % other
    }
}

impl Div for Bool {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match self.0 / other.0 {
            0 => Bool(0),
            1 => Bool(1),
            v => _panic_invalid!(v),
        }
    }
}

impl DivAssign for Bool {
    fn div_assign(&mut self, other: Bool) {
        *self = *self / other
    }
}

impl Default for Bool {
    fn default() -> Bool {
        From::from(cl_bool::default())
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
    #[inline(always)]
    fn from(val: cl_bool) -> Bool {
        match val {
            0 => Bool(0),
            1 => Bool(1),
            v => _panic_invalid!(v),
        }
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

impl Shr<Bool> for Bool {
    type Output = Bool;

    fn shr(self, rhs: Bool) -> Self::Output {
        From::from(self.0 >> rhs.0)
    }
}

impl Shl<Bool> for Bool {
    type Output = Bool;

    fn shl(self, rhs: Bool) -> Self::Output {
        match self.0 << rhs.0 {
            0 => Bool(0),
            1 => Bool(1),
            equals => _shl_overflow!(self.0, rhs.0, equals),
        }
    }
}

impl Not for Bool {
    type Output = Bool;

    fn not(self) -> Bool {
        match self.0 {
            0 => Bool(1),
            1 => Bool(0),
            v => _panic_invalid!(v),
        }
    }
}

impl<T> NumCastFrom<T> for Bool
where
    T: NumCast,
{
    fn num_cast_from(val: T) -> Option<Bool> {
        match val.to_u32()? {
            0 => Some(Bool(0)),
            1 => Some(Bool(1)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{Bool, FALSE, TRUE};
    use num_traits::{One, Zero};

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
            Ok(Bool(_)) => panic!("Bool::try_from_u32(2) did not return an Err as expected"),
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
