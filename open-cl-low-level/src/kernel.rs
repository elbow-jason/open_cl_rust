use std::fmt::Debug;

use libc::{c_void};

use crate::cl_helpers::cl_get_info5;
use crate::ffi::{
    clCreateKernel, clGetKernelInfo, clSetKernelArg, cl_context, cl_kernel, cl_kernel_info, cl_mem,
    cl_program, cl_uint,
};
use crate::{
    build_output, strings, ClContext, ClMem, ClPointer, ClProgram,
    CommandQueueOptions, Dims, KernelInfo, MemPtr, Output, ProgramPtr, Work,
    ObjectWrapper
};

pub unsafe trait KernelArg {
    /// size_of<T> or size_of<T> * len
    fn kernel_arg_size(&self) -> usize;
    unsafe fn kernel_arg_ptr(&self) -> *const c_void;
    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void;
}

unsafe impl KernelArg for ClMem {
    fn kernel_arg_size(&self) -> usize {
        std::mem::size_of::<cl_mem>()
    }
    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self.mem_ptr_ref() as *const _ as *const c_void
    }

    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self.mem_ptr_ref() as *const _ as *mut c_void
    }
}

macro_rules! sized_scalar_kernel_arg {
    ($scalar:ty) => {
        unsafe impl KernelArg for $scalar {
            fn kernel_arg_size(&self) -> usize {
                std::mem::size_of::<$scalar>()
            }

            unsafe fn kernel_arg_ptr(&self) -> *const c_void {
                (self as *const $scalar) as *const c_void
            }

            unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
                (self as *mut $scalar) as *mut c_void
            }
        }
    };
}

sized_scalar_kernel_arg!(isize);
sized_scalar_kernel_arg!(i8);
sized_scalar_kernel_arg!(i16);
sized_scalar_kernel_arg!(i32);
sized_scalar_kernel_arg!(i64);

sized_scalar_kernel_arg!(usize);
sized_scalar_kernel_arg!(u8);
sized_scalar_kernel_arg!(u16);
sized_scalar_kernel_arg!(u32);
sized_scalar_kernel_arg!(u64);

sized_scalar_kernel_arg!(f32);
sized_scalar_kernel_arg!(f64);

// pub use kernel_arg::{KernelArg, KernelArgSizeAndPointer};
// use super::kernel_arg::{KernelArg, KernelArgSizeAndPointer};
// use super::{Kernel, KernelError, KernelLock, KernelPtr, KernelRefCountWithProgram};

pub unsafe fn cl_set_kernel_arg<T: KernelArg>(
    kernel: cl_kernel,
    arg_index: usize,
    arg: &T,
) -> Output<()> {
    cl_set_kernel_arg_raw(
        kernel,
        arg_index as cl_uint,
        arg.kernel_arg_size(),
        arg.kernel_arg_ptr()
    )
}

pub unsafe fn cl_set_kernel_arg_raw(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_size: usize,
    arg_ptr: *const c_void,
) -> Output<()> {
    let err_code = clSetKernelArg(
        kernel,
        arg_index as cl_uint,
        arg_size,
        arg_ptr,
    );

    build_output((), err_code)
}


pub unsafe fn cl_create_kernel(program: cl_program, name: &str) -> Output<cl_kernel> {
    let c_name = strings::to_c_string(name)
        .ok_or_else(|| KernelError::CStringInvalidKernelName(name.to_string()))?;
    let mut err_code = 0;
    let kernel: cl_kernel = clCreateKernel(program, c_name.as_ptr(), &mut err_code);
    build_output(kernel, err_code)
}

pub unsafe fn cl_get_kernel_info<T: Copy>(
    kernel: cl_kernel,
    flag: cl_kernel_info,
) -> Output<ClPointer<T>> {
    cl_get_info5(kernel, flag, clGetKernelInfo)
}

/// An error related to a `Kernel`.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[fail(
        display = "The kernel name '{}' could not be represented as a CString.",
        _0
    )]
    CStringInvalidKernelName(String),

    #[fail(display = "Work is required for kernel operation.")]
    WorkIsRequired,

    #[fail(
        display = "Returning arg index was out of range for kernel operation - index: {:?}, argc: {:?}",
        _0, _1
    )]
    ReturningArgIndexOutOfRange(usize, usize),

    #[fail(display = "The KernelOpArg was not a mem object type.")]
    KernelOpArgWasNotMem,

    #[fail(display = "The KernelOpArg was not a num type.")]
    KernelOpArgWasNotNum,
}

pub unsafe trait KernelPtr: Sized {
    unsafe fn kernel_ptr(&self) -> cl_kernel;

    unsafe fn info<T: Copy>(&self, flag: KernelInfo) -> Output<ClPointer<T>> {
        cl_get_kernel_info(self.kernel_ptr(), flag.into())
    }

