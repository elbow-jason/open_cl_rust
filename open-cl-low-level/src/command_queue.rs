use crate::ffi::{
    clCreateCommandQueue, clEnqueueNDRangeKernel, clEnqueueReadBuffer, clEnqueueWriteBuffer,
    clFinish, clGetCommandQueueInfo, cl_bool, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_context, cl_device_id, cl_event, cl_kernel, cl_mem
};

use crate::cl_helpers::cl_get_info5;
use crate::CommandQueueInfo as CQInfo;
use crate::{
    build_output, BufferReadEvent, ClContext, ClDeviceID, ClEvent, ClKernel, ClMem,
    ClNumber, ClPointer, CommandQueueInfo, CommandQueueProperties, ContextPtr, DevicePtr,
    EventPtr, GlobalWorkSize, KernelPtr, LocalWorkSize, MemPtr, MutVecOrSlice, Output,
    VecOrSlice, Waitlist, WaitlistSizeAndPtr, Work, BufferCreator, ObjectWrapper,
};

#[derive(Debug, Clone)]
pub struct CommandQueueOptions {
    pub is_blocking: bool,
    pub offset: usize,
    pub waitlist: Vec<ClEvent>,
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
}

/// Creates a new cl_command_queue.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn cl_create_command_queue(
    context: cl_context,
    device: cl_device_id,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    device.usability_check()?;
    let mut err_code = 0;
    let command_queue = clCreateCommandQueue(context, device, flags, &mut err_code);
    build_output(command_queue, err_code)
}

/// Blocks until all previously queued tasks are finished.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn cl_finish(command_queue: cl_command_queue) -> Output<()> {
    build_output((), clFinish(command_queue))
}

/// Queues an n-dimensionally ranged kernel to be executed.
///
/// Blocks until the kernel is finished.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn cl_enqueue_nd_range_kernel<W: Waitlist>(
    queue: cl_command_queue,
    kernel: cl_kernel,
    work: &Work,
    waitlist: W,
) -> Output<cl_event> {
    let mut tracking_event: cl_event = new_tracking_event();
    let event_waitlist = waitlist.new_waitlist();
    let wl = event_waitlist.as_slice();

    let gws: GlobalWorkSize = work.global_work_size()?;
    let lws: LocalWorkSize = work.local_work_size()?;
    let err_code = clEnqueueNDRangeKernel(
        queue,
        kernel,
        work.work_dims(),
        work.global_work_offset().as_ptr(),
        gws.as_ptr(),
        lws.as_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );

    build_output((), err_code)?;
    cl_finish(queue)?;

    // TODO: Remove this check when Event checks for null pointer
    debug_assert!(!tracking_event.is_null());
    Ok(tracking_event)
}

fn new_tracking_event() -> cl_event {
    std::ptr::null_mut() as cl_event
}

