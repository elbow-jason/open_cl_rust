use crate::{
    ClDeviceID, list_platforms, DeviceType, list_devices_by_type, ClContext
};

pub fn get_context() -> ClContext {
    let devices = list_devices();
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

