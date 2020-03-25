// use std::convert::{TryFrom, TryInto};
use std::fmt;

use num_traits::NumCast;

use super::cl_number::*;
use super::trait_impls::{ClBool, SizeT};
use super::trait_impls::{ClChar, ClInt, ClLong, ClShort, ClUchar, ClUint, ClUlong, ClUshort};
use super::trait_impls::{
    ClChar2, ClFloat2, ClInt2, ClLong2, ClShort2, ClUchar2, ClUint2, ClUlong2, ClUshort2,
};

use super::trait_impls::{
    ClChar3, ClFloat3, ClInt3, ClLong3, ClShort3, ClUchar3, ClUint3, ClUlong3, ClUshort3,
};

use super::trait_impls::{
    ClChar4, ClFloat4, ClInt4, ClLong4, ClShort4, ClUchar4, ClUint4, ClUlong4, ClUshort4,
};

use super::trait_impls::{
    ClChar8, ClFloat8, ClInt8, ClLong8, ClShort8, ClUchar8, ClUint8, ClUlong8, ClUshort8,
};

use super::trait_impls::{
    ClChar16, ClFloat16, ClInt16, ClLong16, ClShort16, ClUchar16, ClUint16, ClUlong16, ClUshort16,
};
use super::trait_impls::{ClDouble, ClFloat, ClHalf};

use crate::{AsSlice, Error, NumberType, NumberTypedT, Output, Zeroed};

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
    fn casting_failed<T: NumberTypedT + fmt::Debug, U: NumberTypedT>(t: &T) -> Error {
        let e = NumCastError::CastingFailed {
            from: T::number_type(),
            to: U::number_type(),
            val: format!("{:?}", t),
        };
        Error::from(e)
    }
}

pub trait ClTryFrom<T>
where
    Self: Sized,
{
    fn try_from(val: T) -> Output<Self>;
}

pub trait ClTryInto<T> {
    fn try_into(val: Self) -> Output<T>;
}

impl<T, U> ClTryInto<T> for U
where
    T: ClTryFrom<U>,
{
    fn try_into(val: U) -> Output<T> {
        ClTryFrom::<U>::try_from(val)
    }
}

impl<T> ClTryFrom<T> for bool
where
    T: ClTryInto<cl_uint> + NumberTypedT + fmt::Debug + Copy,
{
    fn try_from(val: T) -> Output<bool> {
        ClTryInto::try_into(val)
            .and_then(|int| cast_cl_uint_to_bool(int))
            .map_err(|_| {
                let nce = NumCastError::casting_failed::<T, bool>(&val);
                Error::from(nce)
            })
    }
}

// macro_rules! __impl_try_from_bool {
//     () => {
//         unimplemented!();
//     };
// }

// macro_rules! __impl_primitive_casting_once {
//     ($t1:ident, $t2:ident) => {
//         impl ClTryFrom<$t1> for $t2 {
//             fn try_from(val: $t1) ->
//         }
//     }
// }

