
use crate::{
    Output, build_output, SizeAndPtr, ClEvent, EventPtr
    // Error, EventPtr, cl_release_event,
};
use crate::ffi::{clWaitForEvents, cl_event};

pub unsafe fn cl_wait_for_events(wl: &[cl_event]) -> Output<()> {
    let SizeAndPtr(len, ptr) = wl.waitlist_size_and_ptr();
    build_output((), clWaitForEvents(len as u32, ptr))
}

pub unsafe trait Waitlist: Sized {
    unsafe fn fill_waitlist(&self, wl: &mut Vec<cl_event>);
    unsafe fn new_waitlist(&self) -> Vec<cl_event>;
    unsafe fn wait(self) -> Output<()> {
        let mut waitlist = Vec::new();
        self.fill_waitlist(&mut waitlist);
        cl_wait_for_events(&waitlist[..])
    }
}

pub trait WaitlistSizeAndPtr: Sized {
    unsafe fn waitlist_size_and_ptr(&self) -> SizeAndPtr<*const cl_event>;
}

impl WaitlistSizeAndPtr for &[cl_event] {
    unsafe fn waitlist_size_and_ptr(&self) -> SizeAndPtr<*const cl_event> {
        let len = self.len();
        if len == 0 {
            SizeAndPtr(0, std::ptr::null() as *const cl_event)
        } else {
            // self is &&[cl_event] so we use *self
            SizeAndPtr(len, *self as *const _ as *const cl_event)
        }
    }
}

unsafe impl Waitlist for &[cl_event] {
    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        wait_list.extend_from_slice(self);
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.to_vec()
    }
}

unsafe impl Waitlist for &[ClEvent] {
    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        let waitlist = self.new_waitlist();
        wait_list.extend(waitlist);
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.iter().map(|evt| evt.event_ptr()).collect()
    }
}

unsafe impl Waitlist for &ClEvent {
    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        wait_list.push(self.event_ptr());
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        vec![self.event_ptr()]
    }
}

unsafe impl<W: Waitlist> Waitlist for Option<W> {
    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        match self {
            None => (),
            Some(event) => event.fill_waitlist(wait_list)
        }
    }
    
    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        match self {
            None => vec![],
            Some(event) => event.new_waitlist()
        }
    }
}
