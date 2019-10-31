use crate::ffi::{cl_event, cl_uint};
use crate::event::{Event};

use crate::open_cl::cl_object::CopyClObject;
use crate::open_cl::{
    Output,
    cl_release_event,
    cl_wait_for_events
};

/// WaitList is a holder for `cl_event`s that are to be awaited before
/// the enqueue job that they are passed with is run.
#[derive(Debug, Eq, PartialEq)]
pub struct WaitList {
    events: Vec<cl_event>,
    _unconstructable: (),
}

impl WaitList {
    pub fn empty() -> WaitList {
        WaitList {
            events: vec![],
            _unconstructable: (),
        }
    }

    // In this function the `event` is passed as an owned `Event`. The inner `cl_event` of the
    // `event` has a reference count of `n`. We copy the `cl_event` via `copy_cl_object_ref`
    // increasing its refcount to `n + 1`. We put the copied `cl_event` into the `events`
    // field effectively becoming the "owner" of the `cl_event`. When the scope of `push`
    // ends the `event` that is in scope and owned is `Drop`ped changing the refcount of
    // our new `cl_event` back to `n`.
    pub fn push(&mut self, event: Event) {
        let copied_cl_object_ref = unsafe { event.copy_cl_object_ref() };
        self.events.push(copied_cl_object_ref);
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

    #[inline]
    pub(crate) unsafe fn len_and_ptr_ptr(&self) -> (cl_uint, *const cl_event) {
        let slice = self.cl_object();
        let len = slice.len();
        if len == 0 {
            (0 as cl_uint, std::ptr::null() as *const cl_event)
        } else {
            (len as cl_uint, &slice as *const _ as *const cl_event)
        }
    }
}

impl Drop for WaitList {
    fn drop(&mut self) {
        unsafe {
            for e in self.events.iter_mut() {
                cl_release_event(&e);
            }
        };
    }
}

impl From<Option<Event>> for WaitList {
    fn from(option_event: Option<Event>) -> WaitList {
        match option_event {
            Some(event) => {
                let mut list = WaitList::empty();
                list.push(event);
                list
            }
            None => WaitList::empty(),
        }
    }
}