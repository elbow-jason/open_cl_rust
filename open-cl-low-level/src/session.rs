use std::convert::TryInto;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::numbers::{ClNum, NumberTyped};
use crate::vec_or_slice::{MutVecOrSlice, VecOrSlice};
use crate::{
    cl_enqueue_nd_range_kernel, list_devices_by_type, list_platforms, utils, BufferCreator,
    BufferReadEvent, BuiltClContext, ClCommandQueue, ClContext, ClContextBuilder, ClDeviceID,
    ClEvent, ClKernel, ClMem, ClPlatformID, ClProgram, CommandQueueOptions, CommandQueueProperties,
    CommandQueuePtr, DeviceType, Error, KernelOperation, KernelPtr, MemConfig, Output, Waitlist,
    Work,
};

/// An error related to Session Building.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum SessionError {
    #[fail(display = "The given queue index {} was out of range", _0)]
    QueueIndexOutOfRange(usize),
}

/// Session is the structure that is responsible for Dropping
/// Low-Level OpenCL pointer wrappers in the correct order. Dropping OpenCL
/// pointers in the wrong order can lead to undefined behavior.
#[derive(Debug)]
pub struct Session {
    devices: ManuallyDrop<Vec<ClDeviceID>>,
    context: ManuallyDrop<ClContext>,
    program: ManuallyDrop<ClProgram>,
    queues: ManuallyDrop<Vec<ClCommandQueue>>,
}

impl Session {
    pub fn create_with_devices<'a, D>(devices: D, src: &str) -> Output<Session>
    where
        D: Into<VecOrSlice<'a, ClDeviceID>>,
    {
        unsafe {
            let devices = devices.into();
            let context = ClContext::create(devices.as_slice())?;
            let mut program = ClProgram::create_with_source(&context, src)?;
            program.build(devices.as_slice())?;
            let props = CommandQueueProperties::default();
            let maybe_queues: Result<Vec<ClCommandQueue>, Error> = devices
                .iter()
                .map(|dev| ClCommandQueue::create(&context, dev, Some(props)))
                .collect();

            let queues = maybe_queues?;

            let sess = Session {
                devices: ManuallyDrop::new(devices.to_vec()),
                context: ManuallyDrop::new(context),
                program: ManuallyDrop::new(program),
                queues: ManuallyDrop::new(queues),
            };
            Ok(sess)
        }
    }

    /// Given a string slice of OpenCL source code this function creates a session for
    /// all available platforms and devices. A Session consists of:
    ///
    /// one or more devices
    /// one context (for sharing mem objects between devices)
    /// one program (build on each of the devices)
    /// one or more queues (each queue belongs to exactly one of the devices)
    pub fn create(src: &str) -> Output<Session> {
        let platforms = list_platforms()?;
        let mut devices = Vec::new();
        for platform in platforms.iter() {
            let platform_devices = list_devices_by_type(platform, DeviceType::ALL)?;
            devices.extend(platform_devices);
        }
        Session::create_with_devices(devices, src)
    }

    /// Consumes the session returning the parts as individual parts.
    ///
    /// # Safety
    /// Moving the components of a Session out of the Session can easily lead to
    /// undefined behavior. The Session has a carefully implemented drop that ensures
    /// the an Object is dropped before it's dependencies. If any of the dependencies of an object are ever dropped
    /// in the incorrect order or any dependency of an object is dropped and the object is then used the result is undefined behavior.
    pub unsafe fn decompose(
        mut self,
    ) -> (Vec<ClDeviceID>, ClContext, ClProgram, Vec<ClCommandQueue>) {
        let devices: Vec<ClDeviceID> = utils::take_manually_drop(&mut self.devices);
        let context: ClContext = utils::take_manually_drop(&mut self.context);
        let program: ClProgram = utils::take_manually_drop(&mut self.program);
        let queues: Vec<ClCommandQueue> = utils::take_manually_drop(&mut self.queues);
        std::mem::forget(self);
        (devices, context, program, queues)
    }

    /// A slice of the ClDeviceIDs of this Session.
    pub fn devices(&self) -> &[ClDeviceID] {
        &(*self.devices)[..]
    }

    /// A reference to the ClContext of this Session.
    pub fn context(&self) -> &ClContext {
        &(*self.context)
    }

    /// A reference to the ClProgram of this Session.
    pub fn program(&self) -> &ClProgram {
        &(*self.program)
    }

    /// A slice of the ClCommandQueues of this Session.
    pub fn queues(&self) -> &[ClCommandQueue] {
        &(*self.queues)[..]
    }

