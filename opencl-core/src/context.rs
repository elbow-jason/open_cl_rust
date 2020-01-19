use std::mem::ManuallyDrop;
use std::fmt;

use crate::ll::{Output, ClContext, ContextPtr};
use crate::ffi::cl_context;

use crate::Device;

pub struct Context {
    inner: ManuallyDrop<ClContext>,
    _devices: ManuallyDrop<Vec<Device>>,
    _unconstructable: ()
}

impl Clone for Context {
    fn clone(&self) -> Context {
        let cloned_devices = self._devices.iter().map(Clone::clone).collect();
        Context {
            inner: ManuallyDrop::new((*self.inner).clone()),
            _devices: ManuallyDrop::new(cloned_devices),
            _unconstructable: ()
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner)
        }
    }
}

/// Context is thread-safe. The only mutable actions for a cl_context are its
/// retain and release functions which are (according to OpenCL documentation),
/// thread-safe, atomic reference counting operations. Therefore, Context
/// is safe for Sync + Send.
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl ContextPtr for Context {
    unsafe fn context_ptr(&self) -> cl_context {
        self.inner.context_ptr()
    }
}

impl Context {
    // Context::build is safe because all objects should be reference counted
    // and their wrapping structs should be droppable. If there is a memory
    // error from opencl it will not be caused by Context::build.
    pub fn build(obj: ClContext, devices: Vec<Device>) -> Context {
        Context {
            inner: ManuallyDrop::new(obj),
            _devices: ManuallyDrop::new(devices),
            _unconstructable: (),
        }
    }

    pub fn low_level_context(&self) -> &ClContext {
        &*self.inner
    }

    pub unsafe fn context_ptr(&self) -> cl_context {
        (*self.inner).context_ptr()
    }

    pub fn create(
        devices: &[Device],
    ) -> Output<Context> {
        let ll_context: ClContext = unsafe { ClContext::create(devices) }?;
        Ok(Context::build(ll_context, devices.to_vec()))
    }

    pub fn devices(&self) -> &[Device] {
        &self._devices[..]
    }
}
impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(self.context_ptr(), other.context_ptr()) }
    }
}

impl Eq for Context {}



impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context{{{:?}}}", unsafe { self.context_ptr() })
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use crate::device::Device;
    use crate::testing;

     #[test]
     fn context_can_be_created_via_a_device() {
         let device: Device = testing::get_device();
         let devices = vec![device];
         let _context: Context =
             Context::create(&devices[..]).expect("Failed to create context from a device");
     }
}
