#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;
#[macro_use] extern crate bitflags;

pub extern crate opencl_sys as ffi;


#[macro_use] pub mod ll_testing;
#[macro_use] pub mod macros;
pub mod error;
pub mod output;
pub mod status_code;
pub mod cl_enums;
pub mod cl_bitflags;
pub mod cl_helpers;
pub mod cl_pointer;
pub mod cl_input;
pub mod utils;
pub mod strings;

pub mod platform;
pub mod device;
pub mod context;
pub mod program;
pub mod mem;
pub mod kernel;
pub mod event;
pub mod wait_list;
pub mod command_queue;
pub mod volume;


pub use cl_pointer::ClPointer;
pub use output::{Output, build_output};
pub use status_code::StatusCodeError;
pub use error::Error;

pub use cl_input::*;
pub use cl_enums::*;
pub use cl_bitflags::*;

pub use platform::*;
pub use device::*;
pub use context::*;
pub use program::*;
pub use mem::*;
pub use kernel::*;
pub use event::*;
pub use wait_list::*;
pub use command_queue::*;
pub use volume::*;
// pub use device_ptr::DevicePtr;
