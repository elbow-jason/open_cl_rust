/* cl_build_status */
use crate::open_cl::ffi::cl_build_status;

crate::__codes_enum!(BuildStatus, cl_build_status, {
    Success => 0,
    NoneType => -1,
    Error => -2,
    InProgress => -3
});