    unsafe fn function_name(&self) -> Output<String> {
        self.info(KernelInfo::FunctionName)
            .map(|ret| ret.into_string())
    }

    /// Returns the number of args for a kernel.
    unsafe fn num_args(&self) -> Output<u32> {
        self.info(KernelInfo::NumArgs).map(|ret| ret.into_one())
    }

    /// Returns the OpenCL reference count of the kernel.
    unsafe fn reference_count(&self) -> Output<u32> {
        self.info(KernelInfo::ReferenceCount)
            .map(|ret| ret.into_one())
    }

    unsafe fn context(&self) -> Output<ClContext> {
        self.info::<cl_context>(KernelInfo::Context)
            .and_then(|cl_ptr| ClContext::retain_new(cl_ptr.into_one()))
    }

    unsafe fn program(&self) -> Output<ClProgram> {
        self.info::<cl_program>(KernelInfo::Program)
            .and_then(|cl_ptr| ClProgram::retain_new(cl_ptr.into_one()))
    }

    unsafe fn attributes(&self) -> Output<String> {
        self.info(KernelInfo::Attributes)
            .map(|ret| ret.into_string())
    }

    // // OpenCL v2.0
    // fn max_num_sub_groups(&self) -> Output<String> {
    //     self.info(KernelInfo::MaxNumSubGroups).map(|ret| ret.to_string())
    // }
    // fn compile_num_sub_groups(&self) -> Output<String> {
    //     self.info(KernelInfo::CompileNumSubGroups).map(|ret| ret.to_string())
    // }
}

// /// ClKernel is a low-level object wrapper for cl_kernel. ClKernel implements Drop that
// /// utilizes clRetainKernel and Clone that utilizes clReleaseKernel.
// ///
// /// # Safety
// /// Using ClKernel in an invalid state is undefined behavior.
// ///
// /// # Lifetime Dependencies
// /// ClKernel depends on cl_program and, by proxy, cl_context.
// pub struct ClKernel {
//     object: cl_kernel,
//     _unconstructable: (),
// }

pub type ClKernel = ObjectWrapper<cl_kernel>;

impl ClKernel {
    /// Creates a wrapped cl_kernel object.
    ///
    /// # Safety
    /// Calling this function with an invalid ClProgram is undefined behavior.
    pub unsafe fn create(program: &ClProgram, name: &str) -> Output<ClKernel> {
        cl_create_kernel(program.program_ptr(), name).and_then(|object| ClKernel::new(object))
    }

    /// Set adds and arg to a kernel at a given index.
    ///
    /// # Safety
    /// Calling this function on invalid kernel or with invalid `arg` is undefined behavior.
    pub unsafe fn set_arg<T: KernelArg>(&mut self, arg_index: usize, arg: &mut T) -> Output<()> {
        cl_set_kernel_arg(self.kernel_ptr(), arg_index, arg)
    }

    pub unsafe fn set_arg_raw(&mut self, arg_index: u32, arg_size: usize, arg_ptr: *const c_void) -> Output<()> {
        cl_set_kernel_arg_raw(self.kernel_ptr(), arg_index, arg_size, arg_ptr)
    }
}

unsafe impl KernelPtr for ClKernel {
    unsafe fn kernel_ptr(&self) -> cl_kernel {
        self.cl_object()
    }
}

pub struct KernelOperation {
    _name: String,
    _args: Vec<(usize, *const c_void)>,
    _work: Option<Work>,
    pub command_queue_opts: Option<CommandQueueOptions>,
}

impl KernelOperation{
    pub fn new(name: &str) -> KernelOperation {
        KernelOperation {
            _name: name.to_owned(),
            _args: vec![],
            _work: None,
            command_queue_opts: None,
        }
    }

    pub fn name(&self) -> &str {
        &self._name[..]
    }

    pub fn command_queue_opts(&self) -> Option<CommandQueueOptions> {
        self.command_queue_opts.clone()
    }

    pub fn args(&self) -> &[(usize, *const c_void)] {
        &self._args[..]
    }

    pub fn mut_args(&mut self) -> &mut [(usize, *const c_void)] {
        &mut self._args[..]
    }

    pub fn with_dims<D: Into<Dims>>(mut self, dims: D) -> KernelOperation {
        self._work = Some(Work::new(dims.into()));
        self
    }

    pub fn with_work<W: Into<Work>>(mut self, work: W) -> KernelOperation {
        self._work = Some(work.into());
        self
    }

    pub fn add_arg<A>(mut self, arg: &mut A) -> KernelOperation where A: KernelArg {
        self._args.push((arg.kernel_arg_size(), unsafe { arg.kernel_arg_ptr() }));
        self
    }

    pub fn with_command_queue_options(mut self, opts: CommandQueueOptions) -> KernelOperation {
        self.command_queue_opts = Some(opts);
        self
    }

