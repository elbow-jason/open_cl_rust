use super::{functions, Event, EventPtr};
use crate::cl::cl_event;
use crate::Output;
use libc::c_void;
use std::convert::TryInto;

/// The low level trait for synchronously waiting for events. This trait is used to produce the required
/// size and pointer arguments to the FFI of OpenCL allowing for the synchronously waiting of a given
/// event before execution is allowed to proceed.
///
/// # Safety
/// Due to multiple dangerous memory safety concerns with using events this
/// trait and it's functions are all unsafe. Mismanagement of the reference count,
/// lifetime, context, or resuse of an event is undefined behavior.
///
/// Due to this trait and it's only function involving the use of raw pointers this trait is unsafe.
/// Passing an event that has already been waited to OpenC is undefined behavior.
pub unsafe trait Waitlist: Sized {
    /// Returns the length of the waitlist.
    unsafe fn waitlist_len(&self) -> u32;

    /// Returns the raw pointer to the waitlist.
    unsafe fn waitlist_ptr(&self) -> *const *mut c_void;

    /// Copies the waitlist's (self) events into the passed mutable vector.
    ///
    /// # Safety
    /// Due to the movement of cl_event from one container to another, this function is unsafe
    /// failure to correctly track reference counts of cl_event objects can lead to memory leaks and/or
    /// segfaults.
    ///
    /// Note: When the vec that the events are given to Drops these events will not be released.
    unsafe fn fill_waitlist(&self, wl: &mut Vec<cl_event>);

    /// Consumes the waitlist into a vector of cl_events.
    ///
    /// # Safety
    /// Due to the movement of cl_event from one container to another, this function is unsafe
    /// failure to correctly track reference counts of cl_event objects can lead to memory leaks and/or
    /// segfaults.
    ///
    /// Note: When the vec that the events are given to Drops these events will not be released.
    unsafe fn new_waitlist(&self) -> Vec<cl_event>;

    /// Synchronously waits (blocks the thread) until all events in the waitlist are complete.
    ///
    /// # Safety
    /// Due to call to OpenCL's FFI with raw pointer (or Vec of raw pointers) this call will cause
    /// undefined behavior if any of the events is not in correct state, if the context of the events
    /// has been freed, if any of the events is a null pointer, if the queue the event was created with
    /// is freed, and a plethora of other conditions.
    ///
    /// Note: When the vec that the events are given to Drops these events will not be released.
    unsafe fn wait(self) -> Output<()> {
        let mut waitlist = Vec::new();
        self.fill_waitlist(&mut waitlist);
        functions::wait_for_events(&waitlist[..])
    }
}

unsafe impl Waitlist for &[cl_event] {
    unsafe fn waitlist_len(&self) -> u32 {
        self.len().try_into().unwrap()
    }

    unsafe fn waitlist_ptr(&self) -> *const *mut c_void {
        match self.len() {
            0 => std::ptr::null() as *const *mut c_void,
            _ => *self as *const _ as *const *mut c_void,
        }
    }

    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        wait_list.extend_from_slice(self);
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.to_vec()
    }
}

unsafe impl Waitlist for &[Event] {
    unsafe fn fill_waitlist(&self, wait_list: &mut Vec<cl_event>) {
        let waitlist = self.new_waitlist();
        wait_list.extend(waitlist);
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.iter().map(|evt| evt.event_ptr()).collect()
    }

    unsafe fn waitlist_len(&self) -> u32 {
        self.len().try_into().unwrap()
    }

    unsafe fn waitlist_ptr(&self) -> *const *mut c_void {
        match self.len() {
            0 => std::ptr::null() as *const *mut c_void,
            _ => *self as *const _ as *const *mut c_void,
        }
    }
}

unsafe impl Waitlist for &Event {
    unsafe fn waitlist_len(&self) -> u32 {
        1u32
    }

    unsafe fn waitlist_ptr(&self) -> *const *mut c_void {
        // repr(transparent) provides the ability to do this.
        *self as *const _ as *const *mut c_void
    }

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
            Some(event) => event.fill_waitlist(wait_list),
        }
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        match self {
            None => vec![],
            Some(event) => event.new_waitlist(),
        }
    }

    unsafe fn waitlist_len(&self) -> u32 {
        match self {
            Some(wl) => wl.waitlist_len(),
            None => 0u32,
        }
    }

    unsafe fn waitlist_ptr(&self) -> *const *mut c_void {
        // repr(transparent) provides the ability to do this.
        match self {
            Some(wl) => wl.waitlist_ptr(),
            None => std::ptr::null() as *const *mut c_void,
        }
    }
}
