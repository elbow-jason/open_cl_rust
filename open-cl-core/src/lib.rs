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

pub use ll::cl::{
    ClObject, CommandQueueProperties, DeviceAffinityDomain, DeviceType, MemFlags, StatusCodeError,
};
pub use ll::numbers::{Number, NumberType, NumberTyped, NumberTypedT};

pub use ll::vec_or_slice::{MutVecOrSlice, VecOrSlice};
pub use ll::{
    ArgPtr, AsPtr, BufferBuilder, CommandQueueOptions, Dims, HasDeviceInfo, HostAccess,
    KernelAccess, KernelArg, KernelOperation, Mem, MemAllocation, MemConfig, MemConfigBuilder,
    MemPtr, NumCastFrom, NumCastInto, NumberTypeError, Waitlist, Work,
};

pub mod number_types {
    pub use crate::ll::numbers::{Char, Char16, Char2, Char4, Char8};
    pub use crate::ll::numbers::{Double, Double16, Double2, Double4, Double8};
    pub use crate::ll::numbers::{Float, Float16, Float2, Float4, Float8};
    pub use crate::ll::numbers::{Int, Int16, Int2, Int4, Int8};
    pub use crate::ll::numbers::{Long, Long16, Long2, Long4, Long8};
    pub use crate::ll::numbers::{Short, Short16, Short2, Short4, Short8};
    pub use crate::ll::numbers::{Uchar, Uchar16, Uchar2, Uchar4, Uchar8};
    pub use crate::ll::numbers::{Uint, Uint16, Uint2, Uint4, Uint8};
    pub use crate::ll::numbers::{Ulong, Ulong16, Ulong2, Ulong4, Ulong8};
    pub use crate::ll::numbers::{Ushort, Ushort16, Ushort2, Ushort4, Ushort8};
}
