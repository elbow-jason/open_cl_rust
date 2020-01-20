use crate::{
    ClDeviceID, list_platforms, DeviceType, list_devices_by_type, ClContext,
    ClProgram, ClNumber, ClMem, MemConfig,
};

pub fn get_mem<T: ClNumber>(size: usize) -> (ClMem<T>, ClContext, Vec<ClDeviceID>) {
    let (context, devices) = get_context();
    let mem_config = MemConfig::default();
    let ll_mem = ClMem::create_with_config(&context, size, mem_config).unwrap();
    (ll_mem, context, devices)
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

pub fn context_from_devices(devices: &[ClDeviceID]) -> ClContext {
    unsafe { ClContext::create(&devices[..]).unwrap() }
}

pub fn list_devices() -> Vec<ClDeviceID> {
    let platforms = list_platforms().expect("Failed to list_platforms");
    let mut devices = Vec::new();
    for platform in platforms.iter() {
        let p_devices = list_devices_by_type(&platform, DeviceType::ALL).expect("Failed to list all devices");
        devices.extend(p_devices);
    }
    if devices.is_empty() {
        panic!("Failed to list_devices: devices was an empty vec");
    }
    devices
}

pub fn with_each_device<F>(f: F) where F: Fn(&ClDeviceID) {
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
            panic!("
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
    }
}

