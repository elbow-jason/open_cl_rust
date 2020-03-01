use std::fmt::Debug;

use crate::ffi::*;
use crate::{Output};

use libc::size_t;

#[macro_export]
macro_rules! apply_kind {
    ($primitive_t:ty, $kind:ident, $func:ident, [ $( $arg:expr ),* ]) => {
        paste::item! {
            match $kind {
                $crate::NumberTypeKind::Primitive => $func::<$primitive_t>($( $arg ),*),
                $crate::NumberTypeKind::Two => $func::<[<$primitive_t 2>]>($( $arg ),*),
                $crate::NumberTypeKind::Three => $func::<[<$primitive_t 3>]>($( $arg ),*),
                $crate::NumberTypeKind::Four => $func::<[<$primitive_t 4>]>($( $arg ),*),
                $crate::NumberTypeKind::Eight => $func::<[<$primitive_t 8>]>($( $arg ),*),
                $crate::NumberTypeKind::Sixteen => $func::<[<$primitive_t 16>]>($( $arg ),*),
            }
        }
    }
}

#[macro_export]
macro_rules! apply_number_type {
    ($num_type:expr, $func:ident, [ $( $arg:expr ),* ]) => {
        match $num_type.number_type() {
            $crate::NumberType::ClBool => $func::<cl_bool>($( $arg ),*),
            $crate::NumberType::ClDouble => $func::<cl_double>($( $arg ),*),
            $crate::NumberType::ClSizeT => $func::<size_t>($( $arg ),*),
            $crate::NumberType::ClHalf => $func::<cl_half>($( $arg ),*),
            $crate::NumberType::ClChar(kind) => apply_kind!(cl_char, kind, $func, $( $arg ),*),
            $crate::NumberType::ClUchar(kind) => apply_kind!(cl_u_char, kind, $func, $( $arg ),*),
            $crate::NumberType::ClShort(kind) => apply_kind!(cl_short, kind, $func, $( $arg ),*),
            $crate::NumberType::ClUshort(kind) => apply_kind!(cl_ushort, kind, $func, $( $arg ),*),
            $crate::NumberType::ClInt(kind) => apply_kind!(cl_int, kind, $func, $( $arg ),*),
            $crate::NumberType::ClUint(kind) => apply_kind!(cl_uint, kind, $func, $( $arg ),*),
            $crate::NumberType::ClLong(kind) => apply_kind!(cl_long, kind, $func, $( $arg ),*),
            $crate::NumberType::ClUlong(kind) => apply_kind!(cl_ulong, kind, $func, $( $arg ),*),
            $crate::NumberType::ClFloat(kind) => apply_kind!(cl_float, kind, $func, $( $arg ),*),
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

impl NumberTypeKind {
    fn count(&self) -> usize {
        match self {
            Self::Primitive => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Eight => 5,
            Self::Sixteen => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberType {
    ClChar(NumberTypeKind),
    ClUchar(NumberTypeKind),
    ClShort(NumberTypeKind),
    ClUshort(NumberTypeKind),
    ClInt(NumberTypeKind),
    ClUint(NumberTypeKind),
    ClLong(NumberTypeKind),
    ClUlong(NumberTypeKind),
    ClFloat(NumberTypeKind),
    ClHalf,
    ClBool,
    SizeT,
    ClDouble,
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
            NumberType::ClChar(kind) => std::mem::size_of::<cl_char>() * kind.count(),
            NumberType::ClUchar(kind) => std::mem::size_of::<cl_uchar>() * kind.count(),
            NumberType::ClShort(kind) => std::mem::size_of::<cl_short>() * kind.count(),
            NumberType::ClUshort(kind) => std::mem::size_of::<cl_ushort>() * kind.count(),
            NumberType::ClInt(kind) => std::mem::size_of::<cl_int>() * kind.count(),
            NumberType::ClUint(kind) => std::mem::size_of::<cl_uint>() * kind.count(),
            NumberType::ClLong(kind) => std::mem::size_of::<cl_long>() * kind.count(),
            NumberType::ClUlong(kind) => std::mem::size_of::<cl_ulong>() * kind.count(),
            NumberType::ClFloat(kind) => std::mem::size_of::<cl_float>() * kind.count(),
            NumberType::ClHalf => std::mem::size_of::<cl_half>(),
            NumberType::ClBool => std::mem::size_of::<cl_bool>(),
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