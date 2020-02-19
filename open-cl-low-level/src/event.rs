use std::fmt;
use std::mem::ManuallyDrop;
use std::time::Duration;

use crate::{
    build_output, utils, ClCommandQueue, ClContext, ClNumber, ClPointer, CommandExecutionStatus,
    Error, EventInfo, Output, ProfilingInfo, Waitlist,
};

use crate::ffi::{
    clGetEventInfo, clGetEventProfilingInfo, cl_command_queue, cl_context, cl_event, cl_event_info,
    cl_profiling_info, cl_ulong,
};

use crate::cl_helpers::cl_get_info5;

__release_retain!(event, Event);

unsafe fn release_event(evt: cl_event) {
    cl_release_event(evt).unwrap_or_else(|e| {
        panic!("Failed to release cl_event {:?} due to {:?}", evt, e);
    })
}

unsafe fn retain_event(evt: cl_event) {
    cl_retain_event(evt).unwrap_or_else(|e| {
        panic!("Failed to retain cl_event {:?} due to {:?}", evt, e);
    })
}

// NOTE: Fix cl_profiling_info arg // should be a bitflag or enum.
pub unsafe fn cl_get_event_profiling_info(
    event: cl_event,
    info_flag: cl_profiling_info,
) -> Output<u64> {
    let mut time: cl_ulong = 0;
    let err_code = clGetEventProfilingInfo(
        event,
        info_flag,
        std::mem::size_of::<cl_ulong>() as libc::size_t,
        (&mut time as *mut u64) as *mut libc::c_void,
        std::ptr::null_mut(),
    );
    build_output(time as u64, err_code)
}

pub unsafe fn cl_get_event_info<T: Copy>(
    event: cl_event,
    info_flag: cl_event_info,
) -> Output<ClPointer<T>> {
    cl_get_info5(event, info_flag, clGetEventInfo)
}

pub unsafe trait EventPtr: Sized {
    unsafe fn event_ptr(&self) -> cl_event;

    fn address(&self) -> String {
        format!("{:?}", unsafe { self.event_ptr() })
    }
}

unsafe impl EventPtr for cl_event {
    unsafe fn event_ptr(&self) -> cl_event {
        *self
    }
}

unsafe impl EventPtr for ClEvent {
    unsafe fn event_ptr(&self) -> cl_event {
        self.object
    }
}

unsafe impl EventPtr for &ClEvent {
    unsafe fn event_ptr(&self) -> cl_event {
        self.object
    }
}

/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum EventError {
    #[fail(display = "Encountered a null cl_event.")]
    ClEventCannotBeNull,

    #[fail(display = "Event was already consumed. {:?}", _0)]
    EventAlreadyConsumed(String),
}

pub struct ClEvent {
    object: cl_event,
    _unconstructable: (),
}

impl ClEvent {
    pub unsafe fn unchecked_new(evt: cl_event) -> ClEvent {
        ClEvent {
            object: evt,
            _unconstructable: (),
        }
    }

    pub unsafe fn new(evt: cl_event) -> Output<ClEvent> {
        utils::null_check(evt)?;
        Ok(ClEvent::unchecked_new(evt))
    }
}

impl fmt::Debug for ClEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClEvent{{{:?}}}", self.object)
    }
}

