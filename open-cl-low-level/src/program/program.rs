use super::functions;
use crate::cl::{
    cl_device_id, cl_program, strings, ClObject, ObjectWrapper, ProgramBuildInfo, ProgramInfo,
};
use crate::{Context, ContextPtr, Device, DevicePtr, ErrorT, Output};

/// An error related to Program.
#[derive(ErrorT, Debug, PartialEq, Eq, Clone)]
pub enum ProgramError {
    #[error("The given source code was not a valid CString")]
    InvalidSourceCode,
    #[error("The given program binary was not a valid CString")]
    InvalidProgramBinary,
    #[error("Cannot build a program with an empty list of devices")]
    EmptyDevicesList,
}

use ProgramError::*;

pub type Program = ObjectWrapper<cl_program>;

impl Program {
    // pub unsafe fn unchecked_new(obj: cl_program) -> Program {
    //     Program(ObjectWrapper::new(obj))
    // }

    /// Creates a new Program on the context and device with the given OpenCL source code.
    ///
    /// # Safety
    /// The provided Context and Device must be in valid state or else undefined behavior is
    /// expected.
    pub unsafe fn create_with_src(context: &Context, src: &str) -> Output<Program> {
        let src = strings::to_c_string(src).ok_or_else(|| InvalidSourceCode)?;
        let prog = functions::create_program_with_src(context.context_ptr(), src)?;
        Ok(Program::new(prog))
    }

    /// Creates a new Program on the context and device with the given executable binary.
    ///
    /// # Safety
    /// The provided ClContext and ClDeviceID must be in valid state or else undefined behavior is
    /// expected.
    pub unsafe fn create_with_binary(
        context: &Context,
        device: &Device,
        bin: &[u8],
    ) -> Output<Program> {
        let prog =
            functions::create_program_with_binary(context.context_ptr(), device.device_ptr(), bin)?;
        Ok(Program::new(prog))
    }

    pub fn build<D>(&mut self, devices: &[D]) -> Output<()>
    where
        D: DevicePtr,
    {
        if devices.is_empty() {
            return Err(EmptyDevicesList)?;
        }
        unsafe {
            let device_ptrs: Vec<cl_device_id> = devices.iter().map(|d| d.device_ptr()).collect();
            functions::build_program(self.program_ptr(), &device_ptrs[..])
        }
    }

    pub fn get_log<D: DevicePtr>(&self, device: &D) -> Output<String> {
        unsafe {
            functions::get_program_build_log(
                self.program_ptr(),
                device.device_ptr(),
                ProgramBuildInfo::Log.into(),
            )
        }
    }
}

unsafe impl ProgramPtr for Program {
    unsafe fn program_ptr(&self) -> cl_program {
        self.cl_object()
    }
}

/// ProgramPtr is the trait to access a cl_program for wrappers of that cl_program.
///
/// # Safety
/// Direct interaction with any OpenCL pointer is unsafe so this trait is unsafe.
pub unsafe trait ProgramPtr: Sized {
    /// program_ptr is the trait to access a cl_program for wrappers of that cl_program.
    ///
    /// # Safety
    /// Direct interaction with any OpenCL pointer is unsafe so this trait is unsafe.
    unsafe fn program_ptr(&self) -> cl_program;

    /// The OpenCL reference count of the cl_program.
    unsafe fn reference_count(&self) -> Output<u32> {
        functions::get_program_info_u32(self.program_ptr(), ProgramInfo::ReferenceCount.into())
    }

    /// The number of devices that this cl_program is built on.
    unsafe fn num_devices(&self) -> Output<u32> {
        functions::get_program_info_u32(self.program_ptr(), ProgramInfo::NumDevices.into())
    }

    /// The source code String of this OpenCL program.
    unsafe fn source(&self) -> Output<String> {
        functions::get_program_info_string(self.program_ptr(), ProgramInfo::Source.into())
    }

