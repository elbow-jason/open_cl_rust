// use std::convert::{TryFrom, TryInto};
use std::fmt;

use super::cl_number::*;
use crate::{
    AsSlice, ClRustPrimitiveNum, Error, NumChange, Number, NumberType, NumberTypedT, Output, Zeroed,
};

use num_traits::{FromPrimitive, NumCast, ToPrimitive};

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum NumCastError {
    #[fail(display = "Failed to cast number {} from {:?} to {:?}", val, from, to)]
    CastingFailed {
        from: NumberType,
        to: NumberType,
        val: String,
    },
}

impl NumCastError {
    fn casting_failed<T: NumberTypedT + fmt::Debug, U: NumberTypedT>(t: T) -> Error {
        let e = NumCastError::CastingFailed {
            from: T::number_type(),
            to: U::number_type(),
            val: format!("{:?}", t),
        };
        Error::from(e)
    }
}

pub trait TryClCastNumber<T, X = (), Y = ()> {
    fn try_cl_cast_number(self) -> Output<T>;
}

fn _try_cast_number<T, U>(val: T) -> Output<U>
where
    U: NumberTypedT + NumCast,
    T: ToPrimitive + NumberTypedT + fmt::Debug + Copy,
{
    match NumCast::from(val) {
        Some(u) => Ok(u),
        None => NumCastError::casting_failed::<T, U>(val).as_output(),
    }
}

// pub trait TryClCastVector<T, X, Y>
// where
//     Self: ClVector<X>,
//     T: ClVector<Y>,
//     X: TryClCastNumber<Y> + NumberTypedT + ClPrimitive,
//     Y: NumberTypedT + ClPrimitive,
// {
//     fn try_cl_cast_vector(self) -> Output<T>;
// }

// impl TryClCastVector<cl_uchar2, cl_char, cl_uchar> for cl_char2 {
//     fn try_cl_cast_vector(self) -> Output<cl_uchar2> {
//         let rust_val: [cl_uchar; 2] = unsafe {
//             [
//                 self.s[0].try_cl_cast_number()?,
//                 self.s[1].try_cl_cast_number()?,
//             ]
//         };
//         Ok(rust_val.to_cl_num())
//     }
// }

// impl<T, U, X, Y> TryClCastNumber<U, X, Y> for T
// where
//     T: ClVector2<X> + NumChange + NumberTypedT + AsSlice<X>,
//     U: ClVector2<Y> + Zeroed + AsSlice<Y>,
//     X: ClPrimitive + TryClCastNumber<Y>,
//     Y: ClPrimitive,
// {
//     fn try_cl_cast_number(self) -> Output<U> {
//         let mut vector = U::zeroed();
//         let slice_t: &[X] = self.as_slice();
//         let slice_u: &mut [Y] = vector.as_mut_slice();
//         debug_assert!(slice_u.len() == slice_t.len());
//         for (i, item) in slice_t.iter().enumerate() {
//             slice_u[i] = item.try_cl_cast_number()?;
//         }
//         Ok(vector)
//     }
// }

macro_rules! __impl_vector_casting_once {
    ($vector1:ident, $base1:ident, $vector2:ident, $base2:ident) => {
        impl TryClCastNumber<$vector2> for $vector1 {
            fn try_cl_cast_number(self) -> Output<$vector2> {
                let mut vector = $vector2::zeroed();
                let slice_t: &[$base1] = self.as_slice();
                let slice_u: &mut [$base2] = vector.as_mut_slice();
                debug_assert!(slice_u.len() == slice_t.len());
                for (i, item) in slice_t.iter().enumerate() {
                    slice_u[i] = item.try_cl_cast_number()?;
                }
                Ok(vector)
            }
        }
    };
}

macro_rules! __impl_vector_casting_1 {
    ([ $( $base_t1:ident ),*, ], $types:tt) => {
        $(
            __impl_vector_casting_2!($base_t1, $types);
        )*
    };
}

macro_rules! __impl_vector_casting_2 {
    ($base_t1:ident, [ $( $base_t2:ident ),*,]) => {
        $(
            __impl_vector_casting_3!($base_t1, $base_t2);
        )*
    };
}

macro_rules! __impl_vector_casting_3 {
    ($t1:ident, $t2:ident) => {
        __impl_vector_casting_4!(2, $t1, $t2);
        __impl_vector_casting_4!(4, $t1, $t2);
        __impl_vector_casting_4!(8, $t1, $t2);
        __impl_vector_casting_4!(16, $t1, $t2);
    };
}

macro_rules! __impl_vector_casting_4 {
    ($size:expr, $t1:ident, $t2:ident) => {
        paste::item! {
            __impl_vector_casting_once!([<$t1 $size>], $t1, [<$t2 $size>], $t2);
        }
    };
}

macro_rules! __impl_vector_casting {
    ($types:tt) => {
        __impl_vector_casting_1!($types, $types);
    };
}

__impl_vector_casting!([
    cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float,
]);

// __impl_numcast_vector!(ClVector2);
// __impl_numcast_vector!(ClVector4);
// __impl_numcast_vector!(8);
// __impl_numcast_vector!(16);

