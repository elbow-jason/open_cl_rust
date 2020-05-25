use crate::cl::cl_mem_flags;
use crate::cl::{HostAccessMemFlags, KernelAccessMemFlags, MemAllocationMemFlags, MemFlags};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

/// The enumeration of how memory allocation (or not) can be directed.
///
/// This forum post has some good explanations:
///   https://software.intel.com/en-us/forums/opencl/topic/708049
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MemAllocation {
    KeepInPlace,
    AllocOnDevice,
    CopyToDevice,
    ForceCopyToDevice,
}

impl From<MemAllocation> for MemAllocationMemFlags {
    fn from(mem_allocation: MemAllocation) -> MemAllocationMemFlags {
        match mem_allocation {
            MemAllocation::KeepInPlace => MemAllocationMemFlags::KEEP_IN_PLACE,
            MemAllocation::AllocOnDevice => MemAllocationMemFlags::ALLOC_ON_DEVICE,
            MemAllocation::CopyToDevice => MemAllocationMemFlags::COPY_TO_DEVICE,
            MemAllocation::ForceCopyToDevice => MemAllocationMemFlags::FORCE_COPY_TO_DEVICE,
        }
    }
}

impl From<MemAllocation> for MemFlags {
    fn from(mem_allocation: MemAllocation) -> MemFlags {
        MemFlags::from(MemAllocationMemFlags::from(mem_allocation))
    }
}

impl From<MemAllocation> for cl_mem_flags {
    fn from(mem_allocation: MemAllocation) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(mem_allocation))
    }
}

pub struct MemConfigBuilder {
    pub host_access: HostAccess,
    pub kernel_access: KernelAccess,
    pub mem_allocation: MemAllocation,
}

impl Default for MemConfigBuilder {
    fn default() -> MemConfigBuilder {
        MemConfigBuilder {
            host_access: HostAccess::ReadWrite,
            kernel_access: KernelAccess::ReadWrite,
            mem_allocation: MemAllocation::AllocOnDevice,
        }
    }
}

impl MemConfigBuilder {
    pub fn build(&self) -> MemConfig {
        MemConfig {
            host_access: self.host_access,
            kernel_access: self.kernel_access,
            mem_allocation: self.mem_allocation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MemConfig {
    host_access: HostAccess,
    kernel_access: KernelAccess,
    mem_allocation: MemAllocation,
}

impl MemConfig {
    pub fn new(
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_allocation: MemAllocation,
    ) -> MemConfig {
        MemConfig {
            host_access,
            kernel_access,
            mem_allocation,
        }
    }

    pub fn as_builder(&self) -> MemConfigBuilder {
        MemConfigBuilder {
            host_access: self.host_access,
            kernel_access: self.kernel_access,
            mem_allocation: self.mem_allocation,
        }
    }

    pub fn cl_mem_flags(&self) -> cl_mem_flags {
        cl_mem_flags::from(self.host_access)
            | cl_mem_flags::from(self.kernel_access)
            | cl_mem_flags::from(self.mem_allocation)
    }

    pub fn host_access(&self) -> HostAccess {
        self.host_access
    }

    pub fn kernel_access(&self) -> KernelAccess {
        self.kernel_access
    }

    pub fn mem_allocation(&self) -> MemAllocation {
        self.mem_allocation
    }
}

impl From<MemConfig> for MemFlags {
    fn from(mem_config: MemConfig) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(cl_mem_flags::from(mem_config)) }
    }
}

impl From<MemConfig> for cl_mem_flags {
    fn from(mem_config: MemConfig) -> cl_mem_flags {
        mem_config.cl_mem_flags()
    }
}

impl Default for MemConfig {
    fn default() -> MemConfig {
        MemConfig {
            host_access: HostAccess::ReadWrite,
            kernel_access: KernelAccess::ReadWrite,
            mem_allocation: MemAllocation::AllocOnDevice,
        }
    }
}

impl MemConfig {
    pub fn for_data() -> MemConfig {
        MemConfig {
            mem_allocation: MemAllocation::CopyToDevice,
            ..MemConfig::default()
        }
    }

    pub fn for_size() -> MemConfig {
        MemConfig {
            mem_allocation: MemAllocation::AllocOnDevice,
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
    fn mem_allocation_keep_in_place_conversion_into_mem_allocation_mem_flag() {
        let mem_allocation = MemAllocation::KeepInPlace;
        assert_eq!(
            MemAllocationMemFlags::from(mem_allocation),
            MemAllocationMemFlags::KEEP_IN_PLACE
        );
    }

    #[test]
    fn mem_allocation_alloc_on_device_conversion_into_mem_allocation_mem_flag() {
        let mem_allocation = MemAllocation::AllocOnDevice;
        assert_eq!(
            MemAllocationMemFlags::from(mem_allocation),
            MemAllocationMemFlags::ALLOC_ON_DEVICE
        );
    }

    #[test]
    fn mem_allocation_copy_to_device_conversion_into_mem_allocation_mem_flag() {
        let mem_allocation = MemAllocation::CopyToDevice;
        assert_eq!(
            MemAllocationMemFlags::from(mem_allocation),
            MemAllocationMemFlags::COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_allocation_force_copy_to_device_conversion_into_mem_allocation_mem_flag() {
        let mem_allocation = MemAllocation::ForceCopyToDevice;
        assert_eq!(
            MemAllocationMemFlags::from(mem_allocation),
            MemAllocationMemFlags::FORCE_COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_config_conversion_into_cl_mem_flags() {
        let mem_allocation = MemAllocation::AllocOnDevice;
        let host_access = HostAccess::ReadWrite;
        let kernel_access = KernelAccess::ReadWrite;
        let mem_config = MemConfig {
            mem_allocation,
            host_access,
            kernel_access,
        };
        let expected = MemFlags::ALLOC_HOST_PTR.bits()
            | MemFlags::HOST_READ_WRITE.bits()
            | MemFlags::KERNEL_READ_WRITE.bits();

        assert_eq!(cl_mem_flags::from(mem_config), expected);
    }
}
