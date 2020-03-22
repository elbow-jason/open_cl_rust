use super::traits::{ClNewNum, ClNum, ClRustNum, NumChange, Number, Zeroed};
use crate::ffi::*;
use crate::{ClNewType, ClType, NumberType, NumberTypedT, RustType};
use half::f16;
use libc::size_t;
use std::fmt;

impl<T> ClType for T where T: ClNum {}
impl<T> RustType for T where T: ClRustNum {}
impl<T> ClNewType for T where T: ClNewNum {}

macro_rules! zeroed_vector {
    ($t:ty) => {
        unsafe { std::mem::zeroed::<$t>() }
    };
}

macro_rules! __impl_zeroed_vector {
    ($t:ty) => {
        impl Zeroed for $t {
            fn zeroed() -> $t {
                zeroed_vector!($t)
            }
        }
    };
}

macro_rules! __impl_zeroed_newtype_vector {
    ($NewType:ty, $t:ty) => {
        impl Zeroed for $t {
            fn zeroed() -> $t {
                $NewType(unsafe { std::mem::zeroed::<$t>() })
            }
        }
    };
}

macro_rules! __impl_zeroed {
    ($t:ty => $zero:expr) => {
        impl Zeroed for $t {
            fn zeroed() -> $t {
                $zero
            }
        }
    };
}

macro_rules! __number_typed_t {
    ($num_tag:ident, [ $( $t:ty ),* ]) => {
        $(
            impl NumberTypedT for $t {
                fn number_type() -> NumberType {
                    NumberType::$num_tag
                }
            }
        )*
    }
}

macro_rules! __impl_num_change_for_cl_type_primitive {
    ($cl_type:ty, $new_type:ident, $rust_type:ty) => {
        impl NumChange for $cl_type {
            type ClNum = $cl_type;
            type NewNum = $new_type;
            type RustNum = $rust_type;

            fn to_cl_num(self) -> Self::ClNum {
                self
            }

            fn to_new_num(self) -> Self::NewNum {
                $new_type(self)
            }

            fn to_rust_num(self) -> Self::RustNum {
                self
            }
        }
    };
}

macro_rules! __impl_num_change_for_new_type_primitive {
    ($cl_type:ty, $new_type:ident, $rust_type:ty) => {
        impl NumChange for $new_type {
            type ClNum = $cl_type;
            type NewNum = $new_type;
            type RustNum = $rust_type;

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                self.to_inner()
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                self
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                self.to_inner()
            }
        }
    };
}

macro_rules! __impl_newtype {
    ($new_type:ident, $cl_type:ty) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
        pub struct $new_type(pub $cl_type);
    };
}

macro_rules! __impl_newtype_float {
    ($new_type:ident, $cl_type:ty) => {
        #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
        pub struct $new_type(pub $cl_type);
    };
}

macro_rules! __impl_newtype_funcs {
    ($new_type:ident, $cl_type:ty) => {
        impl $new_type {
            pub fn from_cl(val: $cl_type) -> $new_type {
                $new_type(val)
            }

            pub fn to_inner(self) -> $cl_type {
                self.0
            }

            pub fn inner(&self) -> $cl_type {
                self.0
            }

            pub fn inner_mut_ref(&mut self) -> &$cl_type {
                &mut self.0
            }
        }
    };
}
macro_rules! __impl_number_traits_aliased {
    ($cl_type:ty, $new_type:ident) => {
        impl Number for $cl_type {}
        impl Number for $new_type {}

        impl ClNum for $cl_type {}
        impl ClNewNum for $new_type {}
        impl ClRustNum for $cl_type {}
    };
}

macro_rules! __impl_number_traits {
    ($cl_type:ident, $new_type:ident, $rust_type:ident) => {
        impl Number for $cl_type {}
        impl Number for $new_type {}
        impl Number for $rust_type {}

        impl ClNum for $cl_type {}
        impl ClNewNum for $new_type {}
        impl ClRustNum for $rust_type {}
    };
}

