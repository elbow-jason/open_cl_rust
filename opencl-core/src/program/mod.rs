pub mod flags;
pub mod low_level;

use std::mem::ManuallyDrop;
use std::fmt;

use crate::ffi::{cl_program, cl_context};

use crate::{
    utils,
    Context, ContextRefCount,
    Device, DevicePtr, DeviceRefCount,
    Error, Output
};

// use low_level::{cl_release_program, cl_retain_program};

use crate::cl::ClPointer;

use flags::ProgramInfo;


pub const DEVICE_LIST_CANNOT_BE_EMPTY: Error = Error::ProgramError(ProgramError::CannotBuildProgramWithEmptyDevicesList);

/// An error related to Program.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ProgramError {
    #[fail(display = "The given source code was not a valid CString")]
    CStringInvalidSourceCode,

    #[fail(display = "The given program binary was not a valid CString")]
    CStringInvalidProgramBinary,

    #[fail(display = "Cannot build a program with an empty list of devices")]
    CannotBuildProgramWithEmptyDevicesList,
}


fn get_info<T: Copy, P: ProgramPtr>(program: &P, flag: ProgramInfo) -> Output<ClPointer<T>> {
    low_level::cl_get_program_info(program, flag)
}

pub trait ProgramPtr: Sized {
    unsafe fn program_ptr(&self) -> cl_program;

    fn reference_count(&self) -> Output<u32> {
        get_info(self, ProgramInfo::ReferenceCount)
            .map(|ret| unsafe { ret.into_one() })
    }

    fn context(&self) -> Output<Context> {
        get_info(self, ProgramInfo::Context)
            .and_then(|cl_ptr| unsafe { Context::from_unretained(cl_ptr.into_one()) })
    }

    fn num_devices(&self) -> Output<u32> {
        get_info(self, ProgramInfo::NumDevices)
            .map(|ret| unsafe { ret.into_one() })
    }

    fn devices(&self) -> Output<Vec<Device>> {
        get_info(self, ProgramInfo::Devices)
            .and_then(|ret| unsafe {
                ret.into_vec()
                    .into_iter()
                    .map(|d| Device::from_unretained(d))
                    .collect()
            })
    }

    fn source(&self) -> Output<String> {
        get_info(self, ProgramInfo::Source)
            .map(|ret| unsafe { ret.into_string() })
    }
    fn binary_sizes(&self) -> Output<Vec<usize>> {
        get_info(self, ProgramInfo::BinarySizes)
            .map(|ret| unsafe { ret.into_vec() })
    }

    fn binaries(&self) -> Output<Vec<u8>> {
        get_info(self, ProgramInfo::Binaries)
            .map(|ret| unsafe { ret.into_vec() })
    }

    fn num_kernels(&self) -> Output<usize> {
        get_info(self, ProgramInfo::NumKernels)
            .map(|ret| unsafe { ret.into_one() })
    }

    fn kernel_names(&self) -> Output<Vec<String>> {
        get_info(self, ProgramInfo::KernelNames).map(|ret| {
            let kernels: String = unsafe { ret.into_string() };
            kernels.split(';').map(|s| s.to_string()).collect()
        })
    }
}

pub unsafe fn release_program(program: cl_program) {
    low_level::cl_release_program(program).unwrap_or_else(|e| {
        panic!("Failed to release cl_program {:?} due to {:?}", program, e);
    });
}

pub unsafe fn retain_program(program: cl_program) {
    low_level::cl_retain_program(program).unwrap_or_else(|e| {
        panic!("Failed to retain cl_program {:?} due to {:?}", program, e);
    });
}
    

struct ProgramObject {
    object: cl_program,
    _unconstructable: ()
}

impl ProgramObject {
    pub unsafe fn unchecked_new(program: cl_program) -> ProgramObject {
        ProgramObject{
            object: program,
            _unconstructable: (),
        }
    }

    unsafe fn from_retained(prog: cl_program) -> Output<ProgramObject> {
        utils::null_check(prog, "ProgramObject::from_retained")?;
        Ok(ProgramObject::unchecked_new(prog))
    }
}

impl Drop for ProgramObject {
    fn drop(&mut self) {
        unsafe { release_program(self.object) };
    }
}

impl Clone for ProgramObject {
    fn clone(&self) -> ProgramObject {
        unsafe {
            retain_program(self.object);
            ProgramObject::unchecked_new(self.object)
        }
    }
}

pub struct UnbuiltProgram {
    context: ManuallyDrop<Context>,
    inner: ManuallyDrop<ProgramObject>,
    _unconstructable: (),
}

impl UnbuiltProgram {
    pub unsafe fn new(program: cl_program, context: Context) -> UnbuiltProgram {
        UnbuiltProgram{
            context: ManuallyDrop::new(context),
            inner: ManuallyDrop::new(ProgramObject::unchecked_new(program)),
            _unconstructable: (),
        }
    }
}

impl ProgramPtr for UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).object
    }
}


impl ProgramPtr for &mut UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).object
    }
}


