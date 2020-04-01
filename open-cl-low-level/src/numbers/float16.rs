// Move this to open_cl_rust;

use crate::numbers::cl_number::{cl_half, f16};
use crate::numbers::{ClHalf, Number, Zeroed};
use crate::{
    AsPtr, ClRustNum, ClTryFrom, Error, NumLevelChange, NumberType, NumberTypedT, TypeError,
};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct F16(pub u16);

pub const MAX: F16 = F16(31743);
pub const MIN: F16 = F16(64511);

impl Number for F16 {}
impl ClRustNum for F16 {}

impl NumLevelChange for F16 {
    type ClNum = cl_half;
    type NewNum = ClHalf;
    type RustNum = F16;

    fn change_to_cl_num(self) -> Self::ClNum {
        self.0
    }

    fn change_to_new_num(self) -> Self::NewNum {
        ClHalf(self.0)
    }

    fn change_to_rust_num(self) -> Self::RustNum {
        self
    }
}

impl F16 {
    pub fn from_f32(num: f32) -> F16 {
        F16(f16::from_f32(num).to_bits())
    }

    pub fn to_f32(self) -> f32 {
        f16::from_bits(self.0).to_f32()
    }

    pub const fn max_value() -> F16 {
        MAX
    }

    pub const fn min_value() -> F16 {
        MIN
    }
}

impl Zeroed for F16 {
    fn zeroed() -> F16 {
        F16(f16::zeroed().to_bits())
    }
}

impl AsPtr<cl_half> for F16 {
    fn as_ptr(&self) -> *const cl_half {
        &self.0 as *const cl_half
    }

    fn as_mut_ptr(&mut self) -> *mut cl_half {
        &mut self.0 as *mut cl_half
    }
}

impl NumberTypedT for F16 {
    fn number_type() -> NumberType {
        NumberType::ClHalf
    }
}

impl fmt::Debug for F16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f32())
    }
}

impl ClTryFrom<f32> for F16 {
    fn try_from(num: f32) -> Result<F16, Error> {
        if num > f16::MAX.to_f32() {
            return Err(invalid_f16(num, "value too high"));
        }
        if num < f16::MIN.to_f32() {
            return Err(invalid_f16(num, "value too low"));
        }
        Ok(F16::from_f32(num))
    }
}

fn invalid_f16(num: f32, reason: &'static str) -> Error {
    TypeError::InvalidFloat16(format!("{:?}", num), reason).into()
}

#[cfg(test)]
mod tests {
    use super::F16;
    use crate::{ClTryFrom, Error, Zeroed};

    #[test]
    fn f16_debug_works() {
        let min = F16::min_value();
        assert_eq!(format!("{:?}", min), "-65504.0000");
        let zero = F16::zeroed();
        assert_eq!(format!("{:?}", zero), "0.0000");
        let max = F16::max_value();
        assert_eq!(format!("{:?}", max), "65504.0000");
    }

    #[test]
    fn try_from_succeeds_with_valid_values() {
        let num: F16 = ClTryFrom::try_from(0.0f32).unwrap();
        assert_eq!(num, F16(0u16));
    }

    #[test]
    fn try_from_fails_with_invalid_values() {
        let big_float = 100_000.0f32;
        let result: Result<F16, Error> = ClTryFrom::<f32>::try_from(big_float);
        assert!(result.is_err());
    }

    #[test]
    fn f16_max_is_expected_value() {
        let max_f16 = F16::max_value();
        assert_eq!(max_f16.0, 31743);
    }

    // fn f16_min_is_expected_value() {}
}
