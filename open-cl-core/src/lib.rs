///
/// The OpenCL implementation is thread-safe for API calls that create,
/// retain and release objects such as a context, command-queue, program,
/// kernel and memory objects. OpenCL API calls that queue commands to a
/// command-queue or change the state of OpenCL objects such as command-queue
/// objects, memory objects, program and kernel objects are not thread-safe.

// #[allow(unused_imports)]
// #[macro_use]
// extern crate log;

// // #[macro_use] extern crate lazy_static;

// // #[macro_use] extern crate bitflags;

// #[macro_use]
// extern crate failure;

// #[cfg(test)]
#[macro_use]
mod testing;

// // macros are in there keep it before dependent modules. order matters.
// #[macro_use]
// mod macros;

// // pub mod utils;

// pub extern crate open_cl_low_level;
// pub extern crate open_cl_sys as ffi;

pub use open_cl_low_level as ll;

pub use ll::{Error, Output};

// extern crate num;

pub mod platform;
pub use platform::Platform;

pub mod device;
pub use device::Device;

pub mod context;
pub use context::Context;

pub mod program;
pub use program::{Program, UnbuiltProgram};

pub mod buffer;
pub use buffer::Buffer;

pub mod session;
pub use session::Session;

// pub mod command_queue;

// pub mod kernel;
// pub use kernel::{Kernel, KernelOperation};

// #[cfg(test)]
// mod tests;

// pub use command_queue::CommandQueue;

pub use ll::cl::{ClObject, CommandQueueProperties, DeviceType, MemFlags};
pub use ll::numbers::{Number, NumberType, NumberTyped, NumberTypedT};
pub use ll::vec_or_slice::{MutVecOrSlice, VecOrSlice};
pub use ll::{
    ArgPtr, AsPtr, BufferBuilder, CommandQueueOptions, Dims, HostAccess, KernelAccess, KernelArg,
    KernelOperation, MemConfig, MemLocation, StatusCodeError, Waitlist, Work,
};
