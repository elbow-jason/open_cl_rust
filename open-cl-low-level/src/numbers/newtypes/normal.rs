// The only file other than the `crate::ffi` module that "knows about" cl_* ffi
// number types. All other code in this project should use the newtype Cl*
// number types. The Cl* number types are decorated with #[repr(transparent)]
// meaning the newtypes number structs are laid out in memory the same as the
// cl_* ffi number types and thus can be directly passing into the OpenCL C FFI
// functions.

use std::fmt::Debug;

use std::hash::Hash;

use libc::size_t;

use anyhow::Result;

use crate::{NumberType, NumberTypedT};

// u8
use crate::ffi::{cl_uchar, cl_uchar16, cl_uchar2, cl_uchar3, cl_uchar4, cl_uchar8};

// i8
use crate::ffi::{cl_char, cl_char16, cl_char2, cl_char3, cl_char4, cl_char8};

// u16
use crate::ffi::{cl_ushort, cl_ushort16, cl_ushort2, cl_ushort3, cl_ushort4, cl_ushort8};

// i16
use crate::ffi::{cl_short, cl_short16, cl_short2, cl_short3, cl_short4, cl_short8};

// u32
use crate::ffi::{cl_uint, cl_uint16, cl_uint2, cl_uint3, cl_uint4, cl_uint8};

// i32
use crate::ffi::{cl_int, cl_int16, cl_int2, cl_int3, cl_int4, cl_int8};

// u64
use crate::ffi::{cl_ulong, cl_ulong16, cl_ulong2, cl_ulong3, cl_ulong4, cl_ulong8};

// i64
use crate::ffi::{cl_long, cl_long16, cl_long2, cl_long3, cl_long4, cl_long8};

// f32
use crate::ffi::{cl_float, cl_float16, cl_float2, cl_float3, cl_float4, cl_float8};

// primitive types
use crate::ffi::{cl_bool, cl_double, cl_half};


pub trait ClFFINumber {}

macro_rules! __ffi_numbers(
  ( $( $ffi_t:ty ),* ) => {
    $(
      impl ClFFINumber for $ffi_t {}
    )*
  }
);

__ffi_numbers!(
  // cl_bool,
  size_t,
  cl_double,
  // cl_half,
  cl_char,
  cl_char2,
  // cl_char3,
  cl_char4,
  cl_char8,
  cl_char16,
  cl_uchar,
  cl_uchar2,
  // cl_uchar3,
  cl_uchar4,
  cl_uchar8,
  cl_uchar16,
  cl_short,
  cl_short2,
  // cl_short3,
  cl_short4,
  cl_short8,
  cl_short16,
  cl_ushort,
  cl_ushort2,
  // cl_ushort3,
  cl_ushort4,
  cl_ushort8,
  cl_ushort16,
  cl_int,
  cl_int2,
  // cl_int3,
  cl_int4,
  cl_int8,
  cl_int16,
  cl_uint,
  cl_uint2,
  // cl_uint3,
  cl_uint4,
  cl_uint8,
  cl_uint16,
  cl_long,
  cl_long2,
  // cl_long3,
  cl_long4,
  cl_long8,
  cl_long16,
  cl_ulong,
  cl_ulong2,
  // cl_ulong3,
  cl_ulong4,
  cl_ulong8,
  cl_ulong16,
  cl_float,
  cl_float2,
  // cl_float3,
  cl_float4,
  cl_float8,
  cl_float16
);

pub trait IntoFFINumber<T> where T: ClFFINumber {
  fn into_ffi_number(self) -> T;
}

macro_rules! __define_normal_newtype {
  ($new_type:ident, $cl_type:ty) => {
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct $new_type($cl_type);

    impl NumberTypedT for $new_type {
      fn number_type() -> NumberType {
        NumberType::new::<$new_type>()
      }
    }

    impl From<$cl_type> for $new_type {
      fn from(val: $cl_type) -> $new_type {
        $new_type(val)
      }
    }

    impl From<$new_type> for $cl_type {
      fn from(val: $new_type) -> $cl_type {
        val.0
      }
    }

    impl NumberNewType for $new_type {
      type Inner = $cl_type;

      fn new(val: Self::Inner) -> Result<Self> {
        Ok($new_type(val))
      }

      fn into_ffi_number(self) -> Self::Inner {
        self.0
      }
    }
  };
}

pub trait NumberNewType: Sized + Copy + Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd {
  type Inner: ClFFINumber;

  fn new(val: Self::Inner) -> Result<Self>;
  fn into_ffi_number(self) -> Self::Inner;
}

macro_rules! define_normal_newtypes {
  { $( $new_type:ident => $cl_type:ty),* } => {
    $(
      __define_normal_newtype!($new_type, $cl_type);
    )*
  }
}

// TODO: implement a sane Debug
// TODO: implement a sane PartialEq
// TODO: define Bool => cl_bool
// TODO: define Double => cl_double
// TODO: define Half => cl_half
// TODO: define Float => cl_float

define_normal_newtypes! {
    
    SizeT => size_t,
    
    Char => cl_char,
    Char2 => cl_char2,
    Char3 => cl_char3,
    Char4 => cl_char4,
    Char8 => cl_char8,
    Char16 => cl_char16,
    Uchar => cl_uchar,
    Uchar2 => cl_uchar2,
    Uchar3 => cl_uchar3,
    Uchar4 => cl_uchar4,
    Uchar8 => cl_uchar8,
    Uchar16 => cl_uchar16,
    Short => cl_short,
    Short2 => cl_short2,
    Short3 => cl_short3,
    Short4 => cl_short4,
    Short8 => cl_short8,
    Short16 => cl_short16,
    Ushort => cl_ushort,
    Ushort2 => cl_ushort2,
    Ushort3 => cl_ushort3,
    Ushort4 => cl_ushort4,
    Ushort8 => cl_ushort8,
    Ushort16 => cl_ushort16,
    Int => cl_int,
    Int2 => cl_int2,
    Int3 => cl_int3,
    Int4 => cl_int4,
    Int8 => cl_int8,
    Int16 => cl_int16,
    Uint => cl_uint,
    Uint2 => cl_uint2,
    Uint3 => cl_uint3,
    Uint4 => cl_uint4,
    Uint8 => cl_uint8,
    Uint16 => cl_uint16,
    Long => cl_long,
    Long2 => cl_long2,
    Long3 => cl_long3,
    Long4 => cl_long4,
    Long8 => cl_long8,
    Long16 => cl_long16,
    Ulong => cl_ulong,
    Ulong2 => cl_ulong2,
    Ulong3 => cl_ulong3,
    Ulong4 => cl_ulong4,
    Ulong8 => cl_ulong8,
    Ulong16 => cl_ulong16,

    Float2 => cl_float2,
    Float3 => cl_float3,
    Float4 => cl_float4,
    Float8 => cl_float8,
    Float16 => cl_float16
}