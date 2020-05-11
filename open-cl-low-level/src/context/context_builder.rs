use crate::cl::DeviceType;
use crate::{Context, Device, Output, Platform};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ContextBuilderError {
    #[error("For context building devices and device type cannot both be specified")]
    CannotSpecifyDevicesAndDeviceType,

    #[error("For context building devices and platforms cannot both be specified")]
    CannotSpecifyDevicesAndPlatforms,
}

use ContextBuilderError::*;

pub struct ClContextBuilder<'a> {
    pub platforms: Option<&'a [Platform]>,
    pub device_type: Option<DeviceType>,
    pub devices: Option<&'a [Device]>,
}

impl<'a> ClContextBuilder<'a> {
    pub fn new() -> ClContextBuilder<'a> {
        ClContextBuilder {
            platforms: None,
            device_type: None,
            devices: None,
        }
    }

    pub fn with_platforms(mut self, platforms: &'a [Platform]) -> ClContextBuilder<'a> {
        self.platforms = Some(platforms);
        self
    }

    pub fn with_device_type(mut self, device_type: DeviceType) -> ClContextBuilder<'a> {
        self.device_type = Some(device_type);
        self
    }

    pub fn with_devices(mut self, devices: &'a [Device]) -> ClContextBuilder<'a> {
        self.devices = Some(devices);
        self
    }

    pub unsafe fn build(self) -> Output<BuiltClContext> {
        use ClContextBuilder as B;
        match self {
            B {
                device_type: Some(device_type),
                devices: None,
                platforms: None,
            } => ClContextBuilder::build_from_device_type(device_type),
            B {
                devices: Some(devices),
                device_type: None,
                platforms: None,
            } => ClContextBuilder::build_from_devices(devices),
            B {
                platforms: Some(platforms),
                device_type: None,
                devices: None,
            } => ClContextBuilder::build_from_platforms(platforms),
            B {
                platforms: Some(platforms),
                device_type: Some(device_type),
                devices: None,
            } => ClContextBuilder::build_from_platforms_with_device_type(platforms, device_type),
            B {
                platforms: None,
                device_type: None,
                devices: None,
            } => ClContextBuilder::build_with_defaults(),
            B {
                device_type: Some(_),
                devices: Some(_),
                ..
            } => Err(CannotSpecifyDevicesAndDeviceType)?,
            B {
                devices: Some(_),
                platforms: Some(_),
                ..
            } => Err(CannotSpecifyDevicesAndPlatforms)?,
        }
    }

    pub unsafe fn build_with_defaults() -> Output<BuiltClContext> {
        let platforms = Platform::list_all()?;
        ClContextBuilder::build_from_platforms(&platforms[..])
    }

    pub unsafe fn build_from_platforms(platforms: &[Platform]) -> Output<BuiltClContext> {
        ClContextBuilder::build_from_platforms_with_device_type(platforms, DeviceType::ALL)
    }

    pub unsafe fn build_from_platforms_with_device_type(
        platforms: &[Platform],
        device_type: DeviceType,
    ) -> Output<BuiltClContext> {
        let mut devices = Vec::new();
        for p in platforms.iter() {
            let p_devices = p.list_devices_by_type(device_type)?;
            devices.extend(p_devices);
        }
        let context = Context::create(&devices[..])?;
        Ok(BuiltClContext::ContextWithDevices(context, devices))
    }

    pub unsafe fn build_from_devices(devices: &[Device]) -> Output<BuiltClContext> {
        let context = Context::create(&devices[..])?;
        Ok(BuiltClContext::Context(context))
    }

    pub unsafe fn build_from_device_type(device_type: DeviceType) -> Output<BuiltClContext> {
        let platforms = Platform::list_all()?;
        ClContextBuilder::build_from_platforms_with_device_type(&platforms[..], device_type)
    }
}

pub enum BuiltClContext {
    Context(Context),
    ContextWithDevices(Context, Vec<Device>),
}
