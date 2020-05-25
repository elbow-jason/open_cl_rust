// "half" package aliased to "half_lib" in the Cargo.toml.
use half_lib::f16;

use std::fmt;
use thiserror::Error;

use crate::numbers::{NumCastFrom, NumberOps, One, ToPrimitive, Zero};

use std::cmp::Ordering;
use std::ops::*;

use crate::cl::cl_half;
use crate::Output;

// use super::number::Number;
// use crate::numbers::scalar_traits::Scalar;

// 65504.0000
pub const MAX: Half = Half(31743);

// -65504.0000
pub const MIN: Half = Half(64511);

// 0.0000
pub const ZERO: Half = Half(0);

// 1.0000
pub const ONE: Half = Half(15360);

#[derive(Error, Debug)]
pub enum HalfError {
    #[error("Half value was too high (max_value {}, got {0})", MAX)]
    ValueTooHigh(f32),
    #[error("Half value was too low (min_value {}, got {0})", MIN)]
    ValueTooLow(f32),
}

use HalfError::*;

#[derive(Clone, Copy, Hash)]
#[repr(transparent)]
pub struct Half(pub cl_half);

impl Half {
    pub fn try_from_f32(num: f32) -> Output<Half> {
        if num > MAX.into_f32() {
            return Err(ValueTooHigh(num))?;
        }
        if num < MIN.into_f32() {
            return Err(ValueTooLow(num))?;
        }
        Ok(Half::from_f32(num))
    }

    pub fn from_f32(num: f32) -> Half {
        Half(f16::from_f32(num).to_bits())
    }

    pub fn from_u16(num: u16) -> Half {
        Half(f16::from_bits(num).to_bits())
    }

    pub fn into_f32(self) -> f32 {
        f16::from_bits(self.0).to_f32()
    }

    pub fn into_f64(self) -> f64 {
        f16::from_bits(self.0).to_f64()
    }

    pub const fn max_value() -> Half {
        MAX
    }

    pub const fn min_value() -> Half {
        MIN
    }
}
impl NumberOps for Half {}

impl From<f32> for Half {
    fn from(val: f32) -> Half {
        Half::from_f32(val)
    }
}

impl From<f64> for Half {
    fn from(val: f64) -> Half {
        Half::from_f32(val.to_f32().unwrap())
    }
}

impl From<Half> for f32 {
    fn from(h: Half) -> f32 {
        h.into_f32()
    }
}

impl From<Half> for f64 {
    fn from(h: Half) -> f64 {
        h.into_f64()
    }
}

impl Add for Half {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Half::from_f32(self.into_f32() + other.into_f32())
    }
}

impl Sub for Half {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Half::from_f32(self.into_f32() - other.into_f32())
    }
}

impl Mul for Half {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Half::from_f32(self.into_f32() * other.into_f32())
    }
}

impl Div for Half {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Half::from_f32(self.into_f32() / other.into_f32())
    }
}

impl Rem for Half {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Half::from_f32(self.into_f32() % other.into_f32())
    }
}

impl AddAssign for Half {
    #[inline(always)]
    fn add_assign(&mut self, other: Half) {
        *self = *self + other;
    }
}

impl SubAssign for Half {
    #[inline(always)]
    fn sub_assign(&mut self, other: Half) {
        *self = *self - other;
    }
}

impl MulAssign for Half {
    #[inline(always)]
    fn mul_assign(&mut self, other: Half) {
        *self = *self * other;
    }
}

impl RemAssign for Half {
    #[inline(always)]
    fn rem_assign(&mut self, other: Half) {
        *self = *self % other;
    }
}

impl DivAssign for Half {
    #[inline(always)]
    fn div_assign(&mut self, other: Half) {
        *self = *self / other;
    }
}

impl Default for Half {
    fn default() -> Half {
        Half(u16::default())
    }
}

impl Zero for Half {
    fn zero() -> Self {
        ZERO
    }
    fn is_zero(&self) -> bool {
        self.0 == ZERO.0
    }
}

impl One for Half {
    fn one() -> Self {
        ONE
    }
}

impl PartialEq for Half {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Half {}

impl PartialOrd<Half> for Half {
    fn partial_cmp(&self, other: &Half) -> Option<Ordering> {
        self.into_f32().partial_cmp(&other.into_f32())
    }
}

impl Ord for Half {
    fn cmp(&self, other: &Half) -> Ordering {
        let other_f32 = other.into_f32();
        match self.into_f32() {
            n if n > other_f32 => Ordering::Greater,
            n if n < other_f32 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

// impl Zeroed for Half {
//     fn zeroed() -> Half {
//         Half(0u16)
//     }
// }

// impl NumberTypedT for Half {
//     fn number_type() -> NumberType {
//         NumberType::ClHalf
//     }
// }

impl fmt::Debug for Half {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half({:.4})", self.into_f32())
    }
}

impl fmt::Display for Half {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.into_f32())
    }
}

impl<T> NumCastFrom<T> for Half
where
    T: ToPrimitive,
{
    #[inline]
    fn num_cast_from(val: T) -> Option<Half> {
        let float = val.to_f32()?;
        Half::try_from_f32(float).ok()
    }
}

// impl<T> NumCastFrom<T> for Half
// where
//     T: Scalar + Number,
// {
//     #[inline]
//     fn num_cast_from(val: T) -> Option<Half> {
//         let float = val.to_f32()?;
//         Half::try_from_f32(float).ok()
//     }
// }

#[cfg(test)]
mod half_tests {
    use super::Half;
    // use crate::numbers::{Char, Float};
    use crate::numbers::NumCastFrom; //, NumCastInto, Number};
    use num_traits::{One, Zero};

