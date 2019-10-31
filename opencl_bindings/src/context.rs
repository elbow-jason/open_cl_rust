use std::fmt::Debug;

use crate::ffi::{
    cl_context,
    cl_context_info,
    cl_context_properties,
};
use crate::open_cl::{
    cl_release_context,
    cl_create_context,
    ClObject,
};

use crate::device::Device;

use crate::Output;

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    inner: cl_context,
    _unconstructable: (),
}

impl Context {

    pub fn create(device: &Device) -> Output<Context> {
        cl_create_context(device)
    }
}

impl ClObject<cl_context> for Context {
    unsafe fn raw_cl_object(&self) -> cl_context {
        self.inner
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            cl_release_context(&self.raw_cl_object());
        }
    }
}

impl Context {
    pub unsafe fn new(inner: cl_context) -> Context {
        Context {
            inner,
            _unconstructable: (),
        }
    }
}

crate::__codes_enum!(ContextInfo, cl_context_info, {
    ReferenceCount => 0x1080,
    Devices => 0x1081,
    Properties => 0x1082,
    NumDevices => 0x1083
});

crate::__codes_enum!(ContextProperties, cl_context_properties, {
    Platform => 0x1084,
    InteropUserSync => 0x1085
});
