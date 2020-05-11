/// Low level implementations of sized,
use std::fmt;
use std::ops::*;

use crate::numbers::{NumCastFrom, NumCastInto, One, Zero};

use super::scalar_traits::Scalar;
use crate::numbers::{Char, Double, Float, Int, Long, Short, Uchar, Uint, Ulong, Ushort};

use std::mem::zeroed;

macro_rules! get_index {
    (0, $vector:expr) => {
        $vector.0
    };
    (1, $vector:expr) => {
        $vector.1
    };
    (2, $vector:expr) => {
        $vector.2
    };
    (3, $vector:expr) => {
        $vector.3
    };
    (4, $vector:expr) => {
        $vector.4
    };
    (5, $vector:expr) => {
        $vector.5
    };
    (6, $vector:expr) => {
        $vector.6
    };
    (7, $vector:expr) => {
        $vector.7
    };
    (8, $vector:expr) => {
        $vector.8
    };
    (9, $vector:expr) => {
        $vector.9
    };
    (10, $vector:expr) => {
        $vector.10
    };
    (11, $vector:expr) => {
        $vector.11
    };
    (12, $vector:expr) => {
        $vector.12
    };
    (13, $vector:expr) => {
        $vector.13
    };
    (14, $vector:expr) => {
        $vector.14
    };
    (15, $vector:expr) => {
        $vector.15
    };
}

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector2<T: Scalar>(T, T);

// 3 has 4 slots because we do not align memory correctly.
// #[derive(Clone, Copy, Hash)]
// #[repr(C)]
// pub struct ClVector3<T: Scalar>(T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector4<T: Scalar>(T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector8<T: Scalar>(T, T, T, T, T, T, T, T);

#[derive(Clone, Copy, Hash)]
#[repr(C)]
pub struct ClVector16<T: Scalar>(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T);

pub trait ClVector<T: Scalar, const N: usize> {
    fn new(val: [T; N]) -> Self;
    fn into_array(self) -> [T; N];
    fn as_array(&self) -> [T; N];
}

macro_rules! vector_n_trait {
    ($n:expr) => {
        paste::item! {
            pub trait [<Vector $n>] {}
            impl [<Vector $n>] for [<ClVector $n>]<Char> {}
            impl [<Vector $n>] for [<ClVector $n>]<Uchar> {}
            impl [<Vector $n>] for [<ClVector $n>]<Short> {}
            impl [<Vector $n>] for [<ClVector $n>]<Ushort> {}
            impl [<Vector $n>] for [<ClVector $n>]<Int> {}
            impl [<Vector $n>] for [<ClVector $n>]<Uint> {}
            impl [<Vector $n>] for [<ClVector $n>]<Long> {}
            impl [<Vector $n>] for [<ClVector $n>]<Ulong> {}
            impl [<Vector $n>] for [<ClVector $n>]<Float> {}
            impl [<Vector $n>] for [<ClVector $n>]<Double> {}
        }
    };
}

vector_n_trait!(2);
vector_n_trait!(4);
vector_n_trait!(8);
vector_n_trait!(16);

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
    impl_mul!($name, [$( $i ),+]);
    // impl_sub!($name, [$( $i ),+]);
    // impl_div!($name, [$( $i ),+]);
    // impl_rem!($name, [$( $i ),+]);
    // impl_neg!($name, [$( $i ),+]);
    // impl_not!($name, [$( $i ),+]);
    impl_assigns_ops!($name);
    // impl_shl_signed!($name, [Char, Short, Int, Long], [$( $i ),+]);
    // impl_shl_unsigned!($name, [Uchar, Ushort, Uint, Ulong], [$( $i ),+]);
    // impl_shl_assign!($name);
    // impl_shr_signed!($name, [Char, Short, Int, Long], [$( $i ),+]);
    // impl_shr_unsigned!($name, [Uchar, Ushort, Uint, Ulong], [$( $i ),+]);
    // impl_shr_assign!($name);
    rust_array_conv!($name, $n);
    impl_array_num_cast_from!($name, $n, [$( $i ),+]);
    impl_default!($name, $n);
  };
}

