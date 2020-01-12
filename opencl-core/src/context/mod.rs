use std::mem::ManuallyDrop;
use std::fmt;

pub mod flags;
pub mod low_level;
use crate::device::DevicePtr;
use crate::error::Output;
use crate::cl::ClObjectError;
use crate::ffi::{cl_context, cl_device_id};

use low_level::{cl_create_context, cl_release_context, cl_retain_context};

pub trait ContextPtr {
    unsafe fn context_ptr(&self) -> cl_context;

    unsafe fn release_context(&mut self) {
        cl_release_context(self.context_ptr()).unwrap_or_else(|e| {
            panic!("Failed to release cl_context {:?}", e);
        });
    }

    unsafe fn retain_context(&self) {
        cl_retain_context(self.context_ptr()).unwrap_or_else(|e| {
            panic!("Failed to retain cl_context {:?}", e);
        });
    }
}

struct ContextObject {
    object: cl_context,
}

impl ContextPtr for ContextObject {
    unsafe fn context_ptr(&self) -> cl_context {
        self.object
    }
}

impl Drop for ContextObject {
    fn drop(&mut self) {
        unsafe{
            self.release_context()
        }
    }
}

impl Clone for ContextObject {
    fn clone(&self) -> ContextObject {
        unsafe { self.retain_context() };
        ContextObject{
            object: self.object
        }
    }
}


pub struct Context {
    inner: ManuallyDrop<ContextObject>,
    // devices: Vec<Device>,
    _unconstructable: ()
}

impl Clone for Context {
    fn clone(&self) -> Context {
        Context {
            inner: ManuallyDrop::new((*self.inner).clone()),
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

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub unsafe fn new(ctx: cl_context) -> Context {//, devices: Vec<Device>) -> Output<Context> {
        Context {
            inner: ManuallyDrop::new(ContextObject{object: ctx}),
            // devices,
            _unconstructable: (),
        }
    }

    pub unsafe fn from_unretained_object(obj: cl_context) -> Output<Context> {
         if obj.is_null() {
            let error = ClObjectError::ClObjectCannotBeNull("DeviceMem<T>".to_string());
            return Err(error.into());
        }

        cl_retain_context(obj)?;
        Ok(Context::new(obj))
    }

    pub unsafe fn context_ptr(&self) -> cl_context {
        (*self.inner).object
    }

    pub fn create<D: DevicePtr>(
        devices: &[D],
    ) -> Output<Context> {
        let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| unsafe { (*d).device_ptr() }).collect();
        cl_create_context(&device_ptrs[..])
    }
}
    // pub fn device(&self) -> &[Device] {
    //     &self.devices[..]
    // }

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context{{{:?}}}", unsafe { self.context_ptr() })
    }
}




#[cfg(test)]
mod tests {
    use super::Context;
    use crate::device::Device;

    #[test]
    fn context_can_be_created_via_a_device() {
        let device: Device = Device::default();
        let devices = vec![device];
        let _context: Context =
            Context::create(&devices[..]).expect("Failed to create context from a device");
    }
}
