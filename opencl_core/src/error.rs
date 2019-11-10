use crate::kernel::KernelError;
use crate::event::EventError;
use crate::device::DeviceError;
use crate::program::ProgramError;
use crate::platform::PlatformError;
use crate::utils::ClError;
// use crate::cl::ClValueError;
// use crate::utils::StatusCode;


#[derive(Debug, Fail, PartialEq, Clone)]
pub enum Error {
    // #[fail(display = "{:?}", _0)]
    // ClValueError(ClValueError),

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

pub type Output<T> = Result<T, Error>;