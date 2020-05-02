use crate::Number;
use std::any;
use std::fmt;
use std::cmp;

use thiserror::Error;

#[derive(Clone, Copy, Hash)]
pub struct NumberType {
    type_id: any::TypeId,
    type_name: &'static str,
    size_of: usize,
}

impl NumberType {
    pub fn new<T: any::Any + Number>() -> NumberType {
        NumberType{
            type_id: any::TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
            size_of: std::mem::size_of::<T>(),
        }
    }
}

impl NumberTyped for NumberType {
    fn number_type(&self) -> NumberType {
        (*self).clone()
    }
}

impl cmp::PartialEq for NumberType {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl cmp::Eq for NumberType {}

impl fmt::Debug for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NumberType({}, {:?})", self.type_name, self.type_id)
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum NumberTypeError {
    #[error("Number Type Mismatch - {0:?} vs {1:?}")]
    Mismatch(NumberType, NumberType),
}

pub trait NumberTypedT {
    fn number_type() -> NumberType;

    fn matches(other: NumberType) -> bool {
        Self::number_type() == other
    }
    fn match_or_panic(other: NumberType) {
        _match_or_panic(Self::number_type(), other);
    }

    fn type_name() -> &'static str {
        Self::number_type().type_name
    }

    fn size_of() -> usize {
        Self::number_type().size_of
    }
}

pub trait NumberTyped {
    fn number_type(&self) -> NumberType;

    fn matches(&self, other: NumberType) -> bool {
        self.number_type() == other
    }

    fn match_or_panic(&self, other: NumberType) {
        _match_or_panic(self.number_type(), other);
    }

    fn type_name(&self) -> &'static str {
        self.number_type().type_name
    }

    fn size_of(&self) -> usize {
        self.number_type().size_of
    }
}

#[inline]
fn _match_or_panic(t1: NumberType, t2: NumberType) {
    if t1 != t2 {
        let err = NumberTypeError::Mismatch(t1, t2);
        panic!("{:?}", err);
    }
}

// use std::fmt;
// use std::marker::PhantomData;
// use std::slice;

// use crate::Output;

// use libc::c_void;

// use super::as_ptr::AsPtr;
// use super::cl_number::*;
// use super::ClTryFrom;

// pub fn apply<T: NumberTypedT, F: FnOnce() -> T + Sized>(t: NumberType, fun: F) -> T {
//     t.type_check(T::number_type())
//         .unwrap_or_else(|e| panic!("{:?}", e));
//     fun()
// }

