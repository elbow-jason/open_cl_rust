
pub type Volume = [usize; 3];

#[inline]
pub fn to_ptr(vol: Volume) -> *const libc::size_t {
    ref_to_ptr(&vol)
}

#[inline]
pub fn ref_to_ptr(vol: &Volume) -> *const libc::size_t {
    vol as *const [usize; 3] as *const libc::size_t
}

#[inline]
pub fn option_to_ptr(option_vol: Option<Volume>) -> *const libc::size_t {
    match option_vol {
        Some(ref vol) => ref_to_ptr(vol),
        None => std::ptr::null(),
    }
}

#[inline]
pub fn option_ref_to_ptr(option_vol: Option<&Volume>) -> *const libc::size_t {
    match option_vol {
        Some(vol) => ref_to_ptr(vol),
        None => std::ptr::null(),
    }
}