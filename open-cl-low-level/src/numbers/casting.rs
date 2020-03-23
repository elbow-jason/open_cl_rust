// use std::convert::{TryFrom, TryInto};
use std::fmt;

use super::cl_number::*;
use crate::{
    AsSlice, ClRustPrimitiveNum, Error, NumChange, Number, NumberType, NumberTypedT, Output, Zeroed,
};

use num_traits::{NumCast, ToPrimitive};

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

pub trait TryClCastNumber<T> {
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

impl<T, U> TryClCastNumber<Vec<U>> for Vec<T>
where
    T: TryClCastNumber<U>,
    U: NumberTypedT,
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
#[cfg(test)]
mod tests {
    use crate::TryClCastNumber;

    #[test]
    fn can_cast_a_vec() {
        let data1 = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let data2: Vec<i32> = data1.try_cl_cast_number().unwrap();
        assert_eq!(data2, vec![0i32, 1, 2, 3, 4, 5, 6, 7]);
    }
}

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
