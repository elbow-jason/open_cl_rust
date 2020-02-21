use std::fmt::Debug;
use crate::ffi::{cl_command_queue, cl_context, cl_device_id, cl_event, cl_kernel, cl_mem, cl_program, cl_platform_id};
use crate::{Output, Error, DeviceError};

pub trait CheckValidClObject {
    fn check_valid_cl_object(&self) -> Output<()>;
}

macro_rules! check_is_null {
    ($t:ident) => {
        impl CheckValidClObject for $t {
            fn check_valid_cl_object(&self) -> Output<()> {
                if self.is_null() {
                    return Err(Error::ClObjectCannotBeNull)
                }
                Ok(())
            }
        }
    }
}

pub const UNUSABLE_DEVICE_ID: cl_device_id = 0xFFFF_FFFF as *mut usize as cl_device_id;

impl CheckValidClObject for cl_device_id {
    fn check_valid_cl_object(&self) -> Output<()> {
        if self.is_null() {
            return Err(Error::ClObjectCannotBeNull);
        }
        if *self == UNUSABLE_DEVICE_ID {
            return Err(DeviceError::UnusableDevice.into());
        }
        Ok(())
    }
}

check_is_null!(cl_command_queue);
check_is_null!(cl_context);
check_is_null!(cl_event);
check_is_null!(cl_kernel);
check_is_null!(cl_mem);
check_is_null!(cl_program);
check_is_null!(cl_platform_id);


pub unsafe trait ClObject: Sized + Clone + Copy + Debug + CheckValidClObject {
    fn address(&self) -> String {
        format!("{:?}", *self)
    }
}

unsafe impl ClObject for cl_command_queue {}
unsafe impl ClObject for cl_context {}
unsafe impl ClObject for cl_device_id {}
unsafe impl ClObject for cl_event {}
unsafe impl ClObject for cl_kernel {}
unsafe impl ClObject for cl_mem {}
unsafe impl ClObject for cl_program {}
unsafe impl ClObject for cl_platform_id {}