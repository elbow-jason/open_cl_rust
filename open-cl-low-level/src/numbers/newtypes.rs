use std::ops::Deref;
use libc::size_t;

use super::ffi_types::*;
use super::conversion::*;

macro_rules! define_newtype_and_vectors {
    (cl_float, $new_t:ident, $rust_t:ty) => {
        paste::item! {
            #[derive(Debug, Clone, Copy)]
            pub struct $new_t(pub cl_float);
            
            /// Vector containing 2 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t 2>](pub [<cl_float 2>]);

            /// Vector containing 3 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t 3>](pub [<cl_float 3>]);
            
            /// Vector containing 4 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t 4>](pub [<cl_float 4>]);

            /// Vector containing 8 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t 8>](pub [<cl_float 8>]);

            /// Vector containing 16 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t 16>](pub [<cl_float 16>]);
        }

    };

    ($t:ident, $new_t:ident, $rust_t:ty) => {
        paste::item! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $new_t(pub $t);

            /// Vector containing 2 $rust_t
            define_vector_newtype!($t, $new_t, $rust_t, 2);
            define_vector_newtype!($t, $new_t, $rust_t, 3);
            define_vector_newtype!($t, $new_t, $rust_t, 4);
            define_vector_newtype!($t, $new_t, $rust_t, 8);
            define_vector_newtype!($t, $new_t, $rust_t, 16);
        }
    };
}


macro_rules! define_vector_newtype {
    ($t:ident, $new_t:ident, $rust_t:ty, $count:expr) => {
        paste::item! {            
            /// Vector containing 2 $rust_t
            #[derive(Clone, Copy)]
            pub struct [<$new_t $count>](pub [<$t $count>]);
            
            impl std::fmt::Debug for [<$new_t $count>] {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let rusty_val: [$rust_t; $count] = self.0.convert_to();
                    write!(f, "{}({:?})", stringify!([<$new_t $count>]), rusty_val)
                }
            }
        }
    }
}

define_newtype_and_vectors!(cl_char, ClChar, i8);
define_newtype_and_vectors!(cl_uchar, ClUchar, u8);
define_newtype_and_vectors!(cl_short, ClShort, i16);
define_newtype_and_vectors!(cl_ushort, ClUshort, u16);
define_newtype_and_vectors!(cl_int, ClInt, i32);
define_newtype_and_vectors!(cl_uint, ClUint, u32);
define_newtype_and_vectors!(cl_long, ClLong, i64);
define_newtype_and_vectors!(cl_ulong, ClUlong, u64);
define_newtype_and_vectors!(cl_float, ClFloat, f32);



/// A Boolean condition: true (1) or false (0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClBool {
    True,
    False,
}
/// Signed two’s complement 8-bit integer
/// 64-bit floating-point value.
/// May not be available on some platforms.
/// Check availability with FFI call to `clGetDeviceInfo` or `device.extensions()`
/// Enable in kernels with `#pragma OPENCL EXTENSION cl_khr_fp64 : enable`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClDouble(pub cl_double);

/// Unsigned integer produced by the size of operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeT(pub size_t);

/// 16-bit floating-point value, IEEE-754-2008 conformant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClHalf(pub cl_half);

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

macro_rules! impl_deref_for_vectors {
    ($new_t:ident => $t:ident) => {
        paste::item! {
            impl_deref!($new_t => $t);
            impl_deref!([<$new_t 2>] => [<$t 2>]);
            impl_deref!([<$new_t 3>] => [<$t 3>]);
            impl_deref!([<$new_t 4>] => [<$t 4>]);
            impl_deref!([<$new_t 8>] => [<$t 8>]);
            impl_deref!([<$new_t 16>] => [<$t 16>]);
        }
    }
}

impl_deref!(SizeT => size_t);
impl_deref!(ClDouble => cl_double);
impl_deref!(ClHalf => cl_half);

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

impl_deref_for_vectors!(ClChar => cl_char);
impl_deref_for_vectors!(ClUchar => cl_uchar);
impl_deref_for_vectors!(ClShort => cl_short);
impl_deref_for_vectors!(ClUshort => cl_ushort);
impl_deref_for_vectors!(ClInt => cl_int);
impl_deref_for_vectors!(ClUint => cl_uint);
impl_deref_for_vectors!(ClLong => cl_long);
impl_deref_for_vectors!(ClUlong => cl_ulong);
impl_deref_for_vectors!(ClFloat => cl_float);


// impl PartialEq for [<$new_t $count>] {
//     fn eq(&self, other: &Self) -> bool {
//         let left: [$rust_t; $count] = self.convert_to();
//         let right: [$rust_t; $count] = other.convert_to();
//         left == right
//     }
// }