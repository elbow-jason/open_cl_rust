#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// not warn(improper_ctypes) can be removed if 128-bit integers get a stable ABI
#![allow(improper_ctypes)]


#[link(name = "OpenCL")]
#[cfg(target_os = "linux")]
extern "C" {}

#[link(name = "OpenCL", kind = "framework")]
#[cfg(target_os = "macos")]
extern "C" {}

include!(concat!(env!("OUT_DIR"), "/binding.rs"));