    /// Creates a ClKernel from the session's program.
    ///
    /// # Safety
    /// Note: This function may, in fact, be safe. However, creating a kernel with a
    /// program object that is in an invalid state can lead to undefined behavior.
    /// Using the ClKernel after the session has been released can lead to undefined behavior.
    /// Using the ClKernel outside it's own context/program can lead to undefined behavior.
    pub unsafe fn create_kernel(&self, kernel_name: &str) -> Output<ClKernel> {
        ClKernel::create(self.program(), kernel_name)
    }

    /// Creates a ClMem object in the given context, with the given buffer creator
    /// (either a length or some data). This function uses the BufferCreator's implementation
    /// to retrieve the appropriate MemConfig.
    ///
    /// # Safety
    /// This function can cause undefined behavior if the OpenCL context object that
    /// is passed is not in a valid state (null, released, etc.)
    pub unsafe fn create_mem<T: ClNum, B: BufferCreator<T>>(
        &self,
        buffer_creator: B,
    ) -> Output<ClMem> {
        let cfg = buffer_creator.mem_config();
        ClMem::create_with_config(self.context(), buffer_creator, cfg)
    }

    /// Creates a ClMem object in the given context, with the given buffer creator
    /// (either a length or some data) and a given MemConfig.
    ///
    /// # Safety
    /// This function can cause undefined behavior if the OpenCL context object that
    /// is passed is not in a valid state (null, released, etc.)
    pub unsafe fn create_mem_with_config<T: ClNum, B: BufferCreator<T>>(
        &self,
        buffer_creator: B,
        mem_config: MemConfig,
    ) -> Output<ClMem> {
        ClMem::create_with_config(self.context(), buffer_creator, mem_config)
    }

    #[inline]
    fn get_queue_by_index(&mut self, index: usize) -> Output<&mut ClCommandQueue> {
        self.queues
            .get_mut(index)
            .ok_or_else(|| SessionError::QueueIndexOutOfRange(index).into())
    }

    /// This function copies data from the host buffer into the device mem buffer. The host
    /// buffer must be a mutable slice or a vector to ensure the safety of the read_Buffer
    /// operation.
    ///
    /// # Safety
    /// This function call is safe only if the ClMem object's dependencies are still valid, if the
    /// ClMem is valid, if the ClCommandQueue's dependencies are valid, if the ClCommandQueue's object
    /// itself still valid, if the device's size and type exactly match the host buffer's size and type,
    /// if the waitlist's events are in a valid state and the list goes on...
    pub unsafe fn write_buffer<'a, T: ClNum, H: Into<VecOrSlice<'a, T>>>(
        &mut self,
        queue_index: usize,
        mem: &mut ClMem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        mem.number_type().match_or_panic(T::number_type());
        let queue: &mut ClCommandQueue = self.get_queue_by_index(queue_index)?;
        queue.write_buffer(mem, host_buffer, opts)
    }

    /// This function copies data from a device mem buffer into a host buffer. The host
    /// buffer must be a mutable slice or a vector. For the moment the device mem must also
    /// be passed as mutable; I don't trust OpenCL.
    ///
    /// # Safety
    /// This function call is safe only if the ClMem object's dependencies are still valid, if the
    /// ClMem is valid, if the ClCommandQueue's dependencies are valid, if the ClCommandQueue's object
    /// itself still valid, if the device's size and type exactly match the host buffer's size and type,
    /// if the waitlist's events are in a valid state and the list goes on...
    pub unsafe fn read_buffer<'a, T: ClNum, H: Into<MutVecOrSlice<'a, T>>>(
        &mut self,
        queue_index: usize,
        mem: &mut ClMem,
        host_buffer: H,
        opts: Option<CommandQueueOptions>,
    ) -> Output<BufferReadEvent<T>> {
        let queue: &mut ClCommandQueue = self.get_queue_by_index(queue_index)?;
        queue.read_buffer(mem, host_buffer, opts)
    }

    /// This function enqueues a CLKernel into a command queue
    ///
    /// # Safety
    /// If the ClKernel is not in a usable state or any of the Kernel's dependent object
    /// has been release, or the kernel belongs to a different session, or the ClKernel's
    /// pointer is a null pointer, then calling this function will cause undefined behavior.
    pub unsafe fn enqueue_kernel(
        &mut self,
        queue_index: usize,
        kernel: &mut ClKernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        let queue: &mut ClCommandQueue = self.get_queue_by_index(queue_index)?;
        let cq_opts: CommandQueueOptions = opts.into();
        let event = cl_enqueue_nd_range_kernel(
            queue.command_queue_ptr(),
            kernel.kernel_ptr(),
            work,
            &cq_opts.waitlist[..],
        )?;
        ClEvent::new(event)
    }

