use std::ops::Deref;

use half::f16;
use libc::size_t;

use crate::{NumberType, NumberTypedT, Kind};

// primitive types
use crate::ffi::{
    cl_bool, cl_char, cl_double, cl_float, cl_half, cl_int, cl_long, cl_uchar, cl_uint, cl_ulong,
};

// vector types
use crate::ffi::{
    cl_char16, cl_char2, cl_char3, cl_char4, cl_char8, 
    cl_float16, cl_float2, cl_float3, cl_float4, cl_float8,  cl_int16, cl_int2,
    cl_int3, cl_int4, cl_int8, cl_long16, cl_long2, cl_long3, cl_long4, cl_long8,
    cl_short, cl_short16, cl_short2, cl_short3, cl_short4, cl_short8, cl_uchar16,
    cl_uchar2, cl_uchar3, cl_uchar4, cl_uchar8, cl_uint16, cl_uint2, cl_uint3, cl_uint4,
    cl_uint8, cl_ulong16, cl_ulong2, cl_ulong3, cl_ulong4, cl_ulong8, cl_ushort,
    cl_ushort16, cl_ushort2, cl_ushort3, cl_ushort4, cl_ushort8,
};

// NOTE: f32 does not implement Eq so it's not here. WHYEEEEE...
pub unsafe trait ClNumber:
    Sized + Clone + Copy + Send + Sync + 'static + Zeroed + NumberTypedT
{
}

// pub trait TypeName {
//     fn type_name(&self) -> String;
// }

// macro_rules! impl_type_name {
//     ($t:ident) => {
//         paste::item! {
//             const [<TYPE_NAME_ $t>]: &'static str = stringify!($t);
//             impl TypeName for $t {
//                 fn type_name(&self) -> &'static str {
//                     [<TYPE_NAME_ $t>]
//                 }
//             }
//         }
        
//     }
// }

// impl_type_name!(cl_char);
// impl_type_name!(cl_uchar);
// impl_type_name!(cl_short);
// impl_type_name!(cl_ushort);
// impl_type_name!(cl_int);
// impl_type_name!(cl_uint);
// impl_type_name!(cl_long);
// impl_type_name!(cl_ulong);
// impl_type_name!(cl_half);
// impl_type_name!(cl_float);
// impl_type_name!(cl_double);
// impl_type_name!(size_t);

// #[macro_use]
// macro_rules! impl_type_name_vector {
//     ($t:ident) => {
//         paste::item! {
//             impl_type_name!([<$t 2>]);
//             // impl_type_name!([<$t 3>]);
//             impl_type_name!([<$t 4>]);
//             impl_type_name!([<$t 8>]);
//             impl_type_name!([<$t 16>]);
//         }
//     }
// }



// impl_type_name_vector!(cl_char);
// impl_type_name_vector!(cl_uchar);
// impl_type_name_vector!(cl_short);
// impl_type_name_vector!(cl_ushort);
// impl_type_name_vector!(cl_int);
// impl_type_name_vector!(cl_uint);
// impl_type_name_vector!(cl_long);
// impl_type_name_vector!(cl_ulong);
// impl_type_name_vector!(cl_float);

// impl_type_name_vector!(cl_half);
// impl_type_name_vector!(cl_double);
// impl_type_name_vector!(size_t);


//8
unsafe impl ClNumber for u8 {}
unsafe impl ClNumber for i8 {}
unsafe impl ClNumber for u16 {}
unsafe impl ClNumber for i16 {}
unsafe impl ClNumber for u32 {}
unsafe impl ClNumber for i32 {}
unsafe impl ClNumber for f32 {}
unsafe impl ClNumber for u64 {}
unsafe impl ClNumber for i64 {}
unsafe impl ClNumber for f64 {}

impl NumberTypedT for f64 {
    fn number_type() -> NumberType {
        NumberType::ClDouble
    }
}
// unsafe impl ClNumber for Complex64 {}
//size
// unsafe impl ClNumber for isize {}
// unsafe impl ClNumber for usize {}


macro_rules! impl_deref {
    ($new_t:ty => $t:ty) => {
        impl Deref for $new_t {
            type Target = $t;

            fn deref(&self) -> &$t {
                &self.0
            }
        }
    }
}

const CL_BOOL_FALSE: cl_bool = 0;
const CL_BOOL_TRUE: cl_bool = 1;

impl Deref for ClBool {
    type Target = cl_bool;

