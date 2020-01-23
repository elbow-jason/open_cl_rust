///
/// The OpenCL implementation is thread-safe for API calls that create,
/// retain and release objects such as a context, command-queue, program,
/// kernel and memory objects. OpenCL API calls that queue commands to a
/// command-queue or change the state of OpenCL objects such as command-queue
/// objects, memory objects, program and kernel objects are not thread-safe.

#[allow(unused_imports)]
#[macro_use] extern crate log;

// #[macro_use] extern crate lazy_static;

// #[macro_use] extern crate bitflags;

#[macro_use] extern crate failure;


#[cfg(test)]
#[macro_use]
mod testing;

// macros are in there keep it before dependent modules. order matters.
#[macro_use] mod macros;

// pub mod utils;

pub extern crate opencl_sys as ffi;
pub extern crate opencl_low_level;

pub use opencl_low_level as ll;

extern crate num;

pub mod error;
// pub mod utils;
pub mod traits;

pub mod platform;
pub mod device;
pub mod context;
pub mod program;
pub mod buffer;
pub mod kernel;
pub mod command_queue;

// pub mod session;

pub use platform::Platform;
pub use device::Device;
pub use context::Context;
pub use program::{Program, UnbuiltProgram};
pub use buffer::Buffer;
pub use kernel::Kernel;
pub use command_queue::CommandQueue;

// pub use command_queue::CommandQueue;
// pub use device_mem::DeviceMem;
// pub use event::Event;


// pub use session::Session;

// pub use utils::status_code::StatusCode;
// pub use utils::work::Work;
// pub use utils::Dims;
// pub use utils::Volumetric;