impl ProgramPtr for &UnbuiltProgram {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).object
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
    pub unsafe fn decompose(self) -> (cl_context, cl_program) {
        let program_ptr = self.program_ptr();
        let context_ptr = self.context.context_ptr();
        std::mem::forget(self);
        (context_ptr, program_ptr)
    }

    pub fn create_with_source(context: &Context, src: &str) -> Output<UnbuiltProgram> {
        low_level::cl_create_program_with_source(context, src)
    }

    pub fn create_program_with_binary(
        context: &Context,
        device: &Device,
        binary: &str,
    ) -> Output<UnbuiltProgram> {
        low_level::cl_create_program_with_binary(context, device, binary)
    }

    pub fn build<D>(self, devices: &[D]) -> Output<Vec<Program>> where D: DevicePtr  {
        let len = devices.len() ;
        match len {
            0 => Err(DEVICE_LIST_CANNOT_BE_EMPTY),
            n => {
                let mut built_programs: Vec<Program> = Vec::with_capacity(devices.len());
                for i in 0..(n - 1) {
                    let built_prog: Program = unsafe {
                        low_level::cl_build_program(self.clone(), &devices[i])
                    }?;
                    built_programs.push(built_prog);
                }
                let last_program = unsafe {
                    low_level::cl_build_program(self.clone(), &devices[len - 1])
                }?;
                built_programs.push(last_program);
                Ok(built_programs)
            }
        }
    }
}

pub struct Program {
    _context: ManuallyDrop<Context>,
    _device: ManuallyDrop<Device>,
    inner: ManuallyDrop<ProgramObject>,
    _unconstructable: (),
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._device);
        }
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program{
            _device: ManuallyDrop::new((*self._device).clone()),
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
        (*self.inner).object
    }
}

impl ProgramPtr for &Program {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).object
    }
}

impl ProgramPtr for &mut Program {
      unsafe fn program_ptr(&self) -> cl_program {
        (*self.inner).object
    }
}

impl Program {
    pub unsafe fn new(object: cl_program, mut context: Context, mut device: Device) -> Output<Program> {
        match ProgramObject::from_retained(object) {
            Err(e) => {
                // If an error occures we have to drop the parents in the correct order.
                std::mem::drop(&mut context);
                std::mem::drop(&mut device);
                std::mem::forget(context);
                std::mem::forget(device);
                return Err(e)
            },
            Ok(program_object) => {
                Ok(Program {
                    inner: ManuallyDrop::new(program_object), 
                    _context: ManuallyDrop::new(context),
                    _device: ManuallyDrop::new(device),
                    _unconstructable: (),
                })
            }
        }
    }

    pub fn get_log(program: &Program, device: &Device) -> Output<String> {
        let flag = flags::ProgramBuildInfo::Log;
        low_level::cl_get_program_build_log(program, device, flag)
            .map(|ret| unsafe { ret.into_string() })
    }
    pub fn device(&self) -> &Device {
        &self._device
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

    const TEST_SRC: &str = "
    __kernel void test(__global int *i) {        
        *i += 1;
    }
    ";

    fn get_session() -> Session {
        Session::create_sessions(&[Device::default()], TEST_SRC).expect("Failed to create Session").remove(0)
    }

    #[test]
    fn program_method_reference_count_works() {
        let session = get_session();
        let output: u32 = session
            .program()
            .reference_count()
            .expect("Failed to call program.reference_count()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_context_works() {
        let session = get_session();
        let _output: &Context = session.program().context();
    }

    #[test]
    fn program_method_num_devices_works() {
        let session = get_session();
        let output: u32 = session
            .program()
            .num_devices()
            .expect("Failed to call program.num_devices()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_devices_works() {
        let session = get_session();
        let output: Vec<Device> = session
            .program()
            .devices()
            .expect("Failed to call program.devices()");
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn program_method_source_works() {
        let session = get_session();
        let output: String = session
            .program()
            .source()
            .expect("Failed to call program.source()");
        assert_eq!(output, TEST_SRC.to_string());
    }

    #[test]
    fn program_method_binary_sizes_works() {
        let session = get_session();
        let output: Vec<usize> = session
            .program()
            .binary_sizes()
            .expect("Failed to call program.binary_sizes()");
        let expected = vec![1332];
        assert_eq!(output, expected);
    }

    #[test]
    fn program_method_binaries_works() {
        let session = get_session();
        let output: Vec<u8> = session
            .program()
            .binaries()
            .expect("Failed to call program.binaries()");
        let expected = vec![0u8, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(output, expected);
    }

    #[test]
    fn program_method_num_kernels_works() {
        let session = get_session();
        let output: usize = session
            .program()
            .num_kernels()
            .expect("Failed to call program.num_kernels()");
        assert_eq!(output, 1);
    }

    #[test]
    fn program_method_kernel_names_works() {
        let session = get_session();
        let output: Vec<String> = session
            .program()
            .kernel_names()
            .expect("Failed to call program.kernel_names()");
        let expected = vec!["test".to_string()];
        assert_eq!(output, expected);
    }
}
