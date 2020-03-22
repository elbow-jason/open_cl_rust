// use std::fmt::Debug;
// use half::f16;
// use libc::size_t;
// use num_traits::cast::{ToPrimitive, FromPrimitive, NumCast};

// use super::ffi_types::*;
// use super::newtypes::*;
// use super::number_type::{NumberType, NumberTypedT};
// use super::as_slice::AsSlice;
// use crate::Output;
// use crate::traits::{FFIType, ClType, RustType, ToFFIType, ToClType, ToRustType};

// /// An error related to a Device.
// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum NumberConversionError {
//     #[fail(display = "Failed to cast number {} from {:?} to {:?}", _0, _1, _2)]
//     FailedToCast(String, NumberType, NumberType),
// }


// pub trait ConvertTo<T> {
//     fn convert_to(self) -> T;
// }

// impl ToFFIType<cl_bool> for bool {
//     fn to_ffi_type(self) -> cl_bool {
//         match self {
//             true => 1,
//             false => 0,
//         }
//     }
// }


// impl ToRustType<bool> for cl_bool {
//     fn to_rust_type(self) -> bool {
//         match self {
//             0 => false,
//             1 => true,
//             bad => panic!("Invalid cl_bool value {:?}: must be 0 or 1", bad),
//         }
//     }
// }

// impl ToClType<ClBool> for cl_bool {
//     fn to_cl_type(self) -> ClBool {
//         if self.to_rust_type() {
//             ClBool::True
//         } else {
//             ClBool::False
//         }
//     }
// }

// impl ToClType<ClHalf> for cl_half {
//     fn to_cl_type(self) -> ClHalf {
//         ClHalf(self)
//     }
// }


// impl ToRustType<f16> for ClHalf {
//     fn to_rust_type(self) -> f16 {
//         f16::from_bits(self.0)
//     }
// }

// impl ToRustType<f16> for cl_half {
//     fn to_rust_type(self) -> f16 {
//         f16::from_bits(self)
//     }
// }

// impl ToFFIType<cl_half> for f16 {
//     fn to_ffi_type(self) -> cl_half {
//         self.to_bits()
//     }
// }

// impl ToClType<ClHalf> for f16 {
//     fn to_cl_type(self) -> ClHalf {
//         ClHalf(self.to_bits())
//     }
// }


// macro_rules! impl_trait {
//     ($the_trait:ident, $( $t:ty ),*) => {
//         $( 
//             impl $the_trait for $t {}
//         )*
//     }
// }

// macro_rules! impl_array_size { 
//     ($the_trait:ident, $n:expr) => {
//         impl_trait!($the_trait, [u8; $n], [i8; $n], [u16; $n], [i16; $n], [f16; $n], [i32; $n], [u32; $n], [f32; $n], [u64; $n], [i64; $n], [f64; $n]);
//     }
// }


// impl_trait!(RustType, bool, u8, i8, u16, i16, f16);

// impl_array_size!(RustType, 2);
// impl_array_size!(RustType, 3);
// impl_array_size!(RustType, 4);
// impl_array_size!(RustType, 8);
// impl_array_size!(RustType, 16);






