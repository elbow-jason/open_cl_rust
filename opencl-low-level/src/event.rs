// pub mod event_info;
// pub mod flags;
// pub mod low_level;
// pub mod wait_list;
use std::fmt;

use crate::{
    CommandExecutionStatus, ClPointer, Output, build_output, ProfilingInfo, EventInfo,
    // WaitList, 
    
};

use crate::ffi::{
    // cl_context,
    cl_event, clGetEventInfo, clGetEventProfilingInfo, cl_event_info, cl_profiling_info, cl_ulong,
};

use crate::cl_helpers::cl_get_info5;

__release_retain!(event, Event);

unsafe fn release_event(evt: cl_event) {
    cl_release_event(evt).unwrap_or_else(|e| {
        panic!("Failed to release cl_event {:?} due to {:?}", evt, e);
    })
}

unsafe fn retain_event(evt: cl_event) {
    cl_retain_event(evt).unwrap_or_else(|e| {
        panic!("Failed to retain cl_event {:?} due to {:?}", evt, e);
    })
}

// NOTE: Fix cl_profiling_info arg // should be a bitflag or enum.
pub unsafe fn cl_get_event_profiling_info(event: cl_event, info_flag: cl_profiling_info) -> Output<u64> {
    let mut time: cl_ulong = 0;
    let err_code = clGetEventProfilingInfo(
        event,
        info_flag,
        std::mem::size_of::<cl_ulong>() as libc::size_t,
        (&mut time as *mut u64) as *mut libc::c_void,
        std::ptr::null_mut(),
    );
    build_output(time as u64, err_code)
}

pub fn cl_get_event_info<T: Copy>(event: cl_event, info_flag: cl_event_info) -> Output<ClPointer<T>> {
    unsafe {
        cl_get_info5(
            event,
            info_flag,
            clGetEventInfo,
        )
    }
}

pub trait EventPtr: Sized {
    fn event_ptr(&self) -> cl_event;
}


impl EventPtr for cl_event {
    fn event_ptr(&self) -> cl_event {
        *self
    }
}

impl EventPtr for ClEvent {
    fn event_ptr(&self) -> cl_event {
        self.object
    }
}

impl EventPtr for &ClEvent {
    fn event_ptr(&self) -> cl_event {
        self.object
    }
}


/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum EventError {
    #[fail(display = "KernelEvent encountered a null cl_event.")]
    ClEventCannotBeNull,
}

pub struct ClEvent {
    object: cl_event,
    _unconstructable: ()
}

impl ClEvent {
    pub unsafe fn unchecked_new(evt: cl_event) -> ClEvent {
        ClEvent {
            object: evt,
            _unconstructable: (),
        }
    }
}

impl Clone for ClEvent {
    fn clone(&self) -> ClEvent {
        unsafe { 
            let evt = self.object;
            retain_event(evt);
            ClEvent::unchecked_new(evt)
        }
    }
}

impl Drop for ClEvent {
    fn drop(&mut self) {
        unsafe {
            release_event(self.object);
        }
    }
}

impl fmt::Debug for ClEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event{{{:?}}}", self.object)
    }
}

impl ClEvent {
    #[allow(dead_code)]
    #[inline]
    fn wait(&self) -> Output<()> {
        unimplemented!();
        // WaitList::from_event(self).wait()
    }

    fn time(&self, info: ProfilingInfo) -> Output<u64> {
        unsafe { cl_get_event_profiling_info(self.event_ptr(), info.into()) }
    }

    pub fn queue_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Queued)
    }

    pub fn submit_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Submit)
    }

    pub fn start_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Start)
    }

    pub fn end_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::End)
    }

    fn info<T: Copy>(&self, flag: EventInfo) -> Output<ClPointer<T>> {
        cl_get_event_info::<T>(self.event_ptr(), flag.into())
    }

    pub fn reference_count(&self) -> Output<u32> {
        self.info(EventInfo::ReferenceCount)
            .map(|ret| unsafe { ret.into_one() })
    }

    // pub fn command_queue(&self) -> Output<CommandQueue> {
    //     self.info::<cl_command_queue>(Info::CommandQueue)
    //         .and_then(|ret| unsafe { ret.into_retained_wrapper::<CommandQueue>() })
    // }

    // pub fn context(&self) -> Output<ClContext> {
    //     self.info::<cl_context>(Info::Context)
    //         .and_then(|cl_ptr| unsafe { ClContext::from_unretained(cl_ptr.into_one()) })
    // }

    pub fn command_execution_status(&self) -> Output<CommandExecutionStatus> {
        self.info(EventInfo::CommandExecutionStatus)
            .map(|ret| unsafe { ret.into_one() })
    }
}

