use crate::{CommandQueue, Context, Device, Output, Program};

#[repr(C)]
#[derive(Debug)]
pub struct Session {
    // preserve the ordering of these fields
    // The drop order must be:
    // 1) program
    // 2) command_queue
    // 3) context
    // 4) device
    // Else... SEGFAULT :(
    program: Program,
    command_queue: CommandQueue,
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
        program.build_on_one_device(&device)?;
        let command_queue: CommandQueue = CommandQueue::create(&context, &device, None)?;
        Ok(Session {
            device,
            context,
            program,
            command_queue,
            _unconstructable: (),
        })
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

impl Clone for Session {
    fn clone(&self) -> Session {
        Session {
            device: self.device.clone(),
            context: self.context.clone(),
            program: self.program.clone(),
            command_queue: self.command_queue.clone(),
            _unconstructable: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Device, Session};

    fn get_session() -> Session {
        let src = "__kernel void test(__global int *i) { *i += 1; }";
        let device = Device::default();
        Session::create(device, src).expect("Failed to create Session")
    }

    #[test]
    fn session_implements_clone() {
        let session = get_session();
        let _other = session.clone();
    } 
}