// The only file other than the `crate::ffi` module that "knows about" cl_* ffi
// number types. All other code in this project should use the newtype Cl*
// number types. The Cl* number types are decorated with #[repr(transparent)]
// meaning the newtypes number structs are laid out in memory the same as the
// cl_* ffi number types and thus can be directly passing into the OpenCL C FFI
// functions.

use std::cmp::Ordering;
use std::fmt::Debug;

use std::hash::Hash;

use libc::size_t;

use crate::numbers::cl_primitives::{
    cl_char, cl_double, cl_float, cl_int, cl_long, cl_short, cl_uchar, cl_uint, cl_ulong, cl_ushort,
};

use crate::numbers::half::Half;

use crate::numbers::{NumCast, NumCastFrom, NumCastInto, Number, NumberOps, One, Zero};

use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, Sum,
};

macro_rules! __defstruct {
    (signed, $name:ident, $cl_type:ty) => {
        #[derive(
            Copy,
            Clone,
            Hash,
            Debug,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            Display,
            Add,
            AddAssign,
            Sub,
            SubAssign,
            Mul,
            MulAssign,
            Div,
            DivAssign,
            Rem,
            RemAssign,
            Shr,
            ShrAssign,
            BitAnd,
            BitAndAssign,
            BitOr,
            BitOrAssign,
            BitXor,
            BitXorAssign,
            Neg,
            Not,
            Sum,
        )]
        #[repr(transparent)]
        pub struct $name($cl_type);
    };
    (unsigned, $name:ident, $cl_type:ty) => {
        #[derive(
            Copy,
            Clone,
            Hash,
            Debug,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            Display,
            Add,
            AddAssign,
            Sub,
            SubAssign,
            Mul,
            MulAssign,
            Div,
            DivAssign,
            Rem,
            RemAssign,
            Shr,
            ShrAssign,
            Shl,
            ShlAssign,
            BitAnd,
            BitAndAssign,
            BitOr,
            BitOrAssign,
            BitXor,
            BitXorAssign,
            Not,
            Sum,
        )]
        #[repr(transparent)]
        pub struct $name($cl_type);
    };
    (float, $name:ident, $cl_type:ty) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Display,
            Add,
            AddAssign,
            Sub,
            SubAssign,
            Mul,
            MulAssign,
            Div,
            DivAssign,
            Rem,
            RemAssign,
            Shr,
            ShrAssign,
            Shl,
            ShlAssign,
            Neg,
            Sum,
        )]
        #[repr(transparent)]
        pub struct $name($cl_type);

        impl Eq for $name {}

        impl Ord for $name {
            #[inline(always)]
            fn cmp(&self, other: &$name) -> Ordering {
                match self {
                    x if x > other => Ordering::Greater,
                    x if x < other => Ordering::Less,
                    _ => Ordering::Equal,
                }
            }
        }
    };
}

// #[derive(
//   Copy, Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd,
//   Add,
//   Sub,
//   Mul,
//   Div,
//   Rem,
//   Shr,
//   Shl,
//   BitAnd,
//   BitOr,
//   BitXor,
//   Not,
//   Sum,
// )]
// #[repr(transparent)]
// pub struct $new_type($cl_type);

macro_rules! __defimpl {
    ($new_type:ident, $cl_type:ty) => {
        impl Number for $new_type {
            type Scalar = $new_type;
            type Outer = $cl_type;

            #[inline(always)]
            fn new(val: Self::Outer) -> Self {
                $new_type(val)
            }

            #[inline(always)]
            fn into_outer(self) -> Self::Outer {
                self.0
            }
        }

        impl<T> NumCastFrom<T> for $new_type
        where
            T: NumCast,
        {
            fn num_cast_from(val: T) -> Option<$new_type> {
                Some($new_type(NumCast::from(val)?))
            }
        }

        impl<T> NumCastFrom<$new_type> for T
        where
            T: NumCast,
        {
            fn num_cast_from(val: $new_type) -> Option<T> {
                NumCast::from(val.0)
            }
        }

        impl From<$cl_type> for $new_type {
            #[inline(always)]
            fn from(val: $cl_type) -> $new_type {
                $new_type(val)
            }
        }

        impl From<$new_type> for $cl_type {
            #[inline(always)]
            fn from(val: $new_type) -> $cl_type {
                val.0
            }
        }
        impl Default for $new_type {
            #[inline(always)]
            fn default() -> $new_type {
                $new_type(<$cl_type>::default())
            }
        }

        impl NumberOps for $new_type {}
    };
}

