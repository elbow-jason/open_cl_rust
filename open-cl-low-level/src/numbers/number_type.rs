use std::fmt::Debug;
use std::slice;
use std::marker::PhantomData;

use crate::{Output};

use libc::size_t;

use super::newtypes::*;
use super::ffi_types::*;
// use super::number_type::*;

pub fn apply<T: NumberTypedT, F: FnOnce() -> T + Sized>(t: NumberType, fun: F) -> T {
    t.type_check(T::number_type()).unwrap_or_else(|e| panic!("{:?}", e));
    fun()
}

#[macro_export]
macro_rules! apply_number_type {
    ($num_type:expr, $func:ident, [$( $arg:expr ),*]) => {
        match $num_type.number_type() {
            $crate::NumberType::SizeT => $func::<libc::size_t>($( $arg ),*),
            $crate::NumberType::ClDouble => $func::<cl_double>($( $arg ),*),
            $crate::NumberType::ClHalf => $func::<cl_half>($( $arg ),*),
            $crate::NumberType::ClChar => $func::<cl_char>($( $arg ),*),
            $crate::NumberType::ClChar2 => $func::<cl_char2>($( $arg ),*),
            $crate::NumberType::ClChar3 => $func::<cl_char3>($( $arg ),*),
            $crate::NumberType::ClChar4 => $func::<cl_char4>($( $arg ),*),
            $crate::NumberType::ClChar8 => $func::<cl_char8>($( $arg ),*),
            $crate::NumberType::ClChar16 => $func::<cl_char16>($( $arg ),*),
            $crate::NumberType::ClUchar => $func::<cl_uchar>($( $arg ),*),
            $crate::NumberType::ClUchar2 => $func::<cl_uchar2>($( $arg ),*),
            $crate::NumberType::ClUchar3 => $func::<cl_uchar3>($( $arg ),*),
            $crate::NumberType::ClUchar4 => $func::<cl_uchar4>($( $arg ),*),
            $crate::NumberType::ClUchar8 => $func::<cl_uchar8>($( $arg ),*),
            $crate::NumberType::ClUchar16 => $func::<cl_uchar16>($( $arg ),*),
            $crate::NumberType::ClShort => $func::<cl_short>($( $arg ),*),
            $crate::NumberType::ClShort2 => $func::<cl_short2>($( $arg ),*),
            $crate::NumberType::ClShort3 => $func::<cl_short3>($( $arg ),*),
            $crate::NumberType::ClShort4 => $func::<cl_short4>($( $arg ),*),
            $crate::NumberType::ClShort8 => $func::<cl_short8>($( $arg ),*),
            $crate::NumberType::ClShort16 => $func::<cl_short16>($( $arg ),*), 
            $crate::NumberType::ClUshort => $func::<cl_ushort>($( $arg ),*),
            $crate::NumberType::ClUshort2 => $func::<cl_ushort2>($( $arg ),*),
            $crate::NumberType::ClUshort3 => $func::<cl_ushort3>($( $arg ),*),
            $crate::NumberType::ClUshort4 => $func::<cl_ushort4>($( $arg ),*),
            $crate::NumberType::ClUshort8 => $func::<cl_ushort8>($( $arg ),*),
            $crate::NumberType::ClUshort16 => $func::<cl_ushort16>($( $arg ),*),
            $crate::NumberType::ClInt => $func::<cl_int>($( $arg ),*),
            $crate::NumberType::ClInt2 => $func::<cl_int2>($( $arg ),*),
            $crate::NumberType::ClInt3 => $func::<cl_int3>($( $arg ),*),
            $crate::NumberType::ClInt4 => $func::<cl_int4>($( $arg ),*),
            $crate::NumberType::ClInt8 => $func::<cl_int8>($( $arg ),*),
            $crate::NumberType::ClInt16 => $func::<cl_int16>($( $arg ),*),
            $crate::NumberType::ClUint => $func::<cl_uint>($( $arg ),*),
            $crate::NumberType::ClUint2 => $func::<cl_uint2>($( $arg ),*),
            $crate::NumberType::ClUint3 => $func::<cl_uint3>($( $arg ),*),
            $crate::NumberType::ClUint4 => $func::<cl_uint4>($( $arg ),*),
            $crate::NumberType::ClUint8 => $func::<cl_uint8>($( $arg ),*),
            $crate::NumberType::ClUint16 => $func::<cl_uint16>($( $arg ),*),
            $crate::NumberType::ClLong => $func::<cl_long>($( $arg ),*),
            $crate::NumberType::ClLong2 => $func::<cl_long2>($( $arg ),*),
            $crate::NumberType::ClLong3 => $func::<cl_long3>($( $arg ),*),
            $crate::NumberType::ClLong4 => $func::<cl_long4>($( $arg ),*),
            $crate::NumberType::ClLong8 => $func::<cl_long8>($( $arg ),*),
            $crate::NumberType::ClLong16 => $func::<cl_long16>($( $arg ),*),
            $crate::NumberType::ClUlong => $func::<cl_ulong>($( $arg ),*),
            $crate::NumberType::ClUlong2 => $func::<cl_ulong2>($( $arg ),*),
            $crate::NumberType::ClUlong3 => $func::<cl_ulong3>($( $arg ),*),
            $crate::NumberType::ClUlong4 => $func::<cl_ulong4>($( $arg ),*),
            $crate::NumberType::ClUlong8 => $func::<cl_ulong8>($( $arg ),*),
            $crate::NumberType::ClUlong16 => $func::<cl_ulong16>($( $arg ),*),
            $crate::NumberType::ClFloat => $func::<cl_float>($( $arg ),*),
            $crate::NumberType::ClFloat2 => $func::<cl_float2>($( $arg ),*),
            $crate::NumberType::ClFloat3 => $func::<cl_float3>($( $arg ),*),
            $crate::NumberType::ClFloat4 => $func::<cl_float4>($( $arg ),*),
            $crate::NumberType::ClFloat8 => $func::<cl_float8>($( $arg ),*),
            $crate::NumberType::ClFloat16 => $func::<cl_float16>($( $arg ),*),
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


impl<T: NumberTypedT> NumberTypedT for Vec<T> {
    fn number_type() -> NumberType {
        T::number_type()
    }
}

pub struct NumberTypedSlice<'a> {
    t: NumberType,
    _phantom: PhantomData<&'a libc::c_void>,
    _ptr: *const libc::c_void,
    _len: usize,
}

impl<'a> NumberTypedSlice<'a> {
    pub fn try_as_slice<T: NumberTypedT>(&self) -> Output<&'a [T]> {
        self.t.type_check(T::number_type())?;
        let s = unsafe { slice::from_raw_parts(self._ptr as *const T, self._len) };
        Ok(s)
    }

    pub fn try_as_mut_slice<T: NumberTypedT>(&self) -> Output<&'a mut [T]> {
        self.t.type_check(T::number_type())?;
        let s = unsafe { slice::from_raw_parts_mut(self._ptr as *mut T, self._len) };
        Ok(s)
    }
}

