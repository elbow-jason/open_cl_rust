use crate::ffi::{cl_context_info, cl_context_properties};

crate::__codes_enum!(ContextInfo, cl_context_info, {
    ReferenceCount => 0x1080,
    Devices => 0x1081,
    Properties => 0x1082,
    NumDevices => 0x1083
});

crate::__codes_enum!(ContextProperties, cl_context_properties, {
    Platform => 0x1084,
    InteropUserSync => 0x1085
});