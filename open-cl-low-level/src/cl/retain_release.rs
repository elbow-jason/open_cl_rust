use crate::cl::cl_object::{
    cl_command_queue, cl_context, cl_device_id, cl_event, cl_kernel, cl_mem, cl_platform_id,
    cl_program, ClObject,
};

use crate::cl::{
    clReleaseCommandQueue, clReleaseContext, clReleaseDevice, clReleaseEvent, clReleaseKernel,
    clReleaseMemObject, clReleaseProgram, clRetainCommandQueue, clRetainContext, clRetainDevice,
    clRetainEvent, clRetainKernel, clRetainMemObject, clRetainProgram,
};

use libc::c_void;

pub trait RetainRelease: ClObject {
    /// Retains (increments the reference count of) the ClObject.
    ///
    /// # Safety
    /// Balancing the release and retain reference count of a ClObject must be
    /// done with care. Improper usage of release and retain can lead to
    /// undefined behavior.
    ///
    /// Usage of an invalid ClObject is undefined behavior.
    unsafe fn retain(&self);

    /// Releases (decrements reference count of) the ClObject
    ///
    /// # Safety
    /// Balancing the release and retain reference count of a ClObject
    /// must be done with care. Improper usage of release and
    /// retain can lead to undefined behavior.
    ///
    /// Usage of an invalid ClObject is undefined behavior.
    unsafe fn release(&mut self);
}

macro_rules! impl_retain_release {
    ($snake:ident, $pascal:ident) => {
        paste::item! {
            impl RetainRelease for [<cl_ $snake>] {
                /// This function is used to increase the atomic reference count of the associated
                /// OpenCL ARC object. This function should only be used when the OpenCL interface
                /// returns a ARC object that is not reference counted by OpenCL (yes, OpenCL let's you do that...)
                ///
                /// # Safety
                /// This function atomically decrements the OpenCL reference count. Mismanagement
                /// of an object's OpenCL ARC can lead to undefined behavior.
                unsafe fn retain(&self) {
                    $crate::cl::StatusCodeError::check([<clRetain $pascal>](self.as_ptr() as *mut c_void))
                        .unwrap_or_else(|e| {
                            panic!("Failed to retain cl_{} {:?} due to {:?}", stringify!($snake), self, e);
                        })
                }

                /// This function is used to decrease the OpenCL atomic reference count of the
                /// associated OpenCL ARC object.
                ///
                /// # Safety
                /// This function atomically decrements the OpenCL reference count. Mismanagement
                /// of an object's OpenCL ARC can lead to undefined behavior.
                unsafe fn release(&mut self) {
                    $crate::cl::StatusCodeError::check([<clRelease $pascal>](self.as_mut_ptr()))
                        .unwrap_or_else(|e| {
                            panic!("Failed to release cl_{} {:?} due to {:?}", stringify!($snake), self, e);
                        })
                }
            }
        }
    }
}

impl_retain_release!(command_queue, CommandQueue);
impl_retain_release!(context, Context);
impl_retain_release!(device_id, Device);
impl_retain_release!(event, Event);
impl_retain_release!(kernel, Kernel);
impl_retain_release!(mem, MemObject);
impl_retain_release!(program, Program);

impl RetainRelease for cl_platform_id {
    unsafe fn release(&mut self) {
        ()
    }
    unsafe fn retain(&self) {
        ()
    }
}
