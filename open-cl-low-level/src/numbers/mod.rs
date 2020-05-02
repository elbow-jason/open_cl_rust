// #[macro_use]
pub mod number_type;
pub use number_type::*;

// pub mod traits;
// pub use traits::*;

// pub mod trait_impls;
// pub use trait_impls::*;

// pub mod cl_number;
// pub use cl_number::*;

// pub mod casting;
// pub use casting::*;


// pub mod as_slice;
// pub use as_slice::*;

// pub mod float16;
// pub use float16::*;

pub mod as_ptr;
pub use as_ptr::*;

pub mod newtypes;
pub use newtypes::*;

pub trait Number: NumberNewType {} // + NumberTyped

impl<T> Number for T where T: NumberNewType {}