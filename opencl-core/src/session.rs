use std::mem::ManuallyDrop;
use std::sync::{RwLock, RwLockReadGuard};

use crate::{
    Buffer, BufferCreator, CommandQueueOptions, Context, Device, DeviceType, Kernel, MemConfig,
    MutVecOrSlice, Output, Program, VecOrSlice, Waitlist, Work
};

use crate::ll::Session as ClSession;
use crate::ll::{
    list_devices_by_type, list_platforms, BufferReadEvent, ClCommandQueue, ClContext, ClDeviceID,
    ClEvent, ClKernel, ClNumber, ClProgram, DevicePtr, KernelArg, KernelOpArg, KernelOperation,
    SessionError,
};

#[derive(Debug)]
pub struct Queues {
    inner: Vec<RwLock<ClCommandQueue>>,
}

unsafe impl Send for Queues {}

impl Queues {
    pub fn new(queues: Vec<RwLock<ClCommandQueue>>) -> Queues {
        Queues { inner: queues }
    }

    pub fn get(&self, index: usize) -> Output<&RwLock<ClCommandQueue>> {
        self.inner
            .get(index)
            .ok_or_else(|| SessionError::QueueIndexOutOfRange(index).into())
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[RwLock<ClCommandQueue>] {
        &self.inner[..]
    }
}


#[derive(Debug)]
pub struct Session {
    _devices: ManuallyDrop<Vec<ClDeviceID>>,
    _program: ManuallyDrop<ClProgram>,
    _context: ManuallyDrop<ClContext>,
    _queues: ManuallyDrop<RwLock<Queues>>,
    _unconstructable: (),
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn create_with_devices<'a, D>(devices: D, src: &str) -> Output<Session>
    where
        D: Into<VecOrSlice<'a, Device>>,
    {
        let devices = devices.into();
        let ll_devices: Vec<ClDeviceID> = devices
            .iter()
            .map(|d| unsafe { ClDeviceID::unchecked_new(d.device_ptr()) })
            .collect();

        let ll_session = ClSession::create_with_devices(ll_devices, src)?;

        // (Vec<ClDeviceID>, ClContext, ClProgram, Vec<ClCommandQueue>)
        let (devices, context, program, queues) = unsafe { ll_session.decompose() };
        let queues_with_locks: Vec<RwLock<ClCommandQueue>> = queues
            .into_iter()
            .map(|q| RwLock::new(q))
            .collect();


        let sess = Session {
            _devices: ManuallyDrop::new(devices),
            _context: ManuallyDrop::new(context),
            _program: ManuallyDrop::new(program),
            _queues: ManuallyDrop::new(RwLock::new(Queues::new(queues_with_locks))),
            _unconstructable: (),
        };
        Ok(sess)
    }

    pub fn create(src: &str) -> Output<Session> {
        let platforms = list_platforms()?;
        let mut devices: Vec<Device> = Vec::new();
        for platform in platforms.iter() {
            let platform_devices: Vec<Device> = list_devices_by_type(platform, DeviceType::ALL)
                .map(|ll_devices| ll_devices.into_iter().map(|d| Device::new(d)).collect())?;
            devices.extend(platform_devices);
        }
        Session::create_with_devices(devices, src)
    }

    pub fn context(&self) -> Context {
        Context::build((*self._context).clone(), self.devices())
    }

    pub fn devices(&self) -> Vec<Device> {
        self._devices
            .iter()
            .map(|d| Device::new(d.clone()))
            .collect()
    }

    pub fn queues(&self) -> RwLockReadGuard<Queues> {
        self._queues.read().unwrap()
    }

    pub fn program(&self) -> Program {
        unsafe { Program::new((*self._program).clone(), self.context(), self.devices()) }
    }

    pub fn low_level_devices(&self) -> &[ClDeviceID] {
        &self._devices[..]
    }

    pub fn low_level_context(&self) -> &ClContext {
        &self._context
    }

