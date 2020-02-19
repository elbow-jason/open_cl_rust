use std::fmt;
use std::mem::ManuallyDrop;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    Buffer, ClNumber, CommandQueueOptions, CommandQueueProperties, Context, Device, Kernel, Output,
    Waitlist, Work, NumberTyped,
};

use crate::ll::{ClCommandQueue, ClContext, ClDeviceID, CommandQueuePtr, ContextPtr, DevicePtr};

pub trait CommandQueueLock<P>
where
    P: CommandQueuePtr,
{
    unsafe fn write_lock(&self) -> RwLockWriteGuard<P>;
    unsafe fn read_lock(&self) -> RwLockReadGuard<P>;
    unsafe fn rw_lock(&self) -> &RwLock<P>;

    fn address(&self) -> String {
        unsafe {
            let read_lock = self.read_lock();
            let ptr = read_lock.command_queue_ptr();
            format!("{:?}", ptr)
        }
    }
}

pub struct CommandQueue {
    _queue: ManuallyDrop<Arc<RwLock<ClCommandQueue>>>,
    _context: ManuallyDrop<ClContext>,
    _device: ManuallyDrop<ClDeviceID>,
    _unconstructable: (),
}

impl CommandQueueLock<ClCommandQueue> for CommandQueue {
    unsafe fn read_lock(&self) -> RwLockReadGuard<ClCommandQueue> {
        (*self._queue).read().unwrap()
    }
    unsafe fn write_lock(&self) -> RwLockWriteGuard<ClCommandQueue> {
        (*self._queue).write().unwrap()
    }
    unsafe fn rw_lock(&self) -> &RwLock<ClCommandQueue> {
        &(*self._queue)
    }
}

impl fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandQueue{{{:?}}}", self.address())
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            debug!("cl_command_queue {:?} - CommandQueue::drop", self.address());
            ManuallyDrop::drop(&mut self._queue);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._device);
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> CommandQueue {
        CommandQueue {
            _queue: ManuallyDrop::new((*self._queue).clone()),
            _context: self._context.clone(),
            _device: self._device.clone(),
            _unconstructable: (),
        }
    }
}

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

impl CommandQueue {
    /// Builds a CommandQueue from a low-level ClCommandQueue, a Context and a Device.
    ///
    /// # Safety
    /// Building a CommandQueue with any invalid ClObject, or mismatched ClObjects is undefined behavior.
    unsafe fn new(queue: ClCommandQueue, context: &Context, device: &Device) -> CommandQueue {
        CommandQueue::new_from_low_level(
            queue,
            context.low_level_context(),
            device.low_level_device(),
        )
    }

    /// Builds a CommandQueue from a low-level ClObjects
    ///
    /// # Safety
    /// Building a CommandQueue with any invalid ClObject, or mismatched ClObjects is undefined behavior.
    unsafe fn new_from_low_level(
        queue: ClCommandQueue,
        context: &ClContext,
        device: &ClDeviceID,
    ) -> CommandQueue {
        CommandQueue {
            _queue: ManuallyDrop::new(Arc::new(RwLock::new(queue))),
            _context: ManuallyDrop::new(context.clone()),
            _device: ManuallyDrop::new(device.clone()),
            _unconstructable: (),
        }
    }