    #[test]
    fn test_debug_works() {
        let min: Half = Half::min_value();
        assert_eq!(format!("{:?}", min), "Half(-65504.0000)");
        let zero: Half = Half::zero();
        assert_eq!(format!("{:?}", zero), "Half(0.0000)");
        let max: Half = Half::max_value();
        assert_eq!(format!("{:?}", max), "Half(65504.0000)");
        let one: Half = Half::one();
        assert_eq!(format!("{:?}", one), "Half(1.0000)");
    }

    #[test]
    fn test_zero_impl() {
        let zero: Half = Half::zero();
        assert_eq!(format!("{}", zero), "0.0000");
    }

    #[test]
    fn test_one_impl() {
        let one: Half = Half::one();
        assert_eq!(format!("{}", one), "1.0000");
    }

    #[test]
    fn test_add_impl() {
        let one: Half = From::from(1.0);
        let three: Half = From::from(3.0);
        let four = one + three;
        assert_eq!(four, From::from(4.0));
    }

    #[test]
    fn test_sub_impl() {
        let one: Half = From::from(1.0);
        let three: Half = From::from(3.0);
        let neg_two = one - three;
        assert_eq!(neg_two, From::from(-2.0));
    }

    #[test]
    fn test_mul_impl() {
        let one: Half = From::from(1.0);
        let three: Half = From::from(3.0);
        let nine = three * three;
        let one_again = one * one;
        assert_eq!(nine, From::from(9.0));
        assert_eq!(one_again, one);
    }

    #[test]
    fn test_num_cast_impl() {
        let h01: Half = NumCastFrom::num_cast_from(10i8).unwrap();
        let h02: Half = NumCastFrom::num_cast_from(10u8).unwrap();
        let h03: Half = NumCastFrom::num_cast_from(10i16).unwrap();
        let h04: Half = NumCastFrom::num_cast_from(10u16).unwrap();
        let h05: Half = NumCastFrom::num_cast_from(10i32).unwrap();
        let h06: Half = NumCastFrom::num_cast_from(10u32).unwrap();
        let h07: Half = NumCastFrom::num_cast_from(10.0f32).unwrap();
        let h08: Half = NumCastFrom::num_cast_from(10i64).unwrap();
        let h09: Half = NumCastFrom::num_cast_from(10u64).unwrap();
        let h10: Half = NumCastFrom::num_cast_from(10.0f64).unwrap();

        let ten: Half = Half::from_f32(10.0);
        assert_eq!(h01, ten);
        assert_eq!(h02, ten);
        assert_eq!(h03, ten);
        assert_eq!(h04, ten);
        assert_eq!(h05, ten);
        assert_eq!(h06, ten);
        assert_eq!(h07, ten);
        assert_eq!(h08, ten);
        assert_eq!(h09, ten);
        assert_eq!(h10, ten);

        // TODO: fix Half (or other) num_cast_from.
        // let half_ten = Half::from_f32(10.0f32);
        // let h11: Half = NumCastFrom::num_cast_from(half_ten).unwrap();
        // assert_eq!(h11, ten);

        // fix me.
        // let char10 = Char::new(10i8);
        // let float_from_char10: Float = char10.num_cast_into().unwrap();
        // assert_eq!(float_from_char10, Float::new(10.0f32));
        // let h12: Half = NumCastFrom::num_cast_from(char10).unwrap();
        // println!("{:?}", h12);
        // assert_eq!(h12, ten);
    }

    // #[test]
    // fn try_from_succeeds_with_valid_values() {
    //     let num: Half = ClTryFrom::try_from(0.0f32).unwrap();
    //     assert_eq!(num, Half(0u16));
    // }

    // #[test]
    // fn try_from_fails_with_invalid_values() {
    //     let big_float = 100_000.0f32;
    //     let result: Result<Half, Error> = ClTryFrom::<f32>::try_from(big_float);
    //     assert!(result.is_err());
    // }

    #[test]
    fn test_half_max_is_expected_value() {
        let max_half = Half::max_value();
        assert_eq!(max_half.0, 31743);
    }

    // fn Half_min_is_expected_value() {}
}
