use super::{functions, KernelArgPtr};
use crate::cl::{cl_kernel, strings, KernelInfo, ObjectWrapper};
use crate::{Context, Program, ProgramPtr};
use crate::{ErrorT, Output};
use std::fmt::Debug;

/// An error related to a `Kernel`.
#[derive(Debug, ErrorT, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[error("The kernel name '{0}' could not be represented as a CString.")]
    CStringInvalidKernelName(String),

    #[error("Work is required for kernel operation.")]
    WorkIsRequired,

    #[error("Returning arg index was out of range for kernel operation - index: {0}, argc: {1}")]
    ReturningArgIndexOutOfRange(usize, usize),

    #[error("The KernelOpArg was not a mem object type.")]
    KernelOpArgWasNotMem,

    #[error("The KernelOpArg was not a num type.")]
    KernelOpArgWasNotNum,
}

pub unsafe trait KernelPtr: Sized {
    unsafe fn kernel_ptr(&self) -> cl_kernel;

    unsafe fn function_name(&self) -> Output<String> {
        functions::get_info_string(self.kernel_ptr(), KernelInfo::FunctionName.into())
    }

    /// Returns the number of args for a kernel.
    unsafe fn num_args(&self) -> Output<u32> {
        functions::get_info_u32(self.kernel_ptr(), KernelInfo::NumArgs.into())
    }

    /// Returns the OpenCL reference count of the kernel.
    unsafe fn reference_count(&self) -> Output<u32> {
        functions::get_info_u32(self.kernel_ptr(), KernelInfo::ReferenceCount.into())
    }

    unsafe fn context(&self) -> Output<Context> {
        functions::get_info_context(self.kernel_ptr(), KernelInfo::Context.into())
            .map(|c| Context::retain_new(c))
    }

    unsafe fn program(&self) -> Output<Program> {
        functions::get_info_program(self.kernel_ptr(), KernelInfo::Program.into())
            .map(|p| Program::retain_new(p))
    }

    unsafe fn attributes(&self) -> Output<String> {
        functions::get_info_string(self.kernel_ptr(), KernelInfo::Attributes.into())
    }

    // // OpenCL v2.0
    // fn max_num_sub_groups(&self) -> Output<String> {
    //     self.info(KernelInfo::MaxNumSubGroups).map(|ret| ret.to_string())
    // }
    // fn compile_num_sub_groups(&self) -> Output<String> {
    //     self.info(KernelInfo::CompileNumSubGroups).map(|ret| ret.to_string())
    // }
}

pub type Kernel = ObjectWrapper<cl_kernel>;

impl Kernel {
    /// Creates a wrapped cl_kernel object.
    ///
    /// # Safety
    /// Calling this function with an invalid ClProgram is undefined behavior.
    pub unsafe fn create(program: &Program, name: &str) -> Output<Kernel> {
        let c_name = strings::to_c_string(name)
            .ok_or_else(|| KernelError::CStringInvalidKernelName(name.to_string()))?;
        functions::create_kernel(program.program_ptr(), c_name).map(|k| Kernel::new(k))
    }

    /// Set adds and arg to a kernel at a given index.
    ///
    /// # Safety
    /// Calling this function on invalid kernel or with invalid `arg` is undefined behavior.
    pub unsafe fn set_arg<T: KernelArgPtr>(&mut self, arg_index: usize, arg: &mut T) -> Output<()> {
        functions::set_kernel_arg(self.kernel_ptr(), arg_index, arg)
    }
}

unsafe impl KernelPtr for Kernel {
    unsafe fn kernel_ptr(&self) -> cl_kernel {
        self.cl_object()
    }
}

#[cfg(test)]
mod tests {
    // TODO: make tests for vectors, newtypes, half, bool, and isize.

    use crate::cl::{clSetKernelArg, cl_mem, ClObject};
    use crate::numbers::{Number, Uchar, Uchar2};
    use crate::{ll_testing, Context, Kernel, KernelPtr, MemPtr, Program, Session, SessionBuilder};

    const SRC: &'static str = "
    __kernel void test123(__global int *i) {
        *i += 1;
    }";

    const KERNEL_NAME: &'static str = "test123";

    #[test]
    fn kernel_can_be_created() {
        let (program, _devices, _context) = ll_testing::get_program(SRC);
        let _kernel: Kernel = unsafe { Kernel::create(&program, KERNEL_NAME).unwrap() };
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
        let context: Context = unsafe { kernel.context().unwrap() };
        assert_eq!(context, orig_context);
    }

    #[test]
    fn kernel_program_works() {
        let (_context, _devices, orig_program, kernel) = ll_testing::get_kernel(SRC, KERNEL_NAME);
        let program: Program = unsafe { kernel.program().unwrap() };
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
        let mut arg1 = 1u8;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i8_scalar() {
        let src: &str = "
        __kernel void test123(char i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i8;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u16_scalar() {
        let src: &str = "
        __kernel void test123(ushort i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u16;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i16_scalar() {
        let src: &str = "
        __kernel void test123(short i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i16;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u32_scalar() {
        let src: &str = "
        __kernel void test123(uint i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u32;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i32_scalar() {
        let src: &str = "
        __kernel void test123(int i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i32;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_f32_scalar() {
        let src: &str = "
        __kernel void test123(float i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1.0f32;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_u64_scalar() {
        let src: &str = "
        __kernel void test123(ulong i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1u64;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_args_works_for_i64_scalar() {
        let src: &str = "
        __kernel void test123(long i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1i64;
        assert_eq!(std::mem::size_of::<i64>(), std::mem::size_of::<i64>());
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_arg_works_for_f64_scalar() {
        let src: &str = "
        __kernel void test123(double i) {
            i + 1.0;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = 1.0f64;
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    #[test]
    fn kernel_set_arg_works_for_cl_uchar2() {
        let src: &str = "
        __kernel void test123(uchar2 i) {
            i[0] + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = Uchar2::from([1u8, 1]);
        let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    }

    // TODO: No vector3
    // #[test]
    // fn kernel_set_arg_works_for_cl_uchar3() {
    //     let src: &str = "
    //     __kernel void test123(uchar2 i) {
    //         i[0] + 1;
    //     }";
    //     let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
    //     let mut arg1: cl_uchar3 = *ClUchar4::from([1u8, 1, 1]).convert_to());
    //     assert_eq!(4, std::mem::size_of::<cl_uchar3>());
    //     let () = unsafe { kernel.set_arg(0, &mut arg1) }.unwrap();
    // }

    #[test]
    fn kernel_set_arg_works_for_cl_uchar() {
        let src: &str = "
        __kernel void test123(uchar i) {
            i + 1;
        }";
        let (_context, _devices, _program, mut kernel) = ll_testing::get_kernel(src, KERNEL_NAME);
        let mut arg1 = Uchar::new(1u8);
        assert_eq!(1, std::mem::size_of::<u8>());
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
            let mem1 = session.create_mem::<u8, &[u8]>(&data[..]).unwrap();

            // The `&` below is absolutely essential.
            let mem_ptr = &mem1.mem_ptr().as_ptr() as *const _ as *const libc::c_void;
            let err = clSetKernelArg(
                kernel.kernel_ptr().as_mut_ptr(),
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
            let mut mem1 = session.create_mem::<u8, &[u8]>(&data[..]).unwrap();
            assert_eq!(mem1.len().unwrap(), 2);
            let () = kernel.set_arg(0, &mut mem1).unwrap();
        }
    }
}
