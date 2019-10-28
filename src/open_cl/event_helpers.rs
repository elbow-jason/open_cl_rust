#![allow(dead_code)]
use crate::{cl_event};

#[inline]
pub fn is_null_mut(e: &cl_event) -> bool {
    std::ptr::null_mut() == *e
}



#[inline]
pub fn null_mut_ptr() -> *mut cl_event {
    std::ptr::null_mut() as *mut cl_event
}

#[inline]
pub fn event_as_mut_ptr(e: cl_event) -> *mut cl_event {
    e as *mut cl_event
}

#[inline]
pub fn option_event_as_mut_ptr(option_e: Option<cl_event>) -> *mut cl_event {
    match option_e {
        Some(e) => event_as_mut_ptr(e),
        None => std::ptr::null_mut(),
    }
}
                