    pub fn execute_sync_kernel_operation(
        &mut self,
        queue_index: usize,
        mut kernel_op: KernelOperation,
    ) -> Output<()> {
        unsafe {
            let mut kernel = self.create_kernel(kernel_op.name())?;
            let queue: &mut ClCommandQueue = self.get_queue_by_index(queue_index)?;
            for (arg_index, (arg_size, arg_ptr)) in kernel_op.mut_args().iter_mut().enumerate() {
                kernel.set_arg_raw(arg_index.try_into().unwrap(), *arg_size, *arg_ptr)?;
            }
            let work = kernel_op.work()?;
            let event = queue.enqueue_kernel(&mut kernel, &work, kernel_op.command_queue_opts())?;
            event.wait()?;
            Ok(())
        }
    }
}

/// Session can be safely sent between threads.
///
/// # Safety
/// All the contained OpenCL objects Session are Send so Session is Send. However,
/// The low level Session has ZERO Synchronization for mutable objects Program and
/// CommandQueue. Therefore the low level Session is not Sync. If a Sync Session is
/// required, the Session of opencl_core is Sync by synchronizing mutations of it's
/// objects via RwLocks.
unsafe impl Send for Session {}
// unsafe impl Sync for Session {}

// impl fmt::Debug for Session {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Session{{{:?}}}", self.address())
//     }
// }

// preserve the ordering of these fields
// The drop order must be:
// 1) program
// 2) command_queue
// 3) context
// 4) device
// Else... SEGFAULT :(
impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.queues);
            ManuallyDrop::drop(&mut self.program);
            ManuallyDrop::drop(&mut self.context);
            ManuallyDrop::drop(&mut self.devices);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionQueue<'a> {
    phantom: PhantomData<&'a ClCommandQueue>,
    index: usize,
}

impl<'a> SessionQueue<'a> {
    pub fn new(index: usize) -> SessionQueue<'a> {
        SessionQueue {
            index,
            phantom: PhantomData,
        }
    }
}

/// An error related to Session Building.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum SessionBuilderError {
    #[fail(display = "Given ClMem has no associated cl_mem object")]
    NoAssociatedMemObject,

    #[fail(
        display = "For session building platforms AND devices cannot be specifed together; they are mutually exclusive."
    )]
    CannotSpecifyPlatformsAndDevices,

    #[fail(
        display = "For session building program src AND binaries cannot be specifed together; they are mutually exclusive."
    )]
    CannotSpecifyProgramSrcAndProgramBinaries,

    #[fail(
        display = "For session building either program src or program binaries must be specified."
    )]
    MustSpecifyProgramSrcOrProgramBinaries,

    #[fail(
        display = "Building a session with program binaries requires exactly 1 device: Got {:?} devices",
        _0
    )]
    BinaryProgramRequiresExactlyOneDevice(usize),
}

const CANNOT_SPECIFY_SRC_AND_BINARIES: Error =
    Error::SessionBuilderError(SessionBuilderError::CannotSpecifyProgramSrcAndProgramBinaries);
const MUST_SPECIFY_SRC_OR_BINARIES: Error =
    Error::SessionBuilderError(SessionBuilderError::MustSpecifyProgramSrcOrProgramBinaries);

#[derive(Default)]
pub struct SessionBuilder<'a> {
    pub program_src: Option<&'a str>,
    pub program_binaries: Option<&'a [u8]>,
    pub device_type: Option<DeviceType>,
    pub platforms: Option<&'a [ClPlatformID]>,
    pub devices: Option<&'a [ClDeviceID]>,
    pub command_queue_properties: Option<CommandQueueProperties>,
}

