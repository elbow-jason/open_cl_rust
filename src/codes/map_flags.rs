use crate::open_cl::ffi::cl_map_flags;

crate::__codes_enum!(MapFlags, cl_map_flags, {
    Read => 1,
    Write => 2,
    WriteInvalidateRegion => 4
});
