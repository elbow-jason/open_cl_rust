use std::mem::ManuallyDrop;
use std::fmt;
use std::iter::Iterator;

use crate::ll::{Output, ClContext, ContextPtr, DevicePtr, ContextProperties};
use crate::ffi::{cl_context, cl_device_id};

use crate::Device;

pub struct Context {
    inner: ManuallyDrop<ClContext>,
    _devices: ManuallyDrop<Vec<Device>>,
    _unconstructable: ()
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

    pub fn create<'a, D: Into<Devices<'a>>>(
        devices: D,
    ) -> Output<Context> {
        let devices = devices.into();
        let device_ptrs = devices.device_ptrs();
        let ll_context: ClContext = unsafe { ClContext::create(&device_ptrs[..]) }?;
        Ok(Context::build(ll_context, devices.to_vec()))
    }

    pub fn devices(&self) -> &[Device] {
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

pub enum Devices<'a> {
    V(Vec<Device>),
    S(&'a [Device]),
}

impl<'a> Devices<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Device>  {
        match self {
            Devices::V(devices) => devices.iter(),
            Devices::S(devices) => devices.iter(),
        }
    }

    pub fn device_ptrs(&self) -> Vec<cl_device_id> {
        self.iter()
            .map(|d| unsafe { d.device_ptr() })
            .collect()
    }

    pub fn to_vec(self) -> Vec<Device> {
        match self {
            Devices::V(devices) => devices,
            Devices::S(devices) => devices.to_vec(),
        }
    }
}

impl<'a> From<Vec<Device>> for Devices<'a> {
    fn from(d: Vec<Device>) -> Devices<'a> {
        Devices::V(d)
    }
}

impl<'a> From<&'a [Device]> for Devices<'a> {
    fn from(d: &'a [Device]) -> Devices<'a> {
        Devices::S(d)
    }
}




impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
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

     #[test]
     fn context_ptr_is_implemented() {
        let ctx = testing::get_context();
        ctx.reference_count().unwrap();
        ctx.num_devices();
        ctx.properties().unwrap();
     }

}
