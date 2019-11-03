
pub mod cl_object;
pub mod flags;
pub mod status_code;
pub mod volume;
pub mod work;
pub mod dims;

pub use {
    volume::Volume,
    work::{
        Work,
        Volumetric,
        VolumetricError
    },
    dims::Dims,
    cl_object::{
        ClObject,
        CopyClObject
    },
    status_code::{
        StatusCode,
        ClError,
    }

};

/// Returns a Vec with *actual* length.
pub fn vec_filled_with<T: Clone>(filler: T, len: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(len);
    out.resize(len, filler);
    out
}


pub mod strings {
    use std::ffi::CString;

    pub fn to_c_string(string: &str) -> Option<CString> {
        CString::new(string).ok()
    }

    pub fn to_utf8_string(buf: Vec<u8>) -> String {
        let safe_vec = buf.into_iter().filter(|c| *c != 0u8).collect();
        String::from_utf8(safe_vec).unwrap()
    }
}