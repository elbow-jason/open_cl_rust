use crate::{Error, TypeError};
use half::f16;

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct F16(pub u16);

pub const MAX: F16 = F16(31743);
pub const MIN: F16 = F16(64511);

impl F16 {
    pub fn try_from_f32(num: f32) -> Result<F16, Error> {
        if num > f16::MAX.to_f32() {
            return Err(invalid_f16(num, "value too high"));
        }
        if num < f16::MIN.to_f32() {
            return Err(invalid_f16(num, "value too low"));
        }
        Ok(F16::from_f32(num))
    }
    pub fn from_f32(num: f32) -> F16 {
        F16(f16::from_f32(num).to_bits())
    }

    pub fn from_u16(num: u16) -> F16 {
        F16(f16::from_bits(num).to_bits())
    }

    pub fn to_f32(self) -> f32 {
        f16::from_bits(self.0).to_f32()
    }

    pub fn to_f64(self) -> f64 {
        f16::from_bits(self.0).to_f64()
    }

    pub const fn max_value() -> F16 {
        MAX
    }

    pub const fn min_value() -> F16 {
        MIN
    }
}

impl From<f32> for F16 {
    fn from(val: f32) -> F16 {
        F16::from_f32(val)
    }
}

impl From<F16> for f32 {
    fn from(val: F16) -> f32 {
        val.to_f32()
    }
}

// impl Zeroed for F16 {
//     fn zeroed() -> F16 {
//         F16(0u16)
//     }
// }

// impl NumberTypedT for F16 {
//     fn number_type() -> NumberType {
//         NumberType::ClHalf
//     }
// }

impl fmt::Debug for F16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f32())
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
        let min: F16 = F16::min_value();
        assert_eq!(format!("{:?}", min), "-65504.0000");
        let zero: F16 = F16::zeroed();
        assert_eq!(format!("{:?}", zero), "0.0000");
        let max: F16 = F16::max_value();
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
