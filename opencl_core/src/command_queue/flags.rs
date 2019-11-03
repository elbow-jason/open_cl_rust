#![allow(non_upper_case_globals)]

use crate::ffi::{
    cl_command_queue_info,
    cl_command_queue_properties,
    cl_context,
    cl_device_id,
    cl_uint,
};

use crate::context::Context;
use crate::device::Device;

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

crate::__codes_enum!(CommandQueueInfoFlag, cl_command_queue_info, {
    Context => 0x1090,
    Device => 0x1091,
    ReferenceCount => 0x1092,
    Properties => 0x1093

    // v2.0
    // Size => 0x1094,

    // v2.1
    // DeviceDefault => 0x1095
});

impl CommandQueueInfoFlag {
    pub fn size_t(&self) -> usize {
        use CommandQueueInfoFlag as F;
        match self {
            F::Context => size_t!(cl_context),
            F::Device => size_t!(cl_device_id),
            F::ReferenceCount => size_t!(cl_uint),
            F::Properties => size_t!(cl_command_queue_properties),
        }
    }
}

pub enum CommandQueueInfo {
    Context(Context),
    Device(Device),
    ReferenceCount(usize),
    Properties(CommandQueueProperties)
}

impl CommandQueueInfo {
    pub unsafe fn from_raw_parts(
        info_flag: CommandQueueInfoFlag,
        return_value: *mut libc::c_void
    ) -> CommandQueueInfo {
        use CommandQueueInfoFlag as F;

        match info_flag {
            F::Context => {
                let ctx = Context::new(return_value as cl_context);
                CommandQueueInfo::Context(ctx)
            },
            F::Device => {
                let device = Device::new(return_value as cl_device_id);
                CommandQueueInfo::Device(device)
            },
            F::ReferenceCount => CommandQueueInfo::ReferenceCount(return_value as usize),

            F::Properties => {
                let cq_props = CommandQueueProperties::from_bits(return_value as u64)
                    .expect("failed to cast CommandQueueProperties from return_value");
                CommandQueueInfo::Properties(cq_props)
            }
        }
    }
}