// macro_rules! impl_primitive_conversion {
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty) => {

//         impl ToClType<$new_t> for $ffi_t {
//             fn to_cl_type(self) -> $new_t {
//                 $new_t(self)
//             }
//         }

//         impl ToFFIType<$ffi_t> for $rust_t {
//             fn to_ffi_type(self) -> $ffi_t {
//                 self as $ffi_t
//             }
//         }
//     };
// }


// macro_rules! impl_convert_to_for_vector {
    
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty, 3) => {
//         paste::item! {
//             impl ToRustType<[$rust_t; 3]> for [<$ffi_t 3>] {
//                 fn to_rust_type(self) -> [$rust_t; 3] {
//                     let inner = unsafe { self.s };
//                     [inner[0], inner[1], inner[2]]
//                 }
//             }

//             impl ConvertTo<[$rust_t; 3]> for [<$new_t 3>] {
//                 fn convert_to(self) -> [$rust_t; 3] {
//                     let inner = unsafe { self.0.s };
//                     [inner[0], inner[1], inner[2]]
//                 }
//             }

//             impl ConvertTo<[<$new_t 3>]> for [<$ffi_t 3>] {
//                 fn convert_to(self) -> [<$new_t 3>] {
//                     [<$new_t 3>](self)
//                 }
//             }

//             impl ConvertTo<[<$new_t 3>]> for [$rust_t; 3] {
//                 fn convert_to(self) -> [<$new_t 3>] {
//                     [<$new_t 3>](self.convert_to())
//                 }
//             }

//             impl ConvertTo<[<$ffi_t 3>]> for [$rust_t; 3] {
//                 fn convert_to(self) -> [<$ffi_t 3>] {
//                     let mut num = unsafe { std::mem::zeroed::<[<$ffi_t 3>]>() };
//                     let new_inner = [self[0], self[1], self[2], 0 as $ffi_t];
//                     num.s = new_inner;
//                     num
//                 }
//             }

//             impl ConvertTo<[<$ffi_t 3>]> for [<$new_t 3>] {
//                 fn convert_to(self) -> [<$ffi_t 3>] {
//                     self.0
//                 }
//             }
//         }
//     };
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty, $num:expr) => {
//         paste::item! {
//             impl ConvertTo<[$rust_t; $num]> for [<$ffi_t $num>]{
//                 fn convert_to(self) -> [$rust_t; $num] {
//                     unsafe { self.s }
//                 }
//             }

//             impl ConvertTo<[<$ffi_t $num>]> for [$rust_t; $num] {
//                 fn convert_to(self) -> [<$ffi_t $num>] {
//                     let mut num = unsafe { std::mem::zeroed::<[<$ffi_t $num>]>() };
//                     num.s = self;
//                     num
//                 }
//             }

//             impl ConvertTo<[<$new_t $num>]> for [$rust_t; $num] {
//                 fn convert_to(self) -> [<$new_t $num>] {
//                     [<$new_t $num>](self.convert_to())
//                 }
//             }

//             impl ConvertTo<[<$new_t $num>]> for [<$ffi_t $num>] {
//                 fn convert_to(self) -> [<$new_t $num>] {
//                     [<$new_t $num>](self)
//                 }
//             }

//             impl ConvertTo<[<$ffi_t $num>]> for [<$new_t $num>] {
//                 fn convert_to(self) -> [<$ffi_t $num>] {
//                     self.0
//                 }
//             }

//             impl ConvertTo<[$rust_t; $num]> for [<$new_t $num>] {
//                 fn convert_to(self) -> [$rust_t; $num] {
//                     self.0.convert_to()
//                 }
//             }
//         }
//     };
// }

// macro_rules! impl_convert_to_for_all_vectors {
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty) => {
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 2);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 3);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 4);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 8);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 16 );
//     }
// }


// impl_primitive_conversion!(size_t, SizeT, usize);
// impl_primitive_conversion!(cl_double, ClDouble, f64);
// impl_primitive_conversion!(cl_char, ClChar, i8);
// impl_primitive_conversion!(cl_uchar, ClUchar, u8);
// impl_primitive_conversion!(cl_short, ClShort, i16);
// impl_primitive_conversion!(cl_ushort, ClUshort, u16);
// impl_primitive_conversion!(cl_int, ClInt, i32);
// impl_primitive_conversion!(cl_uint, ClUint, u32);
// impl_primitive_conversion!(cl_long, ClLong, i64);
// impl_primitive_conversion!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(cl_float, ClFloat, f32);

