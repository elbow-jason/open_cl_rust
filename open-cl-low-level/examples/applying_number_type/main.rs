#[macro_use]
extern crate open_cl_low_level;

use open_cl_low_level::numbers::{NumberTypedT, NumberType, NumberTyped};
use open_cl_low_level::numbers::ffi_types::*;

fn return_number_type_of<T: NumberTypedT>() -> NumberType {
    T::number_type()
}

fn main() {
    let t = apply_number_type!(NumberType::ClChar, return_number_type_of, []);
    assert_eq!(t, NumberType::ClChar);
}