impl<T, U> TryClCastNumber<U> for T
where
    T: NumCast + NumberTypedT + NumChange + fmt::Debug + Number + ClRustPrimitiveNum,
    U: NumCast + NumberTypedT + NumChange + Number + ClRustPrimitiveNum,
{
    fn try_cl_cast_number(self) -> Output<U> {
        _try_cast_number::<T, U>(self)
    }
}

const F16_MAX: f32 = 65504.0;
const F16_MIN: f32 = -65504.0;

impl<T> TryClCastNumber<f16> for T
where
    T: TryClCastNumber<f32> + NumChange + fmt::Debug,
{
    fn try_cl_cast_number(self) -> Output<f16> {
        let error_func = || Error::from(NumCastError::casting_failed::<T, f16>(self));
        let float: f32 = self.try_cl_cast_number().map_err(|_| error_func())?;
        if float > F16_MAX {
            return error_func().as_output();
        }

        if float < F16_MIN {
            return error_func().as_output();
        }

        Ok(f16::from_f32(float))
    }
}

impl<T> TryClCastNumber<T> for f16
where
    T: NumberTypedT + NumCast + Number + NumChange + ClRustPrimitiveNum + fmt::Debug,
{
    fn try_cl_cast_number(self) -> Output<T> {
        self.to_f32().try_cl_cast_number()
    }
}