// #[macro_export]
// macro_rules! apply_number_type {
//     // ($t:ty, $num_type:expr, $func:ident, [$( $arg:expr ),*]) => {
//     //     match $num_type.number_type() {
//     //         $crate::NumberType::SizeT => $func::<$t, libc::size_t>($( $arg ),*),
//     //         $crate::NumberType::ClDouble => $func::<$t, cl_double>($( $arg ),*),
//     //         $crate::NumberType::ClHalf => $func::<$t, cl_half>($( $arg ),*),
//     //         $crate::NumberType::ClChar => $func::<$t, cl_char>($( $arg ),*),
//     //         $crate::NumberType::ClChar2 => $func::<$t, cl_char2>($( $arg ),*),
//     //         $crate::NumberType::ClChar3 => $func::<$t, cl_char3>($( $arg ),*),
//     //         $crate::NumberType::ClChar4 => $func::<$t, cl_char4>($( $arg ),*),
//     //         $crate::NumberType::ClChar8 => $func::<$t, cl_char8>($( $arg ),*),
//     //         $crate::NumberType::ClChar16 => $func::<$t, cl_char16>($( $arg ),*),
//     //         $crate::NumberType::ClUchar => $func::<$t, cl_uchar>($( $arg ),*),
//     //         $crate::NumberType::ClUchar2 => $func::<$t, cl_uchar2>($( $arg ),*),
//     //         $crate::NumberType::ClUchar3 => $func::<$t, cl_uchar3>($( $arg ),*),
//     //         $crate::NumberType::ClUchar4 => $func::<$t, cl_uchar4>($( $arg ),*),
//     //         $crate::NumberType::ClUchar8 => $func::<$t, cl_uchar8>($( $arg ),*),
//     //         $crate::NumberType::ClUchar16 => $func::<$t, cl_uchar16>($( $arg ),*),
//     //         $crate::NumberType::ClShort => $func::<$t, cl_short>($( $arg ),*),
//     //         $crate::NumberType::ClShort2 => $func::<$t, cl_short2>($( $arg ),*),
//     //         $crate::NumberType::ClShort3 => $func::<$t, cl_short3>($( $arg ),*),
//     //         $crate::NumberType::ClShort4 => $func::<$t, cl_short4>($( $arg ),*),
//     //         $crate::NumberType::ClShort8 => $func::<$t, cl_short8>($( $arg ),*),
//     //         $crate::NumberType::ClShort16 => $func::<$t, cl_short16>($( $arg ),*),
//     //         $crate::NumberType::ClUshort => $func::<$t, cl_ushort>($( $arg ),*),
//     //         $crate::NumberType::ClUshort2 => $func::<$t, cl_ushort2>($( $arg ),*),
//     //         $crate::NumberType::ClUshort3 => $func::<$t, cl_ushort3>($( $arg ),*),
//     //         $crate::NumberType::ClUshort4 => $func::<$t, cl_ushort4>($( $arg ),*),
//     //         $crate::NumberType::ClUshort8 => $func::<$t, cl_ushort8>($( $arg ),*),
//     //         $crate::NumberType::ClUshort16 => $func::<$t, cl_ushort16>($( $arg ),*),
//     //         $crate::NumberType::ClInt => $func::<$t, cl_int>($( $arg ),*),
//     //         $crate::NumberType::ClInt2 => $func::<$t, cl_int2>($( $arg ),*),
//     //         $crate::NumberType::ClInt3 => $func::<$t, cl_int3>($( $arg ),*),
//     //         $crate::NumberType::ClInt4 => $func::<$t, cl_int4>($( $arg ),*),
//     //         $crate::NumberType::ClInt8 => $func::<$t, cl_int8>($( $arg ),*),
//     //         $crate::NumberType::ClInt16 => $func::<$t, cl_int16>($( $arg ),*),
//     //         $crate::NumberType::ClUint => $func::<$t, cl_uint>($( $arg ),*),
//     //         $crate::NumberType::ClUint2 => $func::<$t, cl_uint2>($( $arg ),*),
//     //         $crate::NumberType::ClUint3 => $func::<$t, cl_uint3>($( $arg ),*),
//     //         $crate::NumberType::ClUint4 => $func::<$t, cl_uint4>($( $arg ),*),
//     //         $crate::NumberType::ClUint8 => $func::<$t, cl_uint8>($( $arg ),*),
//     //         $crate::NumberType::ClUint16 => $func::<$t, cl_uint16>($( $arg ),*),
//     //         $crate::NumberType::ClLong => $func::<$t, cl_long>($( $arg ),*),
//     //         $crate::NumberType::ClLong2 => $func::<$t, cl_long2>($( $arg ),*),
//     //         $crate::NumberType::ClLong3 => $func::<$t, cl_long3>($( $arg ),*),
//     //         $crate::NumberType::ClLong4 => $func::<$t, cl_long4>($( $arg ),*),
//     //         $crate::NumberType::ClLong8 => $func::<$t, cl_long8>($( $arg ),*),
//     //         $crate::NumberType::ClLong16 => $func::<$t, cl_long16>($( $arg ),*),
//     //         $crate::NumberType::ClUlong => $func::<$t, cl_ulong>($( $arg ),*),
//     //         $crate::NumberType::ClUlong2 => $func::<$t, cl_ulong2>($( $arg ),*),
//     //         $crate::NumberType::ClUlong3 => $func::<$t, cl_ulong3>($( $arg ),*),
//     //         $crate::NumberType::ClUlong4 => $func::<$t, cl_ulong4>($( $arg ),*),
//     //         $crate::NumberType::ClUlong8 => $func::<$t, cl_ulong8>($( $arg ),*),
//     //         $crate::NumberType::ClUlong16 => $func::<$t, cl_ulong16>($( $arg ),*),
//     //         $crate::NumberType::ClFloat => $func::<$t, cl_float>($( $arg ),*),
//     //         $crate::NumberType::ClFloat2 => $func::<$t, cl_float2>($( $arg ),*),
//     //         $crate::NumberType::ClFloat3 => $func::<$t, cl_float3>($( $arg ),*),
//     //         $crate::NumberType::ClFloat4 => $func::<$t, cl_float4>($( $arg ),*),
//     //         $crate::NumberType::ClFloat8 => $func::<$t, cl_float8>($( $arg ),*),
//     //         $crate::NumberType::ClFloat16 => $func::<$t, cl_float16>($( $arg ),*),
//     //         $crate::NumberType::Bool => $func::<$t, bool>($( $arg ),*),
//     //     }
//     // };

