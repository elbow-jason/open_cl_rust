use crate::{
    utils, build_output, Output, ClPointer, DevicePtr, ContextInfo,
    ClDeviceID, DeviceRefCount, ContextProperties,
};
use crate::ffi::{
    cl_device_id, clCreateContext, cl_context, clGetContextInfo,
    cl_context_info, cl_context_properties,
};

use crate::cl_helpers::{cl_get_info5};
// use super::{ContextObject, ContextInfo, ContextRefCount};

#[allow(clippy::transmuting_null)]
pub unsafe fn cl_create_context(device_ids: &[cl_device_id]) -> Output<cl_context> {
    let mut err_code = 0;
    let context = clCreateContext(
        std::ptr::null(),
        device_ids.len() as u32,
        device_ids.as_ptr() as *const cl_device_id,
        std::mem::transmute(std::ptr::null::<fn()>()),
        std::ptr::null_mut(),
        &mut err_code,
    );
    build_output(context, err_code)
}

pub fn cl_get_context_info<T>(context: cl_context, flag: cl_context_info) -> Output<ClPointer<T>> where T: Copy {
    utils::null_check(context)?;
    unsafe {
        cl_get_info5(
            context,
            flag as cl_context_info,
            clGetContextInfo,
        )
    }
}

__release_retain!(context, Context);

pub trait ContextPtr: Sized {
    unsafe fn context_ptr(&self) -> cl_context;

    unsafe fn info<T: Copy>(&self, flag: ContextInfo) -> Output<ClPointer<T>> {
        cl_get_context_info::<T>(self.context_ptr(), flag.into())
    }

    fn reference_count(&self) -> Output<u32> {
        unsafe {
            self.info(ContextInfo::ReferenceCount).map(|ret| ret.into_one())
        }
    }

    fn devices(&self) -> Output<Vec<ClDeviceID>> {
        unsafe {
            self.info(ContextInfo::Devices).map(|ret|  {
                let device_ids: Vec<cl_device_id> = ret.into_vec();
                device_ids
                    .into_iter()
                    .map(|device_id| ClDeviceID::from_unretained(device_id))
                    .filter_map(Result::ok)
                    .collect()
            })
        }
    }
    fn properties(&self) -> Output<Vec<ContextProperties>> {
        unsafe {
            self.info(ContextInfo::Properties).map(|ret: ClPointer<cl_context_properties>| {
                ret.into_vec()
                    .into_iter()
                    .map(ContextProperties::from)
                    .collect()
            })
        }
    }

    fn num_devices(&self) -> Output<u32> {
        unsafe {
            self.info(ContextInfo::NumDevices).map(|ret| ret.into_one())
        }
    }

}

unsafe fn release_context(context: cl_context) {
    cl_release_context(context).unwrap_or_else(|e| {
        panic!("Failed to release cl_context {:?}", e);
    });
}

unsafe fn retain_context(context: cl_context) {
    cl_retain_context(context).unwrap_or_else(|e| {
        panic!("Failed to retain cl_context {:?}", e);
    });
}

pub struct ClContext {
    object: cl_context,
    _unconstructable: (),
}

impl ClContext {
    pub unsafe fn unchecked_new(object: cl_context) -> ClContext {
        ClContext {
            object,
            _unconstructable: (),
        }
    }
    pub unsafe fn new(object: cl_context) -> Output<ClContext> {
        utils::null_check(object)?;
        Ok(ClContext::unchecked_new(object))
    }

    pub unsafe fn create<D>(devices: &[D]) -> Output<ClContext> where D: DevicePtr {
        let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
        let object = cl_create_context(&device_ptrs[..])?;
        ClContext::new(object)
    }
}

impl ContextPtr for ClContext {
    unsafe fn context_ptr(&self) -> cl_context {
        self.object
    }
}

impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe{
            release_context(self.context_ptr())
        }
    }
}

impl Clone for ClContext {
    fn clone(&self) -> ClContext {
        unsafe { 
            let context = self.context_ptr();
            retain_context(context);
            ClContext::unchecked_new(context)
        }
    }
}

#[cfg(test)]
mod test_context_ptr {
    use crate::*;

    #[test]
    fn reference_count_works() {
        let ctx = ll_testing::get_context();
        let ref_count = ctx.reference_count().unwrap();
        // this is the only place this context should be.
        assert_eq!(ref_count, 1);
    }
    
    #[test]
    fn devices_works() {
         let ctx = ll_testing::get_context();
         let devices = ctx.devices().unwrap();
         assert!(devices.len() > 0);
    }
    
    #[test]
    fn properties_works() {
         let ctx = ll_testing::get_context();
         let _props = ctx.properties().unwrap();
    }
    
    #[test]
    fn num_devices_works() {
        let ctx = ll_testing::get_context();
         let n_devices = ctx.num_devices().unwrap();
         assert!(n_devices > 0);
    }

        
    #[test]
    fn devices_len_matches_num_devices() {
        let ctx = ll_testing::get_context();
        let num_devices = ctx.num_devices().unwrap();
        let devices = ctx.devices().unwrap();
         assert_eq!(num_devices as usize, devices.len());
    }
}