macro_rules! __impl_zeroed_aliased {
    (true_:ident, $new_type:ident, $cl_type:ident => $cl_zero:expr, _ => _) => {};
}
macro_rules! __impl_zeroed_aliased {
    (false_:ident, $new_type:ident, $cl_type:ident => $cl_zero:expr, $rust_type:ident => $rust_zero:expr) => {};
}

macro_rules! defnumber {
    (
        pub struct $NewType:ident: $cl_type:ty {
            // rust_type: $rust_type:ty,
            cl_zero: $cl_zero:expr,
        }
    ) => {
        __impl_newtype!($NewType, $cl_type);
        __impl_newtype_funcs!($NewType, $cl_type);

        __impl_number_traits_aliased!($cl_type, $NewType);

        __number_typed_t!($NewType, [$cl_type, $NewType]);
        __impl_zeroed!($cl_type => $cl_zero);
        __impl_zeroed!($NewType => $NewType($cl_zero));
        __impl_num_change_for_cl_type_primitive!($cl_type, $NewType, $cl_type);
        __impl_num_change_for_new_type_primitive!($cl_type, $NewType, $cl_type);
    };

    (
        pub struct $NewType:ident: $cl_type:ty | $rust_type:ty {
            cl_zero: $cl_zero:expr,
            rust_zero: $rust_zero:expr,
        }
    ) => {
        __impl_newtype!($NewType, $cl_type);
        __impl_newtype_funcs!($NewType, $cl_type);

        __impl_number_traits_aliased!($cl_type, $NewType);

        __number_typed_t!($NewType, [$cl_type, $NewType]);
        __impl_zeroed!($cl_type => $cl_zero);
        __impl_zeroed!($NewType => $NewType($cl_zero));
        __impl_num_change_for_cl_type_primitive!($cl_type, $NewType, $rust_type);
        __impl_num_change_for_new_type_primitive!($cl_type, $NewType, $rust_type);
    }
}

// macro_rules! deffloat {
//     (
//         $(#[$outer:meta])*
//         pub struct $NewType:ident: $cl_type:ty {
//             // rust_type: $rust_type:ty,
//             cl_zero: $cl_zero:expr,
//         }
//     ) => {
//         __impl_newtype_float!($NewType, $cl_type);
//         __impl_newtype_funcs!($NewType, $cl_type);

//         __impl_number_traits_aliased!($cl_type, $NewType);

//         __number_typed_t!($NewType, [$cl_type, $NewType]);
//         __impl_zeroed!($cl_type => $cl_zero);
//         __impl_zeroed!($NewType => $NewType($cl_zero));
//         __impl_num_change_for_cl_type_primitive!($cl_type, $NewType, $cl_type);
//         __impl_num_change_for_new_type_primitive!($cl_type, $NewType, $cl_type);
//     };

//     (
//         $(#[$outer:meta])*
//         pub struct $NewType:ident: $cl_type:ty | $rust_type:ty {
//             cl_zero: $cl_zero:expr,
//             rust_zero: $rust_zero:expr,
//         }
//     ) => {
//         __impl_newtype_float!($NewType, $cl_type, [$( $outer ),*]);
//         __impl_newtype_funcs!($NewType, $cl_type);

//         __impl_number_traits_aliased!($cl_type, $NewType);

//         __number_typed_t!($NewType, [$cl_type, $NewType]);
//         __impl_zeroed!($cl_type => $cl_zero);
//         __impl_zeroed!($NewType => $NewType($cl_zero));
//         __impl_num_change_for_cl_type_primitive!($cl_type, $NewType, $rust_type);
//         __impl_num_change_for_new_type_primitive!($cl_type, $NewType, $rust_type);
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClBool(pub cl_bool);
// __impl_newtype!(ClBool, cl_bool, []);
__impl_newtype_funcs!(ClBool, cl_bool);
__impl_zeroed!(bool => false);

impl Number for ClBool {}
impl Number for bool {}

impl ClNewNum for ClBool {}
impl ClRustNum for bool {}

