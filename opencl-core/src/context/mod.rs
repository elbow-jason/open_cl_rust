pub mod flags;
pub mod low_level;

use crate::device::Device;
use crate::error::Output;
use crate::ffi::cl_context;

use low_level::{cl_create_context, cl_release_context, cl_retain_context};

__impl_unconstructable_cl_wrapper!(Context, cl_context);
__impl_default_debug_for!(Context);
__impl_cl_object_for_wrapper!(Context, cl_context, cl_retain_context, cl_release_context);
__impl_clone_for_cl_object_wrapper!(Context, cl_retain_context);
__impl_drop_for_cl_object_wrapper!(Context, cl_release_context);

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn create(device: &Device) -> Output<Context> {
        cl_create_context(device)
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use crate::device::Device;

    #[test]
    fn context_can_be_created_via_a_device() {
        let device: Device = Device::default();
        let _context: Context =
            Context::create(&device).expect("Failed to create context from a device");
    }
}
