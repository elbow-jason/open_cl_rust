use std::fmt::Debug;
use crate::ffi::{cl_command_queue, cl_context, cl_device_id, cl_event, cl_kernel, cl_mem, cl_program, cl_platform_id};
use crate::{Output, DeviceError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClObjectError {
    #[error("OpenCL returned a {0} as a null pointer")]
    NullPointer(&'static str)
}

impl ClObjectError {
    fn null_pointer<T: ClObject>() -> ClObjectError {
        ClObjectError::NullPointer(T::type_name())
    }
}

pub trait CheckValidClObject {
    fn check_valid_cl_object(&self) -> Output<()>;
}

macro_rules! check_is_null {
    ($t:ident) => {
        impl CheckValidClObject for $t {
            fn check_valid_cl_object(&self) -> Output<()> {
                if self.is_null() {
                    return Err()
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
            return Err(ClObjectError::ObjectPointerWasNull("cl_device_id"));
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


pub unsafe trait ClObject: Sized + Clone + Copy + Debug + CheckValidClObject + PartialEq {
    fn type_name(&self) -> &'static str;
    fn address(&self) -> String {
        format!("{:?}", *self)
    }
}



unsafe impl ClObject for cl_command_queue {
    fn type_name(&self) -> &'static str {
        "cl_command_queue"
    }
}
unsafe impl ClObject for cl_context {
    fn type_name(&self) -> &'static str {
        "cl_context"
    }
}
unsafe impl ClObject for cl_device_id {
    fn type_name(&self) -> &'static str {
        "cl_device_id"
    }
}
unsafe impl ClObject for cl_event {
    fn type_name(&self) -> &'static str {
        "cl_event"
    }
}
unsafe impl ClObject for cl_kernel {
       fn type_name(&self) -> &'static str {
        "cl_kernel"
    }
}
unsafe impl ClObject for cl_mem {
       fn type_name(&self) -> &'static str {
        "cl_mem"
    }
}
unsafe impl ClObject for cl_program {
       fn type_name(&self) -> &'static str {
        "cl_program"
    }
}
unsafe impl ClObject for cl_platform_id {
       fn type_name(&self) -> &'static str {
        "cl_platform_id"
    }
}