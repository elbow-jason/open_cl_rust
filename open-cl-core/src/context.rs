use std::fmt;
use std::iter::Iterator;
use std::mem::ManuallyDrop;

use crate::ffi::cl_device_id;
use crate::ll::{ClContext, ClDeviceID, ContextProperties, ContextPtr, DevicePtr, VecOrSlice};

use crate::{Device, Output};

pub struct Context {
    inner: ManuallyDrop<ClContext>,
    _devices: ManuallyDrop<Vec<ClDeviceID>>,
    _unconstructable: (),
}

impl Context {
    // Context::build is safe because all objects should be reference counted
    // and their wrapping structs should be droppable. If there is a memory
    // error from opencl it will not be caused by Context::build.
    pub fn build<'a, D>(obj: ClContext, devices: D) -> Context
    where
        D: Into<VecOrSlice<'a, Device>>,
    {
        let devices = devices.into();
        let ll_devices = devices
            .as_slice()
            .iter()
            .map(|d| d.low_level_device().clone())
            .collect();
        Context {
            inner: ManuallyDrop::new(obj),
            _devices: ManuallyDrop::new(ll_devices),
            _unconstructable: (),
        }
    }

    pub fn low_level_context(&self) -> &ClContext {
        &*self.inner
    }

    pub fn from_low_level_context(ll_context: &ClContext) -> Output<Context> {
        let ll_devices = unsafe { ll_context.devices() }?;
        let devices: Vec<Device> = ll_devices.into_iter().map(|d| Device::new(d)).collect();
        Ok(Context::build(ll_context.clone(), devices))
    }

    pub fn create<'a, D: Into<VecOrSlice<'a, Device>>>(devices: D) -> Output<Context> {
        let devices = devices.into();
        let device_ptrs: Vec<cl_device_id> =
            devices.iter().map(|d| unsafe { d.device_ptr() }).collect();

        let ll_context: ClContext = unsafe { ClContext::create(&device_ptrs[..]) }?;
        Ok(Context::build(ll_context, devices))
    }

    pub fn devices(&self) -> &[ClDeviceID] {
        &self._devices[..]
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.inner.reference_count() }
    }

    pub fn properties(&self) -> Output<Vec<ContextProperties>> {
        unsafe { self.inner.properties() }
    }

    pub fn num_devices(&self) -> usize {
        self._devices.len()
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        let cloned_devices = self._devices.iter().map(Clone::clone).collect();
        Context {
            inner: ManuallyDrop::new((*self.inner).clone()),
            _devices: ManuallyDrop::new(cloned_devices),
            _unconstructable: (),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.inner) }
    }
}

/// Context is thread-safe. The only mutable actions for a cl_context are its
/// retain and release functions which are (according to OpenCL documentation),
/// thread-safe, atomic reference counting operations. Therefore, Context
/// is safe for Sync + Send.
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl Eq for Context {}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context{{{:?}}}", unsafe { self.inner.context_ptr() })
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

    #[test]
    fn context_ptr_is_implemented() {
        let ctx = testing::get_context();
        ctx.reference_count().unwrap();
        ctx.num_devices();
        ctx.properties().unwrap();
    }
}
