pub mod flags;
pub mod low_level;

use crate::ffi::cl_program;

use crate::device::Device;
use crate::error::{Error, Output}; 
use crate::context::Context;

use low_level::{cl_retain_program, cl_release_program};

use crate::cl::ClPointer;

use flags::{ProgramInfo};

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
__impl_cl_object_for_wrapper!(Program, cl_program, cl_retain_program, cl_release_program);
__impl_clone_for_cl_object_wrapper!(Program, cl_retain_program);
__impl_drop_for_cl_object_wrapper!(Program, cl_release_program);

unsafe impl Send for Program {}
unsafe impl Sync for Program {}

impl Program {
    pub fn create_with_source(context: &Context, src: &str) -> Output<Program> {
        low_level::cl_create_program_with_source(context, src)
    }

    pub fn create_program_with_binary(context: &Context, device: &Device, binary: &str) -> Output<Program> {
        low_level::cl_create_program_with_binary(context, device, binary)
    }

    pub fn build_on_many_devices(&self, devices: &[&Device]) -> Output<()> {
        low_level::cl_build_program(self, devices)
    }

    pub fn build_on_one_device(&self, device: &Device) -> Output<()> {
        low_level::cl_build_program(self, &vec![device])
    }

    pub fn get_log(program: &Program, device: &Device) -> Output<String> {
        let flag = flags::ProgramBuildInfo::Log;
        low_level::cl_get_program_build_log(program, device, flag).map(|ret| unsafe { ret.into_string() })
    }

    fn get_info<T: Copy>(&self, flag: ProgramInfo) -> Output<ClPointer<T>> {
        low_level::cl_get_program_info(self, flag)
    }

    pub fn reference_count(&self) -> Output<u32> {
        self.get_info(ProgramInfo::ReferenceCount).map(|ret| {
            unsafe { ret.into_one() }
        })
    }


    pub fn context(&self) -> Output<Context> {
        self.get_info(ProgramInfo::Context).and_then(|ret| unsafe { ret.into_retained_wrapper::<Context>() })
    }

    pub fn num_devices(&self) -> Output<u32> {
        self.get_info(ProgramInfo::NumDevices).map(|ret| {
            unsafe { ret.into_one() }
        })
    }

    pub fn devices(&self) -> Output<Vec<Device>> {
        self.get_info(ProgramInfo::Devices).and_then(|ret| unsafe { ret.into_many_retained_wrappers() })
    }

    pub fn source(&self) -> Output<String> {
        self.get_info(ProgramInfo::Source).map(|ret| unsafe { ret.into_string() })
    }
    pub fn binary_sizes(&self) -> Output<Vec<usize>> {
        self.get_info(ProgramInfo::BinarySizes).map(|ret| {
            unsafe { ret.into_many() }
        })
    }

    pub fn binaries(&self) -> Output<Vec<u8>> {
        self.get_info(ProgramInfo::Binaries).map(|ret| {
            unsafe { ret.into_many() }
        })
    }

    pub fn num_kernels(&self) -> Output<usize> {
        self.get_info(ProgramInfo::NumKernels).map(|ret| {
            unsafe { ret.into_one() }
        })
    }

    pub fn kernel_names(&self) -> Output<Vec<String>> {
        self.get_info(ProgramInfo::KernelNames).map(|ret| {
            let kernels: String = unsafe { ret.into_string() };
            kernels
                .split(";")
                .map(|s| s.to_string())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Session,  Context, Device};

    const TEST_SRC: &str = "
    __kernel void test(__global int *i) {        
        *i += 1;
    }
    ";

    
    fn get_session() -> Session {
        let device = Device::default();
        Session::create(device, TEST_SRC).expect("Failed to create Session")
    }


    #[test]
    fn program_method_reference_count_works() {
        let session = get_session();
        let output: u32 = session.program().reference_count().expect("Failed to call program.reference_count()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_context_works() {
        let session = get_session();
        let _output: Context = session.program().context().expect("Failed to call program.context()");
    }

    #[test]
    fn program_method_num_devices_works() {
        let session = get_session();
        let output: u32 = session.program().num_devices().expect("Failed to call program.num_devices()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_devices_works() {
        let session = get_session();
        let output: Vec<Device> = session.program().devices().expect("Failed to call program.devices()");
        inspect!(output);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn program_method_source_works() {
        let session = get_session();
        let output: String = session.program().source().expect("Failed to call program.source()");
        assert_eq!(output, TEST_SRC.to_string());
    }

    #[test]
    fn program_method_binary_sizes_works() {
        let session = get_session();
        let output: Vec<usize> = session.program().binary_sizes().expect("Failed to call program.binary_sizes()");
        let expected = vec![1332];
        assert_eq!(output, expected);
    }

    #[test]
    fn program_method_binaries_works() {
        let session = get_session();
        let output: Vec<u8> = session.program().binaries().expect("Failed to call program.binaries()");
        let expected = vec![0u8, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(output, expected);
    }

    #[test]
    fn program_method_num_kernels_works() {
        let session = get_session();
        let output: usize = session.program().num_kernels().expect("Failed to call program.num_kernels()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_kernel_names_works() {
        let session = get_session();
        let output: Vec<String> = session.program().kernel_names().expect("Failed to call program.kernel_names()");
        let expected = vec![
            "test".to_string(),
        ];
        assert_eq!(output, expected);
    }
}

