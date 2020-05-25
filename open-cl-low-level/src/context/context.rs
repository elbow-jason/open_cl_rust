use super::functions;
use crate::cl::{cl_context, cl_device_id, ClObject, ObjectWrapper};
use crate::cl::{cl_context_properties, ContextInfo, ContextProperties};
use crate::{Device, DevicePtr, Output};

pub unsafe trait ContextPtr: Sized {
    unsafe fn context_ptr(&self) -> cl_context;

    unsafe fn reference_count(&self) -> Output<u32> {
        functions::get_context_info_u32(self.context_ptr(), ContextInfo::ReferenceCount.into())
    }

    unsafe fn devices(&self) -> Output<Vec<Device>> {
        let devices = functions::get_context_info_devices(self.context_ptr())?
            .into_iter()
            .map(|device_id: cl_device_id| match device_id.check() {
                Ok(()) => Ok(Device::retain_new(device_id)),
                Err(e) => Err(e),
            })
            .filter_map(Result::ok)
            .collect();
        Ok(devices)
    }
    unsafe fn properties(&self) -> Output<Vec<ContextProperties>> {
        let ctx_props = functions::get_context_info_vec_u64(
            self.context_ptr(),
            ContextInfo::Properties.into(),
        )?
        .into_iter()
        .map(|p: u64| ContextProperties::from(p as cl_context_properties))
        .collect();
        Ok(ctx_props)
    }

    unsafe fn num_devices(&self) -> Output<u32> {
        functions::get_context_info_u32(self.context_ptr(), ContextInfo::NumDevices.into())
    }
}

pub type Context = ObjectWrapper<cl_context>;

impl Context {
    pub unsafe fn create<D>(devices: &[D]) -> Output<Context>
    where
        D: DevicePtr,
    {
        let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
        let obj = functions::create_context(&device_ptrs[..])?;
        Ok(Context::new(obj))
    }
}

unsafe impl ContextPtr for Context {
    unsafe fn context_ptr(&self) -> cl_context {
        self.cl_object()
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

    // #[test]
    // fn devices_len_matches_num_devices() {
    //     let (ctx, _devices) = ll_testing::get_context();
    //     let num_devices = unsafe { ctx.num_devices() }.unwrap();
    //     let devices = unsafe { ctx.devices() }.unwrap();
    //     assert_eq!(num_devices as usize, devices.len());
    // }
}