//     // ($num_type:expr, $t:ty, $func:ident, [$( $arg:expr ),*]) => {
//     //     match $num_type.number_type() {
//     //         $crate::NumberType::SizeT => $func::<size_t, $t>($( $arg ),*),
//     //         $crate::NumberType::ClDouble => $func::<cl_double, $t>($( $arg ),*),
//     //         $crate::NumberType::ClHalf => $func::<cl_half, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar => $func::<cl_char, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar2 => $func::<cl_char2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar3 => $func::<cl_char3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar4 => $func::<cl_char4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar8 => $func::<cl_char8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClChar16 => $func::<cl_char16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar => $func::<cl_uchar, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar2 => $func::<cl_uchar2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar3 => $func::<cl_uchar3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar4 => $func::<cl_uchar4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar8 => $func::<cl_uchar8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUchar16 => $func::<cl_uchar16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort => $func::<cl_short, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort2 => $func::<cl_short2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort3 => $func::<cl_short3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort4 => $func::<cl_short4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort8 => $func::<cl_short8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClShort16 => $func::<cl_short16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort => $func::<cl_ushort, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort2 => $func::<cl_ushort2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort3 => $func::<cl_ushort3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort4 => $func::<cl_ushort4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort8 => $func::<cl_ushort8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUshort16 => $func::<cl_ushort16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt => $func::<cl_int, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt2 => $func::<cl_int2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt3 => $func::<cl_int3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt4 => $func::<cl_int4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt8 => $func::<cl_int8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClInt16 => $func::<cl_int16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint => $func::<cl_uint, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint2 => $func::<cl_uint2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint3 => $func::<cl_uint3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint4 => $func::<cl_uint4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint8 => $func::<cl_uint8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUint16 => $func::<cl_uint16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong => $func::<cl_long, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong2 => $func::<cl_long2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong3 => $func::<cl_long3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong4 => $func::<cl_long4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong8 => $func::<cl_long8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClLong16 => $func::<cl_long16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong => $func::<cl_ulong, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong2 => $func::<cl_ulong2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong3 => $func::<cl_ulong3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong4 => $func::<cl_ulong4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong8 => $func::<cl_ulong8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClUlong16 => $func::<cl_ulong16, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat => $func::<cl_float, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat2 => $func::<cl_float2, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat3 => $func::<cl_float3, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat4 => $func::<cl_float4, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat8 => $func::<cl_float8, $t>($( $arg ),*),
//     //         $crate::NumberType::ClFloat16 => $func::<cl_float16, $t>($( $arg ),*),
//     //         $crate::NumberType::Bool => $func::<bool, $t>($( $arg ),*),
//     //     }
//     // };