    /// Creates a new CommandQueue with the given Context on the given Device.
    pub fn create(
        context: &Context,
        device: &Device,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<CommandQueue> {
        unsafe {
            let ll_queue = ClCommandQueue::create(
                context.low_level_context(),
                device.low_level_device(),
                opt_props,
            )?;
            Ok(CommandQueue::new(ll_queue, context, device))
        }
    }

    /// Creates a new copy of a CommandQueue with CommandQueue's Context on the CommandQueue's Device.
    ///
    /// This function is useful for executing concurrent operations on a device within the same
    /// Context.
    pub fn create_copy(&self) -> Output<CommandQueue> {
        unsafe {
            let props = self.properties()?;
            let ll_queue = ClCommandQueue::create_from_raw_pointers(
                (*self._context).context_ptr(),
                (*self._device).device_ptr(),
                props.into(),
            )?;
            Ok(CommandQueue::new_from_low_level(
                ll_queue,
                &self._context,
                &self._device,
            ))
        }
    }

    /// The low-level context of the CommandQueue
    pub fn low_level_context(&self) -> ClContext {
        (*self._context).clone()
    }

    pub fn low_level_device(&self) -> ClDeviceID {
        (*self._device).clone()
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &Buffer<T>`.
    pub fn write_buffer<T: ClNumber>(
        &self,
        device_buffer: &Buffer,
        host_buffer: &[T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<()>
    {   

        unsafe {
            let mut qlock = self.write_lock();
            let mut buf_lock = device_buffer.write_lock();
            let event = qlock.write_buffer(&mut *buf_lock, host_buffer, opts.into())?;
            event.wait()
        }
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T: ClNumber>(
        &self,
        device_buffer: &Buffer,
        host_buffer: &mut [T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<Option<Vec<T>>> {
        unsafe {
            device_buffer.number_type().type_check(T::number_type())?;
            let mut qlock = self.write_lock();
            let buf_lock = device_buffer.read_lock();
            let mut event = qlock.read_buffer(&*buf_lock, host_buffer, opts)?;
            event.wait()
        }
    }

    pub fn enqueue_kernel(
        &self,
        kernel: Kernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        unsafe {
            let mut kernel_lock = kernel.write_lock();
            let mut qlock = self.write_lock();
            let event = qlock.enqueue_kernel(&mut (*kernel_lock), work, opts)?;
            event.wait()
        }
    }

    pub fn finish(&self) -> Output<()> {
        unsafe {
            let mut lock = self.write_lock();
            lock.finish()
        }
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.read_lock().reference_count() }
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        unsafe { self.read_lock().properties() }
    }
}

impl PartialEq for CommandQueue {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            std::ptr::eq(
                self.read_lock().command_queue_ptr(),
                other.read_lock().command_queue_ptr(),
            )
        }
    }
}

impl Eq for CommandQueue {}

#[cfg(test)]
mod tests {
    use crate::ll::{ClContext, ClDeviceID, CommandQueueProperties, CommandQueuePtr};
    use crate::testing;

    const SRC: &'static str = "
    __kernel void test(__global int *i) {
        *i += 1;
    }";

    #[test]
    pub fn command_queue_method_context_works() {
        // testing::init_logger();
        let session = testing::get_session(SRC);
        let _context: ClContext = unsafe { session.read_queue().context().unwrap() };
    }

    #[test]
    pub fn command_queue_method_device_works() {
        let session = testing::get_session(SRC);
        let _device: ClDeviceID = unsafe { session.read_queue().device().unwrap() };
    }

    #[test]
    pub fn command_queue_method_reference_count_works() {
        let session = testing::get_session(SRC);
        let ref_count: u32 = unsafe { session.read_queue().reference_count() }
            .expect("CommandQueue method reference_count() failed");
        assert_eq!(ref_count, 1);
    }

    #[test]
    pub fn command_queue_method_properties_works() {
        let session = testing::get_session(SRC);
        let props: CommandQueueProperties = unsafe { session.read_queue().properties() }
            .expect("CommandQueue method properties() failed");
        let bits = props.bits();
        let maybe_same_prop = CommandQueueProperties::from_bits(bits);
        if !maybe_same_prop.is_some() {
            panic!(
                "
                CommandQueue method properties returned \
                an invalid CommandQueueProperties bitflag {:?}\
                ",
                bits
            );
        }
    }

    #[test]
    pub fn command_queue_copy_new_works() {
        let session = testing::get_session(SRC);
        unsafe {
            let cq2 = session.read_queue().create_copy().unwrap();
            assert!(cq2.command_queue_ptr() != session.read_queue().command_queue_ptr());
        }
    }
}