    fn deref(&self) -> &cl_bool {
        match self {
            ClBool::True => &CL_BOOL_TRUE,
            ClBool::False => &CL_BOOL_FALSE,
        }
    }
}


// impl_deref!(ClChar => cl_char);
// impl_deref!(ClUchar => cl_uchar);
// impl_deref!(ClShort => cl_short);
// impl_deref!(ClUshort => cl_ushort);
// impl_deref!(ClInt => cl_int);
// impl_deref!(ClUint => cl_uint);
// impl_deref!(ClLong => cl_long);
// impl_deref!(ClUlong => cl_ulong);
// impl_deref!(ClHalf => cl_half);
// impl_deref!(ClFloat => cl_float);
// impl_deref!(ClDouble => cl_double);
// impl_deref!(SizeT => size_t);
// impl_deref!(ClChar2 => cl_char2);
// impl_deref!(ClUchar2 => cl_uchar2);
// impl_deref!(ClShort2 => cl_short2);
// impl_deref!(ClUshort2 => cl_ushort2);
// impl_deref!(ClInt2 => cl_int2);
// impl_deref!(ClUint2 => cl_uint2);
// impl_deref!(ClLong2 => cl_long2);
// impl_deref!(ClUlong2 => cl_ulong2);
// impl_deref!(ClFloat2 => cl_float2);
// impl_deref!(ClChar3 => cl_char3);
// impl_deref!(ClUchar3 => cl_uchar3);
// impl_deref!(ClShort3 => cl_short3);
// impl_deref!(ClUshort3 => cl_ushort3);
// impl_deref!(ClInt3 => cl_int3);
// impl_deref!(ClUint3 => cl_uint3);
// impl_deref!(ClLong3 => cl_long3);
// impl_deref!(ClUlong3 => cl_ulong3);
// impl_deref!(ClFloat3 => cl_float3);
// impl_deref!(ClChar4 => cl_char4);
// impl_deref!(ClUchar4 => cl_uchar4);
// impl_deref!(ClShort4 => cl_short4);
// impl_deref!(ClUshort4 => cl_ushort4);
// impl_deref!(ClInt4 => cl_int4);
// impl_deref!(ClUint4 => cl_uint4);
// impl_deref!(ClLong4 => cl_long4);
// impl_deref!(ClUlong4 => cl_ulong4);
// impl_deref!(ClFloat4 => cl_float4);
// impl_deref!(ClChar8 => cl_char8);
// impl_deref!(ClUchar8 => cl_uchar8);
// impl_deref!(ClShort8 => cl_short8);
// impl_deref!(ClUshort8 => cl_ushort8);
// impl_deref!(ClInt8 => cl_int8);
// impl_deref!(ClUint8 => cl_uint8);
// impl_deref!(ClLong8 => cl_long8);
// impl_deref!(ClUlong8 => cl_ulong8);
// impl_deref!(ClFloat8 => cl_float8);
// impl_deref!(ClChar16 => cl_char16);
// impl_deref!(ClUchar16 => cl_uchar16);
// impl_deref!(ClShort16 => cl_short16);
// impl_deref!(ClUshort16 => cl_ushort16);
// impl_deref!(ClInt16 => cl_int16);
// impl_deref!(ClUint16 => cl_uint16);
// impl_deref!(ClLong16 => cl_long16);
// impl_deref!(ClUlong16 => cl_ulong16);
// impl_deref!(ClFloat16 => cl_float16);

pub trait ToClNumber<T> {
    fn to_cl_number(self) -> T;
}

pub trait FromClNumber<T> {
    fn from_cl_number(value: T) -> Self;
}

pub trait Zeroed {
    fn zeroed() -> Self;
}

impl ToClNumber<cl_bool> for bool {
    fn to_cl_number(self) -> cl_bool {
        match self {
            true => 1,
            false => 0,
        }
    }
}

impl ToClNumber<cl_bool> for ClBool {
    fn to_cl_number(self) -> cl_bool {
        match self {
            ClBool::True => 1,
            ClBool::False => 0,
        }
    }
}

impl FromClNumber<cl_bool> for bool {
    fn from_cl_number(b: cl_bool) -> bool {
        match b {
            0 => false,
            1 => true,
            bad => panic!("Invalid cl_bool value {:?}: must be 0 or 1", bad),
        }
    }
}

impl FromClNumber<cl_bool> for ClBool {
    fn from_cl_number(b: cl_bool) -> ClBool {
        if bool::from_cl_number(b) {
            ClBool::True
        } else {
            ClBool::False
        }
    }
}

