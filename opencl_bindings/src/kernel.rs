use crate::ffi::{cl_kernel, cl_kernel_info, cl_kernel_work_group_info, cl_mem};
use crate::open_cl::{cl_release_kernel, cl_set_kernel_arg, ClObject, Error, Output};
use libc::{c_void, size_t};
use std::fmt::Debug;

use crate::DeviceMem;

/// An error related to a `Kernel`, `KernelBuilder`, or `KernelCmd`.
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
}

pub type KernelArgSizeAndPointer = (size_t, *const c_void);

impl From<KernelError> for Error {
    fn from(e: KernelError) -> Error {
        Error::KernelError(e)
    }
}

pub trait KernelArg {
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer;
}

impl<T> KernelArg for DeviceMem<T>
where
    T: Debug,
{
    unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
        (
            std::mem::size_of::<cl_mem>() as size_t,
            self.ptr_to_cl_object() as *const c_void,
        )
    }
}

macro_rules! sized_scalar_kernel_arg {
    ($scalar:ty) => {
        impl KernelArg for $scalar {
            unsafe fn as_kernel_arg(&self) -> KernelArgSizeAndPointer {
                (
                    std::mem::size_of::<$scalar>() as size_t,
                    (self as *const $scalar) as *const c_void,
                )
            }
        }
    };
}

sized_scalar_kernel_arg!(isize);
sized_scalar_kernel_arg!(i32);
sized_scalar_kernel_arg!(i64);

sized_scalar_kernel_arg!(usize);
sized_scalar_kernel_arg!(u32);
sized_scalar_kernel_arg!(u64);

sized_scalar_kernel_arg!(f32);
sized_scalar_kernel_arg!(f64);

/// cl_kernel is not thread-safe.
/// cl_kernel should be a short lived, generate as needed structure;
/// to be loaded with args then immediately enqueued and disposed of.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Kernel {
    inner: cl_kernel,
    _unconstructable: (),
}

impl ClObject<cl_kernel> for Kernel {
    unsafe fn raw_cl_object(&self) -> cl_kernel {
        self.inner
    }
}

impl Kernel {
    pub fn new(inner: cl_kernel) -> Kernel {
        Kernel {
            inner,
            _unconstructable: (),
        }
    }

    pub fn set_arg<T>(&self, arg_index: usize, arg: &T) -> Output<()>
    where
        T: KernelArg + Debug,
    {
        cl_set_kernel_arg(self, arg_index, arg)
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        unsafe {
            cl_release_kernel(&self.raw_cl_object());
        }
    }
}

crate::__codes_enum!(KernelInfo, cl_kernel_info, {
    FunctionName => 0x1190,
    NumArgs => 0x1191,
    ReferenceCount => 0x1192,
    Context => 0x1193,
    Program => 0x1194,
    Attributes => 0x1195,
    MaxNumSubGroups => 0x11B9,
    CompileNumSubGroups => 0x11BA
});

crate::__codes_enum!(KernelWorkGroupInfo, cl_kernel_work_group_info, {
    WorkGroupSize => 0x11B0,
    CompileWorkGroupSize => 0x11B1,
    LocalMemSize => 0x11B2,
    PreferredWorkGroupSizeMultiple => 0x11B3,
    PrivateMemSize => 0x11B4,
    GlobalWorkSize => 0x11B5
});

// crate::__codes_enum!(KernelArgAccessQualifier, cl_kernel_arg_access_qualifier, {
//     ReadOnly => 0x11A0,
//     WriteOnly => 0x11A1,
//     ReadWrite => 0x11A2,
//     NoneType => 0x11A3
// });

// crate::__codes_enum!(KernelArgTypeQualifier, cl_kernel_arg_type_qualifier, {
//     NoneType => 0,
//     Const => 1,
//     Restrict => 2,
//     Volatile => 4,
//     Pipe => 8
// });

// crate::__codes_enum!(KernelExecInfo, cl_kernel_exec_info, {
//     SvmPtrs => 0x11B6,
//     SvmFineGrainSystem => 0x11B7
// });

// crate::__codes_enum!(KernelArgAddressQualifier, cl_kernel_arg_address_qualifier, {
//     Global => 0x119B,
//     Local => 0x119C,
//     Constant => 0x119D,
//     Private => 0x119E
// });

// crate::__codes_enum!(KernelArgInfo, cl_kernel_arg_info, {
//     AddressQualifier => 0x1196,
//     AccessQualifier => 0x1197,
//     TypeName => 0x1198,
//     TypeQualifier => 0x1199,
//     Name => 0x119A
// });

// crate::__codes_enum!(KernelSubGroupInfo, cl_kernel_sub_group_info, {
//     MaxSubGroupSizeForNdrange => 0x2033,
//     SubGroupCountForNdrange => 0x2034,
//     LocalSizeForSubGroupCount => 0x11B8
// });