/// A CompleteEvent is the result of making a synchronous ffi call.
///
/// After the `cl_event`'s event is over the event is no longer able
///
/// A CompleteEvent is not for putting into WaitList.
///
/// Don't do it. You'll segfault.
pub struct CompleteEvent {
    event: ClEvent,
    _unconstructable: (),
}

impl CompleteEvent {
    pub fn new(event: ClEvent) -> CompleteEvent {
        CompleteEvent {
            event,
            _unconstructable: (),
        }
    }

    pub unsafe fn inner(&self) -> &ClEvent {
        &self.event
    }
}

impl PartialEq for ClEvent {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.object, other.object)
    }
}

impl Eq for ClEvent {}

// #[cfg(test)]
// mod tests {
//     // use crate::ffi::cl_event;
//     use crate::{Session, SessionBuilder, Event, Kernel, Work, DeviceMem};
//     // use crate::cl::ClObject;
//     use crate::event::event_info::CommandExecutionStatus;

//     fn get_event() -> (Session, Event) {
//         let session: Session = SessionBuilder::new().build();
//         let kernel = Kernel::create(session.program(), "add_one").expect("Failed to Kernel::create/2");
//         let input_data: Vec<usize> = vec![1, 2, 3];
//         let mem_buffer: DeviceMem<usize> = DeviceMem::create_read_write_from(session.context(), &input_data)
//             .expect("Failed to create_read_write_from vec![1, 2, 3]");
//         let () = kernel.set_arg(0, &mem_buffer).expect("Failed to set_arg(0, &mem_buffer)");
//         let work = Work::new(input_data.len());
//         let event = session.command_queue().sync_enqueue_kernel(&kernel, &work).expect("Failed to sync_enqueue_kernel");
//         (session, event)
//     }

//     #[test]
//     fn event_method_queue_time_works() {
//         let (_sess, event) = get_event();
//         let output = event.queue_time().expect("Failed to call event.queue_time()");
//         assert_eq!(output, 0);
//     }

// //     #[test]
// //     fn event_method_submit_time_works() {
// //         let (_sess, event) = get_event();
// //         let output = event.submit_time().expect("Failed to call event.submit_time()");
// //         assert_eq!(output, 0);
// //     }

// //     #[test]
// //     fn event_method_start_time_works() {
// //         let (_sess, event) = get_event();
// //         let output = event.start_time().expect("Failed to call event.start_time()");
// //         assert_eq!(output, 0);
// //     }

// //     #[test]
// //     fn event_method_end_time_works() {
// //         let (_sess, event) = get_event();
// //         let output = event.end_time().expect("Failed to call event.end_time()");
// //         assert_eq!(output, 0);
// //     }

// //     #[test]
// //     fn event_method_reference_count_works() {
// //         let (_sess, event) = get_event();
// //         let output = event.reference_count().expect("Failed to call event.reference_count()");
// //         assert_eq!(output, 0);
// //     }

// //     #[test]
// //     fn event_method_command_queue_works() {
// //         let (_sess, event) = get_event();
// //         let _output: CommandQueue = event.command_queue().expect("Failed to call event.command_queue()");

// //     }

// //     #[test]
// //     fn event_method_context_works() {
// //         let (_sess, event) = get_event();
// //         let _output: Context = event.context().expect("Failed to call event.context()");
// //     }

// //     #[test]
// //     fn event_method_command_execution_status_works() {
// //         let (_sess, event) = get_event();
// //         let _output: CommandExecutionStatus = event.command_execution_status().expect("Failed to call event.command_exection_status()");
// //     }
// }