impl ClEvent {
    pub fn time(&self, info: ProfilingInfo) -> Output<u64> {
        unsafe { cl_get_event_profiling_info(self.event_ptr(), info.into()) }
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

    pub fn profiling(&self) -> Profiling {
        Profiling {
            submit_time: self.submit_time().ok(),
            queue_time: self.queue_time().ok(),
            start_time: self.start_time().ok(),
            end_time: self.end_time().ok(),
        }
    }

    pub unsafe fn info<T: Copy>(&self, flag: EventInfo) -> Output<ClPointer<T>> {
        cl_get_event_info::<T>(self.event_ptr(), flag.into())
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe {
            self.info(EventInfo::ReferenceCount)
                .map(|ret| ret.into_one())
        }
    }

    pub unsafe fn cl_command_queue(&self) -> Output<cl_command_queue> {
        self.info(EventInfo::CommandQueue)
            .map(|cl_ptr| cl_ptr.into_one())
    }

    pub unsafe fn command_queue(&self) -> Output<ClCommandQueue> {
        self.cl_command_queue()
            .and_then(|cq| ClCommandQueue::retain_new(cq))
    }

    pub unsafe fn cl_context(&self) -> Output<cl_context> {
        self.info(EventInfo::Context)
            .map(|cl_ptr| cl_ptr.into_one())
    }

    pub unsafe fn context(&self) -> Output<ClContext> {
        self.cl_context().and_then(|ctx| ClContext::retain_new(ctx))
    }

    pub fn command_execution_status(&self) -> Output<CommandExecutionStatus> {
        unsafe {
            self.info(EventInfo::CommandExecutionStatus)
                .map(|ret| ret.into_one())
        }
    }
}

impl Clone for ClEvent {
    fn clone(&self) -> ClEvent {
        unsafe {
            let evt = self.object;
            retain_event(evt);
            ClEvent::unchecked_new(evt)
        }
    }
}

impl Drop for ClEvent {
    fn drop(&mut self) {
        unsafe {
            release_event(self.object);
        }
    }
}

pub struct Profiling {
    pub queue_time: Option<u64>,
    pub submit_time: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

impl Profiling {
    pub fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_nanos(self.end_time? - self.queue_time?))
    }

    pub fn duration_waiting_in_queue(&self) -> Option<Duration> {
        Some(Duration::from_nanos(self.submit_time? - self.queue_time?))
    }

    pub fn duration_between_submit_and_start(&self) -> Option<Duration> {
        Some(Duration::from_nanos(self.submit_time? - self.queue_time?))
    }
    pub fn duration_of_execution(&self) -> Option<Duration> {
        Some(Duration::from_nanos(self.submit_time? - self.queue_time?))
    }
}

pub struct BufferReadEvent<T: ClNumber> {
    event: ManuallyDrop<ClEvent>,
    host_buffer: ManuallyDrop<Option<Vec<T>>>,
    is_consumed: bool,
}

impl<T: ClNumber> BufferReadEvent<T> {
    pub fn new(event: ClEvent, host_buffer: Option<Vec<T>>) -> BufferReadEvent<T> {
        BufferReadEvent {
            event: ManuallyDrop::new(event),
            host_buffer: ManuallyDrop::new(host_buffer),
            is_consumed: false,
        }
    }

    pub fn wait(&mut self) -> Output<Option<Vec<T>>> {
        if self.is_consumed {
            return Err(Error::EventError(EventError::EventAlreadyConsumed(
                self.event.address(),
            )));
        }
        unsafe {
            self.event.wait()?;
            match *self.host_buffer {
                Some(_) => {
                    let mut output = Some(vec![]);
                    std::mem::swap(&mut *self.host_buffer, &mut output);
                    self.is_consumed = true;
                    Ok(output)
                }
                None => Ok(None),
            }
        }
    }
}

impl<T: ClNumber> Drop for BufferReadEvent<T> {
    fn drop(&mut self) {
        unsafe {
            self.event.wait().unwrap();
            ManuallyDrop::drop(&mut self.event);
            ManuallyDrop::drop(&mut self.host_buffer);
        }
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
    event: ClEvent,
    _unconstructable: (),
}

impl CompleteEvent {
    pub fn new(event: ClEvent) -> CompleteEvent {
        CompleteEvent {
            event,
            _unconstructable: (),
        }
    }

    pub unsafe fn inner(&self) -> &ClEvent {
        &self.event
    }
}

impl PartialEq for ClEvent {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.object, other.object)
    }
}

impl Eq for ClEvent {}

#[cfg(test)]
mod tests {
    use crate::{
        BufferCreator, ClCommandQueue, ClContext, ClEvent, ClKernel, ClMem, CommandExecutionStatus,
        Session, SessionBuilder, Work,
    };
    use std::time::Duration;

    const SRC: &'static str = "
    __kernel void add_one(__global uint *data) {
        data[get_global_id(0)] += 1;
    }
    ";

