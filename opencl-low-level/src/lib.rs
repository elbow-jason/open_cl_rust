/// The opencl-low-level crate is the lowest level crate of opencl that provides
/// functions that take common rust data structures (slice, vec, non-cl
/// primitives) as arguments for OpenCL's FFI.
///
/// The *true* lowest level crate for opencl is opencl-sys, but that package is
/// almost entirely a product of rust-bindgen.
///
/// Given the direct use of FFI in opencl-low-level and the fact that
/// mismanagement of the lifetimes of the pointer objects returned by many
/// OpenCL functions can easily lead to undefined behavior, there is a VERY
/// LARGE CAVEAT for using opencl-low-level functions and data structure
/// directly: KNOW WHAT YOU ARE DOING AND USE AT YOUR OWN RISK.
///
/// The pointers for OpenCL are all safe to send between threads, but neither
/// OpenCL nor the opencl-low-level library (this lib) provide synchronization
/// mechanism to protect Session from concurrent mutable access.

/// Additionally, nearly all structs, functions, method, and traits in
/// opencl-low-level are unsafe. The reasoning behind the, quite frankly,
/// extreme amount of unsafe code in the low-level crate is the danger of
/// working with raw pointers, manually managed atomic reference counting,
/// pointer object lifetime interdependency, dangling pointers, buffer overflows,
/// segmentation faults, shaky knees, sweaty palms, and self doubt...

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate bitflags;

extern crate num_complex;

pub extern crate opencl_sys as ffi;

#[macro_use]
pub mod ll_testing;
#[macro_use]
pub mod macros;
pub mod cl_bitflags;
pub mod cl_enums;
pub mod cl_helpers;
pub mod cl_input;
pub mod cl_number;
pub mod cl_pointer;
pub mod error;
pub mod output;
pub mod status_code;
pub mod strings;
pub mod utils;
pub mod vec_or_slice;

pub mod command_queue;
pub mod context;
pub mod context_builder;
pub mod device;

pub mod dims;
pub mod event;
pub mod kernel;
pub mod mem;
pub mod platform;
pub mod program;
pub mod session;
pub mod waitlist;
pub mod work;

pub use cl_pointer::ClPointer;
pub use error::Error;
pub use output::{build_output, Output};
pub use status_code::StatusCodeError;
pub use vec_or_slice::{MutVecOrSlice, VecOrSlice};

pub use cl_bitflags::*;
pub use cl_enums::*;
pub use cl_input::*;
pub use cl_number::ClNumber;

pub use context::*;
pub use context_builder::*;
pub use device::*;
pub use platform::*;

pub use command_queue::*;
pub use dims::*;
pub use event::*;
pub use kernel::*;
pub use mem::*;
pub use program::*;
pub use session::*;
pub use waitlist::*;
pub use work::*;
