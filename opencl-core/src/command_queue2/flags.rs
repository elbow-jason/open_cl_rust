// #![allow(non_upper_case_globals)]

// use crate::ffi::{
//     cl_command_queue_info, cl_command_queue_properties, cl_context, cl_device_id, cl_uint,
// };

// // bitflags! {
//     pub struct CommandQueueProperties: cl_command_queue_properties {
//         const OutOfOrderExecModeEnable = 1;
//         const ProfilingEnable = 1 << 1;
//         const OnDevice = 1 << 2;
//         const OnDeviceDefault = 1 << 3;
//     }
// }

// impl Default for CommandQueueProperties {
//     fn default() -> CommandQueueProperties {
//         CommandQueueProperties::ProfilingEnable
//     }
// }

// crate::__codes_enum!(CommandQueueInfo, cl_command_queue_info, {
//     Context => 0x1090,
//     Device => 0x1091,
//     ReferenceCount => 0x1092,
//     Properties => 0x1093

//     // v2.0
//     // Size => 0x1094,

//     // v2.1
//     // DeviceDefault => 0x1095
// });

// use CommandQueueInfo as F;

// impl CommandQueueInfo {
//     pub fn size_t(self) -> usize {
//         match self {
//             F::Context => size_t!(cl_context),
//             F::Device => size_t!(cl_device_id),
//             F::ReferenceCount => size_t!(cl_uint),
//             F::Properties => size_t!(cl_command_queue_properties),
//         }
//     }
// }