// impl_convert_to_for_all_vectors!(cl_char, ClChar, i8);
// impl_convert_to_for_all_vectors!(cl_uchar, ClUchar, u8);
// impl_convert_to_for_all_vectors!(cl_short, ClShort, i16);
// impl_convert_to_for_all_vectors!(cl_ushort, ClUshort, u16);
// impl_convert_to_for_all_vectors!(cl_int, ClInt, i32);
// impl_convert_to_for_all_vectors!(cl_uint, ClUint, u32);
// impl_convert_to_for_all_vectors!(cl_long, ClLong, i64);
// impl_convert_to_for_all_vectors!(cl_ulong, ClUlong, u64);
// impl_convert_to_for_all_vectors!(cl_float, ClFloat, f32);



// pub trait CastNumberTo<T> {
//     fn cast_number_to(&self) -> Output<T>;
// }

// impl<P1, P2> CastNumberTo<P2> for P1 where P1: ClPrimitive, P2: ClPrimitive,  {
//     fn cast_number_to(&self) -> Output<P2> {
//         P2::from(*self).ok_or_else(|| {
//             NumberConversionError::FailedToCast(
//                 format!("{:?}", self),
//                 P1::number_type(),
//                 P2::number_type()
//             ).into()
//         })
//     }
// }

// fn _cast_primitive<T: NumCast + NumberTypedT + Debug, U: NumCast + NumberTypedT>(n: T) -> Output<U> {
//     U::from(n).ok_or_else(|| {
//         NumberConversionError::FailedToCast(
//             format!("{:?}", n),
//             T::number_type(),
//             U::number_type()
//         ).into()
//     })
// }

// pub trait CastVector2<T, O, V> where Self: ClVector2 + AsSlice<T>, V: ClVector2 + ClVector<O> + FFIType, T: NumCast + ToPrimitive + FromPrimitive + ClPrimitive, O: ClPrimitive {
//     fn cast_vector(&self) -> Output<V> {
//         let slc = self.as_slice();
//         let p0: O = _cast_primitive(slc[0])?;
//         let p1: O = _cast_primitive(slc[1])?;
//         Ok([p0, p1])
//     }
// }



// // impl<V1, V2> CastNumberTo<V2> for V1 where V1: ClVector<cl_char> + ClVector2, V2: ClVector<cl_uchar> + ClVector2 {
// //     fn cast_number_to(&self) -> Output<V2> {
// //         V2::from(*self).ok_or_else(|| {
// //             NumberConversionError::FailedToCast(
// //                 format!("{:?}", self),
// //                 V1::number_type(),
// //                 V2::number_type()
// //             ).into()
// //         })
// //     }
// // }


// #[cfg(test)]
// mod tests {
    
//     use super::*;
//     use crate::numbers::Zeroed;
//     // use crate::ffi::*;
//     // use float_cmp::ApproxEq;
    

//     macro_rules! conversion_tests {
//         ($ffi_t:ty, $new_t:ty, $rust_t:ty, $num:expr) => {
//             paste::item! {
//                 // PARTIAL_EQ IS NOT WORKING.
//                 // #[allow(non_snake_case)]
//                 // #[test]
//                 // fn [<convert_from_ $t _to_ $new_t>]() {
//                 //     let num: $t = $num;
//                 //     let new_num: $new_t = num.convert_to();
//                 //     let expected = $new_t($num);
//                 //     assert_eq!(new_num, expected);
//                 // }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<to_ffi_type_for_conversion_from_ $new_t _to_ $ffi_t>]() {
                    
