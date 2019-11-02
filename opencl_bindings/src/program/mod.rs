pub mod flags;
pub mod low_level;

use crate::ffi::cl_program;

use crate::device::Device;
use crate::error::{Error, Output}; 
use crate::context::Context;

use low_level::{cl_retain_program, cl_release_program};

/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ProgramError {
    #[fail(display = "The given source code was not a valid CString")]
    CStringInvalidSourceCode,

    #[fail(display = "The given program binary was not a valid CString")]
    CStringInvalidProgramBinary,
}

impl From<ProgramError> for Error {
    fn from(e: ProgramError) -> Error {
        Error::ProgramError(e)
    }
}

__impl_unconstructable_cl_wrapper!(Program, cl_program);
__impl_cl_object_for_wrapper!(Program, cl_program);
__impl_clone_for_cl_object_wrapper!(Program, cl_retain_program);
__impl_drop_for_cl_object_wrapper!(Program, cl_release_program);

impl Program {
    pub fn create_with_source(context: &Context, src: String) -> Output<Program> {
        low_level::cl_create_program_with_source(context, &src[..])
    }

    pub fn create_program_with_binary(context: &Context, device: &Device, binary: String) -> Output<Program> {
        low_level::cl_create_program_with_binary(context, device, &binary[..])
    }

    pub fn build_on_many_devices(&self, devices: &[&Device]) -> Output<()> {
        low_level::cl_build_program(self, devices)
    }

    pub fn build_on_one_device(&self, device: &Device) -> Output<()> {
        low_level::cl_build_program(self, &vec![device])
    }

    pub fn get_log(program: &Program, device: &Device) -> Output<String> {
        low_level::cl_get_program_build_log(
            program,
            device,
            flags::ProgramBuildInfo::Log,
        )
    }
}