impl NumberTypedT for ClBool {
    fn number_type() -> NumberType {
        NumberType::ClUint
    }
}

impl NumberTypedT for bool {
    fn number_type() -> NumberType {
        NumberType::ClUint
    }
}

__impl_zeroed!(ClBool => ClBool(0u32));

impl NumChange for ClBool {
    type ClNum = cl_bool;
    type NewNum = ClBool;
    type RustNum = bool;

    fn to_cl_num(self) -> <Self as NumChange>::ClNum {
        self.to_inner()
    }

    fn to_new_num(self) -> <Self as NumChange>::NewNum {
        self
    }

    fn to_rust_num(self) -> <Self as NumChange>::RustNum {
        match self.to_inner() {
            0 => false,
            1 => true,
            n => panic!("Invalid cl_bool value: {:?}", n),
        }
    }
}

defnumber! {
    pub struct ClUchar: cl_uchar {
        cl_zero: 0u8,
    }
}

defnumber! {
    pub struct ClChar: cl_char {
        cl_zero: 0i8,
    }
}

defnumber! {
    pub struct ClUshort: cl_ushort {
        cl_zero: 0u16,
    }
}

defnumber! {
    pub struct ClShort: cl_short {
        cl_zero: 0i16,
    }
}

defnumber! {
    pub struct ClUint: cl_uint {
        cl_zero: 0u32,
    }
}

defnumber! {
    pub struct ClInt: cl_int {
        cl_zero: 0i32,
    }
}

defnumber! {
    pub struct ClUlong: cl_ulong {
        cl_zero: 0u64,
    }
}

defnumber! {
    pub struct ClLong: cl_long {
        cl_zero: 0i64,
    }
}

#[derive(Clone, Copy)]
pub struct ClHalf(pub cl_half);

impl fmt::Debug for ClHalf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClHalf({:?})", f16::from_bits(self.0))
    }
}
__impl_zeroed!(f16 => f16::from(0u8));
__impl_zeroed!(ClHalf => ClHalf(0u16));
__impl_newtype_funcs!(ClHalf, cl_half);

impl Number for ClHalf {}
impl Number for f16 {}

impl ClNewNum for ClHalf {}
impl ClRustNum for f16 {}
__number_typed_t!(ClHalf, [ClHalf, f16]);

impl NumChange for ClHalf {
    type ClNum = cl_half;
    type NewNum = ClHalf;
    type RustNum = f16;

    fn to_cl_num(self) -> cl_half {
        self.0
    }

    fn to_new_num(self) -> ClHalf {
        self
    }

    fn to_rust_num(self) -> f16 {
        f16::from_bits(self.0)
    }
}

impl NumChange for f16 {
    type ClNum = cl_half;
    type NewNum = ClHalf;
    type RustNum = f16;

    fn to_cl_num(self) -> <Self as NumChange>::ClNum {
        self.to_bits()
    }

    fn to_new_num(self) -> <Self as NumChange>::NewNum {
        ClHalf(self.to_bits())
    }

