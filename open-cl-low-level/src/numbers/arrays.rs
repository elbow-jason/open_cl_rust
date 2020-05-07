use std::fmt;
use std::ops::*;

use num_traits::{Zero, One};
use num_traits::cast::{ToPrimitive, NumCast};

// use super::newtypes::{NumberNewType, Uchar};

use super::cl_primitives::{
  ClPrimitiveNumber,
  cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float, cl_double
};

use std::mem::zeroed;


macro_rules! get_index {
  (0, $vector:expr) => { $vector.0 };
  (1, $vector:expr) => { $vector.1 };
  (2, $vector:expr) => { $vector.2 };
  (3, $vector:expr) => { $vector.3 };
  (4, $vector:expr) => { $vector.4 };
  (5, $vector:expr) => { $vector.5 };
  (6, $vector:expr) => { $vector.6 };
  (7, $vector:expr) => { $vector.7 };
  (8, $vector:expr) => { $vector.8 };
  (9, $vector:expr) => { $vector.9 };
  (10, $vector:expr) => { $vector.10 };
  (11, $vector:expr) => { $vector.11 };
  (12, $vector:expr) => { $vector.12 };
  (13, $vector:expr) => { $vector.13 };
  (14, $vector:expr) => { $vector.14 };
  (15, $vector:expr) => { $vector.15 };
}

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector2<T: ClPrimitiveNumber>(T, T);

// 3 has 4 slots because we do not align memory correctly.
// #[derive(Clone, Copy, Hash)]
// #[repr(C)]
// pub struct ClVector3<T: ClPrimitiveNumber>(T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector4<T: ClPrimitiveNumber>(T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector8<T: ClPrimitiveNumber>(T, T, T, T, T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector16<T: ClPrimitiveNumber>(
  T, T, T, T,
  T, T, T, T,
  T, T, T, T,
  T, T, T, T
);


pub trait ClVector<T: ClPrimitiveNumber, const N: usize> {
  fn new(val: [T; N]) -> Self;
  fn into_array(self) -> [T; N];
  fn as_array(&self) -> [T; N];
}


macro_rules! def_array {
  // { name: $name:ident, size: 3 } => { def_array!($name,  4, [0, 1, 2], [0, 1, 2, 3]); };
  { name: $name:ident, size: 1 } => { def_array!($name, 1, [0]); };
  { name: $name:ident, size: 2 } => { def_array!($name, 2, [0, 1]); };
  { name: $name:ident, size: 4 } => { def_array!($name, 4, [0, 1, 2, 3]); };
  { name: $name:ident, size: 8 } => { def_array!($name, 8, [0, 1, 2, 3, 4, 5, 6, 7]); };
  { name: $name:ident, size: 16 } => { def_array!($name, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]); };

  // $keep_i copies the given index 
  // ($name:ident, $outer_size:expr, $inner_size:expr, [$( $outer_i:expr ),+]) => {
  //   def_array!($name, $outer_size, $inner_size, [$( $outer_i ),+], [$( $outer_i ),+]);
  // };
  ($name:ident, $n:expr, [$( $i:expr ),+]) => {
    impl_index!($name, $n, [$( $i ),+]);
    impl_index_mut!($name, $n, [$( $i ),+]);
    impl_cl_vector!($name, $n, [$( $i ),+]);
    impl_zero!($name, [$( $i ),+]);
    impl_one!($name, [$( $i ),+]);
    impl_debug!($name, [$( $i ),+]);
    impl_cmp!($name, [$( $i ),+]);
    impl_add!($name, [$( $i ),+]);
    impl_sub!($name, [$( $i ),+]);
    impl_mul!($name, [$( $i ),+]);
    impl_div!($name, [$( $i ),+]);
    impl_rem!($name, [$( $i ),+]);
    impl_neg!($name, [$( $i ),+]);
    impl_not!($name, [$( $i ),+]);
    impl_assigns_ops!($name);
    impl_shl_signed!($name, [cl_char, cl_short, cl_int, cl_long], [$( $i ),+]);
    impl_shl_unsigned!($name, [cl_uchar, cl_ushort, cl_uint, cl_ulong], [$( $i ),+]);
    impl_shl_assign!($name);
    impl_shr_signed!($name, [cl_char, cl_short, cl_int, cl_long], [$( $i ),+]);
    impl_shr_unsigned!($name, [cl_uchar, cl_ushort, cl_uint, cl_ulong], [$( $i ),+]);
    impl_shr_assign!($name);
    rust_array_conv!($name, $n);
    impl_array_cast!($name, [$( $i ),+]);
  };
}

macro_rules! impl_add {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Add for $name<T> {
      type Output = $name<T>;

      fn add(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] + other[$i], )+ )
      }
    }
  }
}