    pub fn low_level_program(&self) -> &ClProgram {
        &self._program
    }

    

    pub fn create_copy(&self) -> Output<Session> {
        let cloned_devices = self._devices.clone();
        let cloned_context = self._context.clone();
        let cloned_program = self._program.clone();
        let queues = self._queues.read().unwrap();
        let mut copied_queues = Vec::with_capacity(queues.len());
        for q in queues.inner.iter() {
            let queue_copy = unsafe { q.read().unwrap().create_copy() }?;
            copied_queues.push(RwLock::new(queue_copy));
        }
        Ok(Session {
            _devices: cloned_devices,
            _context: cloned_context,
            _program: cloned_program,
            _queues: ManuallyDrop::new(RwLock::new(Queues::new(copied_queues))),
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
    ) -> Output<Buffer<T>> {
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
    ) -> Output<Buffer<T>> {
        Buffer::create_from_low_level_context(
            self.low_level_context(),
            buffer_creator,
            mem_config.host_access,
            mem_config.kernel_access,
            mem_config.mem_location,
        )
    }

    // #[inline]
    // fn get_queue_by_index(&self, index: usize) -> Output<(RwLockReadGuard<ClCommandQueue>> {
        
    //         .map(|rw_lock| rw_lock.read().unwrap())
    //         .ok_or_else(|| SessionError::QueueIndexOutOfRange(index).into())
            
    // }

    //  #[inline]
    // fn get_queue_by_index_mut(&self, index: usize) -> Output<RwLockWriteGuard<ClCommandQueue>> {
    //     self._queues
    //         .read()
    //         .unwrap()
    //         .get(index)
    //         .ok_or_else(|| SessionError::QueueIndexOutOfRange(index).into())
    //         .map(|rw_lock| rw_lock.write().unwrap())
    // }

    /// This function copies data from the host buffer into the device mem buffer. The host
    /// buffer must be a mutable slice or a vector to ensure the safety of the read_Buffer
    /// operation.
    pub fn sync_write_buffer<'a, T: ClNumber, H: Into<VecOrSlice<'a, T>>>(
        &self,
        queue_index: usize,
        buffer: &Buffer<T>,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        let queues: RwLockReadGuard<Queues> = self.queues();
        let queue_locker: &RwLock<ClCommandQueue> = queues.get(queue_index)?;
        let mut queue = queue_locker.write().unwrap();
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
        queue_index: usize,
        buffer: &Buffer<T>,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<Option<Vec<T>>> {
        let queues: RwLockReadGuard<Queues> = self.queues();
        let queue_locker: &RwLock<ClCommandQueue> = queues.get(queue_index)?;
        let mut queue = queue_locker.write().unwrap();
        
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
        queue_index: usize,
        kernel: &Kernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        let queues: RwLockReadGuard<Queues> = self.queues();
        let queue_locker: &RwLock<ClCommandQueue> = queues.get(queue_index)?;
        let mut queue = queue_locker.write().unwrap();
        let mut kernel_lock = kernel.write_lock();
        unsafe {
            let event = queue.enqueue_kernel(&mut (*kernel_lock), work, opts)?;
            event.wait()
        }
    }

    pub fn execute_sync_kernel_operation<T>(
        &self,
        queue_index: usize,
        mut kernel_op: KernelOperation<T>,
    ) -> Output<Option<KernelOpArg<T>>>
    where
        T: ClNumber + KernelArg,
    {
        unsafe {
            let kernel = self.create_kernel(kernel_op.name())?;

            for (arg_index, arg) in kernel_op.mut_args().iter_mut().enumerate() {
                match arg {
                    KernelOpArg::Num(ref mut num) => kernel.set_arg(arg_index, num)?,
                    KernelOpArg::Mem(ref mut mem) => kernel.set_arg(arg_index, mem)?,
                }
            }
            let work = kernel_op.work()?;
            let queues: RwLockReadGuard<Queues> = self.queues();
            let queue_locker: &RwLock<ClCommandQueue> = queues.get(queue_index)?;
            let mut queue = queue_locker.write().unwrap();
            let mut ll_kernel = kernel.write_lock();
            let event =
                queue.enqueue_kernel(&mut ll_kernel, &work, kernel_op.command_queue_opts())?;
            event.wait()?;
            kernel_op.return_value()
        }
    }
}

impl Clone for Session {
    fn clone(&self) -> Session {
        let cloned_queues: Vec<RwLock<ClCommandQueue>> = self
            ._queues
            .read()
            .unwrap()
            .inner
            .iter()
            .map(|q| RwLock::new(q.read().unwrap().clone()))
            .collect();

        Session {
            _devices: self._devices.clone(),
            _context: self._context.clone(),
            _program: self._program.clone(),
            _queues: ManuallyDrop::new(RwLock::new(Queues::new(cloned_queues))),
            _unconstructable: (),
        }
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        let self_queues = self.queues();
        let other_queues = other.queues();
        // Vecs with the same pointer are the same queues.
        std::ptr::eq(self_queues.inner.as_ptr(), other_queues.inner.as_ptr())
    }
}

impl Eq for Session {}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self._queues);
            ManuallyDrop::drop(&mut self._program);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._devices);
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
        let _session = Session::create(SRC).unwrap_or_else(|e| {
            panic!("Failed to create session: {:?}", e);
        });
    }

