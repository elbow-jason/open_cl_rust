use super::{NumberTypedT, Zeroed};

// primitive types
use super::ffi_types::{
     cl_int, cl_long, cl_uchar, cl_uint, cl_ulong,
    cl_char, cl_half, cl_bool, cl_double, cl_float,
};

use crate::ffi::{
    cl_char16, cl_char2, cl_char3, cl_char4, cl_char8, 
    cl_float16, cl_float2, cl_float3, cl_float4, cl_float8,  cl_int16, cl_int2,
    cl_int3, cl_int4, cl_int8, cl_long16, cl_long2, cl_long3, cl_long4, cl_long8,
    cl_short, cl_short16, cl_short2, cl_short3, cl_short4, cl_short8, cl_uchar16,
    cl_uchar2, cl_uchar3, cl_uchar4, cl_uchar8, cl_uint16, cl_uint2, cl_uint3, cl_uint4,
    cl_uint8, cl_ulong16, cl_ulong2, cl_ulong3, cl_ulong4, cl_ulong8, cl_ushort,
    cl_ushort16, cl_ushort2, cl_ushort3, cl_ushort4, cl_ushort8,
};

use super::newtypes::*;

// NOTE: f32 does not implement Eq so it's not here. WHYEEEEE...
pub trait ClNumber:
    Sized + Clone + Copy + Send + Sync + 'static + Zeroed + NumberTypedT
{
    type Inner;
    fn inner(&self) -> Self::Inner; 
}


macro_rules! impl_cl_number {
    ( $( $new_t:ident => $ffi_t:ident ),* ) => {
        $(
            impl ClNumber for $new_t {
                type Inner = $ffi_t;
                fn inner(&self) -> Self::Inner {
                    self.0
                }
            }
        )*
    };
}

impl ClNumber for ClBool {
    type Inner = cl_bool;
    fn inner(&self) -> Self::Inner {
        match *self {
            ClBool::True => 1,
            ClBool::False => 0,
        }
    }
}

impl_cl_number!(
    ClHalf => cl_half,
    ClDouble => cl_double,
    ClFloat => cl_float, 
    ClChar => cl_char,
    ClUchar => cl_uchar,
    ClInt => cl_int,
    ClUint => cl_uint,
    ClLong => cl_long,
    ClUlong => cl_ulong,
    ClChar16 => cl_char16, 
    ClChar2 => cl_char2,
    ClChar3 => cl_char3,
    ClChar4 => cl_char4,
    ClChar8 => cl_char8,
    ClFloat16 => cl_float16,
    ClFloat2 => cl_float2,
    ClFloat3 => cl_float3,
    ClFloat4 => cl_float4,
    ClFloat8 => cl_float8,
    ClInt16 => cl_int16,
    ClInt2 => cl_int2,
    ClInt3 => cl_int3, 
    ClInt4 => cl_int4,
    ClInt8 => cl_int8,
    ClLong16 => cl_long16,
    ClLong2 => cl_long2,
    ClLong3 => cl_long3,
    ClLong4 => cl_long4,
    ClLong8 => cl_long8,
    ClShort => cl_short,
    ClShort16 => cl_short16,
    ClShort2 => cl_short2,
    ClShort3 => cl_short3,
    ClShort4 => cl_short4,
    ClShort8 => cl_short8,
    ClUchar16 => cl_uchar16,
    ClUchar2 => cl_uchar2, 
    ClUchar3 => cl_uchar3,
    ClUchar4 => cl_uchar4,
    ClUchar8 => cl_uchar8,
    ClUint16 => cl_uint16,
    ClUint2 => cl_uint2,
    ClUint3 => cl_uint3,
    ClUint4 => cl_uint4,
    ClUint8 => cl_uint8, 
    ClUlong16 => cl_ulong16,
    ClUlong2 => cl_ulong2,
    ClUlong3 => cl_ulong3,
    ClUlong4 => cl_ulong4,
    ClUlong8 => cl_ulong8,
    ClUshort => cl_ushort,
    ClUshort16 => cl_ushort16, 
    ClUshort2 => cl_ushort2,
    ClUshort3 => cl_ushort3,
    ClUshort4 => cl_ushort4,
    ClUshort8 => cl_ushort8
);






