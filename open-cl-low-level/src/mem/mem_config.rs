use crate::cl::cl_mem_flags;
use crate::cl::{HostAccessMemFlags, KernelAccessMemFlags, MemFlags, MemLocationMemFlags};

pub enum KernelAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<KernelAccess> for KernelAccessMemFlags {
    fn from(kernel_access: KernelAccess) -> KernelAccessMemFlags {
        match kernel_access {
            KernelAccess::ReadOnly => KernelAccessMemFlags::READ_ONLY,
            KernelAccess::WriteOnly => KernelAccessMemFlags::WRITE_ONLY,
            KernelAccess::ReadWrite => KernelAccessMemFlags::READ_WRITE,
        }
    }
}

impl From<KernelAccess> for MemFlags {
    fn from(kernel_access: KernelAccess) -> MemFlags {
        MemFlags::from(KernelAccessMemFlags::from(kernel_access))
    }
}

impl From<KernelAccess> for cl_mem_flags {
    fn from(kernel_access: KernelAccess) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(kernel_access))
    }
}

pub enum HostAccess {
    ReadOnly,
    WriteOnly,
    NoAccess,
    ReadWrite,
}

impl From<HostAccess> for HostAccessMemFlags {
    fn from(host_access: HostAccess) -> HostAccessMemFlags {
        match host_access {
            HostAccess::ReadOnly => HostAccessMemFlags::READ_ONLY,
            HostAccess::WriteOnly => HostAccessMemFlags::WRITE_ONLY,
            HostAccess::NoAccess => HostAccessMemFlags::NO_ACCESS,
            HostAccess::ReadWrite => HostAccessMemFlags::READ_WRITE,
        }
    }
}

impl From<HostAccess> for MemFlags {
    fn from(host_access: HostAccess) -> MemFlags {
        MemFlags::from(HostAccessMemFlags::from(host_access))
    }
}

impl From<HostAccess> for cl_mem_flags {
    fn from(host_access: HostAccess) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(host_access))
    }
}

// NOTE: MemLocation should be call `MemAllocation`.

/// The enumeration of how memory allocation (or not) can be directed.
///
/// This forum post has some good explanations:
///   https://software.intel.com/en-us/forums/opencl/topic/708049
pub enum MemLocation {
    KeepInPlace,
    AllocOnDevice,
    CopyToDevice,
    ForceCopyToDevice,
}

impl From<MemLocation> for MemLocationMemFlags {
    fn from(mem_location: MemLocation) -> MemLocationMemFlags {
        match mem_location {
            MemLocation::KeepInPlace => MemLocationMemFlags::KEEP_IN_PLACE,
            MemLocation::AllocOnDevice => MemLocationMemFlags::ALLOC_ON_DEVICE,
            MemLocation::CopyToDevice => MemLocationMemFlags::COPY_TO_DEVICE,
            MemLocation::ForceCopyToDevice => MemLocationMemFlags::FORCE_COPY_TO_DEVICE,
        }
    }
}

impl From<MemLocation> for MemFlags {
    fn from(mem_location: MemLocation) -> MemFlags {
        MemFlags::from(MemLocationMemFlags::from(mem_location))
    }
}

impl From<MemLocation> for cl_mem_flags {
    fn from(mem_location: MemLocation) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(mem_location))
    }
}

pub struct MemConfig {
    pub host_access: HostAccess,
    pub kernel_access: KernelAccess,
    pub mem_location: MemLocation,
}

impl MemConfig {
    pub fn build(
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> MemConfig {
        MemConfig {
            host_access,
            kernel_access,
            mem_location,
        }
    }
}

impl From<MemConfig> for MemFlags {
    fn from(mem_config: MemConfig) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(cl_mem_flags::from(mem_config)) }
    }
}

impl From<MemConfig> for cl_mem_flags {
    fn from(mem_config: MemConfig) -> cl_mem_flags {
        cl_mem_flags::from(mem_config.host_access)
            | cl_mem_flags::from(mem_config.kernel_access)
            | cl_mem_flags::from(mem_config.mem_location)
    }
}

impl Default for MemConfig {
    fn default() -> MemConfig {
        MemConfig {
            host_access: HostAccess::ReadWrite,
            kernel_access: KernelAccess::ReadWrite,
            mem_location: MemLocation::AllocOnDevice,
        }
    }
}

impl MemConfig {
    pub fn for_data() -> MemConfig {
        MemConfig {
            mem_location: MemLocation::CopyToDevice,
            ..MemConfig::default()
        }
    }

    pub fn for_size() -> MemConfig {
        MemConfig {
            mem_location: MemLocation::AllocOnDevice,
            ..MemConfig::default()
        }
    }
}

#[cfg(test)]
mod mem_flags_tests {
    use super::*;
    use crate::cl::KernelAccessMemFlags;

    #[test]
    fn kernel_access_read_only_conversion_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::ReadOnly;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::READ_ONLY
        );
    }

    #[test]
    fn kernel_access_write_only_conversion_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::WriteOnly;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::WRITE_ONLY
        );
    }

    #[test]
    fn kernel_access_convert_read_write_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::ReadWrite;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::READ_WRITE
        );
    }

    #[test]
    fn host_access_read_only_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::ReadOnly;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::READ_ONLY
        );
    }

    #[test]
    fn host_access_write_only_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::WriteOnly;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::WRITE_ONLY
        );
    }

    #[test]
    fn host_access_read_write_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::ReadWrite;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::READ_WRITE
        );
    }

    #[test]
    fn host_access_no_access_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::NoAccess;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::NO_ACCESS
        );
    }

    #[test]
    fn mem_location_keep_in_place_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::KeepInPlace;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::KEEP_IN_PLACE
        );
    }

    #[test]
    fn mem_location_alloc_on_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::AllocOnDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::ALLOC_ON_DEVICE
        );
    }

    #[test]
    fn mem_location_copy_to_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::CopyToDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_location_force_copy_to_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::ForceCopyToDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::FORCE_COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_config_conversion_into_cl_mem_flags() {
        let mem_location = MemLocation::AllocOnDevice;
        let host_access = HostAccess::ReadWrite;
        let kernel_access = KernelAccess::ReadWrite;
        let mem_config = MemConfig {
            mem_location,
            host_access,
            kernel_access,
        };
        let expected = MemFlags::ALLOC_HOST_PTR.bits()
            | MemFlags::HOST_READ_WRITE.bits()
            | MemFlags::KERNEL_READ_WRITE.bits();

        assert_eq!(cl_mem_flags::from(mem_config), expected);
    }
}
