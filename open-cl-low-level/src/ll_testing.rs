use crate::{
    list_devices_by_type, list_platforms, ClCommandQueue, ClContext, ClDeviceID, ClKernel, ClMem,
    ClProgram, DeviceType, MemConfig,
};

use crate::numbers::Number;

use std::{thread, time};

pub fn sleep(ms: u64) {
    println!("Sleeping for {:?}ms", ms);
    let dur = time::Duration::from_millis(ms);
    thread::sleep(dur);
}

pub fn get_kernel(
    src: &str,
    kernel_name: &str,
) -> (ClContext, Vec<ClDeviceID>, ClProgram, ClKernel) {
    let (program, devices, context) = get_program(src);
    let kernel: ClKernel = unsafe { ClKernel::create(&program, kernel_name).unwrap() };
    (context, devices, program, kernel)
}

pub fn get_mem<T: Number>(size: usize) -> (Vec<ClDeviceID>, ClContext, ClMem) {
    let (context, devices) = get_context();
    let mem_config = MemConfig::default();
    let ll_mem =
        unsafe { ClMem::create_with_config::<T, usize>(&context, size, mem_config).unwrap() };
    (devices, context, ll_mem)
}

pub fn mem_from_data_and_context<T: Number>(data: &[T], context: &ClContext) -> ClMem {
    unsafe { ClMem::create_with_config(context, data, MemConfig::for_data()).unwrap() }
}

pub fn get_program(src: &str) -> (ClProgram, Vec<ClDeviceID>, ClContext) {
    let devices = list_devices();
    let context = context_from_devices(&devices[..]);
    let mut program = unsafe { ClProgram::create_with_source(&context, src).unwrap() };
    program.build(&devices[..]).unwrap();
    (program, devices, context)
}

pub fn get_context() -> (ClContext, Vec<ClDeviceID>) {
    let devices = list_devices();
    (context_from_devices(&devices[..]), devices)
}

pub fn get_command_queues() -> (Vec<ClCommandQueue>, ClContext, Vec<ClDeviceID>) {
    let (context, devices) = get_context();
    let cqs = devices
        .iter()
        .map(|device| unsafe { ClCommandQueue::create(&context, device, None).unwrap() })
        .collect();

    (cqs, context, devices)
}

pub fn context_from_devices(devices: &[ClDeviceID]) -> ClContext {
    unsafe { ClContext::create(&devices[..]).unwrap() }
}

pub fn list_devices() -> Vec<ClDeviceID> {
    let platforms = list_platforms().expect("Failed to list_platforms");
    let mut devices = Vec::new();
    for platform in platforms.iter() {
        let p_devices =
            list_devices_by_type(&platform, DeviceType::ALL).expect("Failed to list all devices");
        devices.extend(p_devices);
    }
    if devices.is_empty() {
        panic!("Failed to list_devices: devices was an empty vec");
    }
    devices
}

pub fn with_each_device<F>(f: F)
where
    F: Fn(&ClDeviceID),
{
    let devices = list_devices();
    for d in devices.iter() {
        f(d)
    }
}

#[macro_export]
macro_rules! expect_method {
    ($left:expr, $method:ident, $expected:expr) => {
        let left = &$left;
        let right = &$expected;
        if !left.$method(right) {
            panic!(
                "
            Assertion Failed!!!
                failed expression: {}.{}({})
                object: {:?}
                arg: {:?}
            ",
                stringify!($left),
                stringify!($method),
                stringify!($expected),
                left,
                right,
            )
        }
    };
}