// newtype_primitive_and_newtype_vectors!(cl_char, ClChar, i8);
// newtype_primitive_and_newtype_vectors!(cl_uchar, ClUchar, u8);
// newtype_primitive_and_newtype_vectors!(cl_short, ClShort, i16);
// newtype_primitive_and_newtype_vectors!(cl_ushort, ClUshort, u16);
// newtype_primitive_and_newtype_vectors!(cl_int, ClInt, i32);
// newtype_primitive_and_newtype_vectors!(cl_uint, ClUint, u32);
// newtype_primitive_and_newtype_vectors!(cl_long, ClLong, i64);
// newtype_primitive_and_newtype_vectors!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(size_t, SizeT, usize);
// newtype_primitive_and_newtype_vectors!(cl_float, ClFloat, f32);
// impl_primitive_conversion!(cl_double, ClDouble, f64);





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


// impl ClNumber for u8 {}
// impl ClNumber for i8 {}
// impl ClNumber for u16 {}
// impl ClNumber for i16 {}
// impl ClNumber for u32 {}
// impl ClNumber for i32 {}
// impl ClNumber for f32 {}
// impl ClNumber for u64 {}
// impl ClNumber for i64 {}
// impl ClNumber for f64 {}



// pub trait FromClNumber<T: ClNumber> {
//     fn from_cl_number(value: Self) -> T;
// }

// pub trait ClNumberInto<T: ClNumber> where T: ClNumber, Self: FromClNumber<T> {
//     fn cl_number_into(val: T) -> Self {
//         Self::from_cl_number(val)
//     }
// }

// pub trait ClNumberFrom<T, N: ClNumber> {
//     fn cl_number_from(val: T) -> N;
// }

// pub trait IntoClNumber<T, N: ClNumber> where N: ClNumber, Self: ClNumberFrom<T> {
//     fn cl_number_into(val: T) -> Self {
//         Self::from_cl_number(val)
//     }
// }


// impl<T: ClNumber, R> ClNumberInto<T> for R where R: FromClNumber<T> {}














// impl ClNumber for $new_t {}
// impl ClNumber for [<$new_t 2>] {}
// impl ClNumber for [<$new_t 3>] {}
// impl ClNumber for [<$new_t 4>] {}
// impl ClNumber for [<$new_t 8>] {}
// impl ClNumber for [<$new_t 16>] {}



// macro_rules! newtype_primitive_and_newtype_vectors {
//     ($t:ident, $new_t:ident, $rust_t:ident) => {
//         paste::item! {

//             // define_newtypes!($t, $new_t, $rust_t);

//             // // unsafe impl ClNumber for $t {}
//             // impl ClNumber for [<$t 2>] {}
//             // impl ClNumber for [<$t 4>] {}
//             // // unsafe impl ClNumber for [<$t 3>] {}
//             // impl ClNumber for [<$t 8>] {}
//             // impl ClNumber for [<$t 16>] {}

//             // impl_type_name!([<$t 2>]);
//             // impl_type_name!([<$t 4>]);
//             // impl_type_name!([<$t 8>]);
//             // impl_type_name!([<$t 16>]);


            
//             // // from_cl_number_inner_s!($t, $new_t, $rust_t);
//             // from_cl_number_inner_s!([<$t 2>], [<$new_t 2>], [$rust_t; 2]);
//             // from_cl_number_inner_s!([<$t 4>], [<$new_t 4>], [$rust_t; 4]);
//             // from_cl_number_inner_s!([<$t 8>], [<$new_t 8>], [$rust_t; 8]);
//             // from_cl_number_inner_s!([<$t 16>], [<$new_t 16>], [$rust_t; 16]);
//             // from_cl_number_inner_s3!($t, $new_t, $rust_t);
//         }
//     }
// }

// newtype_primitive_and_newtype_vectors!(cl_char, ClChar, i8);
// newtype_primitive_and_newtype_vectors!(cl_uchar, ClUchar, u8);
// newtype_primitive_and_newtype_vectors!(cl_short, ClShort, i16);
// newtype_primitive_and_newtype_vectors!(cl_ushort, ClUshort, u16);
// newtype_primitive_and_newtype_vectors!(cl_int, ClInt, i32);
// newtype_primitive_and_newtype_vectors!(cl_uint, ClUint, u32);
// newtype_primitive_and_newtype_vectors!(cl_long, ClLong, i64);
// newtype_primitive_and_newtype_vectors!(cl_ulong, ClUlong, u64);
// impl_primitive_conversion!(size_t, SizeT, usize);
// newtype_primitive_and_newtype_vectors!(cl_float, ClFloat, f32);
// impl_primitive_conversion!(cl_double, ClDouble, f64);



