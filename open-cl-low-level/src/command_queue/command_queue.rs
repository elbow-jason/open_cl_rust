use super::{functions, CommandQueueOptions};
use crate::cl::{
    cl_command_queue, cl_command_queue_properties, cl_context, cl_device_id, CommandQueueInfo,
    CommandQueueProperties, ObjectWrapper,
};
use crate::vec_or_slice::{MutVecOrSlice, VecOrSlice};
use crate::{
    BufferReadEvent, Context, ContextPtr, Device, DevicePtr, Event, Kernel, KernelPtr, Mem, MemPtr,
    Number, Output, Work,
};

pub type CommandQueue = ObjectWrapper<cl_command_queue>;

impl ObjectWrapper<cl_command_queue> {
    /// Create a new CommandQueue in the given Context on the given
    /// Device with the given CommandQueueProperties (optional).
    ///
    /// # Safety
    /// Calling this function with an invalid Context or Device
    /// is undefined behavior.
    pub unsafe fn create(
        context: &Context,
        device: &Device,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<CommandQueue> {
        let properties = match opt_props {
            None => CommandQueueProperties::PROFILING_ENABLE,
            Some(prop) => prop,
        };
        CommandQueue::create_from_raw_pointers(
            context.context_ptr(),
            device.device_ptr(),
            properties.bits() as cl_command_queue_properties,
        )
    }

    /// Creates a CommandQueue from raw ClObject pointers.
    ///
    /// # Safety
    /// Passing an invalid ClObject is undefined behavior.
    pub unsafe fn create_from_raw_pointers(
        context: cl_context,
        device: cl_device_id,
        props: cl_command_queue_properties,
    ) -> Output<CommandQueue> {
        functions::create_command_queue(context, device, props).map(|cq| CommandQueue::new(cq))
    }

    /// Creates a copy of a CommandQueue. The copy is, in fact, a completely differnt
    /// CommandQueue that has the same cl_context and cl_device_id as the original.
    ///
    /// # Safety
    /// Calling this function on an invalid CommandQueue is undefined behavior.
    pub unsafe fn create_copy(&self) -> Output<CommandQueue> {
        CommandQueue::create_from_raw_pointers(
            self.cl_context()?,
            self.cl_device_id()?,
            self.cl_command_queue_properties()?,
        )
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the mutable OpenCL cl_mem pointer.
    pub unsafe fn write_buffer<'a, T, H>(
        &mut self,
        mem: &mut Mem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<Event>
    where
        T: Number,
        H: Into<VecOrSlice<'a, T>>,
    {
        match host_buffer.into() {
            VecOrSlice::Slice(hb) => self.write_buffer_from_slice(mem, hb, opts),
            VecOrSlice::Vec(hb) => self.write_buffer_from_slice(mem, &hb[..], opts),
        }
    }

    /// Copies data to a ClMem buffer from a host slice of T.
    unsafe fn write_buffer_from_slice<'a, T>(
        &mut self,
        mem: &mut Mem,
        host_buffer: &[T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<Event>
    where
        T: Number,
    {
        functions::enqueue_write_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            host_buffer,
            opts.into(),
        )
        .map(|e| Event::new(e))
    }

    /// Copies data from a ClMem<T> buffer to a &mut [T] or mut Vec<T>.
    pub unsafe fn read_buffer<'a, T, H>(
        &mut self,
        mem: &Mem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<BufferReadEvent<T>>
    where
        T: Number,
        H: Into<MutVecOrSlice<'a, T>>,
    {
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
    unsafe fn read_buffer_into_slice<T>(
        &mut self,
        mem: &Mem,
        host_buffer: &mut [T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<Event>
    where
        T: Number,
    {
        assert_eq!(mem.len().unwrap(), host_buffer.len());
        functions::enqueue_read_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            host_buffer,
            opts.into(),
        )
        .map(|e| Event::new(e))
    }

    /// Enqueues a ClKernel onto a the CommandQueue.
    ///
    /// # Safety
    /// Usage of invalid ClObjects is undefined behavior.
    pub unsafe fn enqueue_kernel(
        &mut self,
        kernel: &mut Kernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<Event> {
        let cq_opts: CommandQueueOptions = opts.into();
        let event = functions::enqueue_kernel(
            self.command_queue_ptr(),
            kernel.kernel_ptr(),
            work,
            &cq_opts.waitlist[..],
        )?;
        Ok(Event::new(event))
    }

    pub unsafe fn finish(&mut self) -> Output<()> {
        functions::finish(self.cl_object())
    }
}

pub unsafe trait CommandQueuePtr: Sized {
    unsafe fn command_queue_ptr(&self) -> cl_command_queue;

    fn address(&self) -> String {
        format!("{:?}", unsafe { self.command_queue_ptr() })
    }

    unsafe fn cl_context(&self) -> Output<cl_context> {
        functions::get_context(self.command_queue_ptr())
    }

    unsafe fn context(&self) -> Output<Context> {
        self.cl_context().map(|c| Context::retain_new(c))
    }

    unsafe fn cl_device_id(&self) -> Output<cl_device_id> {
        functions::get_device(self.command_queue_ptr())
    }

    unsafe fn device(&self) -> Output<Device> {
        self.cl_device_id().map(|d| Device::retain_new(d))
    }

    unsafe fn reference_count(&self) -> Output<u32> {
        functions::get_info_u32(
            self.command_queue_ptr(),
            CommandQueueInfo::ReferenceCount.into(),
        )
    }

    unsafe fn cl_command_queue_properties(&self) -> Output<cl_command_queue_properties> {
        functions::get_info_u64(
            self.command_queue_ptr(),
            CommandQueueInfo::Properties.into(),
        )
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

#[cfg(test)]
mod tests {
    use crate::cl::*;
    use crate::*;

    #[test]
    fn command_queue_can_be_created() {
        let (context, devices) = ll_testing::get_context();
        for d in devices.iter() {
            let cq = unsafe { CommandQueue::create(&context, d, None).unwrap() };
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
            let cq_and_addr: String = format!("{:?}", unsafe { cq.command_queue_ptr() });
            assert!(cq_and_addr.contains(&addr[..]));
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
                assert_eq!(data3.unwrap(), None);
            }
        }
    }
}
