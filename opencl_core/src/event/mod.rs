
pub mod low_level;
pub mod flags;
pub mod event_info;
pub mod wait_list;


#[cfg(test)]
mod tests;

use low_level::{cl_retain_event, cl_release_event};

use crate::ffi::{
    cl_event,
    cl_profiling_info,
};

use event_info::{
    CommandExecutionStatus,
    EventInfo,
    EventInfoFlag
};

pub use wait_list::WaitList;

use crate::command_queue::CommandQueue;
use crate::utils::CopyClObject;
use crate::context::Context;
use crate::error::{Error, Output};

/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum EventError {
    #[fail(display = "KernelEvent encountered a null cl_event.")]
    ClEventCannotBeNull,
}

impl From<EventError> for Error {
    fn from(ee: EventError) -> Error {
        Error::EventError(ee)
    }
}

__impl_unconstructable_cl_wrapper!(Event, cl_event);
__impl_cl_object_for_wrapper!(Event, cl_event);
__impl_clone_for_cl_object_wrapper!(Event, cl_retain_event);
__impl_drop_for_cl_object_wrapper!(Event, cl_release_event);


impl CopyClObject<cl_event> for Event {
    // Super duper unsafe
    unsafe fn copy_cl_object_ref(&self) -> cl_event {
        cl_retain_event(&self.inner);
        self.inner
    }
}

use flags::ProfilingInfo;
use EventInfo as Info;
use EventInfoFlag as Flag;

impl Event {
    #[allow(dead_code)]
    #[inline]
    fn wait(&self) -> Output<()> {
        unimplemented!();
        // WaitList::from_event(self).wait()
    }

    fn time(&self, info: ProfilingInfo) -> Output<u64> {
        let cl_object = unsafe { self.raw_cl_object() };
        low_level::cl_get_event_profiling_info(&cl_object, info as cl_profiling_info)
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

    pub fn reference_count(&self) -> Output<usize> {
        let info = low_level::cl_get_event_info(self, Flag::ReferenceCount)?;
        match info {
            Info::ReferenceCount(output) => Ok(output),
            _ => panic!(
                "The EventInfo flag {:?} returned an invalid variant {:?}",
                Flag::ReferenceCount,
                info
            ),
        }
    }
}

macro_rules! impl_event_info {
    ($fn_name:ident, $pascal:ident, $ret:ident) => {
        impl Event {
            pub fn $fn_name(&self) -> Output<$ret> {
                let info = low_level::cl_get_event_info(self, Flag::$pascal)?;
                match info {
                    Info::$pascal(output) => Ok(output),
                    _ => panic!(
                        "The EventInfo flag {:?} returned an invalid variant {:?}",
                        Flag::$pascal,
                        info
                    ),
                }
            }
        }
    };
}

impl_event_info!(command_queue, CommandQueue, CommandQueue);
impl_event_info!(context, Context, Context);
impl_event_info!(
    command_execution_status,
    CommandExecutionStatus,
    CommandExecutionStatus
);


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
