// list_devices_by_type,
// ClCommandQueue, ClContext, ClDeviceID, ClKernel, ClMem,
// ClProgram, DeviceType, MemConfig,
use crate::{CommandQueue, Context, Device, Kernel, Mem, MemConfig, Number, Platform, Program};

// use crate::numbers::Number;

// use std::{thread, time};

// pub fn sleep(ms: u64) {
//     println!("Sleeping for {:?}ms", ms);
//     let dur = time::Duration::from_millis(ms);
//     thread::sleep(dur);
// }

pub fn get_kernel(src: &str, kernel_name: &str) -> (Context, Vec<Device>, Program, Kernel) {
    let (program, devices, context) = get_program(src);
    let kernel: Kernel = unsafe { Kernel::create(&program, kernel_name).unwrap() };
    (context, devices, program, kernel)
}

pub fn get_mem<T: Number>(size: usize) -> (Vec<Device>, Context, Mem) {
    let (context, devices) = get_context();
    let mem_config = MemConfig::default();
    let ll_mem =
        unsafe { Mem::create_with_config::<T, usize>(&context, size, mem_config).unwrap() };
    (devices, context, ll_mem)
}

pub fn mem_from_data_and_context<T: Number>(data: &[T], context: &Context) -> Mem {
    unsafe { Mem::create_with_config::<T, &[T]>(context, data, MemConfig::for_data()).unwrap() }
}

pub fn get_program(src: &str) -> (Program, Vec<Device>, Context) {
    let devices = list_devices();
    let context = context_from_devices(&devices[..]);
    let mut program = unsafe { Program::create_with_src(&context, src).unwrap() };
    program.build(&devices[..]).unwrap();
    (program, devices, context)
}

pub fn get_context() -> (Context, Vec<Device>) {
    let devices = list_devices();
    (context_from_devices(&devices[..]), devices)
}

pub fn get_command_queues() -> (Vec<CommandQueue>, Context, Vec<Device>) {
    let (context, devices) = get_context();
    let cqs = devices
        .iter()
        .map(|device| unsafe { CommandQueue::create(&context, device, None).unwrap() })
        .collect();

    (cqs, context, devices)
}

pub fn context_from_devices(devices: &[Device]) -> Context {
    unsafe { Context::create(&devices[..]).unwrap() }
}

pub fn list_devices() -> Vec<Device> {
    let platforms = Platform::list_all().expect("Failed to list_platforms");
    let mut devices = Vec::new();
    for platform in platforms.iter() {
        let p_devices = platform.list_devices().expect("Failed to list all devices");
        devices.extend(p_devices);
    }
    if devices.is_empty() {
        panic!("Failed to list_devices: devices was an empty vec");
    }
    devices
}

#[allow(dead_code)]
pub fn with_each_device<F>(f: F)
where
    F: Fn(&Device),
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
