
use crate::ffi::cl_platform_info;

crate::__codes_enum!(PlatformInfo, cl_platform_info, {
    Profile => 0x0900,
    Version => 0x0901,
    Name => 0x0902,
    Vendor => 0x0903,
    Extensions => 0x0904,
    HostTimerResolution => 0x0905
});
