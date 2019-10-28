use crate::{cl_event, cl_uint};

#[inline]
pub unsafe fn len_and_ptr_ptr(slice: &[cl_event]) -> (cl_uint, *const cl_event) {
    let len = slice.len();
    if len == 0 {
        (0 as cl_uint, std::ptr::null() as *const cl_event)
    } else {
        (len as cl_uint, &slice as *const _ as *const cl_event)
    }
}
