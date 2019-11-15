#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
// NOTE: warn(improper_ctypes) can be removed if 128-bit integers get a stable ABI
#![allow(improper_ctypes)]

#[link(name = "OpenCL")]
#[cfg(target_os = "linux")]
extern "C" {}

#[link(name = "OpenCL", kind = "framework")]
#[cfg(target_os = "macos")]
extern "C" {}

#[cfg(all(target_os = "macos", debug_assertions))]
include!("../dev_bindings/macos_dev_bindings.rs");

#[cfg(not(debug_assertions))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
