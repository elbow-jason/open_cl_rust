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
use crate::ffi::{cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float};

use crate::ffi::{cl_bool, cl_double};
use num_traits::{Zero, One};

pub trait ClFFINumber {}

macro_rules! __ffi_numbers(
  ( $( $ffi_t:ty ),* ) => {
    $(
      impl ClFFINumber for $ffi_t {}
    )*
  }
);

__ffi_numbers!(
  size_t,
  cl_char,
  cl_uchar,
  cl_short,
  cl_ushort,
  cl_int,
  cl_uint,
  cl_long,
  cl_ulong,
  cl_float,
  cl_double
);

pub trait IntoFFINumber<T> where T: ClFFINumber {
  fn into_ffi_number(self) -> T;
}

macro_rules! __define_simple_newtype {
  ($new_type:ident, $cl_type:ty) => {
    #[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

macro_rules! define_simple_newtypes {
  { $( $new_type:ident => $cl_type:ty),* } => {
    $(
      __define_simple_newtype!($new_type, $cl_type);
    )*
  }
}

// TODO: implement a sane Debug
// TODO: implement a sane PartialEq
// TODO: define Bool => cl_bool
// TODO: define Double => cl_double
// TODO: define Float => cl_float

define_simple_newtypes! {
    SizeT => size_t,
    Char => cl_char,
    Uchar => cl_uchar,
    Short => cl_short,
    Ushort => cl_ushort,
    Int => cl_int,
    Uint => cl_uint,
    Long => cl_long,
    Ulong => cl_ulong
}