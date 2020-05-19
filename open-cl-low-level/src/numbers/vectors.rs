use std::fmt;
use std::mem::zeroed;
use std::ops::*;

use crate::numbers::cl_vectors::{ClVector, ClVector16, ClVector2, ClVector4, ClVector8};
use crate::numbers::{Char, Double, Float, Int, Long, Short, Uchar, Uint, Ulong, Ushort};
use crate::numbers::{NumCastFrom, NumCastInto, Number, NumberOps, One, Zero};

macro_rules! define_struct {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            #[derive(Clone, Copy)]
            #[repr(transparent)]
            pub struct [<$scalar $n>]([<ClVector $n>]<$scalar>);
        }
    };
}

macro_rules! impl_number {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Number for [<$scalar $n>] {
                type Scalar = $scalar;
                type Outer = [$scalar; $n];

                #[inline(always)]
                fn new(val: [$scalar; $n]) -> [<$scalar $n>] {
                   [<$scalar $n>]([<ClVector $n>]::new(val))
                }

                #[inline(always)]
                fn into_outer(self) -> [$scalar; $n] {
                    self.0.into_array()
                }
            }
        }
    };
}

macro_rules! impl_num_cast {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl<T> NumCastFrom<[T; $n]> for [<$scalar $n>]
            where
                T: NumCastInto<$scalar> + Copy,
            {
                fn num_cast_from(val: [T; $n]) -> Option<[<$scalar $n>]> {
                    Some([<$scalar $n>]([<ClVector $n>]::num_cast_from(val)?))
                }
            }
        }
    };
}

macro_rules! impl_default {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Default for [<$scalar $n>] {
                #[inline(always)]
                fn default() -> [<$scalar $n>] {
                   [<$scalar $n>]([<ClVector $n>]::default())
                }
            }
        }
    };
}

macro_rules! impl_add {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Add for [<$scalar $n>] {
                type Output = [<$scalar $n>];
                #[inline(always)]
                fn add(self, other: [<$scalar $n>]) -> [<$scalar $n>] {
                   [<$scalar $n>](self.0 + other.0)
                }
            }
        }
    };
}

macro_rules! impl_mul {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Mul for [<$scalar $n>] {
                type Output = [<$scalar $n>];
                fn mul(self, other: [<$scalar $n>]) -> [<$scalar $n>] {
                   [<$scalar $n>](self.0 * other.0)
                }
            }
        }
    };
}

macro_rules! impl_index {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Index<usize> for [<$scalar $n>] {
                type Output = $scalar;
                fn index(&self, index: usize) -> &Self::Output {
                    &self.0[index]
                }
            }
        }
    };
}

macro_rules! impl_index_mut {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl IndexMut<usize> for [<$scalar $n>] {
                fn index_mut(&mut self, index: usize) -> &mut $scalar {
                    &mut self.0[index]
                }
            }
        }
    };
}

macro_rules! impl_debug_and_display {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl fmt::Debug for [<$scalar $n>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}{}({:?})", stringify!($scalar), $n, self.0.as_array())
                }
            }

            impl fmt::Display for [<$scalar $n>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{:?}", self.0.as_array())
                }
            }
        }
    };
}

macro_rules! impl_zero {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl Zero for [<$scalar $n>] {
                #[inline(always)]
                fn zero() -> [<$scalar $n>] {
                    unsafe { zeroed() }
                }
                #[inline(always)]
                fn is_zero(&self) -> bool {
                    *self == Self::zero()
                }
            }
        }
    };
}

macro_rules! impl_one {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl One for [<$scalar $n>] {
                fn one() -> [<$scalar $n>] {
                   [<$scalar $n>]([<ClVector $n>]::one())
                }
            }
        }
    };
}

macro_rules! impl_partial_eq {
    ($scalar:ident, $n:expr) => {
        paste::item! {
            impl PartialEq for [<$scalar $n>] {
                fn eq(&self, other: & [<$scalar $n>]) -> bool {
                    self.0 == other.0
                }
            }
        }
    };
}

macro_rules! impl_number_ops {
    ($scalar:expr, $n:expr) => {
        paste::item! {
            impl NumberOps for [<$scalar $n>] {}
        }
    };
}

macro_rules! impl_from {
    ($scalar:expr, $n:expr) => {
        paste::item! {
            impl From<[$scalar; $n]> for [<$scalar $n>] {
                fn from(val: [$scalar; $n]) -> [<$scalar $n>] {
                   [<$scalar $n>]::new(val)
                }
            }

            impl From<[<$scalar $n>]> for [$scalar; $n] {
                fn from(val: [<$scalar $n>]) -> [$scalar; $n] {
                    val.into()
                }
            }
        }
    };
}

macro_rules! impl_vector {
    ($scalar:ident, $n:expr) => {
        define_struct!($scalar, $n);
        impl_add!($scalar, $n);
        impl_debug_and_display!($scalar, $n);
        impl_default!($scalar, $n);
        impl_index_mut!($scalar, $n);
        impl_index!($scalar, $n);
        impl_mul!($scalar, $n);
        impl_num_cast!($scalar, $n);
        impl_number_ops!($scalar, $n);
        impl_number!($scalar, $n);
        impl_one!($scalar, $n);
        impl_partial_eq!($scalar, $n);
        impl_zero!($scalar, $n);
        impl_from!($scalar, $n);
        paste::item! {
            impl Eq for [<$scalar $n>] {}
        }
    };
    ( $( $scalar:ident ),* ) => {
        $(
            impl_vector!($scalar, 2);
            impl_vector!($scalar, 4);
            impl_vector!($scalar, 8);
            impl_vector!($scalar, 16);
        )*
    };
}

impl_vector!(Char, Double, Float, Int, Long, Short, Uchar, Uint, Ulong, Ushort);

macro_rules! impl_from_rust_array {
    ($scalar:ident, $rust_t:ident) => {
        impl_from_rust_array!($scalar, $rust_t, 2);
        impl_from_rust_array!($scalar, $rust_t, 4);
        impl_from_rust_array!($scalar, $rust_t, 8);
        impl_from_rust_array!($scalar, $rust_t, 16);
    };
    ($scalar:ident, $rust_t:ident, 2) => {
        impl_from_rust_array!($scalar, $rust_t, 2, [0, 1]);
    };

    ($scalar:ident, $rust_t:ident, 4) => {
        impl_from_rust_array!($scalar, $rust_t, 4, [0, 1, 2, 3]);
    };

    ($scalar:ident, $rust_t:ident, 8) => {
        impl_from_rust_array!($scalar, $rust_t, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
    };

    ($scalar:ident, $rust_t:ident, 16) => {
        impl_from_rust_array!(
            $scalar,
            $rust_t,
            16,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    };

    ($scalar:ident, $rust_t:ident, $n:expr, [ $( $index:expr ),+ ]) => {
        paste::item! {
            impl From<[$rust_t; $n]> for [<$scalar $n>] {
                fn from(val: [$rust_t; $n]) -> [<$scalar $n>] {
                    [<$scalar $n>]::new([
                        $(
                            $scalar::new(val[$index])
                        ),+
                    ])
                }
            }
        }
    };
}

impl_from_rust_array!(Char, i8);
impl_from_rust_array!(Uchar, u8);
impl_from_rust_array!(Short, i16);
impl_from_rust_array!(Ushort, u16);
impl_from_rust_array!(Int, i32);
impl_from_rust_array!(Uint, u32);
impl_from_rust_array!(Long, i64);
impl_from_rust_array!(Ulong, u64);
impl_from_rust_array!(Float, f32);
impl_from_rust_array!(Double, f64);
