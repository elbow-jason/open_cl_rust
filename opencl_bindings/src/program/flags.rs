use crate::ffi::{
    cl_program_build_info,
    cl_program_info,
    // v2.0+
    // cl_program_binary_type,
};

crate::__codes_enum!(ProgramBuildInfo, cl_program_build_info, {
    Status => 0x1181,
    Options => 0x1182,
    Log => 0x1183,
    // NOTE: Version for BinaryType?
    // BinaryType => 0x1184,
    GlobalVariableTotalSize => 0x1185
});

crate::__codes_enum!(ProgramInfo, cl_program_info, {
    ReferenceCount => 0x1160,
    Context => 0x1161,
    NumDevices => 0x1162,
    Devices => 0x1163,
    Source => 0x1164,
    BinarySizes => 0x1165,
    Binaries => 0x1166,
    NumKernels => 0x1167,
    KernelNames => 0x1168,
    Il => 0x1169,
    ScopeGlobalCtorsPresent => 0x116A,
    ScopeGlobalDtorsPresent => 0x116B

});

// NOTE: Version for cl_program_binary_type?
// crate::__codes_enum!(ProgramBinaryType, cl_program_binary_type, {
//     NONE => 0x0,
//     COMPILED_OBJECT => 0x1,
//     LIBRARY => 0x2,
//     EXECUTABLE => 0x4
// });
