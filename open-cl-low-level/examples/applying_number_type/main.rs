// use std::fmt::Debug;

// #[macro_use]
// extern crate open_cl_low_level;

// use open_cl_low_level::numbers::{NumberType, NumberTypedT};

// fn return_number_type_of<T: NumberTypedT + Debug>(val: T) -> NumberType {
//     let t = T::number_type();
//     println!("apply_number_type_of T: {:?} with val: {:?}", t, val);
//     t
// }

fn main() {
    println!("commented out.")
    // println!("starting apply_number_type example...");
    // let t = apply_number_type!(NumberType::ClChar, return_number_type_of, [1]);
    // assert_eq!(t, NumberType::ClChar);

    // println!("finished apply_number_type example.");

}
