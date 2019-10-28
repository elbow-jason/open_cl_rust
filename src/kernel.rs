use std::marker::PhantomData;
use libc::{c_void, size_t};

use crate::open_cl::{
    cl_kernel,
    cl_kernel_info,
    cl_kernel_work_group_info,
    cl_release_kernel,
    cl_set_kernel_arg,
    cl_mem,
    Output,
    Error,
};

use crate::{
    // DeviceMem,
    KernelReadOnlyMem,
    KernelReadWriteMem,
    KernelWriteOnlyMem,
    // ReadFromDeviceMem,
    // WriteToDeviceMem,
    // AsMutPointer,
};



/// An error related to a `Kernel`, `KernelBuilder`, or `KernelCmd`.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[fail(display = "Kernel arg index out of range. (kernel: {}, index: {})", kernel, index)]
    ArgIndexOutOfRange{kernel: String, index: u32},
    #[fail(display = "Kernel argument type mismatch. (kernel: {}, index: [{}], \
        arg_type {})", kernel, index, arg_type)]
    ArgTypeMismatch{kernel: String, index: u32, arg_type: String },
    #[fail(display = "The wrong number of kernel arguments have been specified \
        (required: {}, specified: {}). Use named arguments with 'None' or zero values to \
        declare arguments you plan to assign a value to at a later time.", required, specified)]
    BuilderWrongArgCount { required: u32, specified: u32 },
}