macro_rules! impl_ops {
    ($name:ident, $inner:ty) => {
        impl std::ops::Mul<$name> for $name {
            type Output = $name;
            #[inline(always)]
            fn mul(self, other: $name) -> Self::Output {
                $name(self.0 * other.0)
            }
        }

        impl std::ops::Add<$inner> for $name {
            type Output = $name;
            #[inline(always)]
            fn add(self, other: $inner) -> Self::Output {
                $name(self.0 + other)
            }
        }

        impl std::ops::Div<$name> for $name {
            type Output = $name;
            #[inline(always)]
            fn div(self, other: $name) -> Self::Output {
                $name(self.0 / other.0)
            }
        }

        impl std::ops::Rem<$name> for $name {
            type Output = $name;
            #[inline(always)]
            fn rem(self, other: $name) -> Self::Output {
                $name(self.0 % other.0)
            }
        }

        impl std::ops::RemAssign<$name> for $name {
            #[inline(always)]
            fn rem_assign(&mut self, other: $name) {
                *self = *self % other;
            }
        }

        impl std::ops::DivAssign<$name> for $name {
            #[inline(always)]
            fn div_assign(&mut self, other: $name) {
                *self = *self / other;
            }
        }

        impl std::ops::MulAssign<$name> for $name {
            #[inline(always)]
            fn mul_assign(&mut self, other: $name) {
                *self = *self * other;
            }
        }

        impl One for $name {
            #[inline(always)]
            fn one() -> $name {
                $name(One::one())
            }
        }

        impl Zero for $name {
            #[inline(always)]
            fn zero() -> $name {
                $name(Zero::zero())
            }

            fn is_zero(&self) -> bool {
                self.0 == Zero::zero()
            }
        }
    };
}

macro_rules! impl_num_cast_from {
    ($name:ident) => {
        impl_num_cast_from!(
            $name,
            [Char, Uchar, Short, Ushort, Int, Uint, Long, Ulong, Float, Double]
        );
    };

    ($name:ident, [ $( $other:ident ),+ ]) => {
        $(
            impl NumCastFrom<$other> for $name {
                #[inline]
                fn num_cast_from(val: $other) -> Option<$name> {
                    Some($name(val.0.num_cast_into()?))
                }
            }
        )+
    };
}

macro_rules! define_signed_newtypes {
  { $( $new_type:ident => $cl_type:ty),* } => {
    $(
      __defstruct!(signed, $new_type, $cl_type);
      __defimpl!($new_type, $cl_type);
      impl_ops!($new_type, $cl_type);
    //   impl_int_ops!($new_type, $cl_type);
    //   impl_to_primitive!($new_type);
      impl_num_cast_from!($new_type);
    )*
  }
}

macro_rules! define_unsigned_newtypes {
  { $( $new_type:ident => $cl_type:ty),* } => {
    $(
      __defstruct!(unsigned, $new_type, $cl_type);
      __defimpl!($new_type, $cl_type);
      impl_ops!($new_type, $cl_type);
      impl_num_cast_from!($new_type);
    )*
  }
}

macro_rules! define_float_newtypes {
  { $( $new_type:ident => $cl_type:ty),* } => {
    $(
      __defstruct!(float, $new_type, $cl_type);
      __defimpl!($new_type, $cl_type);
      impl_ops!($new_type, $cl_type);
      impl_num_cast_from!($new_type);
    )*
  }
}

impl_num_cast_from!(Half);

// TODO: define Bool => cl_bool
// TODO: define Double => cl_double
// TODO: define Float => cl_float

define_signed_newtypes! {
    Char => cl_char,
    Short => cl_short,
    Int => cl_int,
    Long => cl_long
}

define_unsigned_newtypes! {
  SizeT => size_t,
  Uchar => cl_uchar,
  Ushort => cl_ushort,
  Uint => cl_uint,
  Ulong => cl_ulong
}

define_float_newtypes! {
  Float => cl_float,
  Double => cl_double
}

#[cfg(test)]
mod scalars_tests {
    use super::*;
    use crate::numbers::NumCastFrom;

    #[test]
    fn test_impl_add() {
        let n1 = Char(2i8);
        let n2 = Char(2i8);
        let n3 = n1 + n2;
        assert_eq!(n3, Char(4i8));
    }

    #[test]
    fn test_impl_sub() {
        let n1 = Char(2i8);
        let n2 = Char(2i8);
        let n3 = n1 - n2;
        assert_eq!(n3, Char(0i8));
    }

    #[test]
    fn test_impl_div() {
        let n1 = Char(2i8);
        let n2 = Char(2i8);
        let n3 = n1 / n2;
        assert_eq!(n3, Char(1i8));
    }

    #[test]
    fn test_impl_mul() {
        let n1 = Char(2i8);
        let n2 = Char(2i8);
        let n3 = n1 * n2;
        assert_eq!(n3, Char(4i8));
    }

    #[test]
    fn test_impl_rem() {
        let n1 = Char(7i8);
        let n2 = Char(4i8);
        let n3 = n1 % n2;
        assert_eq!(n3, Char(3i8));
    }

    #[test]
    fn test_impl_zero() {
        assert_eq!(Char::zero(), Char(0));
    }

    #[test]
    fn test_impl_one() {
        assert_eq!(Char::one(), Char(1));
    }

    // #[test]
    // fn test_impl_shl() {
    //     let n1 = Char(1i8);
    //     let n2 = 1i8;
    //     let n3 = n1 << n2;
    //     assert_eq!(n3, Char(2i8));
    // }
    //
    #[test]
    fn test_impl_num_cast() {
        let n1 = Char(0i8);
        let n2: Ushort = NumCastFrom::num_cast_from(n1).unwrap();
        assert_eq!(n2, Ushort(0));
    }
}
