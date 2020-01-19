// use std::fmt::Debug;

// use crate::ffi::{clCreateKernel, clSetKernelArg, cl_kernel, cl_uint};

// use crate::error::Output;
// use crate::program::{Program, ProgramPtr};
// use crate::opencl_low_level::strings;
// use crate::utils::StatusCode;

// use super::kernel_arg::{KernelArg, KernelArgSizeAndPointer};
// use super::{Kernel, KernelError, KernelLock, KernelPtr, KernelRefCountWithProgram};

// __release_retain!(kernel, Kernel);

// pub fn cl_set_kernel_arg<T, K, P>(kernel: &K, arg_index: usize, arg: &T) -> Output<()>
// where
//     T: KernelArg + Debug,
//     K: KernelLock<P>,
//     P: KernelPtr,
// {
//     let err_code = unsafe {
//         let (arg_size, arg_ptr): KernelArgSizeAndPointer = arg.as_kernel_arg();

//         let write_lock = kernel.write_lock();
//         let kernel_ptr = write_lock.kernel_ptr();

//         clSetKernelArg(
//             kernel_ptr,
//             arg_index as cl_uint,
//             arg_size,
//             arg_ptr,
//         )
//     };
//     StatusCode::build_output(err_code, ())
// }

// pub fn cl_create_kernel(program: &Program, name: &str) -> Output<Kernel> {
//     let mut err_code = 0;
//     let c_name = strings::to_c_string(name)
//         .ok_or_else(|| KernelError::CStringInvalidKernelName(name.to_string()))?;
//     let kernel = unsafe { clCreateKernel(program.program_ptr(), c_name.as_ptr(), &mut err_code) };
//     StatusCode::build_output(err_code, ())?;
//     unsafe { Kernel::from_retained(kernel, program.clone()) }
// }