macro_rules! impl_sub {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Sub for $name<T> {
      type Output = $name<T>;

      fn sub(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] - other[$i], )+ )
      }
    }
  }
}

macro_rules! impl_mul {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Mul for $name<T> {
      type Output = $name<T>;

      fn mul(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] * other[$i], )+ )
      }
    }
  }
}

macro_rules! impl_div {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Div for $name<T> {
      type Output = $name<T>;

      fn div(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] / other[$i], )+ )
      }
    }
  }
}
macro_rules! impl_neg {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Neg for $name<T> {
      type Output = $name<T>;
      #[inline(always)]
      fn neg(self) -> $name<T> {
        let zero: T = Zero::zero();
        $name( $( zero - self[$i] ),+ )
      }
    }
  }
}

macro_rules! impl_rem {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Rem for $name<T> {
      type Output = $name<T>;
      #[inline(always)]
      fn rem(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] % other[$i] ),+ )
      }
    }
  }
}

macro_rules! impl_index {
  ($name:ident, $len:expr, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> Index<usize> for $name<T> {
      type Output = T;

      fn index(&self, index: usize) -> &Self::Output {
        paste::item! {
          $(
            if $i == index {
              return &get_index!($i, self)
            }
          )+
        }
        panic!("{} index out of bounds. (max_index: {}. got: {})", stringify!($name), $len - 1, index);
      }
    }
  }
}

macro_rules! impl_index_mut {
  ($name:ident, $len:expr, [$( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber> IndexMut<usize> for $name<T> {
      fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        paste::item! {
          $(
            if $i == index {
              return &mut get_index!($i, self)
            }
          )+
        }
        panic!("{} index out of bounds. (max_index: {}. got: {})", stringify!($name), $len - 1, index);
      }
    }
  }
}

macro_rules! impl_cl_vector {
  ($name:ident, $n:expr, [ $( $i:expr ),+ ]) => {
    impl<T: ClPrimitiveNumber> ClVector<T, $n> for $name<T> {
      fn new(val: [T; $n]) -> Self {
        $name($( val[$i] ),+)
      }

      fn into_array(self) -> [T; $n] {
        [ $( self[$i] ),+ ]
      }

      fn as_array(&self) -> [T; $n] {
        [ $( self[$i] ),+ ]
      }
    }
  }
}

macro_rules! impl_debug {
  ($name:ident, [ $( $i:expr ),+ ]) => {
    impl<T: ClPrimitiveNumber> fmt::Debug for $name<T> {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", stringify!($name), self.as_array())
      }
    }
  }
}

macro_rules! impl_zero {
  ($name:ident, [ $( $i:expr ),+ ]) => {
    impl<T: ClPrimitiveNumber> Zero for $name<T> {
      fn zero() -> $name<T> {
        unsafe { zeroed() }
      }

      fn is_zero(&self) -> bool {
        *self == Self::zero()
      }
    }
  }
}

macro_rules! impl_one {
  ($name:ident, [ $( $i:expr ),+ ]) => {
    impl<T: ClPrimitiveNumber> One for $name<T> {
      fn one() -> $name<T> {
        let mut output: $name<T> = Zero::zero();
        let one: T = One::one();
        $(
          output[$i] = one;
        )+
        output
      }
    }
  }
}


macro_rules! impl_cmp {
  ($name:ident, [$( $i:expr ),+]) => {

    impl<T: ClPrimitiveNumber> PartialEq for $name<T> {
        fn eq(&self, other: &$name<T>) -> bool {
          $( (self[$i] == other[$i]) & )+ true
        }
    }

    impl<T: ClPrimitiveNumber> Eq for $name<T> {}
  }
}

macro_rules! impl_shl_assign {
  ($name:ident) => {
    impl<T: ClPrimitiveNumber + Shl> ShlAssign<T> for $name<T> where Self: Shl<T, Output=Self> {
      #[inline(always)]
      fn shl_assign(&mut self, rhs: T) {
          *self = *self << rhs;
      }
    }
  }
}

macro_rules! impl_shl_signed {
  ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
    $(
      impl_shl_signed!($name, $t, $indexes);
    )*
  };
  ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
    impl Shl<$t> for $name<$t> {
      type Output = $name<$t>;

      fn shl(self, shift_by: $t) -> $name<$t> {
        if shift_by < <$t>::zero() {
          $name( $( self[$i] >> -shift_by ),+ )
        } else {
          $name( $( self[$i] << shift_by ),+ )
        }
      }
    }
  };
}

macro_rules! impl_shl_unsigned {
  ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
    $(
      impl_shl_unsigned!($name, $t, $indexes);
    )*
  };
  ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
    impl Shl<$t> for $name<$t>  {
      type Output = $name<$t>;

      fn shl(self, shift_by: $t) -> $name<$t> {
        $name( $( self[$i] << shift_by ),+ )
      }
    }
  }
}

