use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::{CommandQueue, Context, Device, Output, Program};


// preserve the ordering of these fields
// The drop order must be:
// 1) program
// 2) command_queue
// 3) context
// 4) device
// Else... SEGFAULT :(
#[derive(Debug)]
struct SessionInner {
    program: ManuallyDrop<Program>,
    command_queue: ManuallyDrop<CommandQueue>,
    context: ManuallyDrop<Context>,
    device: ManuallyDrop<Device>,
    _unconstructable: (),
}

impl SessionInner {
    fn create(device: Device, src: &str) -> Output<SessionInner> {
        let context: Context = Context::create(&device)?;
        let program: Program = Program::create_with_source(&context, src)?;
        program.build_on_one_device(&device)?;
        let command_queue: CommandQueue = CommandQueue::create(&context, &device, None)?;
        Ok(SessionInner {
            device: ManuallyDrop::new(device),
            context: ManuallyDrop::new(context),
            program: ManuallyDrop::new(program),
            command_queue: ManuallyDrop::new(command_queue),
            _unconstructable: (),
        })
    }
}

unsafe impl Send for SessionInner {}
unsafe impl Sync for SessionInner {}

impl Clone for SessionInner {
    fn clone(&self) -> SessionInner {
        let device = (*self.device).clone();
        let context = (*self.context).clone();
        let program = (*self.program).clone();
        let command_queue = (*self.command_queue).clone();
        SessionInner {
            device: ManuallyDrop::new(device),
            context: ManuallyDrop::new(context),
            program: ManuallyDrop::new(program),
            command_queue: ManuallyDrop::new(command_queue),
            _unconstructable: (),
        }
    }
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self.context);
            ManuallyDrop::drop(&mut self.program);
            ManuallyDrop::drop(&mut self.command_queue);
        }
    }
}

#[derive(Debug)]
pub struct Session {
    inner: Arc<SessionInner>, 
    _unconstructable: (),
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn create(device: Device, src: &str) -> Output<Session> {
        let session_inner = SessionInner::create(device, src)?;
        Ok(Session {
            inner: Arc::new(session_inner),
            _unconstructable: (),
        })
    }

    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    pub fn context(&self) -> &Context {
        &self.inner.context
    }

    pub fn program(&self) -> &Program {
        &self.inner.program
    }

    pub fn command_queue(&self) -> &CommandQueue {
        &self.inner.command_queue
    }
}

impl Clone for Session {
    fn clone(&self) -> Session {
        Session {
            inner: self.inner.clone(),
            _unconstructable: (),
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