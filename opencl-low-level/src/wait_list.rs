
use crate::{
    Output, build_output, ClEvent, ClInput, SizeAndPtr,
    // Error, EventPtr, cl_release_event, 
};
use crate::ffi::{clWaitForEvents, cl_event};


pub fn cl_wait_for_events(wait_list: WaitList) -> Output<()> {
    let err_code = unsafe {
        let SizeAndPtr(len, ptr) = wait_list.size_and_ptr();

        clWaitForEvents(len as u32, ptr)
    };
    build_output((), err_code)
}

/// WaitList is a holder for `cl_event`s that are to be awaited before
/// the enqueue job that they are passed with is run.
#[derive(Debug, Eq, PartialEq)]
pub struct WaitList {
    events: Vec<ClEvent>,
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
    pub fn push(&mut self, event: ClEvent) {
        self.events.push(event);
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
}

unsafe impl ClInput<*const cl_event> for WaitList {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<*const cl_event> {
        let len = self.len();
        if len == 0 {
            SizeAndPtr(0, std::ptr::null() as *const cl_event)
        } else {
            SizeAndPtr(len, &self.events as *const _ as *const cl_event)
        }
    }
}

impl From<Option<ClEvent>> for WaitList {
    fn from(option_event: Option<ClEvent>) -> WaitList {
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