macro_rules! impl_shr_signed {
  ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
    $(
      impl_shr_signed!($name, $t, $indexes);
    )*
  };
  ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
    impl Shr<$t> for $name<$t> {
      type Output = $name<$t>;

      #[inline(always)]
      fn shr(self, shift_by: $t) -> $name<$t> {
        if shift_by < <$t>::zero() {
          $name( $( self[$i] << -shift_by),+ )
        } else {
          $name( $( self[$i] >> shift_by ),+ )
        }
      }
    }
  }
}

macro_rules! impl_shr_unsigned {
  ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
    $(
      impl_shr_unsigned!($name, $t, $indexes);
    )*
  };
  ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
    impl Shr<$t> for $name<$t>  {
      type Output = $name<$t>;

      fn shr(self, shift_by: $t) -> $name<$t> {
        $name( $( self[$i] >> shift_by ),+ )
      }
    }
  }
}

macro_rules! impl_shr_assign {
  ($name:ident) => {
    impl<T: ClPrimitiveNumber + Shr> ShrAssign<T> for $name<T> where Self: Shr<T, Output=Self> {

      #[inline(always)]
      fn shr_assign(&mut self, shift_by: T) {
          *self = *self >> shift_by;
      }
    }
  }
}

macro_rules! rust_array_conv {
  ($name:ident, $n:expr) => {
    impl<T: ClPrimitiveNumber> From<[T; $n]> for $name<T> {
      fn from(val: [T; $n]) -> $name<T> {
        $name::new(val)
      }
    }

    impl<T: ClPrimitiveNumber> From<$name<T>> for [T; $n] {
      fn from(val: $name<T>) -> [T; $n] {
        val.into()
      }
    }
  }
}

macro_rules! impl_assigns_ops {
  ($name:ident) => {
    impl<T: ClPrimitiveNumber> AddAssign for $name<T> {
      #[inline(always)]
      fn add_assign(&mut self, other: $name<T>) {
          *self = *self + other;
      }
    }

    impl<T: ClPrimitiveNumber> SubAssign for $name<T> {
      #[inline(always)]
      fn sub_assign(&mut self, other: $name<T>) {
        *self = *self - other;
      }
    }
    
    impl<T: ClPrimitiveNumber> MulAssign for $name<T> {
      #[inline(always)]
      fn mul_assign(&mut self, other: $name<T>) {
        *self = *self * other;
      }
    }

    impl<T: ClPrimitiveNumber> RemAssign for $name<T> {
      #[inline(always)]
      fn rem_assign(&mut self, other: $name<T>) {
        *self = *self % other;
      }
    }

    impl<T: ClPrimitiveNumber> DivAssign for $name<T> {
      #[inline(always)]
      fn div_assign(&mut self, other: $name<T>) {
        *self = *self / other;
      }
    }
  }
}

macro_rules! impl_not {
  ($name:ident, [ $( $i:expr ),+ ]) => {
    impl<T: ClPrimitiveNumber + Not<Output=T>> Not for $name<T> {
      type Output = $name<T>;

      #[inline(always)]
      fn not(self) -> $name<T> {
        $name($( !self[$i] ),+)
      }
    }
  }
}

pub trait ArrayCast<T> {
  fn cast(&self) -> Option<T>;
  fn cast_into(self) -> Option<T>;
}

macro_rules! impl_array_cast {
  ($name:ident, [ $( $i:expr ),+]) => {
    impl<T: ClPrimitiveNumber, U: ClPrimitiveNumber> ArrayCast<$name<U>> for $name<T> {
      fn cast(&self) -> Option<$name<U>> {
        Some($name( $( NumCast::from(self[$i])? ),+ ))
      }
      fn cast_into(self) -> Option<$name<U>> {
        Some($name( $( NumCast::from(self[$i])? ),+ ))
      }
    }
  }
}


def_array! { name: ClVector2, size: 2 }
def_array! { name: ClVector4, size: 4 }
def_array! { name: ClVector8, size: 8 }
def_array! { name: ClVector16, size: 16 }

// def_array! { name: ClVector3, size: 3 }

#[cfg(test)]
mod arrays_tests {
  // use std::ops::*;
  use super::*;
  // use num_traits::*;

  #[test]
  fn test_add_works_for_array_cl_uchar_cl_array_2() {
    let arr1 = ClVector2(2u8, 1u8);
    let arr2 = ClVector2(2u8, 3u8);
    let arr3 = arr1 + arr2;
    assert_eq!(arr3, ClVector2::new([4u8; 2]))
  }
}