impl<'a> SessionBuilder<'a> {
    pub fn new() -> SessionBuilder<'a> {
        SessionBuilder {
            program_src: None,
            program_binaries: None,
            device_type: None,
            platforms: None,
            devices: None,
            command_queue_properties: None,
        }
    }

    pub fn with_program_src(mut self, src: &'a str) -> SessionBuilder<'a> {
        self.program_src = Some(src);
        self
    }

    pub fn with_program_binaries(mut self, bins: &'a [u8]) -> SessionBuilder<'a> {
        self.program_binaries = Some(bins);
        self
    }

    pub fn with_platforms(mut self, platforms: &'a [ClPlatformID]) -> SessionBuilder<'a> {
        self.platforms = Some(platforms);
        self
    }

    pub fn with_devices(mut self, devices: &'a [ClDeviceID]) -> SessionBuilder<'a> {
        self.devices = Some(devices);
        self
    }

    pub fn with_device_type(mut self, device_type: DeviceType) -> SessionBuilder<'a> {
        self.device_type = Some(device_type);
        self
    }

    pub fn with_command_queue_properties(
        mut self,
        props: CommandQueueProperties,
    ) -> SessionBuilder<'a> {
        self.command_queue_properties = Some(props);
        self
    }
    fn check_for_error_state(&self) -> Output<()> {
        match self {
            Self {
                program_src: Some(_),
                program_binaries: Some(_),
                ..
            } => return Err(CANNOT_SPECIFY_SRC_AND_BINARIES),
            Self {
                program_src: None,
                program_binaries: None,
                ..
            } => return Err(MUST_SPECIFY_SRC_OR_BINARIES),
            _ => Ok(()),
        }
    }

    /// Builds a SessionBuilder into a Session
    ///
    /// # Safety
    /// This function may, in fact, be safe, mismanagement of objects and lifetimes
    /// are not possible as long as the underlying function calls are implemented
    /// as intended. However, this claim needs to be reviewed. For now it remains
    /// marked as unsafe.
    pub unsafe fn build(self) -> Output<Session> {
        self.check_for_error_state()?;
        let context_builder = ClContextBuilder {
            devices: self.devices,
            device_type: self.device_type,
            platforms: self.platforms,
        };
        let built_context = context_builder.build()?;
        let (context, devices): (ClContext, Vec<ClDeviceID>) = match built_context {
            BuiltClContext::Context(ctx) => (ctx, self.devices.unwrap().to_vec()),
            BuiltClContext::ContextWithDevices(ctx, owned_devices) => (ctx, owned_devices),
        };
        let program: ClProgram = match (&self, devices.len()) {
            (
                Self {
                    program_src: Some(src),
                    ..
                },
                _,
            ) => {
                let mut prog: ClProgram = ClProgram::create_with_source(&context, src)?;
                prog.build(&devices[..])?;
                Ok(prog)
            }
            (
                Self {
                    program_binaries: Some(bins),
                    ..
                },
                1,
            ) => {
                let mut prog: ClProgram =
                    ClProgram::create_with_binary(&context, &devices[0], *bins)?;
                prog.build(&devices[..])?;
                Ok(prog)
            }
            (
                Self {
                    program_binaries: Some(_),
                    ..
                },
                n_devices,
            ) => {
                let e = SessionBuilderError::BinaryProgramRequiresExactlyOneDevice(n_devices);
                Err(Error::SessionBuilderError(e))
            }
            _ => unreachable!(),
        }?;

        let props = CommandQueueProperties::default();
        let maybe_queues: Result<Vec<ClCommandQueue>, Error> = devices
            .iter()
            .map(|dev| ClCommandQueue::create(&context, dev, Some(props)))
            .collect();
        let queues = maybe_queues?;

        let sess = Session {
            devices: ManuallyDrop::new(devices),
            context: ManuallyDrop::new(context),
            program: ManuallyDrop::new(program),
            queues: ManuallyDrop::new(queues),
        };
        Ok(sess)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BufferReadEvent, KernelOperation, Session};

    const SRC: &'static str = "__kernel void test(__global int *data) {
        data[get_global_id(0)] += 1;
    }";
    // use crate::ll_testing;
    fn get_session(src: &str) -> Session {
        Session::create(src).unwrap_or_else(|e| panic!("Failed to get_session {:?}", e))
    }

    #[test]
    fn session_execute_sync_kernel_operation_works() {
        let mut session = get_session(SRC);
        let data: Vec<i32> = vec![1, 2, 3, 4, 5];
        let dims = data.len();
        let mut buff = unsafe { session.create_mem(&data[..]) }.unwrap();
        let kernel_op = KernelOperation::new("test")
            .with_dims(dims)
            .add_arg(&mut buff);
        session
            .execute_sync_kernel_operation(0, kernel_op)
            .unwrap_or_else(|e| {
                panic!("Failed to execute sync kernel operation: {:?}", e);
            });
        let data3 = vec![0i32; 5];
        unsafe {
            let mut read_event: BufferReadEvent<i32> = session
                .read_buffer(0, &mut buff, data3, None)
                .unwrap_or_else(|e| {
                    panic!("Failed to read buffer: {:?}", e);
                });
            let data4 = read_event
                .wait()
                .unwrap_or_else(|e| panic!("Failed to wait for read event: {:?}", e))
                .unwrap();
            assert_eq!(data4, vec![2, 3, 4, 5, 6]);
        }
    }
}
