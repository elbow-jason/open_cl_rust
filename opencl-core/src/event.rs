pub mod event_info;
pub mod flags;
pub mod low_level;
pub mod wait_list;

use low_level::{cl_release_event, cl_retain_event};

use crate::ffi::{
    // cl_command_queue,
    cl_context,
    cl_event
};

use event_info::{CommandExecutionStatus, EventInfo};



use crate::cl::{
    ClObject,
    ClPointer,
};


use crate::{Context, ContextRefCount, Output};


/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum EventError {
    #[fail(display = "KernelEvent encountered a null cl_event.")]
    ClEventCannotBeNull,
}

__impl_unconstructable_cl_wrapper!(Event, cl_event);
__impl_default_debug_for!(Event);
__impl_cl_object_for_wrapper!(Event, cl_event, cl_retain_event, cl_release_event);
__impl_clone_for_cl_object_wrapper!(Event, cl_retain_event);
__impl_drop_for_cl_object_wrapper!(Event, cl_release_event);

use flags::ProfilingInfo;
use EventInfo as Info;

pub struct Event<'a> {
    inner: ClEvent,
    command_queue: &'a CommandQueue,
}

impl 


impl Event {
    #[allow(dead_code)]
    #[inline]
    fn wait(&self) -> Output<()> {
        unimplemented!();
        // WaitList::from_event(self).wait()
    }

    fn time(&self, info: ProfilingInfo) -> Output<u64> {
        low_level::cl_get_event_profiling_info(self, info)
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

    fn info<T: Copy>(&self, flag: Info) -> Output<ClPointer<T>> {
        low_level::cl_get_event_info::<T>(self, flag)
    }

    pub fn reference_count(&self) -> Output<u32> {
        self.info(Info::ReferenceCount)
            .map(|ret| unsafe { ret.into_one() })
    }

    // pub fn command_queue(&self) -> Output<CommandQueue> {
    //     self.info::<cl_command_queue>(Info::CommandQueue)
    //         .and_then(|ret| unsafe { ret.into_retained_wrapper::<CommandQueue>() })
    // }

    pub fn context(&self) -> Output<Context> {
        self.info::<cl_context>(Info::Context)
            .and_then(|cl_ptr| unsafe { Context::from_unretained(cl_ptr.into_one()) })
    }

    pub fn command_execution_status(&self) -> Output<CommandExecutionStatus> {
        self.info(Info::CommandExecutionStatus)
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
    event: Event,
    _unconstructable: (),
}

impl CompleteEvent {
    pub fn new(event: Event) -> CompleteEvent {
        CompleteEvent {
            event,
            _unconstructable: (),
        }
    }

    // You better not be calling this.
    pub unsafe fn inner(&self) -> Event {
        self.event.clone()
    }

    // You REALLY better not be calling this.
    pub unsafe fn inner_raw_cl_object(&self) -> cl_event {
        self.event.raw_cl_object()
    }
}

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
