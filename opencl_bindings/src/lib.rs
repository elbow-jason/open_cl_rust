

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
pub mod codes;

#[macro_use]
mod macros {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! size_t {
        ($t:ty) => {
            std::mem::size_of::<$t>() as libc::size_t
        }
    }
}

pub extern crate opencl_sys as ffi;

extern crate num;


pub mod open_cl;

pub mod command_queue;
pub mod context;
pub mod device;
pub mod device_mem;
pub mod event;
pub mod kernel;
pub mod platform;
pub mod program;
pub mod work;
pub mod utils;

pub use command_queue::*;
pub use context::*;
pub use device::*;
pub use device_mem::*;
pub use event::*;
pub use kernel::*;
pub use open_cl::*;
pub use platform::*;
pub use program::*;
pub use work::*;

#[cfg(test)]
mod tests;