macro_rules! impl_default {
    ($name:ident, $n:expr) => {
        impl<T: Scalar> Default for $name<T> {
            fn default() -> $name<T> {
                $name::new([Default::default(); $n])
            }
        }
    };
}

macro_rules! impl_add {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: Scalar> Add for $name<T> {
      type Output = $name<T>;

      fn add(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] + other[$i], )+ )
      }
    }
  }
}

// macro_rules! impl_sub {
//   ($name:ident, [$( $i:expr ),+]) => {
//     impl<T: Scalar> Sub for $name<T> {
//       type Output = $name<T>;

//       fn sub(self, other: $name<T>) -> $name<T> {
//         $name( $( self[$i] - other[$i], )+ )
//       }
//     }
//   }
// }

macro_rules! impl_mul {
  ($name:ident, [$( $i:expr ),+]) => {
    impl<T: Scalar> Mul for $name<T> {
      type Output = $name<T>;

      fn mul(self, other: $name<T>) -> $name<T> {
        $name( $( self[$i] * other[$i], )+ )
      }
    }
  }
}

// macro_rules! impl_div {
//   ($name:ident, [$( $i:expr ),+]) => {
//     impl<T: Scalar> Div for $name<T> {
//       type Output = $name<T>;

//       fn div(self, other: $name<T>) -> $name<T> {
//         $name( $( self[$i] / other[$i], )+ )
//       }
//     }
//   }
// }
// macro_rules! impl_neg {
//   ($name:ident, [$( $i:expr ),+]) => {
//     impl<T: Scalar> Neg for $name<T> {
//       type Output = $name<T>;
//       #[inline(always)]
//       fn neg(self) -> $name<T> {
//         let zero: T = Zero::zero();
//         $name( $( zero - self[$i] ),+ )
//       }
//     }
//   }
// }

// macro_rules! impl_rem {
//   ($name:ident, [$( $i:expr ),+]) => {
//     impl<T: Scalar> Rem for $name<T> {
//       type Output = $name<T>;
//       #[inline(always)]
//       fn rem(self, other: $name<T>) -> $name<T> {
//         $name( $( self[$i] % other[$i] ),+ )
//       }
//     }
//   }
// }

macro_rules! impl_index {
  ($name:ident, $len:expr, [$( $i:expr ),+]) => {
    impl<T: Scalar> Index<usize> for $name<T> {
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
    impl<T: Scalar> IndexMut<usize> for $name<T> {
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
    impl<T: Scalar> ClVector<T, $n> for $name<T> {
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
        impl<T: Scalar> fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.as_array())
            }
        }
    };
}

