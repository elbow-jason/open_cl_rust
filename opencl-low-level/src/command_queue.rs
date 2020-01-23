use std::fmt;

use libc::c_void;


use crate::ffi::{
    clCreateCommandQueue, clEnqueueNDRangeKernel, clEnqueueReadBuffer, clEnqueueWriteBuffer,
    clFinish, clGetCommandQueueInfo, cl_bool, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_event, cl_kernel, cl_context, cl_device_id, cl_mem,
};

use crate::cl_helpers::cl_get_info5;
use crate::{
    Output, CommandQueueInfo, build_output, DevicePtr, Waitlist, WaitlistSizeAndPtr,
    Volume, ClInput, SizeAndPtr, ClPointer, ClEvent, EventPtr, utils, ClContext,
    ClDeviceID, CommandQueueProperties, ContextPtr, ClMem, ClNumber, MemPtr, ClKernel,
    Work, KernelPtr, BufferReadEvent
};
use crate::CommandQueueInfo as CQInfo;


pub struct CommandQueueOptions {
    pub is_blocking: bool,
    pub offset: usize,
    pub waitlist: Vec<ClEvent>,
}

impl Default for CommandQueueOptions {
    fn default() -> CommandQueueOptions {
        CommandQueueOptions {
            is_blocking: true,
            offset: 0,
            waitlist: vec![],
        }
    }
}

unsafe impl Waitlist for CommandQueueOptions {
    unsafe fn fill_waitlist(&self, waitlist: &mut Vec<cl_event>) {
        waitlist.extend(self.new_waitlist());
    }

    unsafe fn new_waitlist(&self) -> Vec<cl_event> {
        self.waitlist.iter().map(|evt| evt.event_ptr()).collect()
    }
}

unsafe impl<T> ClInput<*const c_void> for &[T] {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<*const c_void> {
        let size = std::mem::size_of::<T>() * self.len();
        let ptr = self.as_ptr() as *const c_void;
        SizeAndPtr(size, ptr)
    }
}

unsafe impl<T> ClInput<*mut c_void> for &mut [T] {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<*mut c_void> {
        let size = std::mem::size_of::<T>() * self.len();
        let ptr = self.as_ptr() as *mut c_void;
        SizeAndPtr(size, ptr)
    }
}

__release_retain!(command_queue, CommandQueue);

pub unsafe fn release_command_queue(cq: cl_command_queue) {
    cl_release_command_queue(cq).unwrap_or_else(|e| {
        panic!("Failed to release cl_command_queue {:?} due to {:?}", cq, e);
    })
}

pub unsafe fn retain_command_queue(cq: cl_command_queue) {
    cl_retain_command_queue(cq).unwrap_or_else(|e| {
        panic!("Failed to retain cl_command_queue {:?} due to {:?}", cq, e);
    })
}


// unsafe fn async_enqueue_kernel_with_opts(
//         cq: cl_command_queue,
//         kernel: cl_kernel,
//         work: &Work,
//         command_queue_opts: CommandQueueOptions,
// ) -> Output<cl_event> {
//     cl_enqueue_nd_range_kernel(
//         cq,
//         kernel,
//         work.work_dim(),
//         work.global_work_offset(),
//         work.global_work_size(),
//         work.local_work_size(),
//         &command_queue_opts.waitlist[..],
//     )
// }

pub unsafe fn cl_create_command_queue(
    context: cl_context,
    device: cl_device_id,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    device.usability_check()?;
    let mut err_code = 0;
    let command_queue = clCreateCommandQueue(
        context,
        device,
        flags,
        &mut err_code,
    );
    build_output(command_queue, err_code)
}

pub unsafe fn cl_finish(command_queue: cl_command_queue) -> Output<()> {
    build_output((), clFinish(command_queue))
}

