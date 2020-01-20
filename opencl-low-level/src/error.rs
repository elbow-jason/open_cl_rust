// use crate::cl::ClObjectError;
// use crate::device::DeviceError;
// use crate::device_mem::DeviceMemError;
// use crate::event::EventError;


use crate::program::ProgramError;
use crate::kernel::KernelError;
use crate::platform::PlatformError;
use crate::device::DeviceError;
use crate::status_code::StatusCodeError;
use crate::mem::MemError;

#[derive(Debug, Fail, PartialEq, Clone, Eq)]
pub enum Error {
    #[fail(display = "{:?}", _0)]
    StatusCodeError(StatusCodeError),

    #[fail(display = "{:?}", _0)]
    PlatformError(PlatformError),

    #[fail(display = "{:?}", _0)]
    DeviceError(DeviceError),

    #[fail(display = "{:?}", _0)]
    ProgramError(ProgramError),

    #[fail(display = "{:?}", _0)]
    KernelError(KernelError),

    #[fail(display = "{:?}", _0)]
    MemError(MemError),

    #[fail(display = "OpenCL returned a null pointer")]
    ClObjectCannotBeNull,
}

impl Error {
    pub fn panic(e: Error) {
        panic!("{:?}", e);
    }
}

impl From<StatusCodeError> for Error {
    fn from(err: StatusCodeError) -> Error {
        Error::StatusCodeError(err)
    }
}

impl From<PlatformError> for Error {
    fn from(err: PlatformError) -> Error {
        Error::PlatformError(err)
    }
}

impl From<DeviceError> for Error {
    fn from(err: DeviceError) -> Error {
        Error::DeviceError(err)
    }
}

impl From<ProgramError> for Error {
    fn from(e: ProgramError) -> Error {
        Error::ProgramError(e)
    }
}

impl From<KernelError> for Error {
    fn from(e: KernelError) -> Error {
        Error::KernelError(e)
    }
}

impl From<MemError> for Error {
    fn from(err: MemError) -> Error {
        Error::MemError(err)
    }
}



// impl From<EventError> for Error {
//     fn from(ee: EventError) -> Error {
//         Error::EventError(ee)
//     }
// }


