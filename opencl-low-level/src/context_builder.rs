
use crate::{
    Error, ClPlatformID, DeviceType, ClDeviceID, list_platforms, list_devices_by_type,
    Output, ClContext
};

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ContextBuilderError {
    #[fail(display = "For context building devices and device type cannot both be specified")]
    CannotSpecifyDevicesAndDeviceType,

    #[fail(display = "For context building devices and platforms cannot both be specified")]
    CannotSpecifyDevicesAndPlatforms,
}

const DEVICES_AND_DEVICE_TYPE_ERROR: Error = Error::ContextBuilderError(ContextBuilderError::CannotSpecifyDevicesAndDeviceType);
const DEVICES_AND_PLATFORMS_ERROR: Error = Error::ContextBuilderError(ContextBuilderError::CannotSpecifyDevicesAndPlatforms);

pub struct ClContextBuilder<'a> {
    pub platforms: Option<&'a [ClPlatformID]>,
    pub device_type: Option<DeviceType>,
    pub devices: Option<&'a [ClDeviceID]>,
}

impl<'a> ClContextBuilder<'a> {
    pub fn new() -> ClContextBuilder<'a> {
        ClContextBuilder {
            platforms: None,
            device_type: None,
            devices: None,
        }
    }

    pub fn with_platforms(mut self, platforms: &'a [ClPlatformID]) -> ClContextBuilder<'a> {
        self.platforms = Some(platforms);
        self
    }

    pub fn with_device_type(mut self, device_type: DeviceType) -> ClContextBuilder<'a> {
        self.device_type = Some(device_type);
        self
    }

    pub fn with_devices(mut self, devices: &'a [ClDeviceID]) -> ClContextBuilder<'a> {
        self.devices = Some(devices);
        self
    }

    pub unsafe fn build(self) -> Output<BuiltClContext> {
        use ClContextBuilder as B;
        match self {
            B{device_type: Some(device_type), devices: None, platforms: None} => ClContextBuilder::build_from_device_type(device_type),
            B{devices: Some(devices), device_type: None, platforms: None} => ClContextBuilder::build_from_devices(devices),
            B{platforms: Some(platforms), device_type: None, devices: None} => ClContextBuilder::build_from_platforms(platforms),
            B{platforms: Some(platforms), device_type: Some(device_type), devices: None} => ClContextBuilder::build_from_platforms_with_device_type(platforms, device_type),
            B{platforms: None, device_type: None, devices: None} => ClContextBuilder::build_with_defaults(),
            B{device_type: Some(_), devices: Some(_), ..} => Err(DEVICES_AND_DEVICE_TYPE_ERROR),
            B{devices: Some(_), platforms: Some(_), ..} => Err(DEVICES_AND_PLATFORMS_ERROR),
        }
    }

    pub unsafe fn build_with_defaults() -> Output<BuiltClContext> {
        let platforms = list_platforms()?;
        ClContextBuilder::build_from_platforms(&platforms[..])
    }

    pub unsafe fn build_from_platforms(platforms: &[ClPlatformID]) -> Output<BuiltClContext> {
        ClContextBuilder::build_from_platforms_with_device_type(platforms, DeviceType::ALL)
    }


    pub unsafe fn build_from_platforms_with_device_type(platforms: &[ClPlatformID], device_type: DeviceType) -> Output<BuiltClContext> {
        let mut devices = Vec::new();
        for p in platforms.iter() {
            let p_devices = list_devices_by_type(p, device_type)?;
            devices.extend(p_devices);
        }
        let context = ClContext::create(&devices[..])?;
        Ok(BuiltClContext::ContextWithDevices(context, devices))
    }

    pub unsafe fn build_from_devices(devices: &[ClDeviceID]) -> Output<BuiltClContext> {
        let context = ClContext::create(&devices[..])?;
        Ok(BuiltClContext::Context(context))
    }

    pub unsafe fn build_from_device_type(device_type: DeviceType) -> Output<BuiltClContext> {
        let platforms = list_platforms()?;
        ClContextBuilder::build_from_platforms_with_device_type(&platforms[..], device_type)
    }
}

pub enum BuiltClContext {
    Context(ClContext),
    ContextWithDevices(ClContext, Vec<ClDeviceID>)
}
