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

pub mod codes;
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

// #[test]
// fn test_size_of_static_isize_array_3_is_expected() {
//     let group_size = std::mem::size_of::<[isize; 3]>();
//     let item_size = std::mem::size_of::<isize>();
//     assert_eq!(group_size.checked_div(item_size), Some(3));
// }

// #[test]
// fn test_size_of_static_isize_array_2_is_expected() {
//     let group_size = std::mem::size_of::<[isize; 2]>();
//     let item_size = std::mem::size_of::<isize>();
//     assert_eq!(group_size.checked_div(item_size), Some(2));
// }

// #[test]
// fn test_size_of_isize_is_expected() {
//     let group_size = std::mem::size_of::<isize>();
//     let item_size = std::mem::size_of::<isize>();
//     assert_eq!(group_size.checked_div(item_size), Some(1));
// }

// fn test_global_work_dimensions_for_isize_array_2() {
//     let x: [isize; 2] = [100];
//     assert_eq!(x.work_dim(), 2);
// }
