use std::mem::ManuallyDrop;
use std::marker::PhantomData;

use crate::*;

/// An error related to Session Building.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum SessionError {
    #[fail(display = "The given queue index {} was out of range", _0)]
    QueueIndexOutOfRange(usize),
}

pub struct Session {
    devices: ManuallyDrop<Vec<ClDeviceID>>,
    context: ManuallyDrop<ClContext>,
    program: ManuallyDrop<ClProgram>,
    queues: ManuallyDrop<Vec<ClCommandQueue>>,
}

impl Session {
    pub fn create(src: &str) -> Output<Session> {
        unsafe {
            let platforms = list_platforms()?;
            let mut devices = Vec::new();
            for platform in platforms.iter() {
                let platform_devices = list_devices_by_type(platform, DeviceType::ALL)?;
                devices.extend(platform_devices);
            }
            let context = ClContext::create(&devices[..])?;
            let program = ClProgram::create_with_source(&context, src)?;
            let props = CommandQueueProperties::default();
            let maybe_queues: Result<Vec<ClCommandQueue>, Error> = devices.iter()
                .map(|dev| ClCommandQueue::create(&context, dev, Some(props)))
                .collect();
            let queues = maybe_queues?;
            
            let sess = Session{
                devices: ManuallyDrop::new(devices),
                context: ManuallyDrop::new(context),
                program: ManuallyDrop::new(program),
                queues: ManuallyDrop::new(queues),
            };
            Ok(sess)
        }
    }

    pub unsafe fn devices(&self) -> &[ClDeviceID] {
        &(*self.devices)[..]
    }

    pub unsafe fn context(&self) -> &ClContext {
        &(*self.context)
    }

    pub unsafe fn program(&self) -> &ClProgram {
        &(*self.program)
    }

    pub unsafe fn queues(&self) -> &[ClCommandQueue] {
        &(*self.queues)[..]
    }

    pub unsafe fn create_kernel(&self, kernel_name: &str) -> Output<ClKernel> {
        ClKernel::create(self.program(), kernel_name)
    }

    pub unsafe fn create_mem<T: ClNumber, B: BufferCreator<T>>(&self, buffer_creator: B, mem_config: MemConfig) -> Output<ClMem<T>> {
         ClMem::create_with_config(
            self.context(),
            buffer_creator,
            mem_config
        )
    }

    #[inline]
    fn get_queue_by_index(&mut self, index: usize) -> Output<&mut ClCommandQueue> {
        self.queues
            .get_mut(index)
            .ok_or(SessionError::QueueIndexOutOfRange(index).into())
        
    }

    pub unsafe fn write_buffer<T: ClNumber>(
        &mut self,
        queue_index: usize,
        mem: &mut ClMem<T>,
        host_buffer: &[T],
        opts: Option<CommandQueueOptions>,
    ) -> Output<ClEvent> {
        let queue: &mut ClCommandQueue = self.get_queue_by_index(queue_index)?;
        queue.write_buffer(mem, host_buffer, opts)       
    }   
}

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
pub struct SessionQueue<'a>{
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

    #[fail(display = "For session building platforms AND devices cannot be specifed together; they are mutually exclusive.")]
    CannotSpecifyPlatformsAndDevices,

    #[fail(display = "For session building program src AND binaries cannot be specifed together; they are mutually exclusive.")]
    CannotSpecifyProgramSrcAndProgramBinaries,

    #[fail(display = "For session building either program src or program binaries must be specified.")]
    MustSpecifyProgramSrcOrProgramBinaries,

    #[fail(display = "Building a session with program binaries requires exactly 1 device: Got {:?} devices", _0)]
    BinaryProgramRequiresExactlyOneDevice(usize),
    
}

const CANNOT_SPECIFY_SRC_AND_BINARIES: Error = Error::SessionBuilderError(SessionBuilderError::CannotSpecifyProgramSrcAndProgramBinaries);
const MUST_SPECIFY_SRC_OR_BINARIES: Error = Error::SessionBuilderError(SessionBuilderError::MustSpecifyProgramSrcOrProgramBinaries);


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
        SessionBuilder{
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

    pub fn with_command_queue_properties(mut self, props: CommandQueueProperties) -> SessionBuilder<'a> {
        self.command_queue_properties = Some(props);
        self
    }
    fn check_for_error_state(&self) -> Output<()> {
        match self {
            Self{program_src: Some(_), program_binaries: Some(_), ..} => {
                return Err(CANNOT_SPECIFY_SRC_AND_BINARIES)
            },
            Self{program_src: None, program_binaries: None, ..} => {
                return Err(MUST_SPECIFY_SRC_OR_BINARIES)
            },
            _ => Ok(())
        }
    }

    pub unsafe fn build(self) -> Output<Session> {
        let () = self.check_for_error_state()?; 
        let context_builder = ClContextBuilder{
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
            (Self{program_src: Some(src), ..}, _) => {
                let mut prog: ClProgram = ClProgram::create_with_source(&context, src)?;
                prog.build(&devices[..])?;
                Ok(prog)
            },
            (Self{program_binaries: Some(bins), ..}, 1) => {
                let mut prog: ClProgram = ClProgram::create_with_binary(&context, &devices[0], *bins)?;
                prog.build(&devices[..])?;
                Ok(prog)
            },
            (Self{program_binaries: Some(_), ..}, n_devices) => {
                let e = SessionBuilderError::BinaryProgramRequiresExactlyOneDevice(n_devices);
                Err(Error::SessionBuilderError(e))
            },
            _ => unreachable!()
        }?;
            
        let props = CommandQueueProperties::default();
        let maybe_queues: Result<Vec<ClCommandQueue>, Error> = devices.iter()
            .map(|dev| ClCommandQueue::create(&context, dev, Some(props)))
            .collect();
        let queues = maybe_queues?;
        
        let sess = Session{
            devices: ManuallyDrop::new(devices.clone()),
            context: ManuallyDrop::new(context),
            program: ManuallyDrop::new(program),
            queues: ManuallyDrop::new(queues),
        };
        Ok(sess)
    }
}