pub unsafe fn cl_enqueue_read_buffer<T>(
    queue: cl_command_queue,
    mem: cl_mem,
    buffer: &mut [T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event>
where
    T: ClNumber,
{
    let mut tracking_event = new_tracking_event();
    let waitlist = command_queue_opts.new_waitlist();
    let wl = waitlist.as_slice();

    // TODO: Make this a Error returning check
    // debug_assert!(buffer.len() == device_mem.len());

    let err_code = clEnqueueReadBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer.buffer_byte_size(),
        buffer.buffer_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );
    build_output(tracking_event, err_code)
}

pub unsafe fn cl_enqueue_write_buffer<T: ClNumber>(
    queue: cl_command_queue,
    mem: cl_mem,
    buffer: &[T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event> {
    let mut tracking_event = new_tracking_event();

    let waitlist = command_queue_opts.new_waitlist();
    let wl = waitlist.as_slice();
    
    let err_code = clEnqueueWriteBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer.buffer_byte_size(),
        buffer.buffer_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );

    build_output(tracking_event, err_code)
}

pub unsafe fn cl_get_command_queue_info<T: Copy>(
    command_queue: cl_command_queue,
    flag: CommandQueueInfo,
) -> Output<ClPointer<T>> {
    cl_get_info5(
        command_queue,
        flag as cl_command_queue_info,
        clGetCommandQueueInfo,
    )
}

pub unsafe trait CommandQueuePtr: Sized {
    unsafe fn command_queue_ptr(&self) -> cl_command_queue;

    fn address(&self) -> String {
        format!("{:?}", unsafe { self.command_queue_ptr() })
    }

    unsafe fn info<T: Copy>(&self, flag: CQInfo) -> Output<ClPointer<T>> {
        cl_get_command_queue_info(self.command_queue_ptr(), flag.into())
    }

    unsafe fn cl_context(&self) -> Output<cl_context> {
        self.info(CQInfo::Context).map(|cl_ptr| cl_ptr.into_one())
    }

    unsafe fn context(&self) -> Output<ClContext> {
        self.cl_context().and_then(|obj| ClContext::retain_new(obj))
    }

    unsafe fn cl_device_id(&self) -> Output<cl_device_id> {
        self.info(CQInfo::Device).map(|cl_ptr| cl_ptr.into_one())
    }

    unsafe fn device(&self) -> Output<ClDeviceID> {
        self.cl_device_id()
            .and_then(|obj| ClDeviceID::retain_new(obj))
    }

    unsafe fn reference_count(&self) -> Output<u32> {
        self.info(CQInfo::ReferenceCount).map(|ret| ret.into_one())
    }

    unsafe fn cl_command_queue_properties(&self) -> Output<cl_command_queue_properties> {
        self.info::<cl_command_queue_properties>(CQInfo::Properties)
            .map(|ret| ret.into_one())
    }

    unsafe fn properties(&self) -> Output<CommandQueueProperties> {
        self.cl_command_queue_properties().map(|props| {
            CommandQueueProperties::from_bits(props).unwrap_or_else(|| {
                panic!("Failed to convert cl_command_queue_properties");
            })
        })
    }
}

unsafe impl CommandQueuePtr for ObjectWrapper<cl_command_queue> {
    unsafe fn command_queue_ptr(&self) -> cl_command_queue {
        self.cl_object()
    }
}

pub type ClCommandQueue = ObjectWrapper<cl_command_queue>;

impl ObjectWrapper<cl_command_queue> {
    /// Create a new ClCommandQueue in the given ClContext on the given
    /// ClDeviceID with the given CommandQueueProperties (optional).
    ///
    /// # Safety
    /// Calling this function with an invalid ClContext or ClDeviceID
    /// is undefined behavior.
    pub unsafe fn create(
        context: &ClContext,
        device: &ClDeviceID,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<ClCommandQueue> {
        let properties = match opt_props {
            None => CommandQueueProperties::PROFILING_ENABLE,
            Some(prop) => prop,
        };
        ClCommandQueue::create_from_raw_pointers(
            context.context_ptr(),
            device.device_ptr(),
            properties.bits() as cl_command_queue_properties,
        )
    }

    /// Creates a ClCommandQueue from raw ClObject pointers.
    ///
    /// # Safety
    /// Passing an invalid ClObject is undefined behavior.
    pub unsafe fn create_from_raw_pointers(
        context: cl_context,
        device: cl_device_id,
        props: cl_command_queue_properties,
    ) -> Output<ClCommandQueue> {
        let cq_object = cl_create_command_queue(context, device, props)?;
        ClCommandQueue::new(cq_object)
    }

    /// Creates a copy of a ClCommandQueue. The copy is, in fact, a completely differnt
    /// ClCommandQueue that has the same cl_context and cl_device_id as the original.
    ///
    /// # Safety
    /// Calling this function on an invalid ClCommandQueue is undefined behavior.
    pub unsafe fn create_copy(&self) -> Output<ClCommandQueue> {
        let context = self.cl_context()?;
        let device = self.cl_device_id()?;
        let props = self.cl_command_queue_properties()?;
        ClCommandQueue::create_from_raw_pointers(context, device, props)
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the mutable OpenCL cl_mem pointer.
    pub unsafe fn write_buffer<'a, T: ClNumber, H: Into<VecOrSlice<'a, T>>>(
        &mut self,
        mem: &mut ClMem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        match host_buffer.into() {
            VecOrSlice::Slice(hb) => self.write_buffer_from_slice(mem, hb, opts),
            VecOrSlice::Vec(hb) => self.write_buffer_from_slice(mem, &hb[..], opts),
        }
    }

    /// Copies data to a ClMem buffer from a host slice of T.
    unsafe fn write_buffer_from_slice<'a, T: ClNumber>(
        &mut self,
        mem: &mut ClMem,
        host_buffer: &[T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        let event = cl_enqueue_write_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            host_buffer,
            opts.into(),
        )?;
        ClEvent::new(event)
    }

    /// Copies data from a ClMem<T> buffer to a &mut [T] or mut Vec<T>.
    pub unsafe fn read_buffer<'a, T: ClNumber, H: Into<MutVecOrSlice<'a, T>>>(
        &mut self,
        mem: &ClMem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<BufferReadEvent<T>> {
        match host_buffer.into() {
            MutVecOrSlice::Slice(slc) => {
                let event = self.read_buffer_into_slice(mem, slc, opts)?;
                Ok(BufferReadEvent::new(event, None))
            }
            MutVecOrSlice::Vec(mut hb) => {
                let event = self.read_buffer_into_slice(mem, &mut hb[..], opts)?;
                Ok(BufferReadEvent::new(event, Some(hb)))
            }
        }
    }

    /// Copies data from a ClMem<T> buffer to a &mut [T].
    unsafe fn read_buffer_into_slice<T: ClNumber>(
        &mut self,
        mem: &ClMem,
        host_buffer: &mut [T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        assert_eq!(mem.len().unwrap(), host_buffer.len());
        let raw_event = cl_enqueue_read_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            host_buffer,
            opts.into(),
        )?;
        ClEvent::new(raw_event)
    }

    /// Enqueues a ClKernel onto a the ClCommandQueue.
    ///
    /// # Safety
    /// Usage of invalid ClObjects is undefined behavior.
    pub unsafe fn enqueue_kernel(
        &mut self,
        kernel: &mut ClKernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        let cq_opts: CommandQueueOptions = opts.into();
        let event = cl_enqueue_nd_range_kernel(
            self.command_queue_ptr(),
            kernel.kernel_ptr(),
            work,
            &cq_opts.waitlist[..],
        )?;
        ClEvent::new(event)
    }

    pub unsafe fn finish(&mut self) -> Output<()> {
        cl_finish(self.cl_object())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    
    #[test]
    fn command_queue_can_be_created() {
        let (context, devices) = ll_testing::get_context();
        for d in devices.iter() {
            let cq = unsafe { ClCommandQueue::create(&context, d, None).unwrap() };
            // ensure the compiler does not optimize away.
            let addr = cq.address();
            assert!(addr.len() > 0);
        }
    }

    #[test]
    fn address_works() {
        let (cqs, _context, _devices) = ll_testing::get_command_queues();
        for cq in cqs.iter() {
            let addr: String = cq.address();
            let expected: String = format!("{:?}", unsafe { cq.command_queue_ptr() });
            assert_eq!(addr, expected);
        }
    }

    #[test]
    fn context_works() {
        let (cqs, context, _devices) = ll_testing::get_command_queues();
        for cq in cqs.iter() {
            let queue_ctx = unsafe { cq.context() }.unwrap();
            assert_eq!(queue_ctx, context);
        }
    }

    #[test]
    fn device_works() {
        let (cqs, _context, devices) = ll_testing::get_command_queues();
        for (cq, device) in cqs.iter().zip(devices.iter()) {
            let queue_device = unsafe { cq.device() }.unwrap();
            assert_eq!(&queue_device, device);
        }
    }

    #[test]
    fn reference_count_works() {
        let (cqs, _context, _devices) = ll_testing::get_command_queues();
        for cq in cqs.iter() {
            let ref_count = unsafe { cq.reference_count() }.unwrap();
            assert_eq!(ref_count, 1);
        }
    }

    #[test]
    fn properties_works() {
        let (cqs, _context, _devices) = ll_testing::get_command_queues();
        for cq in cqs.iter() {
            let props = unsafe { cq.properties() }.unwrap();
            assert_eq!(props, CommandQueueProperties::PROFILING_ENABLE);
        }
    }

    #[test]
    fn create_copy_works() {
        let (cqs, _context, _devices) = ll_testing::get_command_queues();
        for cq in cqs.iter() {
            unsafe {
                let copied_cq = cq.create_copy().unwrap();
                assert_eq!(copied_cq.context().unwrap(), cq.context().unwrap());
                assert_eq!(copied_cq.device().unwrap(), cq.device().unwrap());
                assert_ne!(copied_cq.command_queue_ptr(), cq.command_queue_ptr());
            }
        }
    }

    #[test]
    fn buffer_can_be_written_and_waited() {
        let (mut cqs, context, _devices) = ll_testing::get_command_queues();
        let mut data = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let mut buffer = ll_testing::mem_from_data_and_context::<u8>(&mut data, &context);
        for cq in cqs.iter_mut() {
            unsafe {
                let event = cq.write_buffer(&mut buffer, &data[..], None).unwrap();
                event.wait().unwrap();
            }
        }
    }

    #[test]
    fn buffer_vec_can_be_read_and_waited() {
        let (mut cqs, context, _devices) = ll_testing::get_command_queues();
        let mut data = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let buffer = ll_testing::mem_from_data_and_context(&mut data, &context);
        let data_ref = &data;
        for cq in cqs.iter_mut() {
            unsafe {
                let mut event = cq.read_buffer(&buffer, data_ref.clone(), None).unwrap();
                let data2: Option<Vec<u8>> = event.wait().unwrap();
                assert_eq!(data2, Some(data_ref.clone()));
            }
        }
    }

    #[test]
    fn buffer_slice_can_be_read_and_waited() {
        let (mut cqs, context, _devices) = ll_testing::get_command_queues();
        let mut data = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let buffer = ll_testing::mem_from_data_and_context(&mut data, &context);

        for cq in cqs.iter_mut() {
            unsafe {
                let mut data2 = vec![0u8, 0, 0, 0, 0, 0, 0, 0];
                let mut event = cq.read_buffer(&buffer, &mut data2[..], None).unwrap();
                let data3 = event.wait();
                assert_eq!(data3, Ok(None));
            }
        }
    }
}
