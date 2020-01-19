// use crate::ffi::{cl_command_type, cl_event_info, cl_int};

// /* command execution status */
// crate::__codes_enum!(CommandExecutionStatus, cl_int, {
//     Complete => 0x0,
//     Running => 0x1,
//     Submitted => 0x2,
//     Queued => 0x3
// });

// // The cl_command_type is the return type of clGetEventInfo.
// crate::__codes_enum!(CommandType, cl_command_type, {
//     NdrangeKernel => 0x11F0,
//     Task => 0x11F1,
//     NativeKernel => 0x11F2,
//     ReadBuffer => 0x11F3,
//     WriteBuffer => 0x11F4,
//     CopyBuffer => 0x11F5,
//     ReadImage => 0x11F6,
//     WriteImage => 0x11F7,
//     CopyImage => 0x11F8,
//     CopyImageToBuffer => 0x11F9,
//     CopyBufferToImage => 0x11FA,
//     MapBuffer => 0x11FB,
//     MapImage => 0x11FC,
//     UnmapMemObject => 0x11FD,
//     Marker => 0x11FE,
//     AcquireGlObjects => 0x11FF,
//     ReleaseGlObjects => 0x1200,
//     ReadBufferRect => 0x1201,
//     WriteBufferRect => 0x1202,
//     CopyBufferRect => 0x1203,
//     User => 0x1204,
//     Barrier => 0x1205,
//     MigrateMemObjects => 0x1206,
//     FillBuffer => 0x1207,
//     FillImage => 0x1208,
//     SvmFree => 0x1209,
//     SvmMemcpy => 0x120A,
//     SvmMemfill => 0x120B,
//     SvmMap => 0x120C,
//     SvmUnmap => 0x120D
// });

// /* cl_event_info */
// crate::__codes_enum!(EventInfo, cl_event_info, {
//     CommandQueue => 0x11D0,
//     CommandType => 0x11D1,
//     ReferenceCount => 0x11D2,
//     CommandExecutionStatus => 0x11D3,
//     Context => 0x11D4
// });
