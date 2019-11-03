use crate::kernel::KernelError;
use crate::event::EventError;
use crate::device::DeviceError;
use crate::program::ProgramError;
use crate::utils::ClError;
// use crate::utils::StatusCode;


#[derive(Debug, Fail, PartialEq, Clone)]
pub enum Error {
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

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Error::StatusCode(err_code) => {
//                 let status = StatusCode::from(*err_code as cl_int);
//                 write!(f, "Error::({:?})", status)
//             }
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Error::StatusCode(err_code) => {
//                 let status = StatusCode::from(*err_code as cl_int);
//                 write!(f, "Error::({:?})", status)
//             }
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

pub type Output<T> = Result<T, Error>;