    #[test]
    fn session_can_be_created_with_src_and_slice_of_devices() {
        let devices = testing::get_all_devices();
        assert_ne!(devices.len(), 0);
        let _session = Session::create_with_devices(&devices[..], SRC).unwrap_or_else(|e| {
            panic!("Failed to create session with slice of devices: {:?}", e);
        });
    }

    #[test]
    fn session_can_be_created_with_src_and_vec_of_devices() {
        let devices = testing::get_all_devices();
        assert_ne!(devices.len(), 0);
        let _session = Session::create_with_devices(devices, SRC).unwrap_or_else(|e| {
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
        let s1_queues = session.queues();
        let s2_queues = session_copy.queues();

        let zipped_queues = s1_queues.as_slice().iter().zip(s2_queues.as_slice().iter());
        for (orig, copy) in zipped_queues {
            let q_orig = orig.read().unwrap();
            let q_copy = copy.read().unwrap();
            assert_ne!(*q_orig, *q_copy);
        }
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
        let _buffer: Buffer<i32> = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
    }

    #[test]
    fn session_can_create_buffer_of_a_given_length() {
        let session = new_session();
        let buffer: Buffer<i32> = session
            .create_buffer(100)
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 100);
    }

    #[test]
    fn session_can_write_and_read_buffer() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let session = new_session();
        let buffer: Buffer<i32> = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 8);
        let () = session
            .sync_write_buffer(0, &buffer, &data[..], None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            });
        let data2 = vec![0i32; 8];
        let data3 = session
            .sync_read_buffer(0, &buffer, data2, None)
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
        let buffer: Buffer<i32> = session
            .create_buffer(&data[..])
            .unwrap_or_else(|e| panic!("Session failed to create buffer: {:?}", e));
        assert_eq!(buffer.len(), 8);
        let () = session
            .sync_write_buffer(0, &buffer, &data[..], None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            });
        let kernel: Kernel = session.create_kernel("test").unwrap();
        let mut buffer_lock = buffer.write_lock();
        unsafe { kernel.set_arg(0, &mut (*buffer_lock)).unwrap() };
        let work = Work::new(data.len());
        session
            .sync_enqueue_kernel(0, &kernel, &work, None)
            .unwrap();
        std::mem::drop(buffer_lock);

        let data2 = vec![0i32; 8];
        let data3 = session
            .sync_read_buffer(0, &buffer, data2, None)
            .unwrap_or_else(|e| {
                panic!("Failed to write buffer: {:?}", e);
            })
            .unwrap();

        assert_eq!(data3.len(), 8);
        let expected_data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(data3, expected_data);
    }
}
