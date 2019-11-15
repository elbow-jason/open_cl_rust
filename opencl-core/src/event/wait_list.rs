use crate::ffi::{
    cl_event,
    cl_uint,
    clWaitForEvents,
};
use crate::event::{Event};
use crate::event::low_level::cl_release_event;
use crate::utils::StatusCode;
use crate::cl::ClObject;
use crate::error::{Error, Output};

pub fn cl_wait_for_events(wait_list: WaitList) -> Output<()> {
    let err_code = unsafe {
        let (wait_list_len, wait_list_ptr_ptr) = wait_list.len_and_ptr_ptr();

        clWaitForEvents(wait_list_len, wait_list_ptr_ptr)
    };
    StatusCode::build_output(err_code, ())
}

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

    // Here we hand ownership of the `cl_event` of the 
    pub fn push(&mut self, event: Event) {
        let cl_object: cl_event = unsafe { event.raw_cl_object() };
        std::mem::forget(event);
        self.events.push(cl_object);
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

    pub fn is_empty(&self) -> bool {
        self.events.len() == 0
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

    /// Don't panic early when dropping.
    /// Release all the events, accumulate the errors *then* panic if anything when wrong.
    fn drop(&mut self) {
        let mut errors: Vec<Error> = Vec::new();
        for e in self.events.iter_mut() {
            unsafe {
                match cl_release_event(*e) {
                    Ok(()) => (),
                    Err(e) => errors.push(e),
                }
            }
        }
        if !errors.is_empty() {
            panic!("WaitList failed to drop at least one cl_event {:?}", errors);
        }
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