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

// macros are in there keep it first. order matters.
#[macro_use] mod macros;



// pub mod utils;

pub extern crate opencl_sys as ffi;
pub extern crate opencl_low_level;

pub use opencl_low_level as ll;

extern crate num;

pub mod error;
// pub mod utils;

pub mod platform;
pub mod device;
pub mod context;
pub mod program;
// pub mod device_mem;
// pub mod kernel;
// pub mod command_queue;
// pub mod event;
// pub mod session;

pub use platform::Platform;
pub use device::Device;
pub use context::Context;
pub use program::{Program, UnbuiltProgram};

// pub use command_queue::CommandQueue;
// pub use device_mem::DeviceMem;
// pub use event::Event;
// pub use kernel::{Kernel, KernelArg, KernelArgSizeAndPointer, KernelLock, KernelPtr};

// pub use session::Session;

// pub use utils::status_code::StatusCode;
// pub use utils::work::Work;
// pub use utils::Dims;
// pub use utils::Volumetric;

#[cfg(test)]
mod testing {
    #![allow(dead_code)]
    use crate::*;
    use crate::ll::*;
    // use std::sync::RwLock;
    
    pub fn src_buffer_plus_one() -> &'static str {
        "__kernel void test(__global int *i) { *i += 1; }"
    }

    // pub fn test_all<F>(test: &mut F)
    // where
    //     F: FnMut(&Device, &Context, &CommandQueue),
    // {
    //     let platforms = list_platforms().unwrap_or_else(|e| {
    //         panic!("Failed to retrieve plaforms via list_platforms() due to {:?}", e);
    //     });
    //     for p in platforms.iter() {
    //         let devices: Vec<Device> = p
    //             .all_devices()
    //             .unwrap_or_else(|e| {
    //                 panic!("Failed list all devices for {:?} due to {:?}", p, e);
    //             })
    //             .into_iter()
    //             .filter(|d| d.is_usable())
    //             .collect();

    //         assert!(devices.len() > 0, "No usable devices found");
    //         let context = Context::create(&devices[..])
    //                 .unwrap_or_else(|e| {
    //                     panic!("Failed to Context::create with devices {:?} due to {:?}", devices, e);
    //                 });
    //         for device in devices {
    //             let queue = CommandQueue::create(&context, &device, None)
    //                 .unwrap_or_else(|e| {
    //                     panic!("Failed to CommandQueue::create due to {:?}", e);
    //                 });
    //             test(&device, &context, &queue);
    //         }
    //     }
    // }

    // pub fn get_session(src: &str) -> Session {
    //     Session::create_sessions(&[Device::default()], src)
    //         .unwrap_or_else(|e| panic!("Failed to create Session: {:?}", e))
    //         .remove(0)
    // }

    // pub fn all_sessions(src: &str) -> Vec<Session> {
    //     let mut sessions = Vec::new();
    //     let platforms = Platform::all().unwrap();
    //     for p in platforms.iter() {
    //         let devices: Vec<Device> = p.all_devices().unwrap();
    //         let more_sessions: Vec<Session> = Session::create_sessions(&devices[..], src)
    //             .unwrap_or_else(|e| panic!("Failed to create Session: {:?}", e));
    //         sessions.extend(more_sessions);
    //     }
    //     sessions
    // }
        

    pub fn get_device() -> Device {
        let platform = Platform::default();
        let mut devices: Vec<Device> = Device::list_devices_by_type(&platform, DeviceType::ALL)
            .unwrap_or_else(|e| panic!("Failed to list all devices: {:?}", e));
        assert!(devices.len() > 0);
        devices.remove(0)
    }

    // lazy_static! {
    //     static ref LOG_INITED: RwLock<bool> = RwLock::new(false);
    // }

    // pub fn init_logger() {
    //     use std::io::Write;
    //     use chrono::Local;
    //     let _ = env_logger::builder()
    //         .is_test(true)
    //         .format(|buf, record| {
    //             writeln!(buf,
    //                 "{} [{}] - {}",
    //                 Local::now().format("%Y-%m-%dT%H:%M:%S%.6f"),
    //                 record.level(),
    //                 record.args()
    //             )
    //         })
    //         .init();
    //     let read_lock = LOG_INITED.read().unwrap();
    //     if *read_lock == true {
    //         return;
    //     } else {
    //         std::mem::drop(read_lock);
    //         let mut write_lock = LOG_INITED.write().unwrap();
    //         if *write_lock == false {
    //             *write_lock = true;
    //         }
    //     }
    // }

    // #[test]
    // fn logger_init_actually_lets_us_log() {
    //     println!("println in in logger_init_actually_lets_us_log");
    //     debug!("debug in logger_init_actually_lets_us_log");
    //     info!("info in logger_init_actually_lets_us_log");
    //     warn!("info in logger_init_actually_lets_us_log");
    //     error!("info in logger_init_actually_lets_us_log");
    // }
}