//     ($num_type:expr, $func:ident, [$( $arg:expr ),*]) => {
//         match $num_type.number_type() {
//             $crate::NumberType::SizeT => $func::<size_t>($( $arg ),*),
//             $crate::NumberType::ClDouble => $func::<$crate::cl_double>($( $arg ),*),
//             $crate::NumberType::ClHalf => $func::<$crate::cl_half>($( $arg ),*),
//             $crate::NumberType::ClChar => $func::<$crate::cl_char>($( $arg ),*),
//             $crate::NumberType::ClChar2 => $func::<$crate::cl_char2>($( $arg ),*),
//             $crate::NumberType::ClChar3 => $func::<$crate::cl_char3>($( $arg ),*),
//             $crate::NumberType::ClChar4 => $func::<$crate::cl_char4>($( $arg ),*),
//             $crate::NumberType::ClChar8 => $func::<$crate::cl_char8>($( $arg ),*),
//             $crate::NumberType::ClChar16 => $func::<$crate::cl_char16>($( $arg ),*),
//             $crate::NumberType::ClUchar => $func::<$crate::cl_uchar>($( $arg ),*),
//             $crate::NumberType::ClUchar2 => $func::<$crate::cl_uchar2>($( $arg ),*),
//             $crate::NumberType::ClUchar3 => $func::<$crate::cl_uchar3>($( $arg ),*),
//             $crate::NumberType::ClUchar4 => $func::<$crate::cl_uchar4>($( $arg ),*),
//             $crate::NumberType::ClUchar8 => $func::<$crate::cl_uchar8>($( $arg ),*),
//             $crate::NumberType::ClUchar16 => $func::<$crate::cl_uchar16>($( $arg ),*),
//             $crate::NumberType::ClShort => $func::<$crate::cl_short>($( $arg ),*),
//             $crate::NumberType::ClShort2 => $func::<$crate::cl_short2>($( $arg ),*),
//             $crate::NumberType::ClShort3 => $func::<$crate::cl_short3>($( $arg ),*),
//             $crate::NumberType::ClShort4 => $func::<$crate::cl_short4>($( $arg ),*),
//             $crate::NumberType::ClShort8 => $func::<$crate::cl_short8>($( $arg ),*),
//             $crate::NumberType::ClShort16 => $func::<$crate::cl_short16>($( $arg ),*),
//             $crate::NumberType::ClUshort => $func::<$crate::cl_ushort>($( $arg ),*),
//             $crate::NumberType::ClUshort2 => $func::<$crate::cl_ushort2>($( $arg ),*),
//             $crate::NumberType::ClUshort3 => $func::<$crate::cl_ushort3>($( $arg ),*),
//             $crate::NumberType::ClUshort4 => $func::<$crate::cl_ushort4>($( $arg ),*),
//             $crate::NumberType::ClUshort8 => $func::<$crate::cl_ushort8>($( $arg ),*),
//             $crate::NumberType::ClUshort16 => $func::<$crate::cl_ushort16>($( $arg ),*),
//             $crate::NumberType::ClInt => $func::<$crate::cl_int>($( $arg ),*),
//             $crate::NumberType::ClInt2 => $func::<$crate::cl_int2>($( $arg ),*),
//             $crate::NumberType::ClInt3 => $func::<$crate::cl_int3>($( $arg ),*),
//             $crate::NumberType::ClInt4 => $func::<$crate::cl_int4>($( $arg ),*),
//             $crate::NumberType::ClInt8 => $func::<$crate::cl_int8>($( $arg ),*),
//             $crate::NumberType::ClInt16 => $func::<$crate::cl_int16>($( $arg ),*),
//             $crate::NumberType::ClUint => $func::<$crate::cl_uint>($( $arg ),*),
//             $crate::NumberType::ClUint2 => $func::<$crate::cl_uint2>($( $arg ),*),
//             $crate::NumberType::ClUint3 => $func::<$crate::cl_uint3>($( $arg ),*),
//             $crate::NumberType::ClUint4 => $func::<$crate::cl_uint4>($( $arg ),*),
//             $crate::NumberType::ClUint8 => $func::<$crate::cl_uint8>($( $arg ),*),
//             $crate::NumberType::ClUint16 => $func::<$crate::cl_uint16>($( $arg ),*),
//             $crate::NumberType::ClLong => $func::<$crate::cl_long>($( $arg ),*),
//             $crate::NumberType::ClLong2 => $func::<$crate::cl_long2>($( $arg ),*),
//             $crate::NumberType::ClLong3 => $func::<$crate::cl_long3>($( $arg ),*),
//             $crate::NumberType::ClLong4 => $func::<$crate::cl_long4>($( $arg ),*),
//             $crate::NumberType::ClLong8 => $func::<$crate::cl_long8>($( $arg ),*),
//             $crate::NumberType::ClLong16 => $func::<$crate::cl_long16>($( $arg ),*),
//             $crate::NumberType::ClUlong => $func::<$crate::cl_ulong>($( $arg ),*),
//             $crate::NumberType::ClUlong2 => $func::<$crate::cl_ulong2>($( $arg ),*),
//             $crate::NumberType::ClUlong3 => $func::<$crate::cl_ulong3>($( $arg ),*),
//             $crate::NumberType::ClUlong4 => $func::<$crate::cl_ulong4>($( $arg ),*),
//             $crate::NumberType::ClUlong8 => $func::<$crate::cl_ulong8>($( $arg ),*),
//             $crate::NumberType::ClUlong16 => $func::<$crate::cl_ulong16>($( $arg ),*),
//             $crate::NumberType::ClFloat => $func::<$crate::cl_float>($( $arg ),*),
//             $crate::NumberType::ClFloat2 => $func::<$crate::cl_float2>($( $arg ),*),
//             $crate::NumberType::ClFloat3 => $func::<$crate::cl_float3>($( $arg ),*),
//             $crate::NumberType::ClFloat4 => $func::<$crate::cl_float4>($( $arg ),*),
//             $crate::NumberType::ClFloat8 => $func::<$crate::cl_float8>($( $arg ),*),
//             $crate::NumberType::ClFloat16 => $func::<$crate::cl_float16>($( $arg ),*),
//             $crate::NumberType::Bool => $func::<bool>($( $arg ),*),
//         }
//     };
// }


