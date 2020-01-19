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