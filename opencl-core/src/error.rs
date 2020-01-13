use crate::cl::ClObjectError;
use crate::device::DeviceError;
use crate::device_mem::DeviceMemError;
use crate::event::EventError;
use crate::kernel::KernelError;
use crate::platform::PlatformError;
use crate::program::ProgramError;
use crate::utils::ClError;

#[derive(Debug, Fail, PartialEq, Clone, Eq)]
pub enum Error {
    #[fail(display = "{:?}", _0)]
    DeviceMemError(DeviceMemError),

    #[fail(display = "{:?}", _0)]
    ClObjectError(ClObjectError),

    #[fail(display = "{:?}", _0)]
    PlatformError(PlatformError),

    #[fail(display = "{:?}", _0)]
    ProgramError(ProgramError),

    #[fail(display = "{:?}", _0)]
    KernelError(KernelError),

    #[fail(display = "{:?}", _0)]
    EventError(EventError),

    #[fail(display = "{:?}", _0)]
    DeviceError(DeviceError),

    #[fail(display = "OpenCL returned an error status code {:?} {:?}", _0, _1)]
    StatusCode(isize, ClError),
}

impl Error {
    pub fn panic(e: Error) {
        panic!("{:?}", e);
    }
}

pub type Output<T> = Result<T, Error>;


impl From<KernelError> for Error {
    fn from(e: KernelError) -> Error {
        Error::KernelError(e)
    }
}

impl From<ClObjectError> for Error {
    fn from(err: ClObjectError) -> Error {
        Error::ClObjectError(err)
    }
}

impl From<DeviceMemError> for Error {
    fn from(err: DeviceMemError) -> Error {
        Error::DeviceMemError(err)
    }
}

impl From<DeviceError> for Error {
    fn from(err: DeviceError) -> Error {
        Error::DeviceError(err)
    }
}

impl From<EventError> for Error {
    fn from(ee: EventError) -> Error {
        Error::EventError(ee)
    }
}

impl From<PlatformError> for Error {
    fn from(err: PlatformError) -> Error {
        Error::PlatformError(err)
    }
}

impl From<ProgramError> for Error {
    fn from(e: ProgramError) -> Error {
        Error::ProgramError(e)
    }
}