impl_deref!(ClHalf => cl_half);

impl ToClNumber<cl_half> for ClHalf {
    fn to_cl_number(self) -> cl_half {
        *self
    }
}

impl FromClNumber<cl_half> for ClHalf {
    fn from_cl_number(val: cl_half) -> ClHalf {
        ClHalf(val)
    }
}

impl FromClNumber<cl_half> for f16 {
    fn from_cl_number(val: cl_half) -> f16 {
        f16::from_bits(val)
    }
}

macro_rules! impl_primitive_conversion {
    ($t:ty, $new_t:ident, $rust_t:ty) => {
        impl ToClNumber<$t> for $new_t {
            fn to_cl_number(self) -> $t {
                *self
            }
        }

        impl FromClNumber<$t> for $new_t {
            fn from_cl_number(val: $t) -> $new_t {
                $new_t(val)
            }
        }

        impl FromClNumber<$t> for $rust_t {
            fn from_cl_number(val: $t) -> $rust_t {
                val
            }
        }
    }
}

/// A Boolean condition: true (1) or false (0)
pub enum ClBool {
    True,
    False,
}
/// Signed two’s complement 8-bit integer
/// 64-bit floating-point value.
/// May not be available on some platforms.
/// Check availability with FFI call to `clGetDeviceInfo` or `device.extensions()`
/// Enable in kernels with `#pragma OPENCL EXTENSION cl_khr_fp64 : enable`
pub struct ClDouble(cl_double);

/// Unsigned integer produced by the size of operator
pub struct SizeT(size_t);

// /// 16-bit floating-point value, IEEE-754-2008 conformant
pub struct ClHalf(cl_half);

macro_rules! from_cl_number_inner_s {
    ($t:ty, $new_t:ident, $rust_t:ty) => {
        impl FromClNumber<$t> for $rust_t {
            fn from_cl_number(num: $t) -> $rust_t {
                unsafe { num.s }
            }
        }

        impl ToClNumber<$t> for $rust_t {
            fn to_cl_number(self) -> $t {
                let mut num = unsafe { std::mem::zeroed::<$t>() };
                num.s = self;
                num
            }
        }

        impl ToClNumber<$t> for $new_t {
            fn to_cl_number(self) -> $t {
                self.0
            }
        }

    };
}

macro_rules! from_cl_number_inner_s3 {
    ($t:ident, $new_t:ident, $rust_t:ident) => {
        paste::item! {
            impl FromClNumber<[<$t 3>]> for [$rust_t; 3] {
                fn from_cl_number(num: [<$t 3>]) -> [$rust_t; 3] {
                    let inner = unsafe { num.s };
                    [inner[0], inner[1], inner[2]]
                }
            }

            impl FromClNumber<[<$t 3>]> for [<$new_t 3>] {
                fn from_cl_number(num: [<$t 3>]) -> [<$new_t 3>] {
                    [<$new_t 3>](num)
                }
            }

            impl ToClNumber<[<$t 3>]> for [$rust_t; 3] {
                fn to_cl_number(self) -> [<$t 3>] {
                    let mut num = unsafe { std::mem::zeroed::<[<$t 3>]>() };
                    let new_inner = [self[0], self[1], self[2], 0 as $t];
                    num.s = new_inner;
                    num
                }
            }

            impl ToClNumber<[<$t 3>]> for  [<$new_t 3>] {
                fn to_cl_number(self) -> [<$t 3>] {
                    self.0
                }
            }
        }
    };
}

macro_rules! impl_number_typed_t {
    ($snake:ident, $pascal:ident) => {
        impl NumberTypedT for $snake {
            fn number_type() -> NumberType {
                NumberType::$pascal(Kind::Primitive)
            }
        }

        impl NumberTypedT for $pascal {
            fn number_type() -> NumberType {
                NumberType::$pascal(Kind::Primitive)
            }
        }
    };

    ($snake:ident, $pascal:ident, $num:expr) => {
        paste::item! {
            impl NumberTypedT for [<$pascal $num>] {
                fn number_type() -> NumberType {
                    NumberType::[<$pascal>](num_to_kind!($num))
                }
            }

            impl NumberTypedT for [<$snake $num>] {
                fn number_type() -> NumberType {
                    NumberType::[<$pascal>](num_to_kind!($num))
                }
            }
        }
    }
}



