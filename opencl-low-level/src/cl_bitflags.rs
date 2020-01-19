use ffi::*;

bitflags! {
    pub struct DeviceType: cl_device_type {
        const DEFAULT = 1;
        const CPU = 2;
        const GPU = 4;
        const ACCELERATOR = 8;
        const CUSTOM = 16;
        const ALL = 0xFFFF_FFFF;
    }
}

impl From<DeviceType> for cl_device_type {
    fn from(d: DeviceType) -> cl_device_type {
        d.bits()
    }
}

// https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h#L389-L401
bitflags! {
    pub struct DeviceFpConfig: cl_device_fp_config {
        const DENORM = 1;
        const INF_NAN = 2;
        const ROUND_TO_NEAREST = 4;
        const ROUND_TO_ZERO = 8;
        const ROUND_TO_INF = 16;
        const FMA = 32;
        const SOFT_FLOAT = 64;
        const CORRECTLY_ROUNDED_DIVIDE_SQRT = 128;
    }
}

impl From<DeviceFpConfig> for cl_device_fp_config {
    fn from(d: DeviceFpConfig) -> cl_device_fp_config {
        d.bits()
    }
}

bitflags! {
    pub struct DeviceExecCapabilities: cl_device_exec_capabilities {
        const KERNEL = 1;
        const NATIVE_KERNEL = 2;
    }
}

impl From<DeviceExecCapabilities> for cl_device_exec_capabilities {
    fn from(d: DeviceExecCapabilities) -> cl_device_exec_capabilities {
        d.bits()
    }
}


bitflags! {
    pub struct DeviceAffinityDomain: cl_device_affinity_domain {
        const NONE_SUPPORTED = 0;
        const NUMA = 1;
        const L4_CACHE = 2;
        const L3_CACHE = 4;
        const L2_CACHE = 8;
        const L1_CACHE = 16;
        const NEXT_PARTITIONABLE = 32;
    }
}

impl From<DeviceAffinityDomain> for cl_device_affinity_domain {
    fn from(d: DeviceAffinityDomain) -> cl_device_affinity_domain {
        d.bits()
    }
}


bitflags! {
    pub struct MemFlags: cl_mem_flags {
        const READ_WRITE = 1;
        const WRITE_ONLY = 1 << 1;
        const READ_ONLY = 1 << 2;
        const ALLOC_HOST_PTR = 1 << 4;
        const USE_HOST_PTR = 1 << 3;
        const COPY_HOST_PTR = 1 << 5;
        const HOST_WRITE_ONLY = 1 << 7;
        const HOST_READ_ONLY = 1 << 8;
        const HOST_NO_ACCESS = 1 << 9;
        const SVM_FINE_GRAIN_BUFFER = 1 << 10;
        const SVM_ATOMICS = 1 << 11;
        const KERNEL_READ_AND_WRITE = 1 << 12;
        // a few useful custom MemFlags that are also examples.
        const READ_WRITE_ALLOC_HOST_PTR = Self::READ_WRITE.bits | Self::ALLOC_HOST_PTR.bits;
        const READ_ONLY_ALLOC_HOST_PTR = Self::READ_ONLY.bits | Self::ALLOC_HOST_PTR.bits;
        const WRITE_ONLY_ALLOC_HOST_PTR = Self::WRITE_ONLY.bits | Self::ALLOC_HOST_PTR.bits;
    }
}

impl From<MemFlags> for cl_mem_flags {
    fn from(d: MemFlags) -> cl_mem_flags {
        d.bits()
    }
}

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

impl From<CommandQueueProperties> for cl_command_queue_properties {
    fn from(d: CommandQueueProperties) -> cl_command_queue_properties {
        d.bits()
    }
}

