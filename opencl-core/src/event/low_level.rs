use crate::ffi::{
    cl_ulong,
    cl_event,
    cl_profiling_info,
    cl_event_info,
    clGetEventInfo,
    clGetEventProfilingInfo,
};

use crate::error::Output;
use crate::utils::StatusCode;
use crate::cl::{ClPointer, ClObject, cl_get_info5};

use super::Event;
use super::event_info::EventInfo;

__release_retain!(event, Event);


// NOTE: Fix cl_profiling_info arg 
pub fn cl_get_event_profiling_info(event: &cl_event, info: cl_profiling_info) -> Output<u64> {
    let mut time: cl_ulong = 0;
    let err_code = unsafe {
        clGetEventProfilingInfo(
            *event,
            info,
            std::mem::size_of::<cl_ulong>() as libc::size_t,
            (&mut time as *mut u64) as *mut libc::c_void,
            std::ptr::null_mut(),
        )
    };
    StatusCode::into_output(err_code, time as u64)
}

pub fn cl_get_event_info<T: Copy>(event: &Event, info_flag: EventInfo) -> Output<ClPointer<T>> {
    unsafe {
        cl_get_info5(
            event.raw_cl_object(),
            info_flag as cl_event_info,
            clGetEventInfo
        )
    }
}