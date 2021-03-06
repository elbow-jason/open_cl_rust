use std::fmt;
use std::mem::ManuallyDrop;

use crate::ll::cl::ClObject;

use crate::ll::{Context as ClContext, ContextPtr, Program as ClProgram, ProgramPtr};
use crate::{Context, Device, Output};

pub struct UnbuiltProgram {
    context: ManuallyDrop<ClContext>,
    inner: ManuallyDrop<ClProgram>,
    _unconstructable: (),
}

impl UnbuiltProgram {
    pub unsafe fn new(program: ClProgram, context: ClContext) -> UnbuiltProgram {
        UnbuiltProgram {
            context: ManuallyDrop::new(context),
            inner: ManuallyDrop::new(program),
            _unconstructable: (),
        }
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
        write!(f, "UnbuiltProgram{{{:?}}}", self.inner)
    }
}

impl UnbuiltProgram {
    pub fn create_with_source(context: &Context, src: &str) -> Output<UnbuiltProgram> {
        unsafe {
            let ll_prog = ClProgram::create_with_src(context.low_level_context(), src)?;
            Ok(UnbuiltProgram::new(
                ll_prog,
                context.low_level_context().clone(),
            ))
        }
    }

    pub fn create_with_binary(
        context: &Context,
        device: &Device,
        binary: &[u8],
    ) -> Output<UnbuiltProgram> {
        unsafe {
            let ll_prog = ClProgram::create_with_binary(
                context.low_level_context(),
                device.low_level_device(),
                binary,
            )?;
            Ok(UnbuiltProgram::new(
                ll_prog,
                context.low_level_context().clone(),
            ))
        }
    }

    pub fn build(mut self, devices: &[Device]) -> Output<Program> {
        let built_prog: Program = unsafe {
            self.inner.build(devices)?;
            let (program_ptr, context_ptr, context_devices) = (
                self.inner.program_ptr(),
                self.context.context_ptr(),
                self.context.devices()?,
            );

            let context_devices2: Vec<Device> = context_devices
                .into_iter()
                .map(|d| Device::new(d))
                .collect();

            let ll_program = ClProgram::new(program_ptr);
            let ll_context = ClContext::new(context_ptr);
            let hl_context = Context::build(ll_context, context_devices2);
            Program::new(ll_program, hl_context, devices.to_vec())
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
        UnbuiltProgram::create_with_source(context, src)
    }

    pub fn create_with_binary(
        context: &Context,
        device: &Device,
        binary: &[u8],
    ) -> Output<UnbuiltProgram> {
        UnbuiltProgram::create_with_binary(context, device, binary)
    }

    pub unsafe fn new(object: ClProgram, context: Context, devices: Vec<Device>) -> Program {
        Program {
            inner: ManuallyDrop::new(object),
            _context: ManuallyDrop::new(context),
            _devices: ManuallyDrop::new(devices),
            _unconstructable: (),
        }
    }

    pub unsafe fn from_low_level_program(program: &ClProgram) -> Output<Program> {
        let ll_devices = program.devices()?;
        let ll_context = program.context()?;
        let hl_devices = ll_devices.into_iter().map(|d| Device::new(d)).collect();
        let hl_context = Context::from_low_level_context(&ll_context)?;
        Ok(Program::new(program.clone(), hl_context, hl_devices))
    }

    pub fn devices(&self) -> &[Device] {
        &self._devices[..]
    }

    pub fn context(&self) -> &Context {
        &self._context
    }

    pub fn low_level_program(&self) -> &ClProgram {
        &self.inner
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.inner.reference_count() }
    }

    pub fn num_devices(&self) -> Output<u32> {
        unsafe { self.inner.num_devices() }
    }

    pub fn source(&self) -> Output<String> {
        unsafe { self.inner.source() }
    }

    pub fn binary_sizes(&self) -> Output<Vec<usize>> {
        unsafe { self.inner.binary_sizes() }
    }

    pub fn binaries(&self) -> Output<Vec<u8>> {
        unsafe { self.inner.binaries() }
    }

    pub fn num_kernels(&self) -> Output<usize> {
        unsafe { self.inner.num_kernels() }
    }

    pub fn kernel_names(&self) -> Output<Vec<String>> {
        unsafe { self.inner.kernel_names() }
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
        Program {
            _devices: ManuallyDrop::new((*self._devices).clone()),
            _context: self._context.clone(),
            inner: ManuallyDrop::new((*self.inner).clone()),
            _unconstructable: (),
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program{{{}}}", self.inner.address())
    }
}

unsafe impl Sync for Program {}
unsafe impl Send for Program {}

// unsafe impl ProgramPtr for Program {
//     unsafe fn program_ptr(&self) -> cl_program {
//         (*self.inner).program_ptr()
//     }
// }

// unsafe impl ProgramPtr for &Program {
//     unsafe fn program_ptr(&self) -> cl_program {
//         (*self.inner).program_ptr()
//     }
// }

// unsafe impl ProgramPtr for &mut Program {
//     unsafe fn program_ptr(&self) -> cl_program {
//         (*self.inner).program_ptr()
//     }
// }

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            std::ptr::eq(
                self.inner.program_ptr().as_ptr(),
                other.inner.program_ptr().as_ptr(),
            )
        }
    }
}

impl Eq for Program {}

#[cfg(test)]
mod tests {
    use crate::*;

    const SRC: &str = "
    __kernel void test(__global int *i) {        
        *i += 1;
    }
    ";

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
        assert_eq!(output, program.devices().len() as u32);
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
        let output: String = program.source().expect("Failed to call program.source()");
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

    // TODO: This currently causes a segfault on Linux. Fix it.
    // #[test]
    // fn program_method_binaries_works() {
    //     let program: Program = testing::get_program(SRC);
    //     let output: Vec<u8> = program
    //         .binaries()
    //         .expect("Failed to call program.binaries()");
    //     let n_devices = program.devices().len();
    //     let n_bytes = n_devices * 8;
    //     assert_eq!(output.len(), n_bytes);
    //     for byte in output.into_iter() {
    //         assert_eq!(byte, 0u8);
    //     }
    // }

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