    pub fn argc(&self) -> usize {
        self._args.len()
    }

    #[inline]
    pub fn work(&self) -> Output<Work> {
        self._work
            .clone()
            .ok_or_else(|| KernelError::WorkIsRequired.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi::*;
    use crate::*;
    use libc::c_void;

    const SRC: &'static str = "
    __kernel void test123(__global int *i) {
        *i += 1;
    }";

    const KERNEL_NAME: &'static str = "test123";

    #[test]
    fn kernel_can_be_created() {
        let (program, _devices, _context) = ll_testing::get_program(SRC);
        let _kernel: ClKernel = unsafe { ClKernel::create(&program, KERNEL_NAME).unwrap() };
    }

    #[test]
    fn kernel_function_name_works() {
        let (_context, _devices, _program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let function_name = unsafe { kernel.function_name().unwrap() };
        assert_eq!(function_name, KERNEL_NAME);
    }

    #[test]
    fn kernel_num_args_works() {
        let (_context, _devices, _program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let num_args = unsafe { kernel.num_args().unwrap() };
        assert_eq!(num_args, 1);
    }

    #[test]
    fn kernel_reference_count_works() {
        let (_context, _devices, _program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let ref_count = unsafe { kernel.reference_count().unwrap() };
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn kernel_context_works() {
        let (orig_context, _devices, _program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let context: ClContext = unsafe { kernel.context().unwrap() };
        assert_eq!(context, orig_context);
    }

    #[test]
    fn kernel_program_works() {
        let (_context, _devices, orig_program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let program: ClProgram = unsafe { kernel.program().unwrap() };
        assert_eq!(program, orig_program);
    }

    #[test]
    fn kernel_attributes_works() {
        let (_context, _devices, _program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let _attributes: String = unsafe { kernel.attributes().unwrap() };
    }

    #[test]
    fn kernel_set_args_works_for_u8_scalar() {
        let src: &str = "
        __kernel void test123(uchar i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u8 as cl_uchar;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i8_scalar() {
        let src: &str = "
        __kernel void test123(char i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i8 as cl_char;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u16_scalar() {
        let src: &str = "
        __kernel void test123(ushort i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u16 as cl_ushort;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i16_scalar() {
        let src: &str = "
        __kernel void test123(short i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i16 as cl_ushort;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u32_scalar() {
        let src: &str = "
        __kernel void test123(uint i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u32 as cl_uint;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i32_scalar() {
        let src: &str = "
        __kernel void test123(int i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i32 as cl_uint;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_f32_scalar() {
        let src: &str = "
        __kernel void test123(float i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1.0f32 as cl_float;
        assert_eq!(std::mem::size_of::<cl_float>(), 4);
        assert_eq!(std::mem::size_of::<f32>(), std::mem::size_of::<cl_float>());
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u64_scalar() {
        let src: &str = "
        __kernel void test123(ulong i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u64 as cl_ulong;
        assert_eq!(std::mem::size_of::<u64>(), std::mem::size_of::<cl_ulong>());
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i64_scalar() {
        let src: &str = "
        __kernel void test123(long i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i64 as cl_long;
        assert_eq!(std::mem::size_of::<i64>(), std::mem::size_of::<cl_long>());
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_arg_works_for_f64_scalar() {
        let src: &str = "
        __kernel void test123(double i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1.0f64 as cl_double;
        assert_eq!(std::mem::size_of::<f64>(), std::mem::size_of::<cl_double>());
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    fn build_session(src: &str) -> Session {
        unsafe { SessionBuilder::new().with_program_src(src).build().unwrap() }
    }

    #[test]
    fn kernel_set_arg_works_for_ffi_call() {
        unsafe {
            let src: &str = "
            __kernel void test123(__global uchar *i) {
                *i += 1;
            }";

            let session = build_session(src);
            let kernel = session.create_kernel("test123").unwrap();

            let data = vec![0u8, 0u8];
            let mem1 = session.create_mem(&data[..]).unwrap();
            let mem_ptr = &mem1.mem_ptr() as *const _ as *const c_void;
            let err = clSetKernelArg(
                kernel.kernel_ptr(),
                0,
                std::mem::size_of::<cl_mem>(),
                mem_ptr,
            );
            assert_eq!(err, 0);
        }
    }

    #[test]
    fn kernel_set_arg_works_for_buffer_u8() {
        unsafe {
            let src: &str = "
            __kernel void test123(__global uchar *i) {
                *i += 1;
            }";

            let session = build_session(src);
            let mut kernel = session.create_kernel("test123").unwrap();

            let data = vec![0u8, 0u8];
            let mut mem1 = session.create_mem(&data[..]).unwrap();
            assert_eq!(mem1.len().unwrap(), 2);
            let () = kernel.set_arg(0, &mut mem1).unwrap();
        }
    }
}
