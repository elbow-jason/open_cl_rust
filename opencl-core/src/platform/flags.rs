
use crate::ffi::cl_platform_info;

 
// All return these flags return chars except perhaps HostTimerResolution.
crate::__codes_enum!(PlatformInfo, cl_platform_info, {
    Profile => 0x0900,
    Version => 0x0901,
    Name => 0x0902,
    Vendor => 0x0903,
    Extensions => 0x0904
    // v2.1
    // HostTimerResolution => 0x0905
});