    fn get_event() -> (Session, ClEvent) {
        unsafe {
            let mut session: Session = SessionBuilder::new().with_program_src(SRC).build().unwrap();
            let mut kernel =
                ClKernel::create(session.program(), "add_one").expect("Failed to Kernel::create/2");
            let input_data: Vec<u64> = vec![1, 2, 3];
            let data = &input_data[..];
            let mem_cfg = data.mem_config();
            let mut mem_buffer: ClMem =
                ClMem::create_with_config(session.context(), data, mem_cfg)
                    .unwrap_or_else(|e| panic!("Failed to ClMem::create_with_config() {:?}", e));
            let () = kernel
                .set_arg(0, &mut mem_buffer)
                .expect("Failed to set_arg(0, &mem_buffer)");
            let work = Work::new(input_data.len());
            let event = session
                .enqueue_kernel(0, &mut kernel, &work, None)
                .unwrap_or_else(|e| panic!("Failed to sync_enqueue_kernel {:?}", e));
            (session, event)
        }
    }

    #[test]
    fn event_method_queue_time_works() {
        let (_sess, event) = get_event();
        let output = event
            .queue_time()
            .expect("Failed to call event.queue_time()");
        assert!(output > 0);
    }

    #[test]
    fn event_method_submit_time_works() {
        let (_sess, event) = get_event();
        let output = event
            .submit_time()
            .expect("Failed to call event.submit_time()");
        assert!(output > 0);
    }

    #[test]
    fn event_method_start_time_works() {
        let (_sess, event) = get_event();
        let output = event
            .start_time()
            .expect("Failed to call event.start_time()");
        assert!(output > 0);
    }

    #[test]
    fn event_method_end_time_works() {
        let (_sess, event) = get_event();
        let output = event.end_time().expect("Failed to call event.end_time()");
        assert!(output > 0);
    }

    #[test]
    fn event_method_reference_count_works() {
        let (_sess, event) = get_event();
        let output = event
            .reference_count()
            .expect("Failed to call event.reference_count()");
        assert_eq!(output, 1);
    }

    #[test]
    fn event_method_command_queue_works() {
        let (_sess, event) = get_event();
        let _output: ClCommandQueue =
            unsafe { event.command_queue() }.expect("Failed to call event.command_queue()");
    }

    #[test]
    fn event_method_context_works() {
        let (_sess, event) = get_event();
        let _output: ClContext =
            unsafe { event.context() }.expect("Failed to call event.context()");
    }

    #[test]
    fn event_method_command_execution_status_works() {
        let (_sess, event) = get_event();
        let _output: CommandExecutionStatus = event
            .command_execution_status()
            .expect("Failed to call event.command_exection_status()");
    }

    #[test]
    fn event_profiling_works() {
        let (_sess, event) = get_event();
        let exec_status: CommandExecutionStatus = event
            .command_execution_status()
            .expect("Failed to call event.command_exection_status()");
        assert_eq!(exec_status, CommandExecutionStatus::Complete);
        let prof = event.profiling();
        let submitted_at = prof.submit_time.unwrap();
        let queued_at = prof.queue_time.unwrap();
        let started_at = prof.start_time.unwrap();
        let ended_at = prof.end_time.unwrap();
        assert!(queued_at < submitted_at);
        assert!(queued_at < started_at);
        assert!(started_at < ended_at);

        let total = prof.total_duration().unwrap();
        let max_duration = Duration::from_millis(10);
        assert!(
            total < max_duration,
            "total {:?} was greater than max duration {:?}",
            total,
            max_duration
        );

        let duration_waiting_in_queue = prof.duration_waiting_in_queue().unwrap();
        let max_duration_waiting_in_queue = Duration::from_millis(5);
        assert!(
            duration_waiting_in_queue < max_duration_waiting_in_queue,
            "duration_waiting_in_queue {:?} was greater than max duration {:?}",
            duration_waiting_in_queue,
            max_duration_waiting_in_queue
        );

        let duration_waiting_for_init = prof.duration_between_submit_and_start().unwrap();
        let max_duration_waiting_for_init = Duration::from_millis(5);
        assert!(
            duration_waiting_for_init < max_duration_waiting_for_init,
            "duration_waiting_for_init {:?} was greater than max duration {:?}",
            duration_waiting_for_init,
            max_duration_waiting_in_queue
        );

        let duration_of_execution = prof.duration_of_execution().unwrap();
        let max_duration_of_execution = Duration::from_millis(5);
        assert!(
            duration_of_execution < max_duration_of_execution,
            "time_waiting_for_init {:?} was greater than max duration {:?}",
            duration_of_execution,
            max_duration_of_execution
        );
    }
}
