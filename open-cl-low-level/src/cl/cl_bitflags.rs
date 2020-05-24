use crate::cl::{cl_command_queue_properties, cl_mem_flags};

pub use ocl_core::{DeviceAffinityDomain, DeviceExecCapabilities, DeviceFpConfig, DeviceType};

bitflags! {
    pub struct CommandQueueProperties: cl_command_queue_properties {
        const OUT_OF_ORDER_EXEC_MODE_ENABLE = 1;
        const PROFILING_ENABLE = 1 << 1;
        const ON_DEVICE = 1 << 2;
        const ON_DEVICE_DEFAULT = 1 << 3;
    }
}

impl Default for CommandQueueProperties {
    fn default() -> CommandQueueProperties {
        CommandQueueProperties::PROFILING_ENABLE
    }
}

bitflags! {
    pub struct HostAccessMemFlags: cl_mem_flags {
        const READ_WRITE = 0;
        const WRITE_ONLY = 1 << 7;
        const READ_ONLY = 1 << 8;
        const NO_ACCESS = 1 << 9;
    }
}

impl From<HostAccessMemFlags> for MemFlags {
    fn from(access: HostAccessMemFlags) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(access.bits()) }
    }
}

impl From<MemFlags> for Option<HostAccessMemFlags> {
    fn from(mem_flags: MemFlags) -> Option<HostAccessMemFlags> {
        HostAccessMemFlags::from_bits(mem_flags.bits())
    }
}

bitflags! {
    pub struct KernelAccessMemFlags: cl_mem_flags {
        const READ_WRITE = 1;
        const WRITE_ONLY = 1 << 1;
        const READ_ONLY = 1 << 2;
    }
}

impl From<KernelAccessMemFlags> for MemFlags {
    fn from(access: KernelAccessMemFlags) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(access.bits()) }
    }
}

impl From<MemFlags> for Option<KernelAccessMemFlags> {
    fn from(mem_flags: MemFlags) -> Option<KernelAccessMemFlags> {
        KernelAccessMemFlags::from_bits(mem_flags.bits())
    }
}

bitflags! {
    pub struct MemAllocationMemFlags: cl_mem_flags {
        // CL_MEM_USE_HOST_PTR
        const KEEP_IN_PLACE = 1 << 3;

        // CL_MEM_ALLOC_HOST_PTR
        const ALLOC_ON_DEVICE = 1 << 4;

        // CL_MEM_COPY_HOST_PTR
        const COPY_TO_DEVICE = 1 << 5;

        // CL_MEM_ALLOC_HOST_PTR used with CL_MEM_COPY_HOST_PTR
        const FORCE_COPY_TO_DEVICE = Self::ALLOC_ON_DEVICE.bits() | Self::COPY_TO_DEVICE.bits();
    }
}

impl From<MemAllocationMemFlags> for MemFlags {
    fn from(loc: MemAllocationMemFlags) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(loc.bits()) }
    }
}

impl From<MemFlags> for Option<MemAllocationMemFlags> {
    fn from(mem_flags: MemFlags) -> Option<MemAllocationMemFlags> {
        MemAllocationMemFlags::from_bits(mem_flags.bits())
    }
}

bitflags! {
    pub struct MemFlags: cl_mem_flags {
        const EMPTY = 0;
        const KERNEL_READ_WRITE = 1;
        const KERNEL_WRITE_ONLY = 1 << 1;
        const KERNEL_READ_ONLY = 1 << 2;

        const ALLOC_HOST_PTR = 1 << 4;
        const USE_HOST_PTR = 1 << 3;
        const COPY_HOST_PTR = 1 << 5;

        const HOST_WRITE_ONLY = 1 << 7;
        const HOST_READ_ONLY = 1 << 8;
        const HOST_NO_ACCESS = 1 << 9;
        const HOST_READ_WRITE = 0;

        // OpenCL v2.0 ?
        // const SVM_FINE_GRAIN_BUFFER = 1 << 10;
        // const SVM_ATOMICS = 1 << 11;
        // const KERNEL_READ_AND_WRITE = 1 << 12;
        // a few useful custom MemFlags that are also examples.
        const READ_WRITE_ALLOC_HOST_PTR = Self::KERNEL_READ_WRITE.bits() | Self::ALLOC_HOST_PTR.bits();
        const READ_ONLY_ALLOC_HOST_PTR = Self::KERNEL_READ_ONLY.bits() | Self::ALLOC_HOST_PTR.bits();
        const WRITE_ONLY_ALLOC_HOST_PTR = Self::KERNEL_WRITE_ONLY.bits() | Self::ALLOC_HOST_PTR.bits();
    }
}

impl From<MemFlags> for cl_mem_flags {
    fn from(d: MemFlags) -> cl_mem_flags {
        d.bits()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::ffi::*;
    #[test]
    fn test_command_queue_properties_implements_default() {
        let q: CommandQueueProperties = Default::default();
        assert_eq!(q, CommandQueueProperties::PROFILING_ENABLE);
    }

    #[test]
    fn test_command_queue_properties_can_be_assembled_from_bits() {
        let p: cl_command_queue_properties = CommandQueueProperties::PROFILING_ENABLE.bits();
        let q: CommandQueueProperties = CommandQueueProperties::from_bits(p).unwrap();
        assert_eq!(q, CommandQueueProperties::PROFILING_ENABLE);
    }
}

// impl From<CommandQueueProperties> for cl_command_queue_properties {
//     fn from(d: CommandQueueProperties) -> cl_command_queue_properties {
//         d.bits()
//     }
// }

// impl From<DeviceType> for cl_device_type {
//     fn from(d: DeviceType) -> cl_device_type {
//         d.bits()
//     }
// }

// impl From<DeviceFpConfig> for cl_device_fp_config {
//     fn from(d: DeviceFpConfig) -> cl_device_fp_config {
//         d.bits()
//     }
// }

// impl From<DeviceExecCapabilities> for cl_device_exec_capabilities {
//     fn from(d: DeviceExecCapabilities) -> cl_device_exec_capabilities {
//         d.bits()
//     }
// }

// impl From<DeviceAffinityDomain> for cl_device_affinity_domain {
//     fn from(d: DeviceAffinityDomain) -> cl_device_affinity_domain {
//         d.bits()
//     }
// }
