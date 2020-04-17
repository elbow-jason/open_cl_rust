use super::float16::F16;
use super::traits::{ClFrom, ClInto};
use super::traits::{ClNewNum, ClNum, ClPrimitive, ClRustNum, ClRustPrimitiveNum};
use super::traits::{ClVector16, ClVector2, ClVector3, ClVector4, ClVector8};
use super::traits::{FFINumber, Number, Zeroed};
use crate::ffi::*;
use crate::kernel::KernelArg;
use crate::{ClNewType, ClType, NumberType, NumberTypedT, RustType};
use libc::size_t;
use std::fmt;

impl<T> ClType for T where T: ClNum {}
impl<T> RustType for T where T: ClRustNum {}
impl<T> ClNewType for T where T: ClNewNum {}

// impl<T> FFINumber for T where T: ClNum + KernelArg {}
unsafe impl<T> FFINumber for T where T: ClNewNum + KernelArg {}

macro_rules! zeroed_vector {
    ($t:ty) => {
        unsafe { std::mem::zeroed::<$t>() }
    };
}

impl<S, T> ClInto<T> for S
where
    T: ClFrom<S>,
{
    fn cl_into(self) -> T {
        T::cl_from(self)
    }
}

// impl<S, T> ClFrom<T> for S
// where
//     S: From<T>,
// {
//     fn cl_from(val: T) -> S {
//         From::from(val)
//     }
// }

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

macro_rules! __impl_primitive_froms {
    ($cl_type:ty, $new_type:ident, $rust_type:ty) => {
        impl ClFrom<$new_type> for $rust_type {
            fn cl_from(num: $new_type) -> $rust_type {
                num.0
            }
        }

        impl ClFrom<$rust_type> for $new_type {
            fn cl_from(num: $rust_type) -> $new_type {
                $new_type(num)
            }
        }
    };
}

macro_rules! __impl_newtype {
    ($new_type:ident, $cl_type:ty) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
        #[repr(transparent)]
        pub struct $new_type(pub $cl_type);
    };
}

macro_rules! __impl_newtype_float {
    ($new_type:ident, $cl_type:ty) => {
        #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
        #[repr(transparent)]
        pub struct $new_type(pub $cl_type);
    };
}

