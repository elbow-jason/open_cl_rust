use std::mem::ManuallyDrop;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    Buffer, BufferCreator, CommandQueueOptions, Context, Device, DeviceType, Kernel, KernelOpArg,
    KernelOperation, MemConfig, MutVecOrSlice, Output, Program, VecOrSlice, Waitlist,
    Work, NumberTyped,
};

use crate::ll::{
    list_devices_by_type, list_platforms, BufferReadEvent, ClCommandQueue, ClContext, ClDeviceID,
    ClEvent, ClKernel, ClMem, ClNumber, ClProgram, CommandQueueProperties, CommandQueuePtr,
    DevicePtr, KernelArg,
};

#[derive(Debug)]
pub struct Session {
    _device: ManuallyDrop<ClDeviceID>,
    _program: ManuallyDrop<ClProgram>,
    _context: ManuallyDrop<ClContext>,
    _queue: ManuallyDrop<Arc<RwLock<ClCommandQueue>>>,
    _unconstructable: (),
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn create_with_devices<'a, D>(
        devices: D,
        src: &str,
        cq_props: Option<CommandQueueProperties>,
    ) -> Output<Vec<Session>>
    where
        D: Into<VecOrSlice<'a, Device>>,
    {
        let devices: Vec<Device> = devices.into().to_vec();
        unsafe {
            let context = ClContext::create(devices.as_slice())?;
            let mut sessions: Vec<Session> = Vec::with_capacity(devices.len());
            for device in devices.iter() {
                let device = ClDeviceID::unchecked_new(device.device_ptr());
                let mut program = ClProgram::create_with_source(&context, src)?;
                program.build(devices.as_slice())?;

                let queue = ClCommandQueue::create(&context, &device, cq_props)?;
                let session = Session {
                    _device: ManuallyDrop::new(device),
                    _context: ManuallyDrop::new(context.clone()),
                    _program: ManuallyDrop::new(program.clone()),
                    _queue: ManuallyDrop::new(Arc::new(RwLock::new(queue))),
                    _unconstructable: (),
                };
                sessions.push(session);
            }
            Ok(sessions)
        }
    }

    pub fn create(src: &str, cq_props: Option<CommandQueueProperties>) -> Output<Vec<Session>> {
        let platforms = list_platforms()?;
        let mut devices: Vec<Device> = Vec::new();
        for platform in platforms.iter() {
            let platform_devices: Vec<Device> = list_devices_by_type(platform, DeviceType::ALL)
                .map(|ll_devices| ll_devices.into_iter().map(|d| Device::new(d)).collect())?;
            devices.extend(platform_devices);
        }
        Session::create_with_devices(devices, src, cq_props)
    }

    pub fn context(&self) -> Context {
        Context::from_low_level_context(self.low_level_context()).unwrap()
    }

    pub fn device(&self) -> Device {
        Device::new(self.low_level_device().clone())
    }

    pub fn program(&self) -> Program {
        unsafe { Program::from_low_level_program(self.low_level_program()).unwrap() }
    }

    pub fn read_queue(&self) -> RwLockReadGuard<ClCommandQueue> {
        self._queue.read().unwrap()
    }

    pub fn write_queue(&self) -> RwLockWriteGuard<ClCommandQueue> {
        self._queue.write().unwrap()
    }

    pub fn low_level_device(&self) -> &ClDeviceID {
        &*self._device
    }

    pub fn low_level_context(&self) -> &ClContext {
        &self._context
    }

    pub fn low_level_program(&self) -> &ClProgram {
        &self._program
    }

    pub fn create_copy(&self) -> Output<Session> {
        let cloned_device = self._device.clone();
        let cloned_context = self._context.clone();
        let cloned_program = self._program.clone();
        let ll_queue = self._queue.read().unwrap();
        let copied_queue = unsafe { ll_queue.create_copy()? };

        Ok(Session {
            _device: cloned_device,
            _context: cloned_context,
            _program: cloned_program,
            _queue: ManuallyDrop::new(Arc::new(RwLock::new(copied_queue))),
            _unconstructable: (),
        })
    }

    /// Creates a ClKernel from the session's program.
    pub fn create_kernel(&self, kernel_name: &str) -> Output<Kernel> {
        unsafe {
            let ll_kernel = ClKernel::create(self.low_level_program(), kernel_name)?;
            Ok(Kernel::new(ll_kernel, self.program()))
        }
    }

    /// Creates a ClMem object in the given context, with the given buffer creator
    /// (either a length or some data). This function uses the BufferCreator's implementation
    /// to retrieve the appropriate MemConfig.
    pub fn create_buffer<T: ClNumber, B: BufferCreator<T>>(
        &self,
        buffer_creator: B,
    ) -> Output<Buffer> {
        let cfg = buffer_creator.mem_config();
        Buffer::create_from_low_level_context(
            self.low_level_context(),
            buffer_creator,
            cfg.host_access,
            cfg.kernel_access,
            cfg.mem_location,
        )
    }

    /// Creates a ClMem object in the given context, with the given buffer creator
    /// (either a length or some data) and a given MemConfig.
    pub fn create_buffer_with_config<T: ClNumber, B: BufferCreator<T>>(
        &self,
        buffer_creator: B,
        mem_config: MemConfig,
    ) -> Output<Buffer> {
        Buffer::create_from_low_level_context(
            self.low_level_context(),
            buffer_creator,
            mem_config.host_access,
            mem_config.kernel_access,
            mem_config.mem_location,
        )
    }

    /// This function copies data from the host buffer into the device mem buffer. The host
    /// buffer must be a mutable slice or a vector to ensure the safety of the read_Buffer
    /// operation.
    pub fn sync_write_buffer<'a, T: ClNumber, H: Into<VecOrSlice<'a, T>>>(
        &self,
        buffer: &Buffer,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        buffer.number_type().type_check(T::number_type())?;
        let mut queue = self.write_queue();
        let mut buffer_lock = buffer.write_lock();
        unsafe {
            let event: ClEvent = queue.write_buffer(&mut (*buffer_lock), host_buffer, opts)?;
            event.wait()
        }
    }

    /// This function copies data from a device mem buffer into a host buffer. The host
    /// buffer must be a mutable slice or a vector. For the moment the device mem must also
    /// be passed as mutable; I don't trust OpenCL.
    pub fn sync_read_buffer<'a, T: ClNumber, H: Into<MutVecOrSlice<'a, T>>>(
        &self,
        buffer: &Buffer,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<Option<Vec<T>>> {
        buffer.number_type().type_check(T::number_type())?;
        let mut queue = self.write_queue();

        let buffer_lock = buffer.read_lock();
        unsafe {
            let mut event: BufferReadEvent<T> =
                queue.read_buffer(&(*buffer_lock), host_buffer, opts)?;
            event.wait()
        }
    }

    /// This function enqueues a CLKernel into a command queue
    ///
    /// # Safety
    /// If the ClKernel is not in a usable state or any of the Kernel's dependent object
    /// has been release, or the kernel belongs to a different session, or the ClKernel's
    /// pointer is a null pointer, then calling this function will cause undefined behavior.
    pub fn sync_enqueue_kernel(
        &self,
        kernel: &Kernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        let mut queue = self.write_queue();
        let mut kernel_lock = kernel.write_lock();
        unsafe {
            let event = queue.enqueue_kernel(&mut (*kernel_lock), work, opts)?;
            event.wait()
        }
    }

    pub fn execute_sync_kernel_operation<'a, T>(
        &self,
        mut kernel_op: KernelOperation<'a, T>,
    ) -> Output<()>
    where
        T: ClNumber + KernelArg,
    {
        unsafe {
            let kernel = self.create_kernel(kernel_op.name())?;
            let work = kernel_op.work()?;
            let command_queue_opts = kernel_op.command_queue_opts();
            let mut mem_locks: Vec<RwLockWriteGuard<ClMem>> = Vec::new();
            for (arg_index, arg) in kernel_op.mut_args().iter_mut().enumerate() {
                match arg {
                    KernelOpArg::Num(ref mut num) => kernel.set_arg(arg_index, num)?,
                    KernelOpArg::Buffer(ref buffer) => {
                        let mut mem = buffer.write_lock();
                        kernel.set_arg(arg_index, &mut *mem)?;
                        mem_locks.push(mem);
                    }
                }
            }

            let mut queue = self.write_queue();
            let mut ll_kernel = kernel.write_lock();
            let event = queue.enqueue_kernel(&mut ll_kernel, &work, command_queue_opts)?;
            // Wait until queued mems finish being accessed.
            event.wait()?;
            // then drop locks.
            std::mem::drop(mem_locks);
            Ok(())
        }
    }
}