//                     let new_num: $new_t = $new_t($num);
//                     let num: $ffi_t = new_num.to_ffi_type();
//                     assert_eq!(num, $num);
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<to_rust_type_for_conversion_from_ $new_t _to_ $rust_t>]() {
                    
//                     let new_num: $new_t = $new_t($num);
//                     let num: $rust_t = new_num.to_rust_type();
//                     assert_eq!(num, $num);
//                 }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $rust_t _to_ $new_t>]() {
                    
//             //         let new_num: $new_t = $new_t($num);
//             //         let num: $t = new_num.convert_to();
//             //         assert_eq!(num, $num);
//             //     }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $rust_t _to_ $t>]() {
//             //         let num1: $rust_t = $num;d
//             //         let converted: $t = num1.convert_to();
//             //         let expected: $t = $num;
//             //         assert_eq!(converted, expected);
//             //     }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $t _to_ $rust_t>]() {
//             //         let num1: $rust_t = $num;
//             //         let converted: $t = num1.convert_to();
//             //         let expected: $t = $num;
//             //         assert_eq!(converted, expected);
//             //     }
//             }
//         };
//     }

//     macro_rules! conversion_tests_for_float {
//         ($t:ident, $new_t:ident, $rust_t:ident, $num:expr) => {
//             paste::item! {
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $new_t>]() {
//                     let num: $t = $num;
//                     let new_num: $new_t = num.convert_to();
//                     assert!(approx_eq!($rust_t, new_num.0, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $new_t _to_ $t>]() {
                    
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $new_t _to_ $rust_t>]() {
                    
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $rust_t _to_ $new_t>]() {
                    
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $rust_t _to_ $t>]() {
//                     let num1: $rust_t = $num;
//                     let converted: $t = num1.convert_to();
//                     let expected: $t = $num;
//                     assert!(approx_eq!($rust_t, converted, expected));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $rust_t>]() {
//                     let num1: $rust_t = $num;
//                     let converted: $t = num1.convert_to();
//                     let expected: $t = $num;
//                     assert!(approx_eq!($rust_t, converted, expected));
//                 }
//             }
//         };
//     }

//     // conversion_tests!(cl_uchar, ClUchar, u8, 3);
//     // conversion_tests!(cl_char, ClChar, i8, 3);
//     // conversion_tests!(cl_ushort, ClUshort, u16, 3);
//     // conversion_tests!(cl_short, ClShort, i16, 3);
//     // conversion_tests!(cl_int, ClInt, i32, 3);
//     // conversion_tests!(cl_uint, ClUint, u32, 3);
//     // conversion_tests!(cl_long, ClLong, i64, 3);
//     // conversion_tests!(cl_ulong, ClUlong, u64, 3);
//     // conversion_tests!(size_t, SizeT, usize, 3);

//     // conversion_tests_for_float!(cl_float, ClFloat, f32, 3.0);
//     // conversion_tests_for_float!(cl_double, ClDouble, f64, 3.0);
    
//     macro_rules! conversion_tests_for_vector {
//         ($t:ident, $new_t:ident, [$rust_t:ty; $vector_size:expr], $num:expr) => {
//             paste::item! {
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $new_t>]() {
//                     let num = $t::zeroed();
//                     let new_num: $new_t = num.convert_to();
//                     assert_eq!(new_num, $new_t(num));
//                 }
//             }
//         };
//     }

//     conversion_tests_for_vector!(cl_uchar2, ClUchar2, [u8; 2], [3, 4]);

//     fn three_f16() -> f16 {
//         f16::from_f32(3.0)
//     }

//     fn three_half() -> cl_half {
//         three_f16().to_bits()
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_cl_half_to_ClHalf() {
//         let new_num: ClHalf = three_half().to_cl_type();
//         assert_eq!(new_num, ClHalf(three_half()));
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_ClHalf_to_cl_half() {
//         let num2: cl_half = ClHalf(three_half()).to_ffi_type();
//         assert_eq!(num2, three_half());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_ClHalf_to_f16() {
//         let num: f16 = ClHalf(three_half()).to_rust_type();
//         assert_eq!(num, three_f16());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_f16_to_ClHalf() {
//         let num: ClHalf = three_f16().to_cl_type();
//         assert_eq!(num, ClHalf(three_half()));
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_f16_to_cl_half() {
//         let num: cl_half = three_f16().to_ffi_type();
//         assert_eq!(num, three_half());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_cl_half_to_f16() {
//         let got: f16 = three_half().to_rust_type();
//         assert_eq!(got, three_f16());
//     }
// }