pub unsafe fn cl_enqueue_nd_range_kernel<W: Waitlist>(
    queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: u8,
    global_work_offset: Option<Volume>,
    global_work_size: Volume,
    local_work_size: Option<Volume>,
    waitlist: W,
) -> Output<cl_event> {
    let mut tracking_event: cl_event = new_tracking_event();
    let waiting_events = waitlist.new_waitlist();
    let SizeAndPtr(wl_len, wl_ptr) = waiting_events
        .as_slice()
        .waitlist_size_and_ptr();

    let global_work_offset_ptr = global_work_offset.map_or_else(|| Volume::empty_ptr(),|g| Volume::from(g).as_ptr());
    let global_work_size_ptr = Volume::from(global_work_size).as_ptr();
    let local_work_size_ptr = local_work_size.map_or_else(|| Volume::empty_ptr(),|g| Volume::from(g).as_ptr());

    let err_code = clEnqueueNDRangeKernel(
        queue,
        kernel,
        u32::from(work_dim),
        global_work_offset_ptr,
        global_work_size_ptr,
        local_work_size_ptr,
        wl_len as u32,
        wl_ptr,
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

// #[inline]
// fn into_event(err_code: cl_int, tracking_event: cl_event) -> Output<cl_event> {
//     build_output(tracking_event, err_code)?;
// }

pub unsafe fn cl_enqueue_read_buffer<T>(
    queue: cl_command_queue,
    mem: cl_mem,
    buffer: &mut [T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event> where T: ClNumber {
    let mut tracking_event = new_tracking_event();
    let waitlist = command_queue_opts.new_waitlist();
    let SizeAndPtr(wl_len, wl_ptr) = waitlist
        .as_slice()
        .waitlist_size_and_ptr();

    let SizeAndPtr(buffer_size, buffer_ptr) = buffer.size_and_ptr();

    // TODO: Make this a Error returning check
    // debug_assert!(buffer.len() == device_mem.len());

    let err_code = clEnqueueReadBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_size,
        buffer_ptr,
        wl_len as u32,
        wl_ptr,
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
    let SizeAndPtr(wl_len, wl_ptr) = waitlist
        .as_slice()
        .waitlist_size_and_ptr();

    let SizeAndPtr(buffer_size, buffer_ptr) = buffer.size_and_ptr();
        
    let err_code = clEnqueueWriteBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_size,
        buffer_ptr,
        wl_len as u32,
        wl_ptr,
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
        cl_get_command_queue_info(
            self.command_queue_ptr(),
            flag.into()
        )
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
        self.cl_device_id().and_then(|obj| ClDeviceID::retain_new(obj))
    }

    unsafe fn reference_count(&self) -> Output<u32> {
        self.info(CQInfo::ReferenceCount).map(|ret| ret.into_one())
    }

    unsafe fn cl_command_queue_properties(&self) -> Output<cl_command_queue_properties> {
        self.info::<cl_command_queue_properties>(CQInfo::Properties)
            .map(|ret| ret.into_one())
    }

    unsafe fn properties(&self) -> Output<CommandQueueProperties> {
        self.cl_command_queue_properties()
            .map(|props| {
                CommandQueueProperties::from_bits(props).unwrap_or_else(|| {
                    panic!("Failed to convert cl_command_queue_properties");
                })
            })
    }
}

pub struct ClCommandQueue {
    object: cl_command_queue,
    _unconstructable: ()
}

impl ClCommandQueue {
    pub unsafe fn new(cq: cl_command_queue) -> Output<ClCommandQueue> {
        utils::null_check(cq)?;
        Ok(ClCommandQueue::unchecked_new(cq))
    }

    pub unsafe fn unchecked_new(object: cl_command_queue) -> ClCommandQueue {
        ClCommandQueue{
            object,
            _unconstructable: ()
        }
    }

    pub unsafe fn retain_new(cq: cl_command_queue) -> Output<ClCommandQueue> {
        utils::null_check(cq)?;
        retain_command_queue(cq);
        Ok(ClCommandQueue::unchecked_new(cq))
    }

    pub unsafe fn create(
        context: &ClContext,
        device: &ClDeviceID,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<ClCommandQueue> {
        let properties = match opt_props {
            None => CommandQueueProperties::PROFILING_ENABLE,
            Some(prop) => prop,
        };
        ClCommandQueue::create_raw(
            context.context_ptr(),
            device.device_ptr(),
            properties.bits() as cl_command_queue_properties,
        )
    }
    
    pub unsafe fn create_raw(context: cl_context, device: cl_device_id, props: cl_command_queue_properties) -> Output<ClCommandQueue> {
        let cq_object = cl_create_command_queue(context, device, props)?;
        ClCommandQueue::new(cq_object)
    }

    pub unsafe fn create_copy(&self) -> Output<ClCommandQueue> {
        let context = self.cl_context()?;
        let device = self.cl_device_id()?;
        let props = self.cl_command_queue_properties()?;
        ClCommandQueue::create_raw(
            context,
            device,
            props
        )
    }


    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the mutable OpenCL cl_mem pointer.
    pub unsafe fn write_buffer<T: ClNumber>(
        &mut self,
        mem: &mut ClMem<T>,
        host_buffer: &[T],
        command_queue_opts: CommandQueueOptions,
    ) -> Output<ClEvent> {
        let event = cl_enqueue_write_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            host_buffer,
            command_queue_opts
        )?;
        ClEvent::new(event)
    }

    pub unsafe fn read_buffer<T: ClNumber>(
        &mut self,
        mem: &ClMem<T>,
        mut host_buffer: Vec<T>,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<BufferReadEvent<T>> {
        assert_eq!(mem.len().unwrap(), host_buffer.len());
        let raw_event = cl_enqueue_read_buffer(
            self.command_queue_ptr(),
            mem.mem_ptr(),
            &mut host_buffer[..],
            command_queue_opts
        )?;
        let event = ClEvent::new(raw_event)?;
        Ok(BufferReadEvent::new(event, host_buffer))
    }

    pub unsafe fn enqueue_kernel(
        &self,
        kernel: &ClKernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<ClEvent> {

        let event = cl_enqueue_nd_range_kernel(
            self.command_queue_ptr(),
            kernel.kernel_ptr(),
            work.work_dim(),
            work.global_work_offset(),
            work.global_work_size(),
            work.local_work_size(),
            &command_queue_opts.waitlist[..],
        )?;
        ClEvent::new(event)
    }
}

unsafe impl CommandQueuePtr for ClCommandQueue {
    unsafe fn command_queue_ptr(&self) -> cl_command_queue {
        self.object
    }
}

impl Drop for ClCommandQueue {
    fn drop(&mut self) {
        unsafe { release_command_queue(self.object) };
    }
}

impl Clone for ClCommandQueue {
    fn clone(&self) -> ClCommandQueue {
        unsafe {
            ClCommandQueue::retain_new(self.object).unwrap()
        }
    }
}

impl fmt::Debug for ClCommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClCommandQueue{{{:?}}}", self.address())
    }
}

impl PartialEq for ClCommandQueue {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.object, other.object)
    }
}

impl Eq for ClCommandQueue {}

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
        let mut buffer = ll_testing::mem_from_data_and_context(&mut data, &context);
        for cq in cqs.iter_mut() {
            unsafe {
                let opts = CommandQueueOptions::default();
                let event = cq.write_buffer(&mut buffer, &data[..], opts).unwrap();
                event.wait().unwrap();
            }
        }
    }

    #[test]
    fn buffer_can_be_read_and_waited() {
        let (mut cqs, context, _devices) = ll_testing::get_command_queues();
        let mut data = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let buffer = ll_testing::mem_from_data_and_context(&mut data, &context);

        for cq in cqs.iter_mut() {
            unsafe {
                let opts = CommandQueueOptions::default();
                let mut event = cq.read_buffer(&buffer, data.clone(), opts).unwrap();
                let data2 = event.wait().unwrap();
                assert_eq!(data2, data)
            }
        }
    }
}