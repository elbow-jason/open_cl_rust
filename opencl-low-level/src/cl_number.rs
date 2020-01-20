use std::fmt::Debug;

use num_complex::{Complex32, Complex64};

// NOTE: f32 does not implement Eq so it's not here. WHYEEEEE...
pub unsafe trait ClNumber: Sized + Clone + Copy + Send + Sync + PartialEq + Debug + 'static + Default {}

//8
unsafe impl ClNumber for u8 {}
unsafe impl ClNumber for i8 {}
//16
unsafe impl ClNumber for u16 {}
unsafe impl ClNumber for i16 {}
//32
unsafe impl ClNumber for u32 {}
unsafe impl ClNumber for i32 {}
unsafe impl ClNumber for f32 {}
unsafe impl ClNumber for Complex32 {}
//64
unsafe impl ClNumber for u64 {}
unsafe impl ClNumber for i64 {}
unsafe impl ClNumber for f64 {}
unsafe impl ClNumber for Complex64 {}
//size
unsafe impl ClNumber for isize {}
unsafe impl ClNumber for usize {}