    fn to_rust_num(self) -> <Self as NumChange>::RustNum {
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClFloat(pub cl_float);

__impl_zeroed!(ClFloat => ClFloat(0.0f32));
__impl_zeroed!(cl_float => 0.0f32);
__impl_newtype_funcs!(ClFloat, cl_float);
__number_typed_t!(ClFloat, [ClFloat, cl_float]);

#[derive(Clone, Copy, Debug)]
pub struct ClDouble(pub cl_double);

__impl_zeroed!(ClDouble => ClDouble(0.0f64));
__impl_zeroed!(cl_double => 0.0f64);
__impl_newtype_funcs!(ClDouble, cl_double);
__number_typed_t!(ClDouble, [ClDouble, cl_double]);

defnumber! {
    pub struct SizeT: size_t {
        cl_zero: 0usize,
    }
}
// #[derive(Clone, Copy)]
// pub struct ClUchar2(pub cl_uchar2);

// unsafe impl Send for ClUchar2 {}
// unsafe impl Sync for ClUchar2 {}

// // __number_typed_t!()
// __number_typed_t!(ClUchar2, [cl_uchar2, ClUchar2, [u8; 2]]);
// __impl_zeroed_vector!(cl_uchar2);
// __impl_zeroed!(ClUchar2 => ClUchar2(cl_uchar2::zeroed()));
// __impl_zeroed!([u8; 2] => [0u8, 0u8]);

// impl Number for cl_uchar2 {}
// impl Number for ClUchar2 {}
// impl Number for [u8; 2] {}

// impl ClNum for cl_uchar2 {}
// impl ClNewNum for ClUchar2 {}
// impl ClRustNum for [u8; 2] {}

// impl NumChange for ClUchar2 {
//     type ClNum = cl_uchar2;
//     type NewNum = ClUchar2;
//     type RustNum = [u8; 2];

//     fn to_cl_num(self) -> <Self as NumChange>::ClNum {
//         self.0
//     }

//     fn to_new_num(self) -> <Self as NumChange>::NewNum {
//         self
//     }

//     fn to_rust_num(self) -> <Self as NumChange>::RustNum {
//         self.0.s
//     }
// }

// impl NumChange for cl_uchar2 {
//     type ClNum = cl_uchar2;
//     type NewNum = ClUchar2;
//     type RustNum = [u8; 2];

//     fn to_cl_num(self) -> <Self as NumChange>::ClNum {
//         self
//     }

//     fn to_new_num(self) -> <Self as NumChange>::NewNum {
//         ClUchar2(self)
//     }

//     fn to_rust_num(self) -> <Self as NumChange>::RustNum {
//         self.s
//     }
// }

// impl NumChange for [u8; 2] {
//     type ClNum = cl_uchar2;
//     type NewNum = ClUchar2;
//     type RustNum = [u8; 2];

//     fn to_cl_num(self) -> <Self as NumChange>::ClNum {
//         let mut cl_val = cl_uchar2::zeroed();
//         cl_val.s = self;
//         cl_val
//     }

//     fn to_new_num(self) -> <Self as NumChange>::NewNum {
//         ClUchar2(self.to_cl_num())
//     }

//     fn to_rust_num(self) -> <Self as NumChange>::RustNum {
//         self
//     }
// }

macro_rules! __impl_newtype_vector {
     ($cl_type:ty, $NewType:ident, [$rust_t:ty; 3], $rust_zeroed:expr) => {
        #[derive(Clone, Copy)]
        pub struct $NewType(pub $cl_type);

        impl fmt::Debug for $NewType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}({:?})", stringify!($NewType), self.to_rust_num())
            }
        }

        unsafe impl Send for $NewType {}
        unsafe impl Sync for $NewType {}
        // impl fmt::Debug for ClHalf {
        //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //         write!(f, "ClHalf{{{:?}}}", f16::from_bits(self.0))
        //     }
        // }
        // __number_typed_t!()
        __number_typed_t!($NewType, [$NewType, [$rust_t; 3]]);
        // __impl_zeroed_vector!($cl_type);
        __impl_zeroed!($NewType => $NewType(zeroed_vector!($cl_type)));
        __impl_zeroed!([$rust_t; 3] => [$rust_zeroed; 3]);


        impl Number for $NewType {}
        impl Number for [$rust_t; 3] {}

        impl ClNewNum for $NewType {}
        impl ClRustNum for [$rust_t; 3] {}

