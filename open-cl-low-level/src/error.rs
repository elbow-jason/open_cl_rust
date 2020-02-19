// use crate::cl::ClObjectError;
// use crate::device::DeviceError;
// use crate::device_mem::DeviceMemError;
// use crate::event::EventError;

use crate::context_builder::ContextBuilderError;
use crate::device::DeviceError;
use crate::event::EventError;
use crate::kernel::KernelError;
use crate::mem::MemError;
use crate::platform::PlatformError;
use crate::program::ProgramError;
use crate::session::{SessionBuilderError, SessionError};
use crate::status_code::StatusCodeError;
use crate::work::WorkError;
use crate::cl_number_type::TypeError;

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

    #[fail(display = "{:?}", _0)]
    EventError(EventError),

    #[fail(display = "{:?}", _0)]
    ContextBuilderError(ContextBuilderError),

    #[fail(display = "{:?}", _0)]
    SessionBuilderError(SessionBuilderError),

    #[fail(display = "{:?}", _0)]
    SessionError(SessionError),

    #[fail(display = "{:?}", _0)]
    WorkError(WorkError),

    #[fail(display = "{:?}", _0)]
    TypeError(TypeError),

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

impl From<ContextBuilderError> for Error {
    fn from(err: ContextBuilderError) -> Error {
        Error::ContextBuilderError(err)
    }
}

impl From<SessionBuilderError> for Error {
    fn from(err: SessionBuilderError) -> Error {
        Error::SessionBuilderError(err)
    }
}

impl From<SessionError> for Error {
    fn from(err: SessionError) -> Error {
        Error::SessionError(err)
    }
}

impl From<WorkError> for Error {
    fn from(err: WorkError) -> Error {
        Error::WorkError(err)
    }
}

impl From<TypeError> for Error {
    fn from(err: TypeError) -> Error {
        Error::TypeError(err)
    }
}

