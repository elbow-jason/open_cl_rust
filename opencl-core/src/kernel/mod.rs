use std::fmt::Debug;

pub mod flags;
pub mod low_level;
pub mod kernel_arg;

pub use kernel_arg::{KernelArg, KernelArgSizeAndPointer};

use crate::ffi::cl_kernel;

use low_level::cl_release_kernel;

use crate::error::{Error, Output};
use crate::program::Program;

/// An error related to a `Kernel`.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[fail(
        display = "Kernel arg index out of range. (kernel: {}, index: {})",
        kernel, index
    )]
    ArgIndexOutOfRange { kernel: String, index: u32 },
    #[fail(
        display = "Kernel argument type mismatch. (kernel: {}, index: [{}], \
                   arg_type {})",
        kernel, index, arg_type
    )]
    ArgTypeMismatch {
        kernel: String,
        index: u32,
        arg_type: String,
    },
    #[fail(
        display = "The wrong number of kernel arguments have been specified \
        (required: {}, specified: {}). Use named arguments with 'None' or zero values to \
        declare arguments you plan to assign a value to at a later time.",
        required, specified
    )]
    BuilderWrongArgCount { required: u32, specified: u32 },

    #[fail(display = "The kernel name '{}' could not be represented as a CString.",_0)]
    CStringInvalidKernelName(String),

    #[fail(display = "Kernel cannot be retained. It is short lived and only creatable as already.")]
    CannotBeRetained
}

impl From<KernelError> for Error {
    fn from(e: KernelError) -> Error {
        Error::KernelError(e)
    }
}

fn kernel_cannot_be_retained(_k: &cl_kernel) -> Output<()> {
    Err(KernelError::CannotBeRetained.into())
}

// cl_kernel is not thread-safe.
// cl_kernel should be a short lived, generate as needed structure;
// to be loaded with args then immediately enqueued and disposed of.
__impl_unconstructable_cl_wrapper!(Kernel, cl_kernel);
__impl_cl_object_for_wrapper!(Kernel, cl_kernel, kernel_cannot_be_retained, cl_release_kernel);
// Should we even implement clone? No for now.
// __impl_clone_for_cl_object_wrapper!(Kernel, cl_retain_kernel);
__impl_drop_for_cl_object_wrapper!(Kernel, cl_release_kernel);

impl Kernel {

    pub fn create(program: &Program, name: &str) -> Output<Kernel> {
        low_level::cl_create_kernel(program, name)
    }

    pub fn set_arg<T>(&self, arg_index: usize, arg: &T) -> Output<()>
    where
        T: KernelArg + Debug,
    {
        low_level::cl_set_kernel_arg(self, arg_index, arg)
    }
}