        impl NumChange for $NewType {
            type ClNum = $cl_type;
            type NewNum = $NewType;
            type RustNum = [$rust_t; 3];

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                self.0
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                self
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                let val = unsafe{ self.0.s };
                [val[0], val[1], val[2]]
            }
        }

        impl NumChange for [$rust_t; 3] {
            type ClNum = $cl_type;
            type NewNum = $NewType;
            type RustNum = [$rust_t; 3];

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                let mut cl_val: $cl_type = zeroed_vector!($cl_type);
                let inner_s = [self[0], self[1], self[2], $rust_zeroed];
                cl_val.s = inner_s;
                cl_val
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                $NewType(self.to_cl_num())
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                self
            }
        }

    };
    ($cl_type:ty, $NewType:ident, [$rust_t:ty; $size:expr], $rust_zeroed:expr) => {
        #[derive(Clone, Copy)]
        pub struct $NewType(pub $cl_type);

        impl fmt::Debug for $NewType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}({:?})", stringify!($NewType), self.to_rust_num())
            }
        }

        unsafe impl Send for $NewType {}
        unsafe impl Sync for $NewType {}
        // impl fmt::Debug for ClHalf {
        //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //         write!(f, "ClHalf{{{:?}}}", f16::from_bits(self.0))
        //     }
        // }
        // __number_typed_t!()
        __number_typed_t!($NewType, [$cl_type, $NewType, [$rust_t; $size]]);
        __impl_zeroed_vector!($cl_type);
        __impl_zeroed!($NewType => $NewType(zeroed_vector!($cl_type)));
        __impl_zeroed!([$rust_t; $size] => [$rust_zeroed; $size]);

        impl Number for $cl_type {}
        impl Number for $NewType {}
        impl Number for [$rust_t; $size] {}

        impl ClNum for $cl_type {}
        impl ClNewNum for $NewType {}
        impl ClRustNum for [$rust_t; $size] {}

        impl NumChange for $NewType {
            type ClNum = $cl_type;
            type NewNum = $NewType;
            type RustNum = [$rust_t; $size];

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                self.0
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                self
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                unsafe { self.0.s }
            }
        }

        impl NumChange for $cl_type {
            type ClNum = $cl_type;
            type NewNum = $NewType;
            type RustNum = [$rust_t; $size];

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                self
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                $NewType(self)
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                unsafe { self.s }
            }
        }

        impl NumChange for [$rust_t; $size] {
            type ClNum = $cl_type;
            type NewNum = $NewType;
            type RustNum = [$rust_t; $size];

            fn to_cl_num(self) -> <Self as NumChange>::ClNum {
                let mut cl_val = zeroed_vector!($cl_type);
                cl_val.s = self;
                cl_val
            }

            fn to_new_num(self) -> <Self as NumChange>::NewNum {
                $NewType(self.to_cl_num())
            }

            fn to_rust_num(self) -> <Self as NumChange>::RustNum {
                self
            }
        }
    };
}
macro_rules! impl_all_vectors {
    ($cl_base:ident, $new_base:ident, $rust_t:ty, $zero:expr) => {
        paste::item! {
            __impl_newtype_vector!([<$cl_base 2>], [<$new_base 2>], [$rust_t; 2], $zero);
            __impl_newtype_vector!([<$cl_base 3>], [<$new_base 3>], [$rust_t; 3], $zero);
            __impl_newtype_vector!([<$cl_base 4>], [<$new_base 4>], [$rust_t; 4], $zero);
            __impl_newtype_vector!([<$cl_base 8>], [<$new_base 8>], [$rust_t; 8], $zero);
            __impl_newtype_vector!([<$cl_base 16>], [<$new_base 16>], [$rust_t; 16], $zero);
        }
    };
}

impl_all_vectors!(cl_uchar, ClUchar, u8, 0u8);
impl_all_vectors!(cl_char, ClChar, i8, 0i8);
impl_all_vectors!(cl_ushort, ClUshort, u16, 0u16);
impl_all_vectors!(cl_short, ClShort, i16, 0i16);
impl_all_vectors!(cl_uint, ClUint, u32, 0u32);
impl_all_vectors!(cl_int, ClInt, i32, 0i32);
impl_all_vectors!(cl_ulong, ClUlong, u64, 0u64);
impl_all_vectors!(cl_long, ClLong, i64, 0i64);
impl_all_vectors!(cl_float, ClFloat, f32, 0.0f32);

// AVAILABLE WITH EXTENSIONS ONLY.
// impl_all_vectors!(cl_double, ClDouble, f64, 0.0f64);
// impl_all_vectors!(cl_half, ClHalf, f64, 0.0f64);
