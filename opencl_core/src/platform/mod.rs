pub mod low_level;
pub mod flags;

use crate::ffi::cl_platform_id;
use crate::device::Device;
use crate::error::Output;


// NOTE: cl_platform_id is host mem?
// https://stackoverflow.com/questions/17711407/opencl-releasing-platform-object
// so no retain or release for platform 
__impl_unconstructable_cl_wrapper!(Platform, cl_platform_id);
__impl_cl_object_for_wrapper!(Platform, cl_platform_id);

use flags::PlatformInfo;

impl Platform {

    pub fn count() -> Output<u32> {
        low_level::cl_get_platforms_count()
    }

    pub fn all() -> Output<Vec<Platform>> {
        low_level::cl_get_platforms()
    }

    pub fn all_devices(&self) -> Output<Vec<Device>> {
        Device::all(self)
    }

    fn get_info(&self, info_code: PlatformInfo) -> Output<String> {
        low_level::cl_get_platform_info(self, info_code)
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