macro_rules! impl_zero {
    ($name:ident, [ $( $i:expr ),+ ]) => {
        impl<T: Scalar> Zero for $name<T> {
            fn zero() -> $name<T> {
                unsafe { zeroed() }
            }

            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    };
}

macro_rules! impl_one {
  ($name:ident, [ $( $i:expr ),+ ]) => {
    impl<T: Scalar> One for $name<T> {
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

    impl<T: Scalar> PartialEq for $name<T> {
        fn eq(&self, other: &$name<T>) -> bool {
          $( (self[$i] == other[$i]) & )+ true
        }
    }

    impl<T: Scalar> Eq for $name<T> {}
  }
}

// macro_rules! impl_shl_assign {
//     ($name:ident) => {
//         impl<T: Scalar + Shl> ShlAssign<T> for $name<T>
//         where
//             Self: Shl<T, Output = Self>,
//         {
//             #[inline(always)]
//             fn shl_assign(&mut self, rhs: T) {
//                 *self = *self << rhs;
//             }
//         }
//     };
// }

// macro_rules! impl_shl_signed {
//   ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
//     $(
//       impl_shl_signed!($name, $t, $indexes);
//     )*
//   };
//   ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
//     impl Shl<$t> for $name<$t> {
//       type Output = $name<$t>;

//       fn shl(self, shift_by: $t) -> $name<$t> {
//         if shift_by < <$t>::zero() {
//           $name( $( self[$i] >> -shift_by ),+ )
//         } else {
//           $name( $( self[$i] << shift_by ),+ )
//         }
//       }
//     }
//   };
// }

// macro_rules! impl_shl_unsigned {
//   ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
//     $(
//       impl_shl_unsigned!($name, $t, $indexes);
//     )*
//   };
//   ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
//     impl Shl<$t> for $name<$t>  {
//       type Output = $name<$t>;

//       fn shl(self, shift_by: $t) -> $name<$t> {
//         $name( $( self[$i] << shift_by ),+ )
//       }
//     }
//   }
// }

// macro_rules! impl_shr_signed {
//   ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
//     $(
//       impl_shr_signed!($name, $t, $indexes);
//     )*
//   };
//   ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
//     impl Shr<$t> for $name<$t> {
//       type Output = $name<$t>;

//       #[inline(always)]
//       fn shr(self, shift_by: $t) -> $name<$t> {
//         if shift_by < <$t>::zero() {
//           $name( $( self[$i] << -shift_by),+ )
//         } else {
//           $name( $( self[$i] >> shift_by ),+ )
//         }
//       }
//     }
//   }
// }

// macro_rules! impl_shr_unsigned {
//   ($name:ident, [ $( $t:ty ),* ], $indexes:tt) => {
//     $(
//       impl_shr_unsigned!($name, $t, $indexes);
//     )*
//   };
//   ($name:ident, $t:ty, [ $( $i:expr ),+ ]) => {
//     impl Shr<$t> for $name<$t>  {
//       type Output = $name<$t>;

//       fn shr(self, shift_by: $t) -> $name<$t> {
//         $name( $( self[$i] >> shift_by ),+ )
//       }
//     }
//   }
// }

// macro_rules! impl_shr_assign {
//     ($name:ident) => {
//         impl<T: Scalar + Shr> ShrAssign<T> for $name<T>
//         where
//             Self: Shr<T, Output = Self>,
//         {
//             #[inline(always)]
//             fn shr_assign(&mut self, shift_by: T) {
//                 *self = *self >> shift_by;
//             }
//         }
//     };
// }

macro_rules! rust_array_conv {
    ($name:ident, $n:expr) => {
        impl<T: Scalar> From<[T; $n]> for $name<T> {
            fn from(val: [T; $n]) -> $name<T> {
                $name::new(val)
            }
        }

        impl<T: Scalar> From<$name<T>> for [T; $n] {
            fn from(val: $name<T>) -> [T; $n] {
                val.into()
            }
        }
    };
}

macro_rules! impl_assigns_ops {
    ($name:ident) => {
        impl<T: Scalar> AddAssign for $name<T> {
            #[inline(always)]
            fn add_assign(&mut self, other: $name<T>) {
                *self = *self + other;
            }
        }

        //         impl<T: Scalar> SubAssign for $name<T> {
        //             #[inline(always)]
        //             fn sub_assign(&mut self, other: $name<T>) {
        //                 *self = *self - other;
        //             }
        //         }

        //         impl<T: Scalar> MulAssign for $name<T> {
        //             #[inline(always)]
        //             fn mul_assign(&mut self, other: $name<T>) {
        //                 *self = *self * other;
        //             }
        //         }

        //         impl<T: Scalar> RemAssign for $name<T> {
        //             #[inline(always)]
        //             fn rem_assign(&mut self, other: $name<T>) {
        //                 *self = *self % other;
        //             }
        //         }

        //         impl<T: Scalar> DivAssign for $name<T> {
        //             #[inline(always)]
        //             fn div_assign(&mut self, other: $name<T>) {
        //                 *self = *self / other;
        //             }
        //         }
    };
}

// macro_rules! impl_not {
//   ($name:ident, [ $( $i:expr ),+ ]) => {
//     impl<T: Scalar + Not<Output=T>> Not for $name<T> {
//       type Output = $name<T>;

//       #[inline(always)]
//       fn not(self) -> $name<T> {
//         $name($( !self[$i] ),+)
//       }
//     }
//   }
// }

macro_rules! impl_array_num_cast_from {
  ($name:ident, $n:expr, [ $( $i:expr ),+]) => {
    impl<T: Scalar, U: Scalar + NumCastInto<T>> NumCastFrom<$name<U>> for $name<T> {
      fn num_cast_from(other: $name<U>) -> Option<$name<T>> {
        Some($name( $( other[$i].num_cast_into()? ),+ ))
      }
    }

    impl<T: Scalar, U: Copy + NumCastInto<T>> NumCastFrom<[U; $n]> for $name<T> {
        fn num_cast_from(arr: [U; $n]) -> Option<$name<T>> {
          Some($name( $( arr[$i].num_cast_into()? ),+ ))
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
    use crate::numbers::cl_vectors::{ClVector, ClVector2};
    use crate::numbers::{Char, Int, Long, Short, Uchar, Uint, Ulong, Ushort};
    use crate::numbers::{NumCastFrom, Number};
    // use num_traits::*;

    fn constants<T: Number + NumCastFrom<usize>>(high: usize) -> Vec<T> {
        let plus_one = high + 1;
        let mut output = Vec::with_capacity(plus_one);
        for i in 0..plus_one {
            output.push(T::num_cast_from(i).unwrap());
        }
        output
    }

    macro_rules! __run_ops2 {
    ($( $scalar:ident ),*) => {
      paste::item! {
        $(
            #[allow(non_snake_case)]
            #[test]
            fn [<test_add_work_for_array_ $scalar _ClVector2>]() {
                let c: Vec<$scalar> = constants(4);
                let arr1 = ClVector2(c[2], c[1]);
                let arr2 = ClVector2(c[2], c[3]);
                let result = arr1 + arr2;
                assert_eq!(result, ClVector2::new([c[4]; 2]));
            }

        //   #[allow(non_snake_case)]
        //   #[test]
        //   fn [<test_mul_work_for_array_ $scalar _ClVector2>]() {
        //     let c: Vec<$scalar> = constants(4);
        //     let arr1 = ClVector2(c[2], c[1]);
        //     let arr2 = ClVector2(c[2], c[3]);
        //     let result = arr1 * arr2;
        //     assert_eq!(result, ClVector2::new([c[4], c[3]]));
        //   }
        )*
      }
    }
  }
    __run_ops2!(Char, Int, Long, Short, Uchar, Uint, Ulong, Ushort);

    // macro_rules! __run_ops4 {
    // ($( $scalar:ident ),*) => {
    //   paste::item! {
    //     $(
    //         #[allow(non_snake_case)]
    //         #[test]
    //         fn [<test_add_work_for_array_ $scalar _ClVector4>]() {
    //             let c: Vec<$scalar> = constants(4);
    //             let arr1 = ClVector4(c[2], c[1], c[4], c[0]);
    //             let arr2 = ClVector4(c[2], c[3], c[0], c[4]);
    //             let result = arr1 + arr2;
    //             assert_eq!(result, ClVector4::new([c[4]; 4]));
    //         }

    //         #[allow(non_snake_case)]
    //         #[test]
    //         fn [<test_mul_work_for_array_ $scalar _ClVector4>]() {
    //             let c: Vec<$scalar> = constants(4);
    //             let arr1 = ClVector4(c[2], c[1], c[4], c[0]);
    //             let arr2 = ClVector4(c[2], c[3], c[0], c[4]);
    //             let result = arr1 * arr2;

    //             assert_eq!(result, ClVector4::new([c[4], c[3], c[0], c[0]]));
    //         }
    //     )*
    //   }
    // }
    // }
    // __run_ops4!(Char, Int, Long, Short, Uchar, Uint, Ulong, Ushort);
}
