use super::Waitlist;
use crate::cl::{
    clGetEventInfo, clGetEventProfilingInfo, clWaitForEvents, cl_command_execution_status,
    cl_command_queue, cl_context, cl_event, cl_event_info, cl_profiling_info, ClObject, EventInfo,
    StatusCodeError,
};
use crate::Output;
use libc::{c_void, size_t};

// NOTE: Fix cl_profiling_info arg // should be a bitflag or enum.
pub unsafe fn cl_get_event_profiling_info(
    event: cl_event,
    info_flag: cl_profiling_info,
) -> Output<u64> {
    let mut time: u64 = 0;
    let err_code = clGetEventProfilingInfo(
        event.as_ptr() as *mut c_void,
        info_flag,
        std::mem::size_of::<u64>() as size_t,
        (&mut time as *mut u64) as *mut c_void,
        std::ptr::null_mut(),
    );
    StatusCodeError::check(err_code)?;
    Ok(time)
}

// pub unsafe fn cl_get_event_info<T: Copy>(
//     event: cl_event,
//     info_flag: cl_event_info,
// ) -> Output<ClPointer<T>> {
//     cl_get_info5(event, info_flag, clGetEventInfo)
// }

/// Low level helper function for the FFI call to clGetProgramInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_event that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_event_info_u32(event: cl_event, flag: cl_event_info) -> Output<u32> {
    cl_get_info!(
        One,
        u32,
        clGetEventInfo,
        event,
        Into::<cl_event_info>::into(flag)
    )
}

/// Low level helper function for the FFI call to clGetProgramInfo with cl_command_queue expected
///
/// # Safety
/// Calling this function with a cl_event that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_event_info_command_queue(event: cl_event) -> Output<cl_command_queue> {
    cl_get_info!(
        One,
        cl_command_queue,
        clGetEventInfo,
        event,
        Into::<cl_event_info>::into(EventInfo::CommandQueue)
    )
}

/// Low level helper function for the FFI call to clGetProgramInfo with cl_context expected
///
/// # Safety
/// Calling this function with a cl_event that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_event_info_context(event: cl_event) -> Output<cl_context> {
    cl_get_info!(
        One,
        cl_context,
        clGetEventInfo,
        event,
        Into::<cl_event_info>::into(EventInfo::Context)
    )
}

/// Low level helper function for the FFI call to clGetProgramInfo with cl_context expected
///
/// # Safety
/// Calling this function with a cl_event that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_command_execution_status(event: cl_event) -> Output<cl_command_execution_status> {
    cl_get_info!(
        One,
        cl_command_execution_status,
        clGetEventInfo,
        event,
        Into::<cl_event_info>::into(EventInfo::CommandExecutionStatus)
    )
}

/// The low-level function for synchronously waiting events (blocks the calling thread).
///
/// # Safety
/// Due to call to OpenCL's FFI with raw pointer (or slice of raw pointers) this call will cause
/// undefined behavior if any of the events is not in correct state, if the context of the events
/// has been freed, if any of the events is a null pointer, if the queue the event was created with
/// is freed, and a plethora of other conditions.
pub unsafe fn wait_for_events<'a>(wl: &'a [cl_event]) -> Output<()> {
    StatusCodeError::check(clWaitForEvents(wl.waitlist_len(), wl.waitlist_ptr()))
}
