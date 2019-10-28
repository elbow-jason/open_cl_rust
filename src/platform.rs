use crate::open_cl::{
    cl_get_platform_info, cl_get_platforms_count, cl_get_platforms_ids, cl_platform_id,
    cl_platform_info,
};
use crate::Device;
use crate::Output;

pub struct Platform(cl_platform_id);

impl Platform {
    pub fn new(platform_id: cl_platform_id) -> Platform {
        Platform(platform_id)
    }

    pub fn cl_object(&self) -> cl_platform_id {
        self.0
    }

    pub fn count() -> Output<u32> {
        cl_get_platforms_count()
    }

    pub fn all() -> Output<Vec<Self>> {
        let ids = cl_get_platforms_ids()?;
        Ok(ids.into_iter().map(|id| Platform::new(id)).collect())
    }

    pub fn all_devices(&self) -> Output<Vec<Device>> {
        Device::all(self)
    }

    fn get_info<I>(&self, info_code: I) -> Output<String>
    where
        I: Into<cl_platform_info>,
    {
        cl_get_platform_info(&self.cl_object(), I::into(info_code))
    }

    pub fn name(&self) -> Output<String> {
        self.get_info(PlatformInfo::Name)
    }

    pub fn version(&self) -> Output<String> {
        self.get_info(PlatformInfo::Version)
    }

    pub fn profile(&self) -> Output<String> {
        self.get_info(PlatformInfo::Profile)
    }

    pub fn vendor(&self) -> Output<String> {
        self.get_info(PlatformInfo::Vendor)
    }

    pub fn extensions(&self) -> Output<String> {
        self.get_info(PlatformInfo::Extensions)
    }

    pub fn host_timer_resolution(&self) -> Output<String> {
        self.get_info(PlatformInfo::HostTimerResolution)
    }
}

// /// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L260-L268
// /* cl_platform_info */
crate::__codes_enum!(PlatformInfo, cl_platform_info, {
    Profile => 0x0900,
    Version => 0x0901,
    Name => 0x0902,
    Vendor => 0x0903,
    Extensions => 0x0904,
    HostTimerResolution => 0x0905
});
