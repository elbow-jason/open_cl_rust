pub mod low_level;
pub mod flags;

use crate::ffi::cl_context;
use crate::device::Device;
use crate::error::Output;

use low_level::{cl_create_context, cl_retain_context, cl_release_context};

__impl_unconstructable_cl_wrapper!(Context, cl_context);
__impl_cl_object_for_wrapper!(Context, cl_context);
__impl_clone_for_cl_object_wrapper!(Context, cl_retain_context);
__impl_drop_for_cl_object_wrapper!(Context, cl_release_context);

impl Context {
    pub fn create(device: &Device) -> Output<Context> {
        cl_create_context(device)
    }
}