use std::fmt::Debug;


use crate::{Output};

use libc::size_t;

use super::newtypes::*;
use super::ffi_types::*;
// use super::number_type::*;

#[macro_export]
macro_rules! apply_t {
    ($func:ident, $t:ty, [$( $arg:expr ),*]) => {
        $func::<$t>($( $arg ),*)
    }
}

#[macro_export]
macro_rules! apply_number_type {
    ($num_type:expr, $func:ident, [$( $arg:expr ),*]) => {
        match $num_type.number_type() {
            NumberType::ClDouble => $crate::apply_t!($func, cl_double, [$( $arg ),*]),
            NumberType::SizeT => $crate::apply_t!($func, libc::size_t, [$( $arg ),*]),
            NumberType::ClHalf => $crate::apply_t!($func, cl_half, [$( $arg:expr ),*]),
            NumberType::ClChar => $crate::apply_t!($func, cl_char, [$( $arg ),*]),
            NumberType::ClChar2 => $crate::apply_t!($func, cl_char2, [$( $arg ),*]),
            NumberType::ClChar3 => $crate::apply_t!($func, cl_char3, [$( $arg ),*]),
            NumberType::ClChar4 => $crate::apply_t!($func, cl_char4, [$( $arg ),*]),
            NumberType::ClChar8 => $crate::apply_t!($func, cl_char8, [$( $arg ),*]),
            NumberType::ClChar16 => $crate::apply_t!($func, cl_char16, [$( $arg ),*]),
            NumberType::ClUchar => $crate::apply_t!($func, cl_uchar, [$( $arg ),*]),
            NumberType::ClUchar2 => $crate::apply_t!($func, cl_uchar2, [$( $arg ),*]),
            NumberType::ClUchar3 => $crate::apply_t!($func, cl_uchar3, [$( $arg ),*]),
            NumberType::ClUchar4 => $crate::apply_t!($func, cl_uchar4, [$( $arg ),*]),
            NumberType::ClUchar8 => $crate::apply_t!($func, cl_uchar8, [$( $arg ),*]),
            NumberType::ClUchar16 => $crate::apply_t!($func, cl_uchar16, [$( $arg ),*]),
            NumberType::ClShort => $crate::apply_t!($func, cl_short, [$( $arg ),*]),
            NumberType::ClShort2 => $crate::apply_t!($func, cl_short2, [$( $arg ),*]),
            NumberType::ClShort3 => $crate::apply_t!($func, cl_short3, [$( $arg ),*]),
            NumberType::ClShort4 => $crate::apply_t!($func, cl_short4, [$( $arg ),*]),
            NumberType::ClShort8 => $crate::apply_t!($func, cl_short8, [$( $arg ),*]),
            NumberType::ClShort16 => $crate::apply_t!($func, cl_short16, [$( $arg ),*]), 
            NumberType::ClUshort => $crate::apply_t!($func, cl_ushort, [$( $arg ),*]),
            NumberType::ClUshort2 => $crate::apply_t!($func, cl_ushort2, [$( $arg ),*]),
            NumberType::ClUshort3 => $crate::apply_t!($func, cl_ushort3, [$( $arg ),*]),
            NumberType::ClUshort4 => $crate::apply_t!($func, cl_ushort4, [$( $arg ),*]),
            NumberType::ClUshort8 => $crate::apply_t!($func, cl_ushort8, [$( $arg ),*]),
            NumberType::ClUshort16 => $crate::apply_t!($func, cl_ushort16, [$( $arg ),*]),
            NumberType::ClInt => $crate::apply_t!($func, cl_int, [$( $arg ),*]),
            NumberType::ClInt2 => $crate::apply_t!($func, cl_int2, [$( $arg ),*]),
            NumberType::ClInt3 => $crate::apply_t!($func, cl_int3, [$( $arg ),*]),
            NumberType::ClInt4 => $crate::apply_t!($func, cl_int4, [$( $arg ),*]),
            NumberType::ClInt8 => $crate::apply_t!($func, cl_int8, [$( $arg ),*]),
            NumberType::ClInt16 => $crate::apply_t!($func, cl_int16, [$( $arg ),*]),
            NumberType::ClUint => $crate::apply_t!($func, cl_uint, [$( $arg ),*]),
            NumberType::ClUint2 => $crate::apply_t!($func, cl_uint2, [$( $arg ),*]),
            NumberType::ClUint3 => $crate::apply_t!($func, cl_uint3, [$( $arg ),*]),
            NumberType::ClUint4 => $crate::apply_t!($func, cl_uint4, [$( $arg ),*]),
            NumberType::ClUint8 => $crate::apply_t!($func, cl_uint8, [$( $arg ),*]),
            NumberType::ClUint16 => $crate::apply_t!($func, cl_uint16, [$( $arg ),*]),
            NumberType::ClLong => $crate::apply_t!($func, cl_long, [$( $arg ),*]),
            NumberType::ClLong2 => $crate::apply_t!($func, cl_long2, [$( $arg ),*]),
            NumberType::ClLong3 => $crate::apply_t!($func, cl_long3, [$( $arg ),*]),
            NumberType::ClLong4 => $crate::apply_t!($func, cl_long4, [$( $arg ),*]),
            NumberType::ClLong8 => $crate::apply_t!($func, cl_long8, [$( $arg ),*]),
            NumberType::ClLong16 => $crate::apply_t!($func, cl_long16, [$( $arg ),*]),
            NumberType::ClUlong => $crate::apply_t!($func, cl_ulong, [$( $arg ),*]),
            NumberType::ClUlong2 => $crate::apply_t!($func, cl_ulong2, [$( $arg ),*]),
            NumberType::ClUlong3 => $crate::apply_t!($func, cl_ulong3, [$( $arg ),*]),
            NumberType::ClUlong4 => $crate::apply_t!($func, cl_ulong4, [$( $arg ),*]),
            NumberType::ClUlong8 => $crate::apply_t!($func, cl_ulong8, [$( $arg ),*]),
            NumberType::ClUlong16 => $crate::apply_t!($func, cl_ulong16, [$( $arg ),*]),
            NumberType::ClFloat => $crate::apply_t!($func, cl_float, [$( $arg ),*]),
            NumberType::ClFloat2 => $crate::apply_t!($func, cl_float2, [$( $arg ),*]),
            NumberType::ClFloat3 => $crate::apply_t!($func, cl_float3, [$( $arg ),*]),
            NumberType::ClFloat4 => $crate::apply_t!($func, cl_float4, [$( $arg ),*]),
            NumberType::ClFloat8 => $crate::apply_t!($func, cl_float8, [$( $arg ),*]),
            NumberType::ClFloat16 => $crate::apply_t!($func, cl_float16, [$( $arg ),*]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberTypeKind {
    Primitive,
    Two,
    Three,
    Four,
    Eight,
    Sixteen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberType {
    ClHalf,
    SizeT,
    ClDouble,

    ClChar,
    ClUchar,
    ClShort,
    ClUshort,
    ClInt,
    ClUint,
    ClLong,
    ClUlong,
    ClFloat,

    ClChar2,
    ClUchar2,
    ClShort2,
    ClUshort2,
    ClInt2,
    ClUint2,
    ClLong2,
    ClUlong2,
    ClFloat2,

    ClChar3,
    ClUchar3,
    ClShort3,
    ClUshort3,
    ClInt3,
    ClUint3,
    ClLong3,
    ClUlong3,
    ClFloat3,

    ClChar4,
    ClUchar4,
    ClShort4,
    ClUshort4,
    ClInt4,
    ClUint4,
    ClLong4,
    ClUlong4,
    ClFloat4,

    ClChar8,
    ClUchar8,
    ClShort8,
    ClUshort8,
    ClInt8,
    ClUint8,
    ClLong8,
    ClUlong8,
    ClFloat8,

    ClChar16,
    ClUchar16,
    ClShort16,
    ClUshort16,
    ClInt16,
    ClUint16,
    ClLong16,
    ClUlong16,
    ClFloat16,
}

impl NumberTyped for NumberType {
    fn number_type(&self) -> NumberType {
        *self
    }
}

#[inline]
fn _match_or_panic(t1: NumberType, t2: NumberType) {
    if t1 != t2 {
        panic!("Type mismatch - {:?} vs {:?}", t1, t2);
    }
}


/// An error related to CL types.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum TypeError {
    #[fail(display = "TypeMismatchError - expected {:?}, but found {:?}", _0, _1)]
    TypeMismatch(NumberType, NumberType),

    #[fail(display = "InvalidTypeError - the value {:?} is not a valid value for type {}", _0, 1)]
    InvalidValue(NumberType, String),
}

impl NumberType {
    pub fn size_of_t(&self) -> usize {
        match self {
            NumberType::ClChar => std::mem::size_of::<cl_char>(),
            NumberType::ClChar2 => std::mem::size_of::<cl_char2>(),
            NumberType::ClChar3 => std::mem::size_of::<cl_char3>(),
            NumberType::ClChar4 => std::mem::size_of::<cl_char4>(),
            NumberType::ClChar8 => std::mem::size_of::<cl_char8>(),
            NumberType::ClChar16 => std::mem::size_of::<cl_char16>(),
            NumberType::ClUchar => std::mem::size_of::<cl_uchar>(),
            NumberType::ClUchar2 => std::mem::size_of::<cl_uchar2>(),
            NumberType::ClUchar3 => std::mem::size_of::<cl_uchar3>(),
            NumberType::ClUchar4 => std::mem::size_of::<cl_uchar4>(),
            NumberType::ClUchar8 => std::mem::size_of::<cl_uchar8>(),
            NumberType::ClUchar16 => std::mem::size_of::<cl_uchar16>(),
            NumberType::ClShort => std::mem::size_of::<cl_short>(),
            NumberType::ClShort2 => std::mem::size_of::<cl_short2>(),
            NumberType::ClShort3 => std::mem::size_of::<cl_short3>(),
            NumberType::ClShort4 => std::mem::size_of::<cl_short4>(),
            NumberType::ClShort8 => std::mem::size_of::<cl_short8>(),
            NumberType::ClShort16 => std::mem::size_of::<cl_short16>(),
            NumberType::ClUshort => std::mem::size_of::<cl_ushort>(),
            NumberType::ClUshort2 => std::mem::size_of::<cl_ushort2>(),
            NumberType::ClUshort3 => std::mem::size_of::<cl_ushort3>(),
            NumberType::ClUshort4 => std::mem::size_of::<cl_ushort4>(),
            NumberType::ClUshort8 => std::mem::size_of::<cl_ushort8>(),
            NumberType::ClUshort16 => std::mem::size_of::<cl_ushort16>(),
            NumberType::ClInt => std::mem::size_of::<cl_int>(),
            NumberType::ClInt2 => std::mem::size_of::<cl_int2>(),
            NumberType::ClInt3 => std::mem::size_of::<cl_int3>(),
            NumberType::ClInt4 => std::mem::size_of::<cl_int4>(),
            NumberType::ClInt8 => std::mem::size_of::<cl_int8>(),
            NumberType::ClInt16 => std::mem::size_of::<cl_int16>(),
            NumberType::ClUint => std::mem::size_of::<cl_uint>(),
            NumberType::ClUint2 => std::mem::size_of::<cl_uint2>(),
            NumberType::ClUint3 => std::mem::size_of::<cl_uint3>(),
            NumberType::ClUint4 => std::mem::size_of::<cl_uint4>(),
            NumberType::ClUint8 => std::mem::size_of::<cl_uint8>(),
            NumberType::ClUint16 => std::mem::size_of::<cl_uint16>(),
            NumberType::ClLong => std::mem::size_of::<cl_long>(),
            NumberType::ClLong2 => std::mem::size_of::<cl_long2>(),
            NumberType::ClLong3 => std::mem::size_of::<cl_long3>(),
            NumberType::ClLong4 => std::mem::size_of::<cl_long4>(),
            NumberType::ClLong8 => std::mem::size_of::<cl_long8>(),
            NumberType::ClLong16 => std::mem::size_of::<cl_long16>(),
            NumberType::ClUlong => std::mem::size_of::<cl_ulong>(),
            NumberType::ClUlong2 => std::mem::size_of::<cl_ulong2>(),
            NumberType::ClUlong3 => std::mem::size_of::<cl_ulong3>(),
            NumberType::ClUlong4 => std::mem::size_of::<cl_ulong4>(),
            NumberType::ClUlong8 => std::mem::size_of::<cl_ulong8>(),
            NumberType::ClUlong16 => std::mem::size_of::<cl_ulong16>(),
            NumberType::ClFloat => std::mem::size_of::<cl_float>(),
            NumberType::ClFloat2 => std::mem::size_of::<cl_float2>(),
            NumberType::ClFloat3 => std::mem::size_of::<cl_float3>(),
            NumberType::ClFloat4 => std::mem::size_of::<cl_float4>(),
            NumberType::ClFloat8 => std::mem::size_of::<cl_float8>(),
            NumberType::ClFloat16 => std::mem::size_of::<cl_float16>(),
            NumberType::ClHalf => std::mem::size_of::<cl_half>(),
            NumberType::SizeT => std::mem::size_of::<size_t>(),
            NumberType::ClDouble => std::mem::size_of::<cl_double>(),
        }
    }


    pub fn matches(&self, other: NumberType) -> bool {
        *self == other
    }

    pub fn match_or_panic(&self, other: NumberType) {
        _match_or_panic(*self, other)
    }

    pub fn type_check(&self, other: NumberType) -> Output<()> {
        if self.matches(other) {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch(*self, other).into())
        }
    }
}

pub trait NumberTypedT {
    fn number_type() -> NumberType;

    fn matches(other: NumberType) -> bool {
        Self::number_type() == other
    }
    
    fn match_or_panic(other: NumberType) {
        _match_or_panic(Self::number_type(), other);
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
}

impl NumberTypedT for f64 {
    fn number_type() -> NumberType {
        NumberType::ClDouble
    }
}

impl NumberTypedT for bool {
    fn number_type() -> NumberType {
        NumberType::ClUint
    }
}


impl NumberTypedT for ClBool {
    fn number_type() -> NumberType {
        NumberType::ClUint
    }
}

impl NumberTypedT for ClHalf {
    fn number_type() -> NumberType {
        NumberType::ClHalf
    }
}

impl NumberTypedT for ClDouble {
    fn number_type() -> NumberType {
        NumberType::ClDouble
    }
}

impl NumberTypedT for size_t {
    fn number_type() -> NumberType {
        NumberType::SizeT
    }
}

impl NumberTypedT for SizeT {
    fn number_type() -> NumberType {
        NumberType::SizeT
    }
}



macro_rules! impl_number_typed_t {
    ($snake:ident, $pascal:ident) => {
        impl NumberTypedT for $snake {
            fn number_type() -> NumberType {
                NumberType::$pascal
            }
        }

        impl NumberTypedT for $pascal {
            fn number_type() -> NumberType {
                NumberType::$pascal
            }
        }
    };
    ($snake:ident, $pascal:ident, 3) => {
        paste::item! {
            impl NumberTypedT for [<$pascal 3>] {
                fn number_type() -> NumberType {
                    NumberType::[<$pascal 3>]
                }
            }
        }
    };
    ($snake:ident, $pascal:ident, $num:expr) => {
        paste::item! {
            impl NumberTypedT for [<$pascal $num>] {
                fn number_type() -> NumberType {
                    NumberType::[<$pascal $num>]
                }
            }

            impl NumberTypedT for [<$snake $num>] {
                fn number_type() -> NumberType {
                    NumberType::[<$pascal $num>]
                }
            }
        }
    }
}

macro_rules! impl_number_typed_t_for_all {
    ($t:ident, $new_t:ident) => {
        impl_number_typed_t!($t, $new_t);
        impl_number_typed_t!($t, $new_t, 2);
        impl_number_typed_t!($t, $new_t, 3);
        impl_number_typed_t!($t, $new_t, 4);
        impl_number_typed_t!($t, $new_t, 8);
        impl_number_typed_t!($t, $new_t, 16);
    }
}

impl_number_typed_t_for_all!(cl_char, ClChar);
impl_number_typed_t_for_all!(cl_uchar, ClUchar);
impl_number_typed_t_for_all!(cl_short, ClShort);
impl_number_typed_t_for_all!(cl_ushort, ClUshort);
impl_number_typed_t_for_all!(cl_int, ClInt);
impl_number_typed_t_for_all!(cl_uint, ClUint);
impl_number_typed_t_for_all!(cl_long, ClLong);
impl_number_typed_t_for_all!(cl_ulong, ClUlong);
impl_number_typed_t_for_all!(cl_float, ClFloat);


pub struct NumberTypedVec {
    t: NumberType,
    _ptr: *mut libc::c_void,
    _len: usize,
    _cap: usize,
}

impl NumberTyped for NumberTypedVec {
    fn number_type(&self) -> NumberType {
        self.t
    }
}


#[cfg(test)]
mod tests {
    

    use crate::numbers::*;

    fn test_func_to_be_applied<T: NumberTypedT>() -> NumberType {
        T::number_type()
    }

    #[test]
    fn apply_number_type_macro_works() {
        let t = apply_number_type!(cl_bool::number_type(), test_func_to_be_applied, []);
        assert_eq!(t, NumberType::ClUint);

    }
}