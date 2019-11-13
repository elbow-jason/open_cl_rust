pub use crate::kernel::KernelError;
pub use crate::event::EventError;
pub use crate::device::DeviceError;
pub use crate::program::ProgramError;
pub use crate::platform::PlatformError;
pub use crate::utils::ClError;
pub use crate::cl::ClValueError;
// use crate::utils::StatusCode;


#[derive(Debug, Fail, PartialEq, Clone, Eq)]
pub enum Error {
    #[fail(display = "{:?}", _0)]
    ClValueError(ClValueError),

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