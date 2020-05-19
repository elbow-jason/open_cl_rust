#![allow(dead_code)]
use crate::*;
// use std::sync::RwLock;

pub fn src_buffer_plus_one() -> &'static str {
    "__kernel void test(__global int *i) { *i += 1; }"
}

pub fn get_session(src: &str) -> Session {
    get_sessions(src).remove(0)
}

pub fn get_sessions(src: &str) -> Vec<Session> {
    Session::create(src, None).unwrap_or_else(|e| {
        panic!("Failed to create session: {:?}", e);
    })
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

pub fn get_device() -> Device {
    let platform = Platform::default();
    let mut devices: Vec<Device> = Device::list_devices_by_type(&platform, DeviceType::ALL)
        .unwrap_or_else(|e| panic!("Failed to list all devices: {:?}", e));
    assert!(devices.len() > 0);
    devices.remove(0)
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

pub fn get_buffer<T: Number>(size: usize) -> Buffer {
    let context = testing::get_context();
    Buffer::create::<T, usize>(
        &context,
        size,
        HostAccess::ReadWrite,
        KernelAccess::ReadWrite,
        MemLocation::AllocOnDevice,
    )
    .unwrap()
}

// pub fn test_all_devices<F>(callback: &mut F)
// where
//     F: FnMut(&Device, &Context, &CommandQueue),
// {
//     let devices = get_all_devices();

//     assert!(devices.len() > 0, "No usable devices found");

//     let context = Context::create(&devices[..]).unwrap_or_else(|e| {
//         panic!(
//             "Failed to Context::create with devices {:?} due to {:?}",
//             devices, e
//         );
//     });
//     for device in devices {
//         let queue = CommandQueue::create(&context, &device, None).unwrap_or_else(|e| {
//             panic!("Failed to CommandQueue::create due to {:?}", e);
//         });
//         callback(&device, &context, &queue);
//     }
// }