impl From<KernelError> for Error {
    fn from(e: KernelError) -> Error {
        Error::KernelError(e)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KernelArg<T> {
    arg_size: size_t,
    arg_ptr: *const c_void,
    _phantom: PhantomData<T>,
}

impl<T> KernelArg<T> {
    pub fn new(arg_size: usize, arg_ptr: *const c_void) -> KernelArg<T> {
        KernelArg {
            arg_size: arg_size,
            arg_ptr: arg_ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<&KernelReadWriteMem<T>> for KernelArg<cl_mem> {
    fn from(mem: &KernelReadWriteMem<T>) -> KernelArg<cl_mem> {
        KernelArg::new(
            std::mem::size_of::<cl_mem>() as size_t,
            mem.cl_object() as *const c_void
        )
    }
}

impl<T> From<&KernelReadOnlyMem<T>> for KernelArg<cl_mem> {
    fn from(mem: &KernelReadOnlyMem<T>) -> KernelArg<cl_mem> {
        KernelArg::new(
            std::mem::size_of::<cl_mem>() as size_t,
            mem.cl_object() as *const c_void
        )
    }
}

impl<T> From<&KernelWriteOnlyMem<T>> for KernelArg<cl_mem> {
    fn from(mem: &KernelWriteOnlyMem<T>) -> KernelArg<cl_mem> {
        KernelArg::new(
            std::mem::size_of::<cl_mem>() as size_t,
            mem.cl_object() as *const c_void
        )
    }
}

pub trait AsKernelArg<T> {
    fn as_kernel_arg(&self) -> KernelArg<T>;
}

impl AsKernelArg<isize> for isize {
    fn as_kernel_arg(&self) -> KernelArg<isize> {
        KernelArg::new(
            std::mem::size_of::<isize>() as size_t,
            self as *const isize as *const c_void,
        )
    }
}

impl<T> AsKernelArg<cl_mem> for &KernelReadWriteMem<T> {
    fn as_kernel_arg(&self) -> KernelArg<cl_mem> {
        KernelArg::new(
             std::mem::size_of::<cl_mem>() as libc::size_t,
             unsafe { self.id_ptr() } as *const libc::c_void
        )
    }
}



impl From<usize> for KernelArg<usize> {
    fn from(num: usize) -> KernelArg<usize> {
        KernelArg::new(
            std::mem::size_of::<usize>() as size_t,
            num as *const usize as *const c_void,
        )
    }
}

impl From<isize> for KernelArg<isize> {
    fn from(num: isize) -> KernelArg<isize> {
        KernelArg::new(
            std::mem::size_of::<isize>() as size_t,
            num as *const usize as *const c_void,
        )
    }
}

// pub trait ToKernelArg<T> {
//     fn to_kernel_arg(&self) -> (usize, *const T);
// }

// impl<T> ToKernelArg<cl_mem> for &KernelReadWriteMem<T> {
//     fn to_kernel_arg(&self) -> (usize, *const cl_mem) {
//         (
//             std::mem::size_of::<cl_mem>(),
//             self.cl_object() as *const cl_mem
//         )
//     }
// }

// impl<T> ToKernelArg<cl_mem> for &KernelReadOnlyMem<T> {
//     fn to_kernel_arg(&self) -> (usize, *const cl_mem) {
//         (
//             std::mem::size_of::<cl_mem>(),
//             self.cl_object() as *const cl_mem
//         )
//     }
// }

// impl<T> ToKernelArg<cl_mem> for &KernelWriteOnlyMem<T> {
//     fn to_kernel_arg(&self) -> (usize, *const cl_mem) {
//         (
//             std::mem::size_of::<cl_mem>(),
//             self.cl_object() as *const cl_mem
//         )
//     }
// }

// impl<T> AsKernelArg<cl_mem> for KernelReadWriteMem<T> {
//     fn as_kernel_arg(&self) -> KernelArg<cl_mem> {
//         KernelArg {
//             mem_size: std::mem::size_of::<cl_mem>(),
//             ptr: self.cl_object() as *const libc::c_void,
//             _phantom: PhantomData,
//         }
//     }
// }

// // #[derive(Debug)]
// // pub struct KernelMutArg<T> {
// //     mem_size: usize,
// //     ptr: *const libc::c_void,
// //     _phantom: PhantomData<T>,
// // }

// // pub trait AsKernelMutArg<T> {
// //     fn as_kernel_mut_arg(&mut self) -> KernelMutArg<T>;
// // }

 
// //     // KernelWriteOnlyMem,
// //     // KernelReadOnlyMem,



// // impl<T> AsKernelMutArg<cl_mem> for KernelReadWriteMem<T> {
// //     fn as_kernel_mut_arg(&mut self) -> KernelMutArg<cl_mem> {
// //         KernelMutArg {
// //             mem_size: std::mem::size_of::<cl_mem>(),
// //             ptr: self.cl_object() as *const libc::c_void,
// //             _phantom: PhantomData,
// //         }
// //     }
// // }


// // macro_rules! as_kernel_arg_for_mem {
// //     (write $name:ident) => {
// //         impl AsKernelMutArg<cl_mem> for $name<cl_mem> {
// //             fn as_kernel_mut_arg(&mut self) -> KernelMutArg<cl_mem> {
// //                 let size_of_type = 
// //                 println!("as_kernel_mut_arg/1 for {:?} with size of type {:?}", self, size_of_type);

// //                 let k: KernelMutArg<cl_mem> = KernelMutArg {
// //                     size: size_of_type,
// //                     value: self.cl_mem() as *const cl_mem,
// //                     _phantom: PhantomData,
// //                 };
// //                 println!("KernelMutArg k is {:?}", k);
// //                 k
// //             }
// //         }
// //     };

// //     (read $name:ident) => {
// //         impl<T> AsKernelArg for $name<T> where T: std::fmt::Debug {
// //             fn as_kernel_arg(&self) -> KernelArg<$name<T>> {
// //                 let size_of_type = std::mem::size_of::<cl_mem>() as libc::size_t;

// //                 println!("as_kernel_arg/1 for {:?} with size of type {:?}", self, size_of_type);
// //                 let k = KernelArg {
// //                     size: size_of_type,
// //                     value: self.as_const_ptr() as *const libc::c_void,
// //                     _phantom: PhantomData,
// //                 };
// //                 println!("KernelArg k is {:?}", k);
// //                 k
// //             }
// //         }
// //     };
// // }




// // kernel can write, but not read
// as_kernel_arg_for_mem!(write KernelWriteOnlyMem);
// // kernel cannot write, only read
// as_kernel_arg_for_mem!(read KernelReadOnlyMem);

// // kernel can both read and write
// as_kernel_arg_for_mem!(write KernelReadWriteMem);
// as_kernel_arg_for_mem!(read KernelReadWriteMem);


// macro_rules! kernel_size {
//     ([$t:ty; 2]) => {
//         std::mem::size_of::<[$t; 2]>() as libc::size_t
//     };
//     ([$t:ty; 3]) => {
//         4 * std::mem::size_of::<$t>() as libc::size_t
//     };
//     ($t:ty) => {
//         std::mem::size_of::<$t>() as libc::size_t
//     };
// }

// macro_rules! kernel_arg_type {
//     ([$t:ty; _:expr]) => {
//         $t
//     };
//     ($t:ty) => {
//         $t
//     };
// }
// // std::mem::size_of::<$t>() as libc::size_t,
// macro_rules! as_kernel_arg {
//     ($source:ty, $target:ty) => {
//         impl AsKernelArg<$target> for $source {
//              fn as_kernel_arg(&self) -> KernelArg<$target> {
//                 println!("as_kernel_arg for {} as {:?}", stringify!($source), stringify!($target));
//                 KernelArg {
//                     size: kernel_size!($t),
//                     value: self as *const $target,
//                     _phantom: PhantomData,
//                 }
//             }
//         }
//     };
// }

// macro_rules! as_kernel_mut_arg {
//     ($source:ty, $target:ty) => {
//         impl AsKernelMutArg<$target> for $source {
//              fn as_kernel_mut_arg(&self) -> KernelMutArg<$target> {
//                 println!("as_kernel_mut_arg for {} as {:?}", stringify!($source), stringify!($target));
//                 KernelArg {
//                     size: kernel_size!($t),
//                     value: self as *const $target,
//                     _phantom: PhantomData,
//                 }
//             }
//         }
//     };
// }

// as_kernel_arg!(isize, isize);
// as_kernel_arg!(usize);
// as_kernel_arg!(u32);
// as_kernel_arg!(u64);
// as_kernel_arg!(i32);
// as_kernel_arg!(i64);
// as_kernel_arg!(f32);
// as_kernel_arg!(f64);
// as_kernel_arg!([f32; 2]);
// as_kernel_arg!([f64; 2]);
// as_kernel_arg!([f32; 3]);
// as_kernel_arg!([f64; 3]);

// as_kernel_mut_arg!(isize, isize);
// as_kernel_mut_arg!(usize);
// as_kernel_mut_arg!(u32);
// as_kernel_mut_arg!(u64);
// as_kernel_mut_arg!(i32);
// as_kernel_mut_arg!(i64);
// as_kernel_mut_arg!(f32);
// as_kernel_mut_arg!(f64);
// as_kernel_mut_arg!([f32; 2]);
// as_kernel_mut_arg!([f64; 2]);
// as_kernel_mut_arg!([f32; 3]);
// as_kernel_mut_arg!([f64; 3]);

/// cl_kernel is not thread-safe.
/// cl_kernel should be a short lived, generate as needed structure;
/// to be loaded with args then immediately enqueued and disposed of.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Kernel(cl_kernel);

impl Kernel {
    pub fn new(k: cl_kernel) -> Kernel {
        Kernel(k)
    }

    pub(crate) fn cl_object(&self) -> cl_kernel {
        self.0
    }

    // pub fn set_mut_arg<T>(&self, arg_index: usize, kernel_arg: KernelMutArg<T>) -> Output<()> {
    //     cl_set_kernel_arg(
    //         self.cl_object(),
    //         arg_index,
    //         kernel_arg.size,
    //         kernel_arg.value as *mut libc::c_void,
    //     )
    // }

    pub fn set_arg<T, P>(&self, arg_index: usize, arg: T) -> Output<()>
        where T: AsKernelArg<P>,
        P: std::fmt::Debug
    {
        let kernel_arg: KernelArg<P> = arg.as_kernel_arg();
        println!("kernel arg {:?}", kernel_arg);
        let result = cl_set_kernel_arg(
            self.cl_object(),
            arg_index,
            kernel_arg.arg_size,
            kernel_arg.arg_ptr,
        );
        println!("cl_set_kernel_arg result {:?}", result);
        result
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        println!("dropping kernel here");
        cl_release_kernel(&self.cl_object())
            .unwrap_or_else(|e| panic!("Failed to drop cl_kernel {:?}", e));
    }
}

// /* cl_kernel_arg_access_qualifier */
// crate::__codes_enum!(KernelArgAccessQualifier, cl_kernel_arg_access_qualifier, {
//     ReadOnly => 0x11A0,
//     WriteOnly => 0x11A1,
//     ReadWrite => 0x11A2,
//     NoneType => 0x11A3
// });

// /* cl_kernel_arg_address_qualifier */
// crate::__codes_enum!(KernelArgAddressQualifier, cl_kernel_arg_address_qualifier, {
//     Global => 0x119B,
//     Local => 0x119C,
//     Constant => 0x119D,
//     Private => 0x119E
// });

// /* cl_kernel_arg_info */
// crate::__codes_enum!(KernelArgInfo, cl_kernel_arg_info, {
//     AddressQualifier => 0x1196,
//     AccessQualifier => 0x1197,
//     TypeName => 0x1198,
//     TypeQualifier => 0x1199,
//     Name => 0x119A
// });

// /* cl_kernel_arg_type_qualifier */
// crate::__codes_enum!(KernelArgTypeQualifier, cl_kernel_arg_type_qualifier, {
//     NoneType => 0,
//     Const => 1,
//     Restrict => 2,
//     Volatile => 4,
//     Pipe => 8
// });

// /*  cl_kernel_exec_info */
// crate::__codes_enum!(KernelExecInfo, cl_kernel_exec_info, {
//     SvmPtrs => 0x11B6,
//     SvmFineGrainSystem => 0x11B7
// });

/*  cl_kernel_info */
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

/*  cl_kernel_sub_group_info */

// crate::__codes_enum!(KernelSubGroupInfo, cl_kernel_sub_group_info, {
//     MaxSubGroupSizeForNdrange => 0x2033,
//     SubGroupCountForNdrange => 0x2034,
//     LocalSizeForSubGroupCount => 0x11B8
// });

/*  cl_kernel_sub_group_info */
crate::__codes_enum!(KernelWorkGroupInfo, cl_kernel_work_group_info, {
    WorkGroupSize => 0x11B0,
    CompileWorkGroupSize => 0x11B1,
    LocalMemSize => 0x11B2,
    PreferredWorkGroupSizeMultiple => 0x11B3,
    PrivateMemSize => 0x11B4,
    GlobalWorkSize => 0x11B5
});
