use crate::{
    Device,
    Context,
    CommandQueue,
    Program,
    Output,
};

#[repr(C)]
#[derive(Debug)]
pub struct Session {
    // preserve the ordering of these fields
    // the (command_queue/program) must be dropped before
    // the context must be dropped before
    // the device.
    // Else... SEGFAULT.
    command_queue: CommandQueue,
    program: Program,
    context: Context,
    device: Device,
    _unconstructable: (),
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn create(device: Device, src: &str) -> Output<Session> {
        let context: Context = Context::create(&device)?;
        let program: Program = Program::create_with_source(&context, src)?;
        let () = program.build_on_one_device(&device)?;
        let command_queue: CommandQueue = CommandQueue::create(&context, &device, None)?;
        Ok(Session{ device, context, program, command_queue, _unconstructable: () })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn command_queue(&self) -> &CommandQueue {
        &self.command_queue
    }
}

