///
/// The OpenCL implementation is thread-safe for API calls that create,
/// retain and release objects such as a context, command-queue, program,
/// kernel and memory objects. OpenCL API calls that queue commands to a
/// command-queue or change the state of OpenCL objects such as command-queue
/// objects, memory objects, program and kernel objects are not thread-safe.
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

pub mod cl;
pub mod command_queue;
pub mod context;
pub mod device;
pub mod device_mem;
pub mod error;
pub mod event;
pub mod kernel;
pub mod platform;
pub mod program;
pub mod session;
pub mod utils;

pub use command_queue::CommandQueue;
pub use context::Context;
pub use device::{Device, DevicePtr, DeviceType, DeviceInfo};
pub use device_mem::DeviceMem;
pub use error::{Error, Output};
pub use event::Event;
pub use kernel::{Kernel, KernelArg, KernelArgSizeAndPointer};
pub use platform::Platform;
pub use program::{ProgramPtr, Program, UnbuiltProgram};
pub use session::Session;

pub use utils::status_code::StatusCode;
pub use utils::work::Work;
pub use utils::Dims;
pub use utils::Volumetric;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod testing {
    #![allow(dead_code)]
    use crate::*;
    
    pub fn src_buffer_plus_one() -> &'static str {
        "__kernel void test(__global int *i) { *i += 1; }"
    }

    pub fn get_session(src: &str) -> Session {
        Session::create_sessions(&[Device::default()], src)
            .expect("Failed to create Session")
            .remove(0)
    }

    fn get_device() -> Device {
        let platform = Platform::default();
        let mut devices: Vec<Device> = Device::all_by_type(&platform, DeviceType::ALL).expect("Failed to list all devices");
        assert!(devices.len() > 0);
        devices.remove(0)
    }
}