pub struct NumberTypedVec {
    t: NumberType,
    _ptr: *mut libc::c_void,
    _len: usize,
    _cap: usize,
}

impl NumberTypedVec {
    pub fn len(&self) -> usize {
        self._len
    }

    pub fn capacity(&self) -> usize {
        self._cap
    }

    pub fn try_to_vec<T: NumberTypedT>(self) -> Output<Vec<T>> {
        self.t.type_check(T::number_type())?;
        let v = unsafe { Vec::from_raw_parts(self._ptr as *mut T, self._len, self._cap) };
        std::mem::forget(self);
        Ok(v)
    }

    pub fn as_number_typed_slice<'a>(&self) -> NumberTypedSlice<'a> {
        NumberTypedSlice {
            t: self.t,
            _ptr: self._ptr as *const libc::c_void,
            _len: self._len,
            _phantom: PhantomData
        }
    }

    pub fn try_as_slice<T: NumberTypedT>(&self) -> Output<&[T]> {
        self.as_number_typed_slice().try_as_slice()
    }

    pub fn try_as_mut_slice<T: NumberTypedT>(&mut self) -> Output<&mut [T]> {
        self.as_number_typed_slice().try_as_mut_slice()
    }
}


impl<T: NumberTypedT> From<Vec<T>> for NumberTypedVec {
    fn from(mut v: Vec<T>) -> NumberTypedVec {
        let ntv = NumberTypedVec{
            t: T::number_type(),
            _ptr: v.as_mut_ptr() as *mut libc::c_void,
            _len: v.len(),
            _cap: v.capacity()
        };
        std::mem::forget(v);
        ntv
    }
}

unsafe fn _ntv_drop<T: NumberTypedT>(ntv: &mut NumberTypedVec) {
    ntv.number_type().type_check(T::number_type()).unwrap();
    Vec::from_raw_parts(ntv._ptr as *const _ as *mut T, ntv._len, ntv._cap);
}

impl NumberTyped for NumberTypedVec {
    fn number_type(&self) -> NumberType {
        self.t
    }
}

impl Drop for NumberTypedVec {
    fn drop(&mut self) {
        unsafe {
            apply_number_type!(self.t, _ntv_drop, [self])
        };
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

    #[test]
    fn apply_number_type_macro_works_with_a_variable() {
        let a = cl_bool::number_type();
        let t = apply_number_type!(a, test_func_to_be_applied, []);
        assert_eq!(t, NumberType::ClUint);
    }
}