// impl NumberTyped for NumberType {
//     fn number_type(&self) -> NumberType {
//         *self
//     }
// }



// /// An error related to CL types.
// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum TypeError {
//     #[fail(display = "TypeMismatchError - expected {:?}, but found {:?}", _0, _1)]
//     TypeMismatch(NumberType, NumberType),

//     #[fail(
//         display = "InvalidTypeError - the value {:?} is not a valid value for type {}",
//         _0, 1
//     )]
//     InvalidValue(NumberType, String),

//     #[fail(
//         display = "InvalidFloat16Error - the value {:?} is not a valid f16 value due to {:?}",
//         _0, _1
//     )]
//     InvalidFloat16(String, &'static str),
// }

// #[inline]
// fn _size_of<T: Sized>() -> usize {
//     std::mem::size_of::<T>()
// }

// #[inline]
// fn _align_of<T>() -> usize {
//     std::mem::align_of::<T>()
// }

// impl NumberType {
//     pub fn size_of(&self) -> usize {
//         apply_number_type!(self, _size_of, [])
//     }

//     pub fn align_of(&self) -> usize {
//         apply_number_type!(self, _align_of, [])
//     }

//     pub fn matches(&self, other: NumberType) -> bool {
//         *self == other
//     }

//     pub fn match_or_panic(&self, other: NumberType) {
//         _match_or_panic(*self, other)
//     }

//     pub fn type_check(&self, other: NumberType) -> Output<()> {
//         if self.matches(other) {
//             Ok(())
//         } else {
//             Err(TypeError::TypeMismatch(*self, other).into())
//         }
//     }
// }

// pub trait NumberTypedT {
//     fn number_type() -> NumberType;

//     fn matches(other: NumberType) -> bool {
//         Self::number_type() == other
//     }
//     fn match_or_panic(other: NumberType) {
//         _match_or_panic(Self::number_type(), other);
//     }

//     fn type_name() -> String {
//         format!("{:?}", Self::number_type())
//     }
// }

// pub trait NumberTyped {
//     fn number_type(&self) -> NumberType;

//     fn matches(&self, other: NumberType) -> bool {
//         self.number_type() == other
//     }
//     fn match_or_panic(&self, other: NumberType) {
//         _match_or_panic(self.number_type(), other);
//     }
// }

// impl<T: NumberTypedT> NumberTypedT for Vec<T> {
//     fn number_type() -> NumberType {
//         T::number_type()
//     }
// }

// impl<T: NumberTypedT> NumberTyped for Vec<T> {
//     fn number_type(&self) -> NumberType {
//         T::number_type()
//     }
// }

// pub struct NumberTypedSlice<'a> {
//     t: NumberType,
//     _phantom: PhantomData<&'a c_void>,
//     _ptr: *const c_void,
//     _len: usize,
// }

// impl<'a> fmt::Debug for NumberTypedSlice<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "NumberTypedSlice{{t: {:?}, ptr: {:?}, len: {:?}}}",
//             self.t, self._ptr, self._len
//         )
//     }
// }

// impl<'a> AsPtr<c_void> for NumberTypedSlice<'a> {
//     fn as_ptr(&self) -> *const c_void {
//         self._ptr
//     }

//     fn as_mut_ptr(&mut self) -> *mut c_void {
//         self._ptr as *mut c_void
//     }
// }

// impl<'a> NumberTyped for NumberTypedSlice<'a> {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

// impl<'a> NumberTypedSlice<'a> {
//     pub fn len(&self) -> usize {
//         self._len
//     }

//     pub fn try_as_slice<T: NumberTypedT>(&self) -> Output<&'a [T]> {
//         self.t.type_check(T::number_type())?;
//         let s = unsafe { slice::from_raw_parts(self._ptr as *const T, self._len) };
//         Ok(s)
//     }

//     pub fn try_as_mut_slice<T: NumberTypedT>(&self) -> Output<&'a mut [T]> {
//         self.t.type_check(T::number_type())?;
//         let s = unsafe { slice::from_raw_parts_mut(self._ptr as *mut T, self._len) };
//         Ok(s)
//     }
// }

// pub struct NumberTypedVec {
//     t: NumberType,
//     _ptr: *mut c_void,
//     _len: usize,
//     _cap: usize,
// }

// impl fmt::Debug for NumberTypedVec {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "NumberTypedVec{{t: {:?}, ptr: {:?}, len: {:?}, capacity: {:?}}}",
//             self.t, self._ptr, self._len, self._cap
//         )
//     }
// }

