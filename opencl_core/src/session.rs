use std::default::Default;

use crate::platform::Platform;
use crate::command_queue::CommandQueue;
use crate::context::Context;
use crate::device::Device;
use crate::program::Program;

const OPENCL_ADD_ONE: &str = "
__kernel void add_one(__global int *i) {
    *i += 1;
}
";

pub struct SessionBuilder {
    device: Option<Device>,
    src: Option<String>
}

impl SessionBuilder {
    pub fn new() -> SessionBuilder {
        SessionBuilder{
            device: None,
            src: None,
        }
    }

    pub fn with_src(mut self, src: &str) -> SessionBuilder {
        self.src = Some(src.to_string());
        self
    }

    pub fn with_device(mut self, device: Device) -> SessionBuilder {
        self.device = Some(device);
        self
    }

    pub fn build(self) -> Session {
        let src: String = self.src.unwrap_or_else(|| OPENCL_ADD_ONE.to_string());
        let device: Device = self.device.unwrap_or_else(|| Device::default());
        let context: Context = Context::create(&device)
            .expect("Failed to get context for device");
        let command_queue: CommandQueue = CommandQueue::create(&context, &device, None)
            .expect("Failed to create CommandQueue from device");
        let program: Program = Program::create_with_source(&context, &src)
            .expect("Failed to Program::create_with_source with zero (default) function");
        let () = program.build_on_one_device(&device)
            .expect("Failed to build program on device");
        Session{ device, context, command_queue, program }
    }
}


#[repr(C)]
pub struct Session {
    command_queue: CommandQueue,
    program: Program,
    context: Context,
    device: Device,
}

impl Session {

    pub fn command_queue(&self) -> &CommandQueue {
        &self.command_queue
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}


impl Default for Session {
    fn default() -> Session {
        let platform: Platform = Platform::default();

        let device: Device = platform.default_device()
            .expect("Failed unwrap platform.default_device()");
        
        SessionBuilder::new()
            .with_src(OPENCL_ADD_ONE)
            .with_device(device)
            .build()
    }
}
    
