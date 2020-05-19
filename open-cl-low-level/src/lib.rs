#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(proc_macro_hygiene)]

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
// extern crate proc_macro;

// public, but not `pub use ffi::*;`. Users shouldn't be poking in there unless
// they *REALLY* want it.

#[macro_use]
extern crate derive_more;

pub mod numbers;
pub use numbers::*;

pub type Output<T> = anyhow::Result<T>;

pub use thiserror::Error;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[macro_use]
#[cfg(test)]
mod ll_testing;

#[macro_use]
pub mod cl;

pub mod platform;
pub use platform::*;

pub mod device;
pub use device::*;

pub mod context;
pub use context::*;

pub mod program;
pub use program::*;

mod dims;
pub use dims::*;

pub mod mem;
pub use mem::*;

pub mod kernel;
pub use kernel::*;

pub mod command_queue;
pub use command_queue::*;

pub mod vec_or_slice;

pub mod session;
pub use session::*;