macro_rules! __impl_vector_casting_once {
    ($vector1:ident, $base1:ident, $vector2:ident, $base2:ident) => {
        impl ClTryFrom<$vector1> for $vector2 {
            fn try_from(val: $vector1) -> Output<$vector2> {
                let mut vector = $vector2::zeroed();
                let slice1: &[$base1] = val.as_slice();
                let slice2: &mut [$base2] = vector.as_mut_slice();
                debug_assert!(slice1.len() == slice2.len());
                for (i, item) in slice1.iter().enumerate() {
                    slice2[i] = ClTryFrom::<$base1>::try_from(*item)?;
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

macro_rules! __impl_try_from_new_types {
    ($t1:ident, $t2:ident) => {
        impl ClTryFrom<$t1> for $t2 {
            fn try_from(val: $t1) -> Output<$t2> {
                match ClTryFrom::try_from(val.0) {
                    Ok(casted_val) => Ok($t2(casted_val)),
                    Err(_) => Err(NumCastError::casting_failed::<$t1, $t2>(&val)),
                }
            }
        }
    };
}

macro_rules! __try_from_new_types_3 {
    ($t1:ident, [$( $t2:ident ),*]) => {
        $(
            __impl_try_from_new_types!($t1, $t2);
        )*
    };
}

macro_rules! __try_from_new_types_2 {
    ([$( $t1:ident ),*], $types:tt) => {
        $(
            __try_from_new_types_3!($t1, $types);
        )*
    };
}

macro_rules! __try_from_all_new_types {
    ($types:tt) => {
        __try_from_new_types_2!($types, $types);
    };
}

__try_from_all_new_types!([
    ClUchar, ClChar, ClUshort, ClShort, ClUint, ClInt, ClUlong, ClLong, ClHalf, ClFloat, ClDouble,
    ClBool, SizeT
]);

fn cast_cl_uint_to_bool(val: cl_uint) -> Output<bool> {
    match val {
        0 => Ok(false),
        1 => Ok(true),
        got => {
            let nce = NumCastError::CastingFailed {
                from: cl_uint::number_type(),
                to: bool::number_type(),
                val: format!("{:?}", got),
            };
            Err(Error::from(nce))
        }
    }
}

__try_from_all_new_types!([
    ClUchar2, ClChar2, ClUshort2, ClShort2, ClUint2, ClInt2, ClUlong2, ClLong2, ClFloat2
]);

__try_from_all_new_types!([
    ClChar4, ClFloat4, ClInt4, ClLong4, ClShort4, ClUchar4, ClUint4, ClUlong4, ClUshort4
]);

__try_from_all_new_types!([
    ClChar8, ClFloat8, ClInt8, ClLong8, ClShort8, ClUchar8, ClUint8, ClUlong8, ClUshort8
]);

__try_from_all_new_types!([
    ClChar16, ClFloat16, ClInt16, ClLong16, ClShort16, ClUchar16, ClUint16, ClUlong16, ClUshort16
]);

__try_from_all_new_types!([
    ClChar3, ClFloat3, ClInt3, ClLong3, ClShort3, ClUchar3, ClUint3, ClUlong3, ClUshort3
]);

macro_rules! __impl_try_from_cl_types {
    ($t1:ident, $t2:ident) => {
        impl ClTryFrom<$t1> for $t2 {
            fn try_from(val: $t1) -> Output<$t2> {
                NumCast::from(val).ok_or_else(|| {
                    let nce = NumCastError::CastingFailed {
                        from: $t1::number_type(),
                        to: $t2::number_type(),
                        val: format!("{:?}", val),
                    };
                    Error::from(nce)
                })
            }
        }
    };
}

macro_rules! __try_from_cl_types_3 {
    ($t1:ident, [$( $t2:ident ),*]) => {
        $(
            __impl_try_from_cl_types!($t1, $t2);
        )*
    };
}

macro_rules! __try_from_cl_types_2 {
    ([$( $t1:ident ),*], $types:tt) => {
        $(
            __try_from_cl_types_3!($t1, $types);
        )*
    };
}

macro_rules! __try_from_all_cl_types {
    ($types:tt) => {
        __try_from_cl_types_2!($types, $types);
    };
}

__try_from_all_cl_types!([
    cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_long, cl_ulong, cl_float,
    cl_double, size_t
]);

const F16_MAX: f32 = 65504.0;
const F16_MIN: f32 = -65504.0;

impl<T> ClTryFrom<T> for f16
where
    T: ClTryInto<f32> + NumberTypedT + fmt::Debug + Copy,
{
    fn try_from(val: T) -> Output<f16> {
        let error_func = || Error::from(NumCastError::casting_failed::<T, f16>(&val));
        let float: f32 = ClTryInto::try_into(val).map_err(|_| error_func())?;
        if float > F16_MAX {
            return error_func().as_output();
        }

        if float < F16_MIN {
            return error_func().as_output();
        }

        Ok(f16::from_f32(float))
    }
}

macro_rules! __impl_try_from_f16 {
    ($( $t:ident ),*) => {
        $(
            impl ClTryFrom<f16> for $t {
                fn try_from(val: f16) -> Output<$t> {
                    ClTryFrom::try_from(val.to_f32())
                }
            }
        )*
    };
}

__impl_try_from_f16!(
    cl_uchar, cl_char, cl_ushort, cl_short, cl_uint, cl_int, cl_ulong, cl_long, cl_float, cl_double
);

impl<S, T> ClTryFrom<&[S]> for Vec<T>
where
    T: ClTryFrom<S>,
    S: Copy,
{
    fn try_from(val: &[S]) -> Output<Vec<T>> {
        let mut out: Vec<T> = Vec::with_capacity(val.len());
        for item in val {
            match ClTryFrom::try_from(*item) {
                Ok(u) => out.push(u),
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}

impl<S, T> ClTryFrom<Vec<S>> for Vec<T>
where
    T: ClTryFrom<S>,
    S: Copy,
{
    fn try_from(val: Vec<S>) -> Output<Vec<T>> {
        ClTryFrom::try_from(&val[..])
    }
}

#[cfg(test)]
mod tests {
    use crate::ClTryFrom;

    #[test]
    fn can_cast_a_vec() {
        let data1 = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let data2: Vec<i32> = ClTryFrom::try_from(&data1[..]).unwrap();
        assert_eq!(data2, vec![0i32, 1, 2, 3, 4, 5, 6, 7]);
    }
}

#[cfg(test)]
mod primitive_tests {
    use super::{ClTryFrom, NumCastError};
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
                    let b: f16 = ClTryFrom::try_from(a).unwrap_or_else(|e| {
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
                    let b: f16 = ClTryFrom::try_from(a).unwrap_or_else(|e| {
                        panic!("failed to cast to f16 {:?}", e);
                    });
                    assert_eq!(f32::from(b), 3.0);
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_bool_when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<f16> = ClTryFrom::try_from(a);
                    let e = NumCastError::casting_failed::<$t1, f16>(&a);
                    assert_eq!(b, Err(e));
                }
            }
        };

        ($t1:ty, bool, $good_num:expr, $bad_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_for_conversion_from_ $t1 _to_bool>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: bool = ClTryFrom::try_from(a).unwrap_or_else(|e| {
                        panic!("failed to cast bool {:?}", e);
                    });
                    assert_eq!(b, true);
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_bool_when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<bool> = ClTryFrom::try_from(a);
                    let e = NumCastError::casting_failed::<$t1, bool>(&a);
                    assert_eq!(b, Err(e));
                }
            }
        };
        ($t1:ty, $t2:ty, $good_num:expr, $bad_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_ $t2>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: Output<$t2> = ClTryFrom::try_from(a);
                    assert_eq!(b, Ok($good_num as $t2));
                }

                #[test]
                fn [<cl_casting_fails_from_ $t1 _to_ $t2 _when_out_of_range>]() {
                    let a: $t1 = $bad_num as $t1;
                    let b: Output<$t2> = ClTryFrom::try_from(a);
                    let e = NumCastError::casting_failed::<$t1, $t2>(&a);
                    assert_eq!(b, Err(e));
                }
            }
        };
        ($t1:ty, f16, $good_num:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_works_from_ $t1 _to_f16>]() {
                    let a: $t1 = $good_num as $t1;
                    let b: f16 = ClTryFrom::try_from(a).unwrap_or_else(|e| {
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
                    let b: $t2 = ClTryFrom::try_from(a).unwrap_or_else(|e| {
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
    use crate::{ClTryFrom, NumChange};

    macro_rules! __test_casting_once {
        ($t1:ident, $t2:ident, $good_data:expr, $expected:expr) => {
            paste::item! {
                #[test]
                fn [<cl_casting_vector_from_ $t1 _to_ $t2>]() {
                    let data1: $t1 = $good_data.to_cl_num();
                    let data2: $t2 = ClTryFrom::try_from(data1).unwrap_or_else(|e| {
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
                        let data2: [<$right_t $size>] = ClTryFrom::try_from(data1).unwrap_or_else(|e| {
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
