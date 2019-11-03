use crate::ffi::{
    cl_profiling_info,
};

crate::__codes_enum!(ProfilingInfo, cl_profiling_info, {
    Queued => 0x1280,
    Submit => 0x1281,
    Start => 0x1282,
    End => 0x1283,
    Complete => 0x1284
});
