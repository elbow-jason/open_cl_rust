///
/// The OpenCL implementation is thread-safe for API calls that create,
/// retain and release objects such as a context, command-queue, program,
/// kernel and memory objects. OpenCL API calls that queue commands to a
/// command-queue or change the state of OpenCL objects such as command-queue
/// objects, memory objects, program and kernel objects are not thread-safe.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

// #[macro_use] extern crate lazy_static;

// #[macro_use] extern crate bitflags;

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
mod testing;

// macros are in there keep it before dependent modules. order matters.
#[macro_use]
mod macros;

// pub mod utils;

pub extern crate opencl_low_level;
pub extern crate opencl_sys as ffi;

pub use opencl_low_level as ll;

extern crate num;

pub mod buffer;
pub mod command_queue;
pub mod context;
pub mod device;
pub mod kernel;
pub mod platform;
pub mod program;
pub mod session;

pub use buffer::Buffer;
pub use command_queue::CommandQueue;
pub use context::Context;
pub use device::Device;
pub use kernel::{Kernel, KernelOpArg, KernelOperation, ReturnArg};
pub use platform::Platform;
pub use program::{Program, UnbuiltProgram};
pub use session::Session;

pub use ll::{
    BufferCreator, ClNumber, CommandQueueOptions, CommandQueueProperties, DeviceType, Error,
    HostAccess, KernelAccess, MemConfig, MemLocation, MutVecOrSlice, Output, VecOrSlice, Waitlist,
    Work,
};
