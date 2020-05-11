use crate::Output;
use libc::c_void;
use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClObjectError {
    #[error("OpenCL returned a {0} as a null pointer")]
    NullPointer(&'static str),

    #[error("OpenCL returned an unusable device address (0xFFFF_FFFF)")]
    UnusableDeviceAddress,
}

macro_rules! defobject {
    ($name:ident) => {
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq)]
        pub struct $name(*mut c_void);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.address())
            }
        }
    };
}

defobject!(cl_platform_id);
defobject!(cl_device_id);
defobject!(cl_context);
defobject!(cl_command_queue);
defobject!(cl_mem);
defobject!(cl_program);
defobject!(cl_kernel);
defobject!(cl_event);
defobject!(cl_sampler);

pub unsafe trait ClObject: Sized + Clone + Copy + fmt::Debug + PartialEq {
    unsafe fn new(val: *mut c_void) -> Output<Self>;
    fn check(&self) -> Output<()>;
    unsafe fn as_mut_ptr(&mut self) -> *mut c_void;
    unsafe fn as_ptr(&self) -> *const c_void;
    unsafe fn null_ptr() -> Self {
        std::mem::zeroed()
    }
    fn type_name() -> &'static str;
    fn address(&self) -> String {
        format!("{:?}", unsafe { self.as_ptr() })
    }
}

macro_rules! impl_cl_object {
    ($name:ident) => {
        unsafe impl ClObject for $name {
            unsafe fn new(val: *mut c_void) -> Output<$name> {
                let new = $name(val);
                new.check()?;
                Ok(new)
            }
            fn type_name() -> &'static str {
                stringify!($name)
            }

            unsafe fn as_ptr(&self) -> *const c_void {
                self.0 as *const c_void
            }

            unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
                self.0
            }

            fn check(&self) -> Output<()> {
                if self.0.is_null() {
                    Err(ClObjectError::NullPointer(stringify!($name)))?
                } else {
                    Ok(())
                }
            }
        }
    };
}

impl_cl_object!(cl_command_queue);
impl_cl_object!(cl_context);
impl_cl_object!(cl_event);
impl_cl_object!(cl_kernel);
impl_cl_object!(cl_mem);
impl_cl_object!(cl_program);
impl_cl_object!(cl_platform_id);
impl_cl_object!(cl_sampler);

/// NOTE: UNUSABLE_DEVICE_ID might be osx specific? or OpenCL
/// implementation specific?
/// UNUSABLE_DEVICE_ID was the cl_device_id encountered on my Macbook
/// Pro for a Radeon graphics card that becomes unavailable when
/// powersaving mode enables. Apparently the OpenCL platform can still
/// see the device, instead of a "legit" cl_device_id the inactive
/// device's cl_device_id is listed as 0xFFFF_FFFF.
pub const UNUSABLE_DEVICE_PTR: *mut c_void = 0xFFFF_FFFF as *mut c_void;

#[inline(always)]
fn _is_unusable_device_ptr(device_ptr: *mut c_void) -> bool {
    std::ptr::eq(
        device_ptr as *const c_void,
        UNUSABLE_DEVICE_PTR as *const c_void,
    )
}

unsafe impl ClObject for cl_device_id {
    unsafe fn new(val: *mut c_void) -> Output<cl_device_id> {
        let new = cl_device_id(val);
        new.check()?;
        Ok(new)
    }
    fn type_name() -> &'static str {
        "cl_device_id"
    }

    unsafe fn as_ptr(&self) -> *const c_void {
        self.0 as *const c_void
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.0 as *mut c_void
    }
    fn check(&self) -> Output<()> {
        if self.0.is_null() {
            return Err(ClObjectError::NullPointer("cl_device_id"))?;
        }
        if _is_unusable_device_ptr(self.0) {
            return Err(ClObjectError::UnusableDeviceAddress)?;
        }
        Ok(())
    }
}
