use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::{
    CommandQueue,
    Context,
    Device, Output, UnbuiltProgram, Program
};


// preserve the ordering of these fields
// The drop order must be:
// 1) program
// 2) command_queue
// 3) context
// 4) device
// Else... SEGFAULT :(
#[derive(Debug)]
struct SessionInner {
    _program: ManuallyDrop<Program>,
    _command_queue: ManuallyDrop<CommandQueue>,
    _unconstructable: (),
}

impl SessionInner {
    fn create(devices: Vec<Device>, src: &str) -> Output<Vec<SessionInner>> {
        let context: Context = Context::create(&devices[..])?;
        let unbuilt_program: UnbuiltProgram = UnbuiltProgram::create_with_source(&context, src)?;
        debug_assert!(devices.len() > 0);
        let programs: Vec<Program> = unbuilt_program.build(&devices[..])?;
        let mut sessions: Vec<SessionInner> = Vec::with_capacity(devices.len());
        for program in programs.into_iter() {
            let device: &Device = program.device();
            let context: &Context = program.context();
            let command_queue = CommandQueue::create(&context, device, None)?;
            let session_inner = SessionInner {
                _program: ManuallyDrop::new(program),
                _command_queue: ManuallyDrop::new(command_queue),
                _unconstructable: (),
            };
            sessions.push(session_inner);
        }
        Ok(sessions)
    }

    fn context(&self) -> &Context {
        self._program.context()
    }

    fn command_queue(&self) -> &CommandQueue {
        &self._command_queue
    }

    fn program(&self) -> &Program {
        &self._program
    }

    fn with_copied_command_queue(&self) -> SessionInner {
        let copied_queue = self.command_queue().new_copy().unwrap();
        SessionInner {
            _program: self._program.clone(),
            _command_queue: ManuallyDrop::new(copied_queue),
            _unconstructable: (),
        }
    }
}

unsafe impl Send for SessionInner {}
// unsafe impl Sync for SessionInner {}

impl Clone for SessionInner {
    fn clone(&self) -> SessionInner {
        let program = (*self._program).clone();
        let command_queue = (*self._command_queue).clone();
        SessionInner {
            _program: ManuallyDrop::new(program),
            _command_queue: ManuallyDrop::new(command_queue),
            _unconstructable: (),
        }
    }
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self._command_queue);
            ManuallyDrop::drop(&mut self._program);
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
    pub fn create_sessions(devices: &[Device], src: &str) -> Output<Vec<Session>> {
        let owned_devices = devices.clone().to_owned();
        let session_inners = SessionInner::create(owned_devices, src)?;
        let sessions = session_inners
            .into_iter()
            .map(|inner| Session {inner: Arc::new(inner), _unconstructable: ()})
            .collect();
        Ok(sessions)
    }

    fn inner(&self) -> &SessionInner {
        &*self.inner
    }

    pub fn program_device(&self) -> &Device {
        self.program().device()
    }

    pub fn context(&self) -> &Context {
        self.inner().context()
    }

    pub fn context_devices(&self) -> &[Device] {
        self.inner().context().devices()
    }

    pub fn command_queue_device(&self) -> &Device {
        self.command_queue().device()
    }

    pub fn program(&self) -> &Program {
        self.inner().program()
    }

    pub fn command_queue(&self) -> &CommandQueue {
        self.inner().command_queue()
    }

    pub fn with_copied_command_queue(&self) -> Session {
        Session {
            inner: Arc::new(self.inner().with_copied_command_queue()),
            _unconstructable: (),
        } 
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
impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.command_queue() == other.command_queue()
    }
}

impl Eq for Session {}



// #[cfg(test)]
// mod tests {
//     use crate::testing;

//     const SRC: &'static str = "__kernel void test(__global int *i) { *i += 1; }";

//     #[test]
//     fn session_implements_clone() {
//         let session = testing::get_session(SRC);
//         let _other = session.clone();
//     }

//     #[test]
//     fn session_implementation_of_fmt_debug_works() {
//         for session in testing::all_sessions(SRC) {
//             let formatted = format!("{:?}", session);
//             assert!(formatted.starts_with("Session"), "Formatted did not start with the correct value. Got: {:?}", formatted);
//         }
//     }

//     #[test]
//     fn session_with_copied_command_queue_works() {
//         for session in testing::all_sessions(SRC) {
//             let session2 = session.with_copied_command_queue();
//             assert_ne!(session.command_queue(), session2.command_queue());
//             assert_eq!(session.context(), session2.context());
//             assert_eq!(session.program(), session2.program());
//             assert_ne!(session, session2);
//         }
//     }
// }