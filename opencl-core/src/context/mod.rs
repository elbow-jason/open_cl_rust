use std::mem::ManuallyDrop;
use std::fmt;

pub mod flags;
pub mod low_level;
pub use flags::ContextInfo;

use crate::device::{DevicePtr, Device};
use crate::error::Output;
// use crate::cl::ClObjectError;
use crate::ffi::{cl_context, cl_device_id};
use crate::cl::ClPointer;


fn get_info<T: Copy, C: ContextPtr>(context: &C, flag: ContextInfo) -> Output<ClPointer<T>> {
    low_level::cl_get_context_info(unsafe { context.context_ptr() }, flag)
}


pub trait ContextPtr: Sized {
    unsafe fn context_ptr(&self) -> cl_context;

    fn load_devices(&self) -> Output<Vec<Device>> {
        get_info::<cl_device_id, Self>(self, ContextInfo::Devices).map(|cl_ptr| {
            unsafe {
                cl_ptr
                    .into_vec()
                    .into_iter()
                    .map(|device| Device::new(device).unwrap().clone())
                    .collect()
            }
        })
    }
}

pub trait ContextRefCount: ContextPtr {
    unsafe fn from_retained(ctx: cl_context) -> Output<Self>;
    unsafe fn from_unretained(ctx: cl_context) -> Output<Self>;

    unsafe fn release_context(&mut self) {
        low_level::cl_release_context(self.context_ptr()).unwrap_or_else(|e| {
            panic!("Failed to release cl_context {:?}", e);
        });
    }

    unsafe fn retain_context(&self) {
        low_level::cl_retain_context(self.context_ptr()).unwrap_or_else(|e| {
            panic!("Failed to retain cl_context {:?}", e);
        });
    }

}


pub struct ContextObject {
    object: cl_context,
}

impl ContextPtr for ContextObject {
    unsafe fn context_ptr(&self) -> cl_context {
        self.object
    }
}
impl ContextRefCount for ContextObject {
    unsafe fn from_unretained(object: cl_context) -> Output<ContextObject> {
        let new_self: Self = ContextObject { object };
        new_self.retain_context();
        Ok(new_self)
    }

    unsafe fn from_retained(object: cl_context) -> Output<ContextObject> {
        Ok(ContextObject { object })
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

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl ContextPtr for Context {
    unsafe fn context_ptr(&self) -> cl_context {
        self.inner.context_ptr()
    }
}
impl ContextRefCount for Context {
    unsafe fn from_unretained(ctx: cl_context) -> Output<Context> {
        let context_object = ContextObject::from_unretained(ctx)?;
        let devices: Vec<Device> = context_object.load_devices()?;
        Ok(Context::build(context_object, devices))
    }

    unsafe fn from_retained(raw_ctx: cl_context) -> Output<Context> {
        let context_object = ContextObject::from_retained(raw_ctx)?;
        let devices: Vec<Device> = context_object.load_devices()?;
        Ok(Context::build(context_object, devices))
    }
}



// Identify uncounted references.


impl Context {

    // pub unsafe fn new(ctx: cl_context, devices: Vec<Device>) -> Context {//, devices: Vec<Device>) -> Output<Context> {
    //     let context_object = ContextObject::from_retained(ctx);
    //     Context::build(context_object, devices)
    // }

    // pub unsafe fn from_cl_pointer(cl_ptr: ClPointer<cl_context>) -> Output<Context> {
    //     let ctx_obj = ContextObject::from_unretained(cl_ptr.into_one());
    //     Context::from_context_object(ctx_obj)
    // }

    // unsafe fn from_context_object(obj: ContextObject) -> Output<Context> {//, devices: Vec<Device>) -> Output<Context> {
    //     let devices: Vec<Device> = obj.load_devices()?;
    //     Ok(Context::build(obj, devices))
    // }

    /// This call is safe because all structures should be reference counted
    /// and droppable. If there is a memory error from opencl it will not be in
    /// this function.
    fn build(obj: ContextObject, devices: Vec<Device>) -> Context {
        Context {
            inner: ManuallyDrop::new(obj),
            _devices: ManuallyDrop::new(devices),
            _unconstructable: (),
        }
    }

    pub unsafe fn context_ptr(&self) -> cl_context {
        (*self.inner).object
    }

    pub fn create<D: DevicePtr>(
        device_ptrs: &[D],
    ) -> Output<Context> {
        let obj: ContextObject = unsafe { low_level::cl_create_context(device_ptrs) }?;
        let devices = Device::clone_slice(device_ptrs)?;
        Ok(Context::build(obj, devices))
    }

    pub fn devices(&self) -> &[Device] {
        &self._devices[..]
    }
}
   

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
