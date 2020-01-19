use std::mem::ManuallyDrop;
use std::fmt;

use crate::ffi::cl_program;

use crate::{Context, Device};
use crate::ll::{ClProgram, ProgramPtr, Output, ClContext};

// pub const DEVICE_LIST_CANNOT_BE_EMPTY: Error = Error::ProgramError(ProgramError::CannotBuildProgramWithEmptyDevicesList);

// /// An error related to Program.
// #[derive(Debug, Fail, PartialEq, Eq, Clone)]
// pub enum ProgramError {
//     #[fail(display = "The given source code was not a valid CString")]
//     CStringInvalidSourceCode,

//     #[fail(display = "The given program binary was not a valid CString")]
//     CStringInvalidProgramBinary,

//     #[fail(display = "Cannot build a program with an empty list of devices")]
//     CannotBuildProgramWithEmptyDevicesList,
// }




pub struct UnbuiltProgram {
    context: ManuallyDrop<Context>,
    inner: ManuallyDrop<ClProgram>,
    _unconstructable: (),
}

impl UnbuiltProgram {
    pub unsafe fn new(program: ClProgram, context: Context) -> UnbuiltProgram {
        UnbuiltProgram{
            context: ManuallyDrop::new(context),
            inner: ManuallyDrop::new(program),
            _unconstructable: (),
        }
    }
}

impl ProgramPtr for UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}


impl ProgramPtr for &mut UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}


impl ProgramPtr for &UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}

unsafe impl Send for UnbuiltProgram {}

impl Drop for UnbuiltProgram {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
            ManuallyDrop::drop(&mut self.context);
        }
    }
}

impl Clone for UnbuiltProgram {
    fn clone(&self) -> UnbuiltProgram {
        UnbuiltProgram {
            context: self.context.clone(),
            inner: ManuallyDrop::new((*self.inner).clone()),
            _unconstructable: (),
        }
    }
}

impl fmt::Debug for UnbuiltProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnbuiltProgram{{{:?}}}", unsafe { self.program_ptr() })
    }
}

impl UnbuiltProgram {
    pub fn low_level_program(&self) -> &ClProgram {
        &self.inner
    }

    pub fn build(mut self, devices: &[Device]) -> Output<Program> {
        self.inner.build(devices)?;
        let built_prog: Program = unsafe {
            let (program_ptr, context_ptr, context_devices) = (
                self.program_ptr(),
                self.context.context_ptr(),
                self.context.devices()
            );
            let ll_program = ClProgram::new(program_ptr)?;
            let ll_context = ClContext::new(context_ptr)?;
            let hl_context = Context::build(ll_context, context_devices.to_vec());

            Program::new(
                ll_program,
                hl_context,
                devices.to_vec()
            )
        };
        std::mem::forget(self);
        Ok(built_prog)
    }
}

pub struct Program {
    _context: ManuallyDrop<Context>,
    _devices: ManuallyDrop<Vec<Device>>,
    inner: ManuallyDrop<ClProgram>,
    _unconstructable: (),
}

impl Program {
    pub fn create_with_source(context: &Context, src: &str) -> Output<UnbuiltProgram> {
        unsafe {
            let ll_prog = ClProgram::create_with_source(context.low_level_context(), src)?;
            Ok(UnbuiltProgram::new(ll_prog, context.clone()))
        }
    }

    pub fn create_program_with_binary(
        context: &Context,
        device: &Device,
        binary: &[u8],
    ) -> Output<UnbuiltProgram> {
        unsafe {
            let ll_prog = ClProgram::create_with_binary(
                context.low_level_context(),
                device.low_level_device(),
                binary
            )?;
            Ok(UnbuiltProgram::new(ll_prog, context.clone()))
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._devices);
        }
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program{
            _devices: ManuallyDrop::new((*self._devices).clone()),
            _context: self._context.clone(),
            inner: ManuallyDrop::new((*self.inner).clone()),
            _unconstructable: ()
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program{{{:?}}}", unsafe { self.program_ptr() })
    }
}

unsafe impl Sync for Program {}
unsafe impl Send for Program {}

impl ProgramPtr for Program {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}

impl ProgramPtr for &Program {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}

impl ProgramPtr for &mut Program {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).program_ptr()
    }
}

impl Program {
    pub unsafe fn new(object: ClProgram, context: Context, devices: Vec<Device>) -> Program {
        Program {
            inner: ManuallyDrop::new(object), 
            _context: ManuallyDrop::new(context),
            _devices: ManuallyDrop::new(devices),
            _unconstructable: (),
        }
    }

    pub fn devices(&self) -> &[Device] {
        &self._devices[..]
    }

    pub fn context(&self) -> &Context {
        &self._context
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(self.program_ptr(), other.program_ptr()) }
    }
}

impl Eq for Program {}



#[cfg(test)]
mod tests {
    use crate::*;
    use crate::ll::*;

    const SRC: &str = "
    __kernel void test(__global int *i) {        
        *i += 1;
    }
    ";

    // fn get_session() -> Session {
    //     Session::create_sessions(&[Device::default()], TEST_SRC).expect("Failed to create Session").remove(0)
    // }

    #[test]
    fn program_method_reference_count_works() {
        let program: Program = testing::get_program(SRC);
        let output: u32 = program
            .reference_count()
            .expect("Failed to call program.reference_count()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_context_works() {
        let program: Program = testing::get_program(SRC);
        let _output: &Context = program.context();
    }

    #[test]
    fn program_method_num_devices_works() {
        let program: Program = testing::get_program(SRC);
        let output: u32 = program
            .num_devices()
            .expect("Failed to call program.num_devices()");
        assert!(output > 0);
    }

    #[test]
    fn program_method_devices_works() {
        let program: Program = testing::get_program(SRC);
        let devices: &[Device] = program.devices();
        assert!(devices.len() > 0);
    }

    #[test]
    fn program_method_source_works() {
        let program: Program = testing::get_program(SRC);
        let output: String = program
            .source()
            .expect("Failed to call program.source()");
        assert_eq!(output, SRC.to_string());
    }

    #[test]
    fn program_method_binary_sizes_works() {
        let program: Program = testing::get_program(SRC);
        let output: Vec<usize> = program
            .binary_sizes()
            .expect("Failed to call program.binary_sizes()");
        assert_eq!(output.len(), program.devices().len());
    }

    #[test]
    fn program_method_binaries_works() {
        let program: Program = testing::get_program(SRC);
        let output: Vec<u8> = program
            .binaries()
            .expect("Failed to call program.binaries()");
        let n_devices = program.devices().len();
        let n_bytes = n_devices * 8;
        assert_eq!(output.len(), n_bytes);
        for byte in output.into_iter() {
            assert_eq!(byte, 0u8);
        }
    }

    #[test]
    fn program_method_num_kernels_works() {
        let program: Program = testing::get_program(SRC);
        let output: usize = program
            .num_kernels()
            .expect("Failed to call program.num_kernels()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_kernel_names_works() {
        let program: Program = testing::get_program(SRC);
        let output: Vec<String> = program
            .kernel_names()
            .expect("Failed to call program.kernel_names()");
        let expected = vec!["test".to_string()];
        assert_eq!(output, expected);
    }
}