impl<S, T, U> TryClCastNumber<Vec<U>> for S
where
    S: Iterator<Item = T>,
    T: TryClCastNumber<U>,
    U: ToPrimitive + FromPrimitive + NumCast + NumberTypedT + NumChange + Number,
{
    fn try_cl_cast_number(self) -> Output<Vec<U>> {
        let mut out: Vec<U> = Vec::new();
        for item in self {
            match item.try_cl_cast_number() {
                Ok(u) => out.push(u),
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}

#[inline]
fn _cl_uint_to_bool<T: NumberTypedT>(val: cl_uint) -> Output<bool> {
    match val {
        0 => Ok(false),
        1 => Ok(true),
        got => {
            let nce = NumCastError::CastingFailed {
                from: T::number_type(),
                to: bool::number_type(),
                val: format!("{:?}", got),
            };
            Err(Error::from(nce))
        }
    }
}

impl<T> TryClCastNumber<bool> for T
where
    T: TryClCastNumber<cl_uint> + NumberTypedT + fmt::Debug + NumChange,
{
    fn try_cl_cast_number(self) -> Output<bool> {
        let val: cl_uint = self.try_cl_cast_number().map_err(|_| {
            let nce = NumCastError::casting_failed::<T, bool>(self);
            Error::from(nce)
        })?;
        _cl_uint_to_bool::<T>(val)
    }
}

// impl<T, U> TryClCastNumber<T> for U
// where
//     T: ClVector2,
//     U: ClVector2,
// {
//     fn try_cl_cast_number(self) -> Output<f16> {
//         // u8 does not go high enough to fail here.
//         Ok(f16::from_f32(self as f32))
//     }
// }

// impl TryClCastNumber<f16> for u8 {
//     fn try_cl_cast_number(self) -> Output<f16> {
//         let float: cl_float = self.try_cl_cast_number().map_err(|_| {
//             let nce = NumCastError::casting_failed::<T, f16>(self);
//             Error::from(nce)
//         })?;
//     }
// }
// impl TryClCastNumber<cl_char> for cl_uchar {
//     fn try_cl_cast_number(self) -> Output<cl_char> {
//         self.try_into()
//             .map_err(|_e| casting_error::<cl_uchar, cl_char>(self).into())
//     }
// }

// impl TryClCastNumber<cl_char2> for cl_uchar2 {
//     fn try_cl_cast_number(self) -> Output<cl_char2> {
//         let data: Vec<cl_char> = self.as_slice().try_cl_cast_number();
//     }
// }

#[cfg(test)]
mod primitive_tests {
    use super::{NumCastError, TryClCastNumber};
    use crate::numbers::cl_number::*;
    use crate::Output;
    use half::f16;
    use libc::size_t;

    macro_rules! __test_casting_f16 {
        ($t:ty) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_f16_to_ $t>]() {
                    let a = f16::from_f32(3.0);
                    let b: f16 = a.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("failed to cast from f16 {:?} to {:?}", e, stringify!($t));
                    });
                    assert_eq!(f32::from(b), 3.0);
                }
            }
        };
    }

    macro_rules! __test_casting {
        ($t1:ty, f16, $good_num:expr, $bad_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_f16>]() {
                    let a: $t1 = 3.0 as $t1;
                    let b: f16 = a.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("failed to cast to f16 {:?}", e);
                    });
                    assert_eq!(f32::from(b), 3.0);
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_bool_when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<f16> = a.try_cl_cast_number();
                    let e = NumCastError::casting_failed::<$t1, f16>(a);
                    assert_eq!(b, Err(e));
                }
            }
        };

        ($t1:ty, bool, $good_num:expr, $bad_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_for_conversion_from_ $t1 _to_bool>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: bool = a.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("failed to cast bool {:?}", e);
                    });
                    assert_eq!(b, true);
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_bool_when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<bool> = a.try_cl_cast_number();
                    let e = NumCastError::casting_failed::<$t1, bool>(a);
                    assert_eq!(b, Err(e));
                }
            }
        };
        ($t1:ty, $t2:ty, $good_num:expr, $bad_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_ $t2>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: Output<$t2> = a.try_cl_cast_number();
                    assert_eq!(b, Ok($good_num as $t2));
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_ $t2 _when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<$t2> = a.try_cl_cast_number();
                    let e = NumCastError::casting_failed::<$t1, $t2>(a);
                    assert_eq!(b, Err(e));
                }
            }
        };
        ($t1:ty, f16, $good_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_f16>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: f16 = a.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("{:?}", e);
                    });
                    assert_eq!(f32::from(b), f32::from(b));
                }
            }
        };

        ($t1:ty, $t2:ty, $good_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_ $t2>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: $t2 = a.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("{:?}", e);
                    });
                    assert_eq!(b as $t1, $good_num);
                }
            }
        };
    }

    const BOOL_OOR: u8 = 2;

    const CL_UCHAR_OOR: usize = cl_uchar::max_value() as usize + 1;
    const CL_USHORT_OOR: usize = cl_ushort::max_value() as usize + 1;
    const CL_UINT_OOR: usize = cl_uint::max_value() as usize + 1;
    const CL_CHAR_OOR: isize = cl_char::max_value() as isize + 1;
    const CL_SHORT_OOR: isize = cl_short::max_value() as isize + 1;
    const CL_INT_OOR: i64 = cl_int::max_value() as i64 + 1;
    const CL_LONG_OOR: usize = cl_long::max_value() as usize + 1;

    const CL_HALF_OOR: isize = cl_half::max_value() as isize + 1;
    fn f16_oor() -> f64 {
        f16::MAX.to_f64() + 1.0f64
    }

    __test_casting!(cl_uchar, cl_uchar, 1);
    __test_casting!(cl_uchar, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_uchar, cl_ushort, 1);
    __test_casting!(cl_uchar, cl_short, 1);
    __test_casting!(cl_uchar, cl_uint, 1);
    __test_casting!(cl_uchar, cl_int, 1);
    __test_casting!(cl_uchar, cl_ulong, 1);
    __test_casting!(cl_uchar, cl_long, 1);
    __test_casting!(cl_uchar, cl_half, 1);
    __test_casting!(cl_uchar, cl_float, 1);
    __test_casting!(cl_uchar, cl_double, 1);
    __test_casting!(cl_uchar, size_t, 1);
    __test_casting!(cl_uchar, f16, 3);
    __test_casting!(cl_uchar, bool, 1, BOOL_OOR);

    __test_casting!(cl_char, cl_uchar, 1, -1);
    __test_casting!(cl_char, cl_char, 1);
    __test_casting!(cl_char, cl_ushort, 1, -1);
    __test_casting!(cl_char, cl_short, 1);
    __test_casting!(cl_char, cl_uint, 1, -1);
    __test_casting!(cl_char, cl_int, 1);
    __test_casting!(cl_char, cl_ulong, 1, -1);
    __test_casting!(cl_char, cl_long, 1);
    __test_casting!(cl_char, size_t, 1, -1);
    __test_casting!(cl_char, cl_half, 1);
    __test_casting!(cl_char, f16, 3);
    __test_casting!(cl_char, cl_float, 1);
    __test_casting!(cl_char, cl_double, 1);
    __test_casting!(cl_char, bool, 1, BOOL_OOR);

    __test_casting!(cl_ushort, cl_uchar, 1);
    __test_casting!(cl_ushort, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_ushort, cl_ushort, 1);
    __test_casting!(cl_ushort, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_ushort, cl_uint, 1);
    __test_casting!(cl_ushort, cl_int, 1);
    __test_casting!(cl_ushort, cl_ulong, 1);
    __test_casting!(cl_ushort, cl_long, 1);
    __test_casting!(cl_ushort, cl_half, 1);
    __test_casting!(cl_ushort, cl_float, 1);
    __test_casting!(cl_ushort, cl_double, 1);
    __test_casting!(cl_ushort, size_t, 1);
    __test_casting!(cl_ushort, f16, 3.0, f16_oor());

    __test_casting!(cl_short, cl_uchar, 1, -1);
    __test_casting!(cl_short, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_short, cl_ushort, 1, -1);
    __test_casting!(cl_short, cl_short, 1);
    __test_casting!(cl_short, cl_uint, 1, -1);
    __test_casting!(cl_short, cl_int, 1);
    __test_casting!(cl_short, cl_ulong, 1, -1);
    __test_casting!(cl_short, cl_long, 1);
    __test_casting!(cl_short, size_t, 1, -1);
    __test_casting!(cl_short, cl_half, 1);
    __test_casting!(cl_short, f16, 3.0);
    __test_casting!(cl_short, cl_float, 1);
    __test_casting!(cl_short, cl_double, 1);

    __test_casting!(cl_half, cl_uchar, 1);
    __test_casting!(cl_half, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_half, cl_ushort, 1);
    __test_casting!(cl_half, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_half, cl_uint, 1);
    __test_casting!(cl_half, cl_int, 1);
    __test_casting!(cl_half, cl_ulong, 1);
    __test_casting!(cl_half, cl_long, 1);
    __test_casting!(cl_half, cl_half, 1);
    __test_casting!(cl_half, cl_float, 1);
    __test_casting!(cl_half, cl_double, 1);
    __test_casting!(cl_half, size_t, 1);
    __test_casting!(cl_half, f16, 3.0, f16_oor());

    __test_casting_f16!(cl_uchar);
    __test_casting_f16!(cl_char);
    __test_casting_f16!(cl_ushort);
    __test_casting_f16!(cl_short);
    __test_casting_f16!(cl_uint);
    __test_casting_f16!(cl_int);
    __test_casting_f16!(cl_ulong);
    __test_casting_f16!(cl_long);
    __test_casting_f16!(cl_half);
    __test_casting_f16!(cl_float);
    __test_casting_f16!(cl_double);
    __test_casting_f16!(size_t);
    __test_casting_f16!(f16);

    __test_casting!(cl_uint, cl_uchar, 1, CL_UCHAR_OOR);
    __test_casting!(cl_uint, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_uint, cl_ushort, 1, CL_USHORT_OOR);
    __test_casting!(cl_uint, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_uint, cl_uint, 1);
    __test_casting!(cl_uint, cl_int, 1, CL_INT_OOR);
    __test_casting!(cl_uint, cl_ulong, 1);
    __test_casting!(cl_uint, cl_long, 1);
    __test_casting!(cl_uint, cl_half, 1, CL_HALF_OOR);
    __test_casting!(cl_uint, cl_float, 1);
    __test_casting!(cl_uint, cl_double, 1);
    __test_casting!(cl_uint, size_t, 1);
    __test_casting!(cl_uint, f16, 3.0, f16_oor());

    __test_casting!(cl_int, cl_uchar, 1, -1);
    __test_casting!(cl_int, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_int, cl_ushort, 1, -1);
    __test_casting!(cl_int, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_int, cl_uint, 1, -1);
    __test_casting!(cl_int, cl_int, 1);
    __test_casting!(cl_int, cl_ulong, 1, -1);
    __test_casting!(cl_int, cl_long, 1);
    __test_casting!(cl_int, size_t, 1, -1);
    __test_casting!(cl_int, cl_half, 1);
    __test_casting!(cl_int, f16, 3.0, f16_oor());
    __test_casting!(cl_int, cl_float, 1);
    __test_casting!(cl_int, cl_double, 1);

    __test_casting!(cl_ulong, cl_uchar, 1, CL_UCHAR_OOR);
    __test_casting!(cl_ulong, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_ulong, cl_ushort, 1, CL_USHORT_OOR);
    __test_casting!(cl_ulong, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_ulong, cl_uint, 1, CL_UINT_OOR);
    __test_casting!(cl_ulong, cl_int, 1, CL_INT_OOR);
    __test_casting!(cl_ulong, cl_ulong, 1);
    __test_casting!(cl_ulong, cl_long, 1, CL_LONG_OOR);
    __test_casting!(cl_ulong, cl_half, 1, CL_HALF_OOR);
    __test_casting!(cl_ulong, cl_float, 1);
    __test_casting!(cl_ulong, cl_double, 1);
    __test_casting!(cl_ulong, size_t, 1);
    __test_casting!(cl_ulong, f16, 3.0, f16_oor());

    __test_casting!(cl_long, cl_uchar, 1, -1);
    __test_casting!(cl_long, cl_char, 1, CL_CHAR_OOR);
    __test_casting!(cl_long, cl_ushort, 1, -1);
    __test_casting!(cl_long, cl_short, 1, CL_SHORT_OOR);
    __test_casting!(cl_long, cl_uint, 1, -1);
    __test_casting!(cl_long, cl_int, 1, CL_INT_OOR);
    __test_casting!(cl_long, cl_ulong, 1, -1);
    __test_casting!(cl_long, cl_long, 1);
    __test_casting!(cl_long, size_t, 1, -1);
    __test_casting!(cl_long, cl_half, 1);
    __test_casting!(cl_long, f16, 3.0, f16_oor());
    __test_casting!(cl_long, cl_float, 1);
    __test_casting!(cl_long, cl_double, 1);
}

