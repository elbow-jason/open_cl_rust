use std::mem::ManuallyDrop;
use crate::{CommandQueue, Context, Device, Output, Program};

#[derive(Debug)]
pub struct Session {
    // preserve the ordering of these fields
    // The drop order must be:
    // 1) program
    // 2) command_queue
    // 3) context
    // 4) device
    // Else... SEGFAULT :(
    program: ManuallyDrop<Program>,
    command_queue: ManuallyDrop<CommandQueue>,
    context: ManuallyDrop<Context>,
    device: ManuallyDrop<Device>,
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
            device: ManuallyDrop::new(device),
            context: ManuallyDrop::new(context),
            program: ManuallyDrop::new(program),
            command_queue: ManuallyDrop::new(command_queue),
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

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.program);
            ManuallyDrop::drop(&mut self.command_queue);
            ManuallyDrop::drop(&mut self.context);
            ManuallyDrop::drop(&mut self.device);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Device, Session, Platform};

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

    #[test]
    fn session_fmt_works() {
        let src = "__kernel void test(__global int *i) { *i += 1; }";
        let mut sessions = Vec::new();
        let platforms = Platform::all().unwrap();
        for p in platforms.iter() {
            let devices: Vec<Device> = p.all_devices().unwrap();
            for (i, d) in devices.into_iter().enumerate() {
                if d.is_usable() {
                    let session = Session::create(d, src).expect("Failed to create Session");
                    sessions.push(session);
                } else {
                    println!("unsable device on platform {:?} at index {:?}", p, i);
                }
                
            }
        }
        for session in sessions {
            println!("Session printing: {:?}", session);
        }
    }
}