macro_rules! __impl_newtype_funcs {
    ($new_type:ident, $cl_type:ty) => {
        impl $new_type {
            pub fn from_cl_num(val: $cl_type) -> $new_type {
                $new_type(val)
            }

            pub fn to_inner(self) -> $cl_type {
                self.0
            }

            pub fn inner(&self) -> $cl_type {
                self.0
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
            cl_zero: $cl_zero:expr,
        }
    ) => {
        __impl_newtype!($NewType, $cl_type);
        impl ClRustPrimitiveNum for $cl_type {}
        impl ClPrimitive for $cl_type {}
        __impl_newtype_funcs!($NewType, $cl_type);

        __impl_number_traits_aliased!($cl_type, $NewType);

        __number_typed_t!($NewType, [$cl_type, $NewType]);
        __impl_zeroed!($cl_type => $cl_zero);
        __impl_zeroed!($NewType => $NewType($cl_zero));
        __impl_primitive_froms!($cl_type, $NewType, $cl_type);
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
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
        NumberType::Bool
    }
}

impl NumberTypedT for bool {
    fn number_type() -> NumberType {
        NumberType::Bool
    }
}

__impl_zeroed!(ClBool => ClBool(0u32));

impl ClFrom<bool> for ClBool {
    fn cl_from(val: bool) -> ClBool {
        match val {
            false => ClBool(0),
            true => ClBool(1),
        }
    }
}

impl ClFrom<ClBool> for bool {
    fn cl_from(val: ClBool) -> bool {
        match val {
            ClBool(0) => false,
            ClBool(1) => true,
            ClBool(invalid) => panic!("Invalid ClBool: {:?}", invalid),
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
#[repr(transparent)]
pub struct ClHalf(pub cl_half);

impl fmt::Debug for ClHalf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClHalf({:?})", F16::from_u16(self.0))
    }
}
__impl_zeroed!(F16 => F16::from_u16(0u16));
__impl_zeroed!(ClHalf => ClHalf(0u16));
__impl_newtype_funcs!(ClHalf, cl_half);

impl Number for ClHalf {}
impl Number for F16 {}

impl ClNewNum for ClHalf {}
impl ClRustNum for F16 {}

__number_typed_t!(ClHalf, [ClHalf, F16]);

impl ClFrom<F16> for ClHalf {
    fn cl_from(val: F16) -> ClHalf {
        ClHalf(val.0)
    }
}

impl ClFrom<f32> for ClHalf {
    fn cl_from(val: f32) -> ClHalf {
        ClHalf(F16::from_f32(val).0)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ClFloat(pub cl_float);

impl Number for ClFloat {}
impl Number for cl_float {}

impl ClPrimitive for cl_float {}
impl ClNum for cl_float {}
impl ClNewNum for ClFloat {}
impl ClRustNum for f32 {}
impl ClRustPrimitiveNum for f32 {}

__impl_zeroed!(ClFloat => ClFloat(0.0f32));
__impl_zeroed!(cl_float => 0.0f32);
__impl_newtype_funcs!(ClFloat, cl_float);
__number_typed_t!(ClFloat, [ClFloat, cl_float]);
__impl_primitive_froms!(cl_float, ClFloat, f32);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ClDouble(pub cl_double);

impl Number for ClDouble {}
impl Number for cl_double {}

impl ClNum for cl_double {}
impl ClNewNum for ClDouble {}
impl ClRustNum for f64 {}
impl ClRustPrimitiveNum for f64 {}

__impl_zeroed!(ClDouble => ClDouble(0.0f64));
__impl_zeroed!(cl_double => 0.0f64);
__impl_newtype_funcs!(ClDouble, cl_double);
__number_typed_t!(ClDouble, [ClDouble, cl_double]);
__impl_primitive_froms!(cl_double, ClDouble, f64);

defnumber! {
    pub struct SizeT: size_t {
        cl_zero: 0usize,
    }
}

macro_rules! __impl_newtype_vector {
     ($cl_base:ident, $cl_type:ty, $NewType:ident, [$rust_t:ty; 3], $rust_zeroed:expr) => {
        #[derive(Clone, Copy)]
        #[repr(transparent)]
        pub struct $NewType(pub $cl_type);

        impl ClVector3<$cl_base> for $cl_type {}

        impl fmt::Debug for $NewType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let val4: [$rust_t; 4] = unsafe { self.0.s };
                let val3: [$rust_t; 3] = [val4[0], val4[1], val4[2]];
                write!(f, "{:?}({:?})", stringify!($NewType), val3)
            }
        }

        unsafe impl Send for $NewType {}
        unsafe impl Sync for $NewType {}

        __number_typed_t!($NewType, [$NewType, [$rust_t; 3]]);
        __impl_zeroed!($NewType => $NewType(zeroed_vector!($cl_type)));
        __impl_zeroed!([$rust_t; 3] => [$rust_zeroed; 3]);


        impl Number for $NewType {}
        impl Number for [$rust_t; 3] {}

        impl ClNewNum for $NewType {}
        impl ClRustNum for [$rust_t; 3] {}

        impl ClFrom<$NewType> for $cl_type {
            fn cl_from(val: $NewType) -> $cl_type {
                val.0
            }
        }

        impl ClFrom<$cl_type> for $NewType {
            fn cl_from(val: $cl_type) -> $NewType {
                $NewType(val)
            }
        }

        impl ClFrom<[$rust_t; 3]> for $NewType {
            fn cl_from(val: [$rust_t; 3]) -> $NewType {
                let mut cl_val: $cl_type = zeroed_vector!($cl_type);
                let inner_s = [val[0], val[1], val[2], $rust_zeroed];
                cl_val.s = inner_s;
                $NewType(cl_val)
            }
        }

        impl ClFrom<$NewType> for [$rust_t; 3] {
            fn cl_from(val: $NewType) -> [$rust_t; 3] {
                let s = unsafe{ val.0.s };
                [s[0], s[1], s[2]]
            }
        }

    };
    ($cl_base:ident, $cl_type:ty, $NewType:ident, [$rust_t:ty; $size:expr], $rust_zeroed:expr) => {
        #[derive(Clone, Copy)]
        pub struct $NewType(pub $cl_type);

        impl fmt::Debug for $NewType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let rust_val: [$rust_t; $size] = ClFrom::cl_from(*self);
                write!(f, "{:?}({:?})", stringify!($NewType), rust_val)
            }
        }

        // impl InnerMutRef<[$cl_base; $size]> for $cl_type {
        //     fn inner_mut_ref(&mut self) -> &mut [$cl_base; $size] {
        //         unsafe { &mut self.s }
        //     }
        // }

        unsafe impl Send for $NewType {}
        unsafe impl Sync for $NewType {}

        paste::item! {
            impl [<ClVector $size>]<$cl_base> for $cl_type {}
        }

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

        impl ClFrom<$NewType> for $cl_type {
            fn cl_from(val: $NewType) -> $cl_type {
                val.0
            }
        }

        impl ClFrom<$cl_type> for $NewType {
            fn cl_from(val: $cl_type) -> $NewType {
                $NewType(val)
            }
        }

        impl ClFrom<$NewType> for [$rust_t; $size] {
            fn cl_from(val: $NewType) -> [$rust_t; $size] {
                unsafe { val.0.s }
            }
        }

        impl ClFrom<[$rust_t; $size]> for $cl_type {
            fn cl_from(val: [$rust_t; $size]) -> $cl_type {
                let mut cl_val = zeroed_vector!($cl_type);
                cl_val.s = val;
                cl_val
            }
        }

        impl ClFrom<[$rust_t; $size]> for $NewType {
            fn cl_from(val: [$rust_t; $size]) -> $NewType {
                $NewType(ClFrom::cl_from(val))
            }
        }
    };
}
macro_rules! impl_all_vectors {
    ($cl_base:ident, $new_base:ident, $rust_t:ty, $zero:expr) => {
        paste::item! {
            __impl_newtype_vector!($cl_base, [<$cl_base 2>], [<$new_base 2>], [$rust_t; 2], $zero);
            __impl_newtype_vector!($cl_base, [<$cl_base 3>], [<$new_base 3>], [$rust_t; 3], $zero);
            __impl_newtype_vector!($cl_base, [<$cl_base 4>], [<$new_base 4>], [$rust_t; 4], $zero);
            __impl_newtype_vector!($cl_base, [<$cl_base 8>], [<$new_base 8>], [$rust_t; 8], $zero);
            __impl_newtype_vector!($cl_base, [<$cl_base 16>], [<$new_base 16>], [$rust_t; 16], $zero);
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
