use std::fmt;

use crate::ffi::{
    clCreateContext, clGetContextInfo, cl_context, cl_context_info, cl_context_properties,
    cl_device_id,
};
use crate::{
    build_output, ClDeviceID, ClPointer, ContextInfo, ContextProperties, DevicePtr, Output,
    CheckValidClObject, RetainRelease,
};

use crate::cl_helpers::cl_get_info5;

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

pub fn cl_get_context_info<T>(context: cl_context, flag: cl_context_info) -> Output<ClPointer<T>>
where
    T: Copy,
{
    unsafe { cl_get_info5(context, flag as cl_context_info, clGetContextInfo) }
}

pub unsafe trait ContextPtr: Sized {
    unsafe fn context_ptr(&self) -> cl_context;

    unsafe fn info<T: Copy>(&self, flag: ContextInfo) -> Output<ClPointer<T>> {
        cl_get_context_info::<T>(self.context_ptr(), flag.into())
    }

    unsafe fn reference_count(&self) -> Output<u32> {
        self.info(ContextInfo::ReferenceCount)
            .map(|ret| ret.into_one())
    }

    unsafe fn devices(&self) -> Output<Vec<ClDeviceID>> {
        self.info(ContextInfo::Devices).map(|ret| {
            let device_ids: Vec<cl_device_id> = ret.into_vec();
            device_ids
                .into_iter()
                .map(|device_id| ClDeviceID::retain_new(device_id))
                .filter_map(Result::ok)
                .collect()
        })
    }

    unsafe fn properties(&self) -> Output<Vec<ContextProperties>> {
        self.info(ContextInfo::Properties)
            .map(|ret: ClPointer<cl_context_properties>| {
                ret.into_vec()
                    .into_iter()
                    .map(ContextProperties::from)
                    .collect()
            })
    }

    unsafe fn num_devices(&self) -> Output<u32> {
        self.info(ContextInfo::NumDevices).map(|ret| ret.into_one())
    }
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
        object.check_valid_cl_object()?;
        Ok(ClContext::unchecked_new(object))
    }

    pub unsafe fn retain_new(object: cl_context) -> Output<ClContext> {
        object.check_valid_cl_object()?;
        object.retain();
        Ok(ClContext::unchecked_new(object))
    }

    pub unsafe fn create<D>(devices: &[D]) -> Output<ClContext>
    where
        D: DevicePtr,
    {
        let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
        let object = cl_create_context(&device_ptrs[..])?;
        ClContext::new(object)
    }
}

unsafe impl ContextPtr for ClContext {
    unsafe fn context_ptr(&self) -> cl_context {
        self.object
    }
}

impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe { self.context_ptr().release() }
    }
}

impl Clone for ClContext {
    fn clone(&self) -> ClContext {
        unsafe {
            let context = self.context_ptr();
            context.retain();
            ClContext::unchecked_new(context)
        }
    }
}

impl PartialEq for ClContext {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.object, other.object)
    }
}

impl Eq for ClContext {}

impl fmt::Debug for ClContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClContext{{{:?}}}", self.object)
    }
}

#[cfg(test)]
mod test_context_ptr {
    use crate::*;

    #[test]
    fn reference_count_works() {
        let (ctx, _devices) = ll_testing::get_context();
        let ref_count = unsafe { ctx.reference_count() }.unwrap();
        // this is the only place this context should be.
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn devices_works() {
        let (ctx, _devices) = ll_testing::get_context();
        let devices = unsafe { ctx.devices() }.unwrap();
        assert!(devices.len() > 0);
    }

    #[test]
    fn properties_works() {
        let (ctx, _devices) = ll_testing::get_context();
        let _props = unsafe { ctx.properties() }.unwrap();
    }

    #[test]
    fn num_devices_works() {
        let (ctx, _devices) = ll_testing::get_context();
        let n_devices = unsafe { ctx.num_devices() }.unwrap();
        assert!(n_devices > 0);
    }

    #[test]
    fn devices_len_matches_num_devices() {
        let (ctx, _devices) = ll_testing::get_context();
        let num_devices = unsafe { ctx.num_devices() }.unwrap();
        let devices = unsafe { ctx.devices() }.unwrap();
        assert_eq!(num_devices as usize, devices.len());
    }
}
