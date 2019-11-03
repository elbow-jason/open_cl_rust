///
/// The OpenCL implementation is thread-safe for API calls that create,
/// retain and release objects such as a context, command-queue, program,
/// kernel and memory objects. OpenCL API calls that queue commands to a
/// command-queue or change the state of OpenCL objects such as command-queue
/// objects, memory objects, program and kernel objects are not thread-safe.
///
/// Options here: Don't allow
///

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate failure;

// macros are in there keep it first. order matters.
#[macro_use]
mod macros;

// pub mod utils;


pub extern crate opencl_sys as ffi;

extern crate num;

// pub mod open_cl;

pub mod command_queue;
pub mod context;
pub mod device;
pub mod device_mem;
pub mod event;
pub mod kernel;
pub mod platform;
pub mod program;
pub mod utils;
pub mod error;

pub use command_queue::CommandQueue;
pub use context::Context;
pub use device::Device;
pub use device_mem::DeviceMem;
pub use error::{Error, Output};
pub use event::Event;
pub use kernel::Kernel;
pub use platform::Platform;
pub use program::Program;
pub use utils::work::Work;
pub use utils::cl_object::ClObject;
pub use utils::status_code::StatusCode;
pub use utils::Volumetric;
pub use utils::Dims;

#[cfg(test)]
mod tests;
