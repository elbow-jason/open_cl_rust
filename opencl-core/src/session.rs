use std::mem::ManuallyDrop;
use std::sync::RwLock;

use crate::{Device, DeviceType, Output, VecOrSlice};

use crate::ll::Session as ClSession;
use crate::ll::{
    list_devices_by_type, list_platforms, ClCommandQueue, ClContext, ClDeviceID, ClProgram,
    DevicePtr,
};

//     fn with_copied_command_queue(&self) -> SessionInner {
//         let copied_queue = self.command_queue().create_copy().unwrap();
//         SessionInner {
//             _program: self._program.clone(),
//             _command_queue: ManuallyDrop::new(copied_queue),
//             _unconstructable: (),
//         }
//     }
// }

// unsafe impl Sync for SessionInner {}

#[derive(Debug)]
pub struct Session {
    _devices: ManuallyDrop<Vec<ClDeviceID>>,
    _program: ManuallyDrop<ClProgram>,
    _context: ManuallyDrop<ClContext>,
    _queues: ManuallyDrop<Vec<RwLock<ClCommandQueue>>>,
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

        let ll_session =  ClSession::create_with_devices(ll_devices, src)?;
        let (devices, context, program, queues) = unsafe { ll_session.decompose() };
        // (Vec<ClDeviceID>, ClContext, ClProgram, Vec<ClCommandQueue>)

        let queues_with_locks: Vec<RwLock<ClCommandQueue>> =
            queues.into_iter().map(|q| RwLock::new(q)).collect();

        let sess = Session {
            _devices: ManuallyDrop::new(devices),
            _context: ManuallyDrop::new(context),
            _program: ManuallyDrop::new(program),
            _queues: ManuallyDrop::new(queues_with_locks),
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

    pub fn create_copy(&self) -> Output<Session> {
        let cloned_devices = self._devices.clone();
        let cloned_context = self._context.clone();
        let cloned_program = self._program.clone();
        let mut copied_queues = Vec::with_capacity(self._queues.len());
        for q in self._queues.iter() {
            let queue_copy = unsafe { q.read().unwrap().create_copy() }?;
            copied_queues.push(RwLock::new(queue_copy));
        }
        Ok(Session {
            _devices: cloned_devices,
            _context: cloned_context,
            _program: cloned_program,
            _queues: ManuallyDrop::new(copied_queues),
            _unconstructable: (),
        })
    }
}

impl Clone for Session {
    fn clone(&self) -> Session {
        let cloned_queues: Vec<RwLock<ClCommandQueue>> = self
            ._queues
            .iter()
            .map(|q| RwLock::new(q.read().unwrap().clone()))
            .collect();

        Session {
            _devices: self._devices.clone(),
            _context: self._context.clone(),
            _program: self._program.clone(),
            _queues: ManuallyDrop::new(cloned_queues),
            _unconstructable: (),
        }
    }
}
impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self._queues.len() == other._queues.len()
            && self
                ._queues
                .iter()
                .zip(other._queues.iter())
                .fold(true, |acc, (q, o)| {
                    acc && match (q.try_read(), o.try_read()) {
                        (Ok(l), Ok(r)) => *l == *r,
                        _ => false,
                    }
                })
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