#[cfg(test)]
mod vector_tests {
    use crate::numbers::cl_number::*;
    use crate::{NumChange, TryClCastNumber};

    macro_rules! __test_casting_once {
        ($t1:ident, $t2:ident, $good_data:expr, $expected:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_vector_from_ $t1 _to_ $t2>]() {
                    let data1: $t1 = $good_data.to_cl_num();
                    let data2: $t2 = data1.try_cl_cast_number().unwrap_or_else(|e| {
                        panic!("{:?}", e);
                    });
                    assert_eq!(unsafe { data2.s }, $expected)
                }
            }
        };
    }

    macro_rules! __test_casting_2 {
        ($size:expr, $left_t:ident, $left_data:expr, { $( $right_t:ident => $right_data:expr ),*, }) => {
            paste::item! {
                $(
                    #[test]
                    fn [<cl_casting_vector_from_ $left_t $size _to_ $right_t $size>]() {
                        let data1: [<$left_t $size>] = $left_data.to_cl_num();
                        let data2: [<$right_t $size>] = data1.try_cl_cast_number().unwrap_or_else(|e| {
                            panic!("{:?}", e);
                        });
                        assert_eq!(unsafe { data2.s }, $right_data);
                    }
                )*
            }
        }
    }

    macro_rules! __test_casting_1 {
        ($size:expr, { $( $left_t:ident => $left_data:expr ),*,}, $casts:tt) => {
            $(
                __test_casting_2!($size, $left_t, $left_data, $casts);
            )*
        };
    }

    macro_rules! __test_casting {
        ($size:expr, $casts:tt) => {
            paste::item! {
                __test_casting_1!($size, $casts, $casts);
            }
        };
    }

    __test_casting!(2, {
        cl_uchar => [3u8, 4],
        cl_char => [3i8, 4],
        cl_ushort => [3u16, 4],
        cl_short => [3i16, 4],
        cl_uint => [3u32, 4],
        cl_int => [3i32, 4],
        cl_ulong => [3u64, 4],
        cl_long => [3i64, 4],
        cl_float => [3.0f32, 4.0],
    });

    __test_casting!(4, {
        cl_uchar => [3u8, 4, 5, 6],
        cl_char => [3i8, 4, 5, 6],
        cl_ushort => [3u16, 4, 5, 6],
        cl_short => [3i16, 4, 5, 6],
        cl_uint => [3u32, 4, 5, 6],
        cl_int => [3i32, 4, 5, 6],
        cl_ulong => [3u64, 4, 5, 6],
        cl_long => [3i64, 4, 5, 6],
        cl_float => [3.0f32, 4.0, 5.0, 6.0],
    });

    __test_casting!(8, {
        cl_uchar => [3u8, 4, 5, 6, 7, 8, 9, 10],
        cl_char => [3i8, 4, 5, 6, 7, 8, 9, 10],
        cl_ushort => [3u16, 4, 5, 6, 7, 8, 9, 10],
        cl_short => [3i16, 4, 5, 6, 7, 8, 9, 10],
        cl_uint => [3u32, 4, 5, 6, 7, 8, 9, 10],
        cl_int => [3i32, 4, 5, 6, 7, 8, 9, 10],
        cl_ulong => [3u64, 4, 5, 6, 7, 8, 9, 10],
        cl_long => [3i64, 4, 5, 6, 7, 8, 9, 10],
        cl_float => [3.0f32, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    });

    __test_casting!(16, {
        cl_uchar => [3u8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_char => [3i8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_ushort => [3u16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_short => [3i16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_uint => [3u32, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_int => [3i32, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_ulong => [3u64, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_long => [3i64, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        cl_float => [3.0f32, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0],
    });
}
// #[test]
// fn cl_casting_works_from_cl_ushort_to_f16() {
//     let a: cl_ushort = 3 as cl_ushort;
//     let b: f16 = a.try_cl_cast_number().unwrap_or_else(|e| {
//         panic!("{:?}", e);
//     });
//     assert_eq!(3.0f32, f32::from(b));
// }

// #[test]
// fn cl_casting_fails_from_cl_ushort_to_f16_when_out_of_range() {
//     let a: cl_ushort = 65534 as cl_ushort;
//     let b: Output<f16> = a.try_cl_cast_number();
//     let e = NumCastError::casting_failed::<cl_ushort, f16>(a);
//     assert_eq!(b, Err(e));
// }

// #[test]
// fn cl_casting_works_for_conversion_from_cl_uchar_to_cl_char() {
//     let a: cl_uchar = 1 as cl_uchar;
//     let b = a.try_cl_cast_number();
//     assert_eq!(b, Ok(1 as cl_char));
// }

// #[test]
// fn cl_casting_fails_for_conversion_from_cl_uchar_to_cl_char_when_out_of_range() {
//     let a: cl_uchar = 130 as cl_uchar;
//     let b: Output<cl_char> = a.try_cl_cast_number();
//     let e = NumCastError::CastingFailed {
//         from: cl_uchar::number_type(),
//         to: cl_char::number_type(),
//         val: format!("{:?}", a),
//     };
//     assert_eq!(b, Err(e.into()));
// }
// }
// #[test]
// fn size_2_vector_casting_works() {
//     let a: cl_uchar2 = cl_uchar2::zeroed();
//     let b: cl_char2 = a
//         .try_cl_cast_number()
//         .expect("failed to cast cl_uchar2 to cl_char2");
// }
// }

// impl TryClCastNumber<cl_char2> for cl_uchar2 {}

// impl<S, T, X, Y> TryClCastVector<T, X, Y> for S
// where
//     S: ClVector<X> + ClVector2 + NumChange,
//     T: ClVector<Y> + ClVector2 + NumChange,
//     X: ClPrimitive + TryClCastNumber<Y>,
//     Y: ClPrimitive,
//     Z: NumChange,
//     <S as NumChange>::RustNum: AsSlice<X> + NumChange,
// {
//     fn try_cl_cast_vector(self) -> Output<T> {
//         let casted: Vec<Y> = self
//             .to_rust_num()
//             .as_slice()
//             .into_iter()
//             .map(|n| n.try_cl_cast_num())
//             .collect()?;
//         debug_assert!(casted.len() == 2);
//         let casted_rust_val: Z = [casted[0], casted[1]];
//         Ok(casted_rust_val.to_cl_num())
//     }
// }

// use std::fmt::Debug;
// use half::f16;
// use libc::size_t;
// use num_traits::cast::{ToPrimitive, FromPrimitive, NumCast};

// use super::ffi_types::*;
// use super::newtypes::*;
// use super::number_type::{NumberType, NumberTypedT};
// use super::as_slice::AsSlice;
// use crate::Output;
// use crate::traits::{FFIType, ClType, RustType, ToFFIType, ToClType, ToRustType};

// /// An error related to a Device.
// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum NumberConversionError {
//     #[fail(display = "Failed to cast number {} from {:?} to {:?}", _0, _1, _2)]
//     FailedToCast(String, NumberType, NumberType),
// }

// pub trait ConvertTo<T> {
//     fn convert_to(self) -> T;
// }

// impl ToFFIType<cl_bool> for bool {
//     fn to_ffi_type(self) -> cl_bool {
//         match self {
//             true => 1,
//             false => 0,
//         }
//     }
// }

// impl ToRustType<bool> for cl_bool {
//     fn to_rust_type(self) -> bool {
//         match self {
//             0 => false,
//             1 => true,
//             bad => panic!("Invalid cl_bool value {:?}: must be 0 or 1", bad),
//         }
//     }
// }

// impl ToClType<ClBool> for cl_bool {
//     fn to_cl_type(self) -> ClBool {
//         if self.to_rust_type() {
//             ClBool::True
//         } else {
//             ClBool::False
//         }
//     }
// }

// impl ToClType<ClHalf> for cl_half {
//     fn to_cl_type(self) -> ClHalf {
//         ClHalf(self)
//     }
// }

// impl ToRustType<f16> for ClHalf {
//     fn to_rust_type(self) -> f16 {
//         f16::from_bits(self.0)
//     }
// }

// impl ToRustType<f16> for cl_half {
//     fn to_rust_type(self) -> f16 {
//         f16::from_bits(self)
//     }
// }

// impl ToFFIType<cl_half> for f16 {
//     fn to_ffi_type(self) -> cl_half {
//         self.to_bits()
//     }
// }

// impl ToClType<ClHalf> for f16 {
//     fn to_cl_type(self) -> ClHalf {
//         ClHalf(self.to_bits())
//     }
// }

// macro_rules! impl_trait {
//     ($the_trait:ident, $( $t:ty ),*) => {
//         $(
//             impl $the_trait for $t {}
//         )*
//     }
// }

// macro_rules! impl_array_size {
//     ($the_trait:ident, $n:expr) => {
//         impl_trait!($the_trait, [u8; $n], [i8; $n], [u16; $n], [i16; $n], [f16; $n], [i32; $n], [u32; $n], [f32; $n], [u64; $n], [i64; $n], [f64; $n]);
//     }
// }

// impl_trait!(RustType, bool, u8, i8, u16, i16, f16);

// impl_array_size!(RustType, 2);
// impl_array_size!(RustType, 3);
// impl_array_size!(RustType, 4);
// impl_array_size!(RustType, 8);
// impl_array_size!(RustType, 16);

// macro_rules! impl_primitive_conversion {
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty) => {

//         impl ToClType<$new_t> for $ffi_t {
//             fn to_cl_type(self) -> $new_t {
//                 $new_t(self)
//             }
//         }

//         impl ToFFIType<$ffi_t> for $rust_t {
//             fn to_ffi_type(self) -> $ffi_t {
//                 self as $ffi_t
//             }
//         }
//     };
// }

// macro_rules! impl_convert_to_for_vector {
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty, 3) => {
//         paste::item! {
//             impl ToRustType<[$rust_t; 3]> for [<$ffi_t 3>] {
//                 fn to_rust_type(self) -> [$rust_t; 3] {
//                     let inner = unsafe { self.s };
//                     [inner[0], inner[1], inner[2]]
//                 }
//             }

//             impl ConvertTo<[$rust_t; 3]> for [<$new_t 3>] {
//                 fn convert_to(self) -> [$rust_t; 3] {
//                     let inner = unsafe { self.0.s };
//                     [inner[0], inner[1], inner[2]]
//                 }
//             }

//             impl ConvertTo<[<$new_t 3>]> for [<$ffi_t 3>] {
//                 fn convert_to(self) -> [<$new_t 3>] {
//                     [<$new_t 3>](self)
//                 }
//             }

//             impl ConvertTo<[<$new_t 3>]> for [$rust_t; 3] {
//                 fn convert_to(self) -> [<$new_t 3>] {
//                     [<$new_t 3>](self.convert_to())
//                 }
//             }

//             impl ConvertTo<[<$ffi_t 3>]> for [$rust_t; 3] {
//                 fn convert_to(self) -> [<$ffi_t 3>] {
//                     let mut num = unsafe { std::mem::zeroed::<[<$ffi_t 3>]>() };
//                     let new_inner = [self[0], self[1], self[2], 0 as $ffi_t];
//                     num.s = new_inner;
//                     num
//                 }
//             }

//             impl ConvertTo<[<$ffi_t 3>]> for [<$new_t 3>] {
//                 fn convert_to(self) -> [<$ffi_t 3>] {
//                     self.0
//                 }
//             }
//         }
//     };
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty, $num:expr) => {
//         paste::item! {
//             impl ConvertTo<[$rust_t; $num]> for [<$ffi_t $num>]{
//                 fn convert_to(self) -> [$rust_t; $num] {
//                     unsafe { self.s }
//                 }
//             }

//             impl ConvertTo<[<$ffi_t $num>]> for [$rust_t; $num] {
//                 fn convert_to(self) -> [<$ffi_t $num>] {
//                     let mut num = unsafe { std::mem::zeroed::<[<$ffi_t $num>]>() };
//                     num.s = self;
//                     num
//                 }
//             }

//             impl ConvertTo<[<$new_t $num>]> for [$rust_t; $num] {
//                 fn convert_to(self) -> [<$new_t $num>] {
//                     [<$new_t $num>](self.convert_to())
//                 }
//             }

//             impl ConvertTo<[<$new_t $num>]> for [<$ffi_t $num>] {
//                 fn convert_to(self) -> [<$new_t $num>] {
//                     [<$new_t $num>](self)
//                 }
//             }

//             impl ConvertTo<[<$ffi_t $num>]> for [<$new_t $num>] {
//                 fn convert_to(self) -> [<$ffi_t $num>] {
//                     self.0
//                 }
//             }

//             impl ConvertTo<[$rust_t; $num]> for [<$new_t $num>] {
//                 fn convert_to(self) -> [$rust_t; $num] {
//                     self.0.convert_to()
//                 }
//             }
//         }
//     };
// }

// macro_rules! impl_convert_to_for_all_vectors {
//     ($ffi_t:ty, $new_t:ident, $rust_t:ty) => {
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 2);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 3);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 4);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 8);
//         impl_convert_to_for_vector!($ffi_t, $new_t, $rust_t, 16 );
//     }
// }

// impl_primitive_conversion!(size_t, SizeT, usize);
// impl_primitive_conversion!(cl_double, ClDouble, f64);
// impl_primitive_conversion!(cl_char, ClChar, i8);
// impl_primitive_conversion!(cl_uchar, ClUchar, u8);
// impl_primitive_conversion!(cl_short, ClShort, i16);
// impl_primitive_conversion!(cl_ushort, ClUshort, u16);
// impl_primitive_conversion!(cl_int, ClInt, i32);
// impl_primitive_conversion!(cl_uint, ClUint, u32);
// impl_primitive_conversion!(cl_long, ClLong, i64);
// impl_primitive_conversion!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(cl_float, ClFloat, f32);

// impl_convert_to_for_all_vectors!(cl_char, ClChar, i8);
// impl_convert_to_for_all_vectors!(cl_uchar, ClUchar, u8);
// impl_convert_to_for_all_vectors!(cl_short, ClShort, i16);
// impl_convert_to_for_all_vectors!(cl_ushort, ClUshort, u16);
// impl_convert_to_for_all_vectors!(cl_int, ClInt, i32);
// impl_convert_to_for_all_vectors!(cl_uint, ClUint, u32);
// impl_convert_to_for_all_vectors!(cl_long, ClLong, i64);
// impl_convert_to_for_all_vectors!(cl_ulong, ClUlong, u64);
// impl_convert_to_for_all_vectors!(cl_float, ClFloat, f32);

// pub trait CastNumberTo<T> {
//     fn cast_number_to(&self) -> Output<T>;
// }

// impl<P1, P2> CastNumberTo<P2> for P1 where P1: ClPrimitive, P2: ClPrimitive,  {
//     fn cast_number_to(&self) -> Output<P2> {
//         P2::from(*self).ok_or_else(|| {
//             NumberConversionError::FailedToCast(
//                 format!("{:?}", self),
//                 P1::number_type(),
//                 P2::number_type()
//             ).into()
//         })
//     }
// }

// fn _cast_primitive<T: NumCast + NumberTypedT + Debug, U: NumCast + NumberTypedT>(n: T) -> Output<U> {
//     U::from(n).ok_or_else(|| {
//         NumberConversionError::FailedToCast(
//             format!("{:?}", n),
//             T::number_type(),
//             U::number_type()
//         ).into()
//     })
// }

// pub trait CastVector2<T, O, V> where Self: ClVector2 + AsSlice<T>, V: ClVector2 + ClVector<O> + FFIType, T: NumCast + ToPrimitive + FromPrimitive + ClPrimitive, O: ClPrimitive {
//     fn cast_vector(&self) -> Output<V> {
//         let slc = self.as_slice();
//         let p0: O = _cast_primitive(slc[0])?;
//         let p1: O = _cast_primitive(slc[1])?;
//         Ok([p0, p1])
//     }
// }

// // impl<V1, V2> CastNumberTo<V2> for V1 where V1: ClVector<cl_char> + ClVector2, V2: ClVector<cl_uchar> + ClVector2 {
// //     fn cast_number_to(&self) -> Output<V2> {
// //         V2::from(*self).ok_or_else(|| {
// //             NumberConversionError::FailedToCast(
// //                 format!("{:?}", self),
// //                 V1::number_type(),
// //                 V2::number_type()
// //             ).into()
// //         })
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::numbers::Zeroed;
//     // use crate::ffi::*;
//     // use float_cmp::ApproxEq;

//     macro_rules! conversion_tests {
//         ($ffi_t:ty, $new_t:ty, $rust_t:ty, $num:expr) => {
//             paste::item! {
//                 // PARTIAL_EQ IS NOT WORKING.
//                 // #[allow(non_snake_case)]
//                 // #[test]
//                 // fn [<convert_from_ $t _to_ $new_t>]() {
//                 //     let num: $t = $num;
//                 //     let new_num: $new_t = num.convert_to();
//                 //     let expected = $new_t($num);
//                 //     assert_eq!(new_num, expected);
//                 // }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<to_ffi_type_for_conversion_from_ $new_t _to_ $ffi_t>]() {
//                     let new_num: $new_t = $new_t($num);
//                     let num: $ffi_t = new_num.to_ffi_type();
//                     assert_eq!(num, $num);
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<to_rust_type_for_conversion_from_ $new_t _to_ $rust_t>]() {
//                     let new_num: $new_t = $new_t($num);
//                     let num: $rust_t = new_num.to_rust_type();
//                     assert_eq!(num, $num);
//                 }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $rust_t _to_ $new_t>]() {
//             //         let new_num: $new_t = $new_t($num);
//             //         let num: $t = new_num.convert_to();
//             //         assert_eq!(num, $num);
//             //     }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $rust_t _to_ $t>]() {
//             //         let num1: $rust_t = $num;d
//             //         let converted: $t = num1.convert_to();
//             //         let expected: $t = $num;
//             //         assert_eq!(converted, expected);
//             //     }

//             //     #[allow(non_snake_case)]
//             //     #[test]
//             //     fn [<convert_from_ $t _to_ $rust_t>]() {
//             //         let num1: $rust_t = $num;
//             //         let converted: $t = num1.convert_to();
//             //         let expected: $t = $num;
//             //         assert_eq!(converted, expected);
//             //     }
//             }
//         };
//     }

//     macro_rules! conversion_tests_for_float {
//         ($t:ident, $new_t:ident, $rust_t:ident, $num:expr) => {
//             paste::item! {
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $new_t>]() {
//                     let num: $t = $num;
//                     let new_num: $new_t = num.convert_to();
//                     assert!(approx_eq!($rust_t, new_num.0, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $new_t _to_ $t>]() {
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $new_t _to_ $rust_t>]() {
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $rust_t _to_ $new_t>]() {
//                     let new_num: $new_t = $new_t($num);
//                     let num: $t = new_num.convert_to();
//                     assert!(approx_eq!($rust_t, num, $num));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $rust_t _to_ $t>]() {
//                     let num1: $rust_t = $num;
//                     let converted: $t = num1.convert_to();
//                     let expected: $t = $num;
//                     assert!(approx_eq!($rust_t, converted, expected));
//                 }

//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $rust_t>]() {
//                     let num1: $rust_t = $num;
//                     let converted: $t = num1.convert_to();
//                     let expected: $t = $num;
//                     assert!(approx_eq!($rust_t, converted, expected));
//                 }
//             }
//         };
//     }

//     // conversion_tests!(cl_uchar, ClUchar, u8, 3);
//     // conversion_tests!(cl_char, ClChar, i8, 3);
//     // conversion_tests!(cl_ushort, ClUshort, u16, 3);
//     // conversion_tests!(cl_short, ClShort, i16, 3);
//     // conversion_tests!(cl_int, ClInt, i32, 3);
//     // conversion_tests!(cl_uint, ClUint, u32, 3);
//     // conversion_tests!(cl_long, ClLong, i64, 3);
//     // conversion_tests!(cl_ulong, ClUlong, u64, 3);
//     // conversion_tests!(size_t, SizeT, usize, 3);

//     // conversion_tests_for_float!(cl_float, ClFloat, f32, 3.0);
//     // conversion_tests_for_float!(cl_double, ClDouble, f64, 3.0);
//     macro_rules! conversion_tests_for_vector {
//         ($t:ident, $new_t:ident, [$rust_t:ty; $vector_size:expr], $num:expr) => {
//             paste::item! {
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<convert_from_ $t _to_ $new_t>]() {
//                     let num = $t::zeroed();
//                     let new_num: $new_t = num.convert_to();
//                     assert_eq!(new_num, $new_t(num));
//                 }
//             }
//         };
//     }

//     conversion_tests_for_vector!(cl_uchar2, ClUchar2, [u8; 2], [3, 4]);

//     fn three_f16() -> f16 {
//         f16::from_f32(3.0)
//     }

//     fn three_half() -> cl_half {
//         three_f16().to_bits()
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_cl_half_to_ClHalf() {
//         let new_num: ClHalf = three_half().to_cl_type();
//         assert_eq!(new_num, ClHalf(three_half()));
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_ClHalf_to_cl_half() {
//         let num2: cl_half = ClHalf(three_half()).to_ffi_type();
//         assert_eq!(num2, three_half());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_ClHalf_to_f16() {
//         let num: f16 = ClHalf(three_half()).to_rust_type();
//         assert_eq!(num, three_f16());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_f16_to_ClHalf() {
//         let num: ClHalf = three_f16().to_cl_type();
//         assert_eq!(num, ClHalf(three_half()));
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_f16_to_cl_half() {
//         let num: cl_half = three_f16().to_ffi_type();
//         assert_eq!(num, three_half());
//     }

//     #[allow(non_snake_case)]
//     #[test]
//     fn convert_from_cl_half_to_f16() {
//         let got: f16 = three_half().to_rust_type();
//         assert_eq!(got, three_f16());
//     }
// }