    /// The size of the binaries for this OpenCL program.
    unsafe fn binary_sizes(&self) -> Output<Vec<usize>> {
        functions::get_program_info_vec_usize(self.program_ptr(), ProgramInfo::BinarySizes.into())
    }

    /// The executable binaries for this OpenCL program.
    unsafe fn binaries(&self) -> Output<Vec<u8>> {
        functions::get_program_info_bytes(self.program_ptr(), ProgramInfo::Binaries.into())
    }

    /// The number of kernels (defined functions) in this OpenCL program.
    unsafe fn num_kernels(&self) -> Output<usize> {
        functions::get_program_info_usize(self.program_ptr(), ProgramInfo::NumKernels.into())
    }

    /// The names of the kernels (defined functions) in this OpenCL program.
    unsafe fn kernel_names(&self) -> Output<Vec<String>> {
        let names = functions::get_program_info_string(
            self.program_ptr(),
            ProgramInfo::KernelNames.into(),
        )?
        .split(';')
        .map(|s| s.to_string())
        .collect();
        Ok(names)
    }

    unsafe fn devices(&self) -> Output<Vec<Device>> {
        let devices = functions::get_program_info_vec_device(self.program_ptr())?
            .into_iter()
            .map(|d| {
                d.check().unwrap();
                Device::retain_new(d)
            })
            .collect();
        Ok(devices)
    }

    unsafe fn context(&self) -> Output<Context> {
        let raw_ctx = functions::get_program_info_context(self.program_ptr())?;
        raw_ctx.check()?;
        Ok(Context::retain_new(raw_ctx))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SRC: &'static str = "
    __kernel void test123(__global int *i) {
        *i += 1;
    }";

    #[test]
    fn program_ptr_reference_count() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let ref_count = unsafe { prog.reference_count().unwrap() };
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn cloning_increments_reference_count() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let prog2 = prog.clone();
        let prog3 = prog.clone();
        let ref_count = unsafe { prog.reference_count().unwrap() };
        assert_eq!(ref_count, 3);
        assert_eq!(prog, prog2);
        assert_eq!(prog, prog3);
    }

    #[test]
    fn program_ptr_num_devices() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let num_devices = unsafe { prog.num_devices().unwrap() };
        assert!(num_devices > 0);
    }

    #[test]
    fn program_ptr_devices() {
        let (prog, devices, _context) = ll_testing::get_program(SRC);
        let prog_devices = unsafe { prog.devices().unwrap() };
        let num_devices = unsafe { prog.num_devices().unwrap() };
        assert_eq!(num_devices as usize, prog_devices.len());
        assert_eq!(prog_devices.len(), devices.len());
    }

    #[test]
    fn program_ptr_context() {
        let (prog, _devices, context) = ll_testing::get_program(SRC);
        let prog_context = unsafe { prog.context().unwrap() };
        assert_eq!(prog_context, context);
    }

    #[test]
    fn num_devices_matches_devices_len() {
        let (prog, devices, _context) = ll_testing::get_program(SRC);
        let num_devices = unsafe { prog.num_devices().unwrap() };
        assert_eq!(num_devices as usize, devices.len());
    }

    #[test]
    fn program_ptr_source_matches_creates_src() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let prog_src = unsafe { prog.source().unwrap() };
        assert_eq!(prog_src, SRC.to_string());
    }

    #[test]
    fn program_ptr_num_kernels() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let num_kernels = unsafe { prog.num_kernels().unwrap() };
        assert_eq!(num_kernels, 1);
    }

    #[test]
    fn program_ptr_kernel_names() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let kernel_names = unsafe { prog.kernel_names().unwrap() };
        assert_eq!(kernel_names, vec!["test123"]);
    }

    #[test]
    fn num_kernels_matches_kernel_names_len() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let kernel_names = unsafe { prog.kernel_names().unwrap() };
        let num_kernels = unsafe { prog.num_kernels().unwrap() };
        assert_eq!(num_kernels, kernel_names.len());
    }
}
