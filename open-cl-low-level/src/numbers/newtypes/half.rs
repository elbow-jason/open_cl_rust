
// "half" package aliased to "half_lib" in the Cargo.toml.
use half_lib::f16;
use ffi::cl_half;
use thiserror::Error;
use std::fmt;

use crate::Output;


// 65504.0000
pub const MAX: Half = Half(31743);

// -65504.0000
pub const MIN: Half = Half(64511);

#[derive(Error, Debug)]
pub enum HalfNumberError {
    #[error("Half value was too high (max_value {}, got {0})", MAX)]
    ValueTooHigh(f32),
    #[error("Half value was too low (min_value {}, got {0})", MIN)]
    ValueTooLow(f32),
}

use HalfNumberError::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Half(pub cl_half);

impl Half {
    pub fn try_from_f32(num: f32) -> Output<Half> {
        if num > MAX.to_f32() {
            return Err(ValueTooHigh(num))?;
        }
        if num < MIN.to_f32() {
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

    pub fn to_f32(self) -> f32 {
        f16::from_bits(self.0).to_f32()
    }

    pub fn to_f64(self) -> f64 {
        f16::from_bits(self.0).to_f64()
    }

    pub const fn max_value() -> Half {
        MAX
    }

    pub const fn min_value() -> Half {
        MIN
    }
}

impl From<f32> for Half {
    fn from(val: f32) -> Half {
        Half::from_f32(val)
    }
}

impl From<Half> for f32 {
    fn from(val: Half) -> f32 {
        val.to_f32()
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
        write!(f, "Half({:.4})", self.to_f32())
    }
}

impl fmt::Display for Half {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::Half;

    #[test]
    fn test_half_debug_works() {
        let min: Half = Half::min_value();
        assert_eq!(format!("{:?}", min), "-65504.0000");
        let zero: Half = Half::zeroed();
        assert_eq!(format!("{:?}", zero), "0.0000");
        let max: Half = Half::max_value();
        assert_eq!(format!("{:?}", max), "65504.0000");
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
        let max_Half = Half::max_value();
        assert_eq!(max_Half.0, 31743);
    }

    // fn Half_min_is_expected_value() {}
}
