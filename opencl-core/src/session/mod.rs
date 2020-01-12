use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::{
    CommandQueue,
    Context,
    Device, Output, UnbuiltProgram, Program, DevicePtr,
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
        println!("S1");
        let context: Context = Context::create(&devices[..])?;
        println!("S2");
        let unbuilt_program: UnbuiltProgram = UnbuiltProgram::create_with_source(&context, src)?;
        println!("S3");
        debug_assert!(devices.len() > 0);
        // println!("kernel_names: {:?}", unbuilt_program.kernel_names());
        // let device: &Device = &devices[0];
        let programs: Vec<Program> = unbuilt_program.build(&devices[..])?;
        println!("S4");
        
        let mut sessions: Vec<SessionInner> = Vec::with_capacity(devices.len());
        println!("S5");
        for program in programs.into_iter() {
            println!("S6");
            let device: &Device = program.device();
            let name = device.name();
            let context: &Context = program.context();
            let ctx_device_names: Output<Vec<String>> = context.devices().into_iter().map(|d| d.name()).collect();
            println!("S6 device name: {:?}", name);
            println!("S6 ctx_device_names: {:?}", ctx_device_names);

            
            let command_queue = CommandQueue::create(&context, device, None)?;
            println!("S7");
            let session_inner = SessionInner {
                _program: ManuallyDrop::new(program),
                _command_queue: ManuallyDrop::new(command_queue),
                _unconstructable: (),
            };
            println!("S8");
            sessions.push(session_inner);
            println!("S9");
        }
        println!("S10");
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
}

unsafe impl Send for SessionInner {}
unsafe impl Sync for SessionInner {}

impl Clone for SessionInner {
    fn clone(&self) -> SessionInner {
        // let device = (*self.device).clone();
        // let context = (*self.context).clone();
        let program = (*self._program).clone();
        let command_queue = (*self._command_queue).clone();
        SessionInner {
            // device: ManuallyDrop::new(device),
            // context: ManuallyDrop::new(context),
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
        println!("H1");
        let owned_devices = devices.clone().to_owned();
        println!("H2");
        let session_inners = SessionInner::create(owned_devices, src)?;
        println!("H3");
        let sessions = session_inners
            .into_iter()
            .map(|inner| Session {inner: Arc::new(inner), _unconstructable: ()})
            .collect();
        println!("H4");
        Ok(sessions)
    }

    fn inner(&self) -> &SessionInner {
        &*self.inner
    }

    pub fn device(&self) -> &Device {
        self.program().device()
    }

    pub fn context(&self) -> &Context {
        self.inner().context()
    }

    pub fn program(&self) -> &Program {
        self.inner().program()
    }

    pub fn command_queue(&self) -> &CommandQueue {
        self.inner().command_queue()
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
        Session::create_sessions(&[device], src).expect("Failed to create Session").pop().unwrap()
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
            println!("devices {:?}", devices);
            let more_sessions: Vec<Session> = Session::create_sessions(&devices[..], src)
                .expect("Failed to create Session");
            sessions.extend(more_sessions);
        }
        for session in sessions {
            println!("Session printing: {:?}", session);
        }
    }
}