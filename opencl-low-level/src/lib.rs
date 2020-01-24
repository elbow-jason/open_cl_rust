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
pub mod volume;
pub mod waitlist;
pub mod work;

pub use cl_pointer::ClPointer;
pub use error::Error;
pub use output::{build_output, Output};
pub use status_code::StatusCodeError;

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
pub use volume::*;
pub use waitlist::*;
pub use work::*;
// pub use device_ptr::DevicePtr;
