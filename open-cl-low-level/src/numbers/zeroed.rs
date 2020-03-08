use half::f16;
use super::ffi_types::*;
use super::newtypes::*;

pub trait Zeroed {
    fn zeroed() -> Self;
}


impl Zeroed for f64 {
    fn zeroed() -> Self {
        0.0 as f64
    }
}

impl Zeroed for ClBool {
    fn zeroed() -> Self {
        ClBool::False
    }
}

impl Zeroed for f16 {
    fn zeroed() -> Self {
        f16::from_f32(0.0)
    }
}

impl Zeroed for ClHalf {
    fn zeroed() -> Self {
        ClHalf(0)
    }
}

impl Zeroed for ClDouble {
    fn zeroed() -> Self {
        ClDouble(0.0)
    }
}

impl Zeroed for SizeT {
    fn zeroed() -> SizeT {
        SizeT(0 as libc::size_t)
    }
}


impl Zeroed for libc::size_t {
    fn zeroed() -> libc::size_t {
        0 as libc::size_t
    }
}

macro_rules! impl_ffi {
    ($ffi_t:ident) => {
        impl Zeroed for $ffi_t {
            fn zeroed() -> Self {
                0 as $ffi_t
            }
        }
    };

    ($ffi_t:ident, $num:expr) => {
        paste::item! {
            impl Zeroed for [<$ffi_t $num>] {
                fn zeroed() -> Self {
                    unsafe { std::mem::zeroed::<[<$ffi_t $num>]>() }   
                }
            }
        }
    };
}


macro_rules! impl_new_t {
    ($ffi_t:ty, $new_t:ident) => {
        impl Zeroed for $new_t {
            fn zeroed() -> Self {
                $new_t(<$ffi_t>::zeroed())
            }
        }
    };
    ($snake:ident, $pascal:ident, $num:expr) => {
        paste::item! {
            impl Zeroed for [<$pascal $num>] {
                fn zeroed() -> Self {
                    let zero = unsafe { std::mem::zeroed::<[<$snake $num>]>() };
                    [<$pascal $num>](zero)
                }
            }
        }
    };
}


macro_rules! impl_primitive_and_vectors {
    ($snake:ident, $pascal:ident) => {
        impl_ffi!($snake);
        impl_ffi!($snake, 2);
        // impl_ffi!($snake, 3);
        impl_ffi!($snake, 4);
        impl_ffi!($snake, 8);
        impl_ffi!($snake, 16);

        impl_new_t!($snake, $pascal);
        impl_new_t!($snake, $pascal, 2);
        impl_new_t!($snake, $pascal, 3);
        impl_new_t!($snake, $pascal, 4);
        impl_new_t!($snake, $pascal, 8);
        impl_new_t!($snake, $pascal, 16);
    }
}


impl_primitive_and_vectors!(cl_uchar, ClUchar);
impl_primitive_and_vectors!(cl_char, ClChar);
impl_primitive_and_vectors!(cl_short, ClShort);
impl_primitive_and_vectors!(cl_ushort, ClUshort);
impl_primitive_and_vectors!(cl_int, ClInt);
impl_primitive_and_vectors!(cl_uint, ClUint);
impl_primitive_and_vectors!(cl_long, ClLong);
impl_primitive_and_vectors!(cl_ulong, ClUlong);
impl_primitive_and_vectors!(cl_float, ClFloat);