// fn _cast_ntv_to_vec<SelfT, T>(ntv: &NumberTypedVec) -> Output<Vec<T>>
// where
//     T: NumberTypedT + Copy + ClTryFrom<SelfT>,
//     SelfT: NumberTypedT + Copy,
// {
//     let mut out = Vec::with_capacity(ntv.len());
//     let mut num2: T;
//     for num1 in ntv.try_as_slice().unwrap().iter() {
//         num2 = <T as ClTryFrom<SelfT>>::try_from(*num1)?;
//         out.push(num2);
//     }
//     Ok(out)
// }

// impl AsPtr<c_void> for NumberTypedVec {
//     fn as_ptr(&self) -> *const c_void {
//         self._ptr as *const c_void
//     }

//     fn as_mut_ptr(&mut self) -> *mut c_void {
//         self._ptr
//     }
// }

// impl NumberTypedVec {
//     pub fn as_mut_ptr(&self) -> *mut c_void {
//         self._ptr
//     }

//     pub fn len(&self) -> usize {
//         self._len
//     }

//     pub fn capacity(&self) -> usize {
//         self._cap
//     }

//     pub fn try_to_vec<T: NumberTypedT>(self) -> Output<Vec<T>> {
//         self.t.type_check(T::number_type())?;
//         let v = unsafe { Vec::from_raw_parts(self._ptr as *mut T, self._len, self._cap) };
//         std::mem::forget(self);
//         Ok(v)
//     }

//     pub fn as_number_typed_slice<'a>(&self) -> NumberTypedSlice<'a> {
//         NumberTypedSlice {
//             t: self.t,
//             _ptr: self._ptr as *const c_void,
//             _len: self._len,
//             _phantom: PhantomData,
//         }
//     }

//     pub fn try_as_slice<T: NumberTypedT>(&self) -> Output<&[T]> {
//         self.as_number_typed_slice().try_as_slice()
//     }

//     pub fn try_as_mut_slice<T: NumberTypedT>(&mut self) -> Output<&mut [T]> {
//         self.as_number_typed_slice().try_as_mut_slice()
//     }

//     pub fn try_iter<T: NumberTypedT>(&self) -> Output<std::slice::Iter<T>> {
//         Ok(self.try_as_slice()?.iter())
//     }

//     pub fn try_iter_mut<T: NumberTypedT>(&mut self) -> Output<std::slice::IterMut<T>> {
//         Ok(self.try_as_mut_slice()?.iter_mut())
//     }

//     pub fn try_into_iter<T: NumberTypedT>(self) -> Output<std::vec::IntoIter<T>> {
//         Ok(self.try_to_vec()?.into_iter())
//     }
// }

// impl<T: NumberTypedT> From<Vec<T>> for NumberTypedVec {
//     fn from(mut v: Vec<T>) -> NumberTypedVec {
//         let ntv = NumberTypedVec {
//             t: T::number_type(),
//             _ptr: v.as_mut_ptr() as *mut c_void,
//             _len: v.len(),
//             _cap: v.capacity(),
//         };
//         std::mem::forget(v);
//         ntv
//     }
// }

// unsafe fn _ntv_drop<T: NumberTypedT>(ntv: &mut NumberTypedVec) {
//     ntv.number_type().type_check(T::number_type()).unwrap();
//     Vec::from_raw_parts(ntv._ptr as *const _ as *mut T, ntv._len, ntv._cap);
// }

// impl NumberTyped for NumberTypedVec {
//     fn number_type(&self) -> NumberType {
//         self.t
//     }
// }

// impl Drop for NumberTypedVec {
//     fn drop(&mut self) {
//         unsafe { apply_number_type!(self.t, _ntv_drop, [self]) };
//     }
// }

// #[cfg(test)]
// mod tests {

//     use crate::numbers::*;

//     fn test_func_to_be_applied<T: NumberTypedT>() -> NumberType {
//         T::number_type()
//     }

//     #[test]
//     fn apply_number_type_macro_works() {
//         let t = apply_number_type!(cl_bool::number_type(), test_func_to_be_applied, []);
//         assert_eq!(t, NumberType::ClUint);
//     }

//     #[test]
//     fn apply_number_type_macro_works_with_a_variable() {
//         let a = cl_bool::number_type();
//         let t = apply_number_type!(a, test_func_to_be_applied, []);
//         assert_eq!(t, NumberType::ClUint);
//     }
// }
