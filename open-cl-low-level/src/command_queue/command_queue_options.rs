use super::{Event, EventPtr, Waitlist};
use crate::cl::cl_event;
use libc::c_void;

#[derive(Debug, Clone)]
pub struct CommandQueueOptions {
    pub is_blocking: bool,
    pub offset: usize,
    pub waitlist: Vec<Event>,
}

impl Default for CommandQueueOptions {
    /// Default constructor for CommandQueueOptions.
    fn default() -> CommandQueueOptions {
        CommandQueueOptions {
            is_blocking: true,
            offset: 0,
            waitlist: vec![],
        }
    }
}

impl From<Option<CommandQueueOptions>> for CommandQueueOptions {
    fn from(maybe_cq_opts: Option<CommandQueueOptions>) -> CommandQueueOptions {
        maybe_cq_opts.unwrap_or(CommandQueueOptions::default())
    }
}

unsafe impl Waitlist for CommandQueueOptions {
    /// Fill waitlist extends the waitlist from the CommandQueueOptions' waitlist.
    unsafe fn fill_waitlist(&self, waitlist: &mut Vec<cl_event>) {
        waitlist.extend(self.new_waitlist());
    }

    /// Creates a waitlist Vec<cl_event> for using in OpenCL FFI.
    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.waitlist.iter().map(|evt| evt.event_ptr()).collect()
    }

    unsafe fn waitlist_ptr(&self) -> *const *mut c_void {
        (&self.waitlist[..]).waitlist_ptr()
    }

    unsafe fn waitlist_len(&self) -> u32 {
        (&self.waitlist[..]).waitlist_len()
    }
}
