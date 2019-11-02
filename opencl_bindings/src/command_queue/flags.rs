#![allow(non_upper_case_globals)]

use crate::ffi::{
    cl_command_queue_info,
    cl_command_queue_properties,
};

bitflags! {
    pub struct CommandQueueProperties: cl_command_queue_properties {
        const OutOfOrderExecModeEnable = 1 << 0;
        const ProfilingEnable = 1 << 1;
        const OnDevice = 1 << 2;
        const OnDeviceDefault = 1 << 3;
    }
}

impl Default for CommandQueueProperties {
    fn default() -> CommandQueueProperties {
        CommandQueueProperties::ProfilingEnable
    }
}

crate::__codes_enum!(CommandQueueInfo, cl_command_queue_info, {
    Context => 0x1090,
    Device => 0x1091,
    ReferenceCount => 0x1092,
    Properties => 0x1093,
    Size => 0x1094,
    DeviceDefault => 0x1095
});
