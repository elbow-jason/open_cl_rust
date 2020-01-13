pub mod dims;
pub mod flags;
pub mod status_code;
pub mod volume;
pub mod work;

pub use {
    dims::Dims,
    status_code::{ClError, StatusCode},
    volume::Volume,
    work::{Volumetric, VolumetricError, Work},
};

use crate::Error;
use crate::cl::ClObjectError;


/// Returns a Vec with *actual* length.
pub fn vec_filled_with<T: Clone>(filler: T, len: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(len);
    out.resize(len, filler);
    out
}

pub fn null_check<T>(ptr: *mut T, name: &str) -> Result<(), Error> {
    if ptr.is_null() {
        Err(ClObjectError::CannotBeNull(name.to_owned()).into())
    } else {
        Ok(())
    }
}
     

pub mod strings {
    use std::ffi::CString;

    pub fn to_c_string(string: &str) -> Option<CString> {
        CString::new(string).ok()
    }

    pub fn to_utf8_string(buffer: Vec<u8>) -> String {
        let safe_vec = buffer.into_iter().filter(|c| *c != 0u8).collect();
        String::from_utf8(safe_vec).unwrap_or_else(|err| {
            panic!("Failed to turn buffer (Vec<u8>) to UTF8 string. {:?}", err);
        })
    }
}

