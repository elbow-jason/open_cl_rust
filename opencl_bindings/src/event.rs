use crate::ffi::{cl_event, cl_profiling_info};

use crate::open_cl::{
    cl_get_event_profiling_info, cl_release_event, cl_retain_event,
};

use crate::open_cl::events::{cl_get_event_info, CommandExecutionStatus, EventInfo, EventInfoFlag};

use crate::{ClObject, CommandQueue, Context, CopyClObject, Error, Output};

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

#[inline]
fn panic_on_null(e: cl_event) {
    if e.is_null() {
        panic!("Event cannot be created with a null pointer")
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct Event {
    inner: cl_event,
    _unconstructable: (),
}

impl Event {
    /// Users cannot create *these* events.
    ///
    /// We check for null here in case of catastrophe.
    pub(crate) fn new(inner: cl_event) -> Event {
        panic_on_null(inner);
        Event {
            inner,
            _unconstructable: (),
        }
    }
}

impl Drop for Event {
    /// Since an event might have been created as `null` inside the
    /// unsafe blocks calling the OpenCL ffi we must make sure
    /// that `drop` does not try to release `null`.
    fn drop(&mut self) {
        unsafe { cl_release_event(&self.inner) };
    }
}

impl Clone for Event {
    /// We check for null here in case of catastrophe.
    fn clone(&self) -> Event {
        let new_event = Event::new(self.inner);
        unsafe {
            cl_retain_event(&new_event.inner);
        }
        new_event
    }
}

impl ClObject<cl_event> for Event {
    unsafe fn raw_cl_object(&self) -> cl_event {
        self.inner
    }
}

impl CopyClObject<cl_event> for Event {
    // Super duper unsafe
    unsafe fn copy_cl_object_ref(&self) -> cl_event {
        cl_retain_event(&self.inner);
        self.inner
    }
}

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
        cl_get_event_profiling_info(&cl_object, info as cl_profiling_info)
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
        let info = cl_get_event_info(self, Flag::ReferenceCount)?;
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
                let info = cl_get_event_info(self, Flag::$pascal)?;
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

crate::__codes_enum!(ProfilingInfo, cl_profiling_info, {
    Queued => 0x1280,
    Submit => 0x1281,
    Start => 0x1282,
    End => 0x1283,
    Complete => 0x1284
});
