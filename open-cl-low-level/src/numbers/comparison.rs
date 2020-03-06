use super::newtypes::*;
use super::conversion::ConvertTo;

macro_rules! eq_new_t_vector {
    ($new_t:ty, $ffi_t:ty, $rust_t:ty, $num:expr) => {
        paste::item! {
            impl PartialEq for [<$new_t $num>] {
                fn eq(&self, other: &Self) -> bool {
                    let left: [$rust_t; $num] = self.0.convert_to();
                    let right: [$rust_t; $num] = other.0.convert_to();
                    left.iter().zip(right.iter()).all(|(a,b)| a == b)
                }
            }
            
            impl Eq for [<$new_t $num>] {}
        }
    };
}

macro_rules! eq_new_t_all_vectors {
    ($new_t:ty, $ffi_t:ty, $rust_t:ty) => {
        eq_new_t_vector!($new_t, $ffi_t, $rust_t, 2);
        eq_new_t_vector!($new_t, $ffi_t, $rust_t, 3);
        eq_new_t_vector!($new_t, $ffi_t, $rust_t, 4);
        eq_new_t_vector!($new_t, $ffi_t, $rust_t, 8);
        eq_new_t_vector!($new_t, $ffi_t, $rust_t, 16);
    }
}

eq_new_t_all_vectors!(ClUchar, cl_uchar, u8);
eq_new_t_all_vectors!(ClChar, cl_char, i8);
eq_new_t_all_vectors!(ClUshort, cl_ushort, u16);
eq_new_t_all_vectors!(ClShort, cl_short, i16);
eq_new_t_all_vectors!(ClInt, cl_int, i32);
eq_new_t_all_vectors!(ClUint, cl_uint, u32);
eq_new_t_all_vectors!(ClLong, cl_long, i64);
eq_new_t_all_vectors!(ClUlong, cl_ulong, u64);



