use std::marker::PhantomData;

use crate::open_cl::{
    cl_event,
    cl_event_info,
    cl_get_event_profiling_info,
    cl_profiling_info,
    cl_release_event,
    cl_int,
    cl_wait_for_events,
};

use crate::{Output, Error};


/// An error related to an Event or EventList.
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
fn cl_event_cannot_be_null(e: cl_event) -> Result<(), EventError> {
    if e.is_null() {
        Err(EventError::ClEventCannotBeNull)
    } else {
        Ok(())
    }
}

// // // An Event::Input is constructed in user-land (by you) and passed
// // // to a kernel command function as a `&mut Event`. Where it will be
// // // mutated into an Event::Output. 
// // struct UserEvent(cl_event);

// #[repr(C)]
// #[derive(Debug, Hash, PartialEq, Eq)]
// pub struct UserEvent {
//     inner: cl_event,
//     _phantom: PhantomData<()>
// }

// impl UserEvent {

//     /// A new User event. Used as input for kernel commands.
//     #[inline]
//     pub fn new() -> UserEvent {
//         UserEvent {
//             inner: 0 as cl_event,
//             _phantom: PhantomData
//         }
//     }

//     #[inline]
//     pub fn set_complete(&self) -> Output<()> {
//         unimplemented!();
//     }

//     #[inline]
//     pub fn is_complete(&self) -> Output<bool> {
//         unimplemented!();
//     }

//     pub fn cl_object(&self) -> cl_event {
//         self.inner
//     }
// }

// impl Event for UserEvent {
//     fn get_cl_event(&self) -> cl_event {
//         self.inner
//     }
// }



#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ClEvent {
    inner: cl_event,
    _phantom: PhantomData<()>
}

impl ClEvent {
    pub fn new(e: cl_event) -> Result<ClEvent, EventError> {
        cl_event_cannot_be_null(e)?;
        Ok(ClEvent{inner: e, _phantom: PhantomData})
    }
}

impl Drop for ClEvent {
    fn drop(&mut self) {
        println!("Dropping event {:?}", self.inner);
        cl_release_event(&self.inner).unwrap_or_else(|e| panic!("Failed to cl_release_event {:?}", e))
    }
}

impl Event for ClEvent {

    fn get_cl_event(&self) -> cl_event {
        self.inner
    }
}

pub trait Event {

    #[inline]
    fn get_cl_event(&self) -> cl_event;


    #[inline]
    fn wait(&self) -> Output<()> {
        unimplemented!();
        // EventList::from_event(self).wait()
    }


    fn time(&self, info: ProfilingInfo) -> Output<u64> {
        cl_get_event_profiling_info(&self.get_cl_event(), info as cl_profiling_info)
    }

    fn queue_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Queued)
    }

    fn submit_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Submit)
    }

    fn start_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::Start)
    }

    fn end_time(&self) -> Output<u64> {
        self.time(ProfilingInfo::End)
    }
}





#[derive(Debug, Eq, PartialEq)]
pub struct EventList {
    events: Vec<cl_event>,
}

impl EventList {
    pub fn empty() -> EventList {
        EventList{events: vec![]}
    }

    pub fn cl_object(&self) -> &[cl_event] {
        &self.events
    }

    pub fn wait(self) -> Output<()> {
        cl_wait_for_events(self)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

impl From<ClEvent> for EventList {
    fn from(event: ClEvent) -> EventList {
        EventList { events: vec![event.get_cl_event()] }
    }
}

impl From<Option<ClEvent>> for EventList {
    fn from(option_event: Option<ClEvent>) -> EventList {
        match option_event {
            Some(event) => EventList::from(event),
            None => EventList::empty()
        }
    }
}


// fn resolve_event_ptrs(event: Option<Event>) -> *mut cl_event {
//     match new_event {
//         Some(mut ne) => ne.alloc_new(),
//         None => ptr::null_mut() as *mut cl_event,
//     }
// }

// /* command execution status */
crate::__codes_enum!(CommandExecutionStatus, cl_int, {
    Complete => 0x0,
    Running => 0x1,
    Submitted => 0x2,
    Queued => 0x3
});


/* cl_profiling_info */
crate::__codes_enum!(ProfilingInfo, cl_profiling_info, {
    Queued => 0x1280,
    Submit => 0x1281,
    Start => 0x1282,
    End => 0x1283,
    Complete => 0x1284
});

/* cl_event_info */
crate::__codes_enum!(EventInfo, cl_event_info, {
    CommandQueue => 0x11D0,
    CommandType => 0x11D1,
    ReferenceCount => 0x11D2,
    CommandExecutionStatus => 0x11D3,
    Context => 0x11D4
});

