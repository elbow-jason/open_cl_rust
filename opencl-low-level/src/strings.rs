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