macro_rules! impl_zeroed_num_vector {
    ($t:ident, $num:expr) => {
        paste::item! {
            impl Zeroed for [<$t $num>] {
                fn zeroed() -> Self {
                    unsafe { std::mem::zeroed::<[<$t $num>]>() }   
                }
            }
        }
    }
}

impl Zeroed for f64 {
    fn zeroed() -> Self {
        0.0 as f64
    }
}

macro_rules! impl_zeroed_num {
    ($t:ident) => {
        impl Zeroed for $t {
            fn zeroed() -> Self {
                0 as $t
            }
        }
        
        impl_zeroed_num_vector!($t, 2);
        // impl_zeroed_num_vector!($t, 3);
        impl_zeroed_num_vector!($t, 4);
        impl_zeroed_num_vector!($t, 8);
        impl_zeroed_num_vector!($t, 16);
    }
}




macro_rules! num_to_kind {
    (1) => { Kind::Primitive };
    (2) => { Kind::Two };
    (3) => { Kind::Three };
    (4) => { Kind::Four };
    (8) => { Kind::Eight };
    (16) => { Kind::Sixteen };
}

macro_rules! newtype_primitive_and_newtype_vectors {
    ($t:ident, $new_t:ident, $rust_t:ident) => {
        paste::item! {
            pub struct $new_t($t);
            /// Vector containing 2 $rust_t
            pub struct [<$new_t 2>]([<$t 2>]);
            
            /// Vector containing 3 $rust_t
            pub struct [<$new_t 3>]([<$t 3>]);
            
            /// Vector containing 4 $rust_t
            pub struct [<$new_t 4>]([<$t 4>]);

            /// Vector containing 8 $rust_t
            pub struct [<$new_t 8>]([<$t 8>]);

            /// Vector containing 16 $rust_t
            pub struct [<$new_t 16>]([<$t 16>]);

            // unsafe impl ClNumber for $t {}
            unsafe impl ClNumber for [<$t 2>] {}
            unsafe impl ClNumber for [<$t 4>] {}
            // unsafe impl ClNumber for [<$t 3>] {}
            unsafe impl ClNumber for [<$t 8>] {}
            unsafe impl ClNumber for [<$t 16>] {}

            impl_zeroed_num!($t);

            // impl_type_name!([<$t 2>]);
            // impl_type_name!([<$t 4>]);
            // impl_type_name!([<$t 8>]);
            // impl_type_name!([<$t 16>]);

            impl_deref!($new_t => $t);
            impl_deref!([<$new_t 2>] => [<$t 2>]);
            impl_deref!([<$new_t 3>] => [<$t 3>]);
            impl_deref!([<$new_t 4>] => [<$t 4>]);
            impl_deref!([<$new_t 8>] => [<$t 8>]);
            impl_deref!([<$new_t 16>] => [<$t 16>]);

            impl_number_typed_t!($t, $new_t);
            impl_number_typed_t!($t, $new_t, 2);
            // impl_number_typed_t!($t, $new_t, 3);
            impl_number_typed_t!($t, $new_t, 4);
            impl_number_typed_t!($t, $new_t, 8);
            impl_number_typed_t!($t, $new_t, 16);

            impl_primitive_conversion!($t, $new_t, $rust_t);
            from_cl_number_inner_s!([<$t 2>], [<$new_t 2>], [$rust_t; 2]);
            from_cl_number_inner_s!([<$t 4>], [<$new_t 4>], [$rust_t; 4]);
            from_cl_number_inner_s!([<$t 8>], [<$new_t 8>], [$rust_t; 8]);
            from_cl_number_inner_s!([<$t 16>], [<$new_t 16>], [$rust_t; 16]);
            from_cl_number_inner_s3!($t, $new_t, $rust_t);
        }
    }
}

newtype_primitive_and_newtype_vectors!(cl_char, ClChar, i8);
newtype_primitive_and_newtype_vectors!(cl_uchar, ClUchar, u8);
newtype_primitive_and_newtype_vectors!(cl_short, ClShort, i16);
newtype_primitive_and_newtype_vectors!(cl_ushort, ClUshort, u16);
newtype_primitive_and_newtype_vectors!(cl_int, ClInt, i32);
newtype_primitive_and_newtype_vectors!(cl_uint, ClUint, u32);
newtype_primitive_and_newtype_vectors!(cl_long, ClLong, i64);
newtype_primitive_and_newtype_vectors!(cl_ulong, ClUlong, u64);
newtype_primitive_and_newtype_vectors!(cl_float, ClFloat, f32);
