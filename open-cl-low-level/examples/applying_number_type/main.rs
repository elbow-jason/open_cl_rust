#[macro_use]
extern crate open_cl_low_level;

use open_cl_low_level::numbers::cl_number::*;
use open_cl_low_level::numbers::{NumberType, NumberTyped, NumberTypedT};

fn return_number_type_of<T: NumberTypedT>() -> NumberType {
    T::number_type()
}

fn main() {
    let t = apply_number_type!(NumberType::ClChar, return_number_type_of, []);
    assert_eq!(t, NumberType::ClChar);
}