impl Clone for Session {
    fn clone(&self) -> Session {
        Session {
            _device: self._device.clone(),
            _context: self._context.clone(),
            _program: self._program.clone(),
            _queue: self._queue.clone(),
            _unconstructable: (),
        }
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let self_queue_ptr = self.read_queue().command_queue_ptr();
            let other_queue_ptr = other.read_queue().command_queue_ptr();
            std::ptr::eq(self_queue_ptr, other_queue_ptr)
        }
    }
}

impl Eq for Session {}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self._queue);
            ManuallyDrop::drop(&mut self._program);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._device);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing, Buffer, Kernel, Session, Work};

    const SRC: &'static str = "__kernel void test(__global int *data) {
        data[get_global_id(0)] += 1;
    }";

    fn new_session() -> Session {
        testing::get_session(SRC)
    }

    #[test]
    fn session_can_be_created_with_src() {
        let _session = Session::create(SRC, None).unwrap_or_else(|e| {
            panic!("Failed to create session: {:?}", e);
        });
    }

    #[test]
    fn session_can_be_created_with_src_and_slice_of_devices() {
        let devices = testing::get_all_devices();
        assert_ne!(devices.len(), 0);
        let _session = Session::create_with_devices(&devices[..], SRC, None).unwrap_or_else(|e| {
            panic!("Failed to create session with slice of devices: {:?}", e);
        });
    }

    #[test]
    fn session_can_be_created_with_src_and_vec_of_devices() {
        let devices = testing::get_all_devices();
        assert_ne!(devices.len(), 0);
        let _session = Session::create_with_devices(devices, SRC, None).unwrap_or_else(|e| {
            panic!("Failed to create session with vec of devices: {:?}", e);
        });
    }

    #[test]
    fn session_implements_clone() {
        let _other: Session = new_session().clone();
    }

    #[test]
    fn session_implementation_of_fmt_debug_works() {
        let session = new_session();
        let formatted = format!("{:?}", session);
        assert!(
            formatted.starts_with("Session"),
            "Formatted did not start with the correct value. Got: {:?}",
            formatted
        );
    }

    #[test]
    fn session_create_copy_copies_command_queue_and_clones_the_rest() {
        let session = new_session();

        let session_copy = session.create_copy().unwrap_or_else(|e| {
            panic!("Failed to create_copy of session: {:?}", e);
        });
        let s1_queue = session.read_queue();
        let s2_queue = session_copy.read_queue();
        assert_ne!(*s1_queue, *s2_queue);
        assert_eq!(
            session.low_level_context(),
            session_copy.low_level_context()
        );
        assert_eq!(
            session.low_level_program(),
            session_copy.low_level_program()
        );
        assert_ne!(session, session_copy);
    }

    #[test]
    fn session_can_create_kernel() {
        let src = "__kernel void add_one_i32(__global int *i) { *i += 1; }";
        let session = testing::get_session(src);
        let _kernel: Kernel = session.create_kernel("add_one_i32").unwrap_or_else(|e| {
            panic!("Failed to create kernel for session: {:?}", e);
        });
    }

    #[test]
    fn session_can_create_buffer_from_data() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let session = new_session();
        let _buffer: Buffer = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
    }

    #[test]
    fn session_can_create_buffer_of_a_given_length() {
        let session = new_session();
        let buffer: Buffer = session
            .create_buffer::<i32, usize>(100)
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 100);
    }

    #[test]
    fn session_can_write_and_read_buffer() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let session = new_session();
        let buffer: Buffer = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 8);
        let () = session
            .sync_write_buffer(&buffer, &data[..], None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            });
        let data2 = vec![0i32; 8];
        let data3 = session
            .sync_read_buffer(&buffer, data2, None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            })
            .unwrap();

        assert_eq!(data3.len(), 8);
        assert_eq!(data3, data);
    }

    #[test]
    fn session_sync_enqueue_kernel_and_read_buffer() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let session = new_session();
        let buffer: Buffer = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 8);
        let () = session
            .sync_write_buffer(&buffer, &data[..], None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            });
        let kernel: Kernel = session.create_kernel("test").unwrap();
        let mut buffer_lock = buffer.write_lock();
        unsafe { kernel.set_arg(0, &mut (*buffer_lock)).unwrap() };
        let work = Work::new(data.len());
        session.sync_enqueue_kernel(&kernel, &work, None).unwrap();
        std::mem::drop(buffer_lock);

        let data2 = vec![0i32; 8];
        let data3 = session
            .sync_read_buffer(&buffer, data2, None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            })
            .unwrap();

        assert_eq!(data3.len(), 8);
        let expected_data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(data3, expected_data);
    }
}
