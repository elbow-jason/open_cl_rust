#![allow(dead_code)]
use crate::ll::*;
use crate::*;
// use std::sync::RwLock;

pub fn src_buffer_plus_one() -> &'static str {
    "__kernel void test(__global int *i) { *i += 1; }"
}

pub fn get_platforms() -> Vec<Platform> {
    Platform::list_all().unwrap()
}

pub fn get_all_devices() -> Vec<Device> {
    let platforms = get_platforms();
    let mut devices = Vec::new();
    for p in platforms.iter() {
        devices.extend(Device::list_all_devices(p).unwrap());
    }
    devices
}

fn unwrap_ctx(o: Output<Context>) -> Context {
    o.unwrap_or_else(|e| panic!("Failed to create context: {:?}", e))
}

pub fn get_context() -> Context {
    unwrap_ctx(Context::create(get_all_devices()))
}

pub fn get_program(src: &str) -> Program {
    let platforms = Platform::list_all().unwrap();
    let mut devices = Vec::new();
    for p in platforms.iter() {
        devices.extend(Device::list_all_devices(p).unwrap());
    }
    let context = unwrap_ctx(Context::create(&devices[..]));
    let unbuilt_program = Program::create_with_source(&context, src).unwrap();
    unbuilt_program.build(&devices[..]).unwrap()
}

pub fn get_buffer<T: ClNumber>(size: usize) -> Buffer<T> {
    let context = testing::get_context();
    Buffer::<T>::create(
        &context,
        size,
        HostAccess::ReadWrite,
        KernelAccess::ReadWrite,
        MemLocation::AllocOnDevice,
    )
    .unwrap()
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
