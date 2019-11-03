use crate::ffi::{
    cl_kernel_info,
    cl_kernel_work_group_info,
    cl_kernel_arg_access_qualifier,
    cl_kernel_arg_type_qualifier,
    cl_kernel_arg_address_qualifier,
    cl_kernel_arg_info,
    // v2.0+
    // cl_kernel_sub_group_info,
    // cl_kernel_exec_info,
};


crate::__codes_enum!(KernelInfo, cl_kernel_info, {
    FunctionName => 0x1190,
    NumArgs => 0x1191,
    ReferenceCount => 0x1192,
    Context => 0x1193,
    Program => 0x1194,
    Attributes => 0x1195,
    MaxNumSubGroups => 0x11B9,
    CompileNumSubGroups => 0x11BA
});

crate::__codes_enum!(KernelWorkGroupInfo, cl_kernel_work_group_info, {
    WorkGroupSize => 0x11B0,
    CompileWorkGroupSize => 0x11B1,
    LocalMemSize => 0x11B2,
    PreferredWorkGroupSizeMultiple => 0x11B3,
    PrivateMemSize => 0x11B4,
    GlobalWorkSize => 0x11B5
});

crate::__codes_enum!(KernelArgAccessQualifier, cl_kernel_arg_access_qualifier, {
    ReadOnly => 0x11A0,
    WriteOnly => 0x11A1,
    ReadWrite => 0x11A2,
    NoneType => 0x11A3
});

crate::__codes_enum!(KernelArgTypeQualifier, cl_kernel_arg_type_qualifier, {
    NoneType => 0,
    Const => 1,
    Restrict => 2,
    Volatile => 4,
    Pipe => 8
});


crate::__codes_enum!(KernelArgAddressQualifier, cl_kernel_arg_address_qualifier, {
    Global => 0x119B,
    Local => 0x119C,
    Constant => 0x119D,
    Private => 0x119E
});

crate::__codes_enum!(KernelArgInfo, cl_kernel_arg_info, {
    AddressQualifier => 0x1196,
    AccessQualifier => 0x1197,
    TypeName => 0x1198,
    TypeQualifier => 0x1199,
    Name => 0x119A
});

// NOTE: v2.0+
// crate::__codes_enum!(KernelExecInfo, cl_kernel_exec_info, {
//     SvmPtrs => 0x11B6,
//     SvmFineGrainSystem => 0x11B7
// });

// NOTE: v2.0+
// crate::__codes_enum!(KernelSubGroupInfo, cl_kernel_sub_group_info, {
//     MaxSubGroupSizeForNdrange => 0x2033,
//     SubGroupCountForNdrange => 0x2034,
//     LocalSizeForSubGroupCount => 0x11B8
// });
