#![allow(non_upper_case_globals)]


use std::marker::PhantomData;
use std::fmt::Debug;
use std::fmt;

use crate::ffi::{
    cl_buffer_create_type,
    cl_mem,
    cl_mem_flags,
    cl_mem_info,
    cl_mem_object_type,
};
use crate::open_cl::{
    cl_release_mem,
    cl_get_mem_object_info,
    ClObject,
    Output
};

#[derive(Eq, PartialEq)]
pub struct DeviceMem<T> where T: Debug {
    inner: cl_mem,
    _phantom: PhantomData<T>,
}

/// No matter the kernel read/write capabilities all devices need the expose raw_cl_object.
impl<T> ClObject<cl_mem> for DeviceMem<T> where T: Debug {
    unsafe fn raw_cl_object(&self) -> cl_mem {
        self.inner
    }
}

impl<T> ClObject<cl_mem> for &DeviceMem<T> where T: Debug {
    unsafe fn raw_cl_object(&self) -> cl_mem {
        self.inner
    }
}

impl<T> Drop for DeviceMem<T> where T: Debug { 
    fn drop(&mut self) {
        // println!("Dropping DeviceMem {:?}", self);
        unsafe { cl_release_mem(&self.raw_cl_object()); }
    }
}

impl<T> DeviceMem<T> where T: Debug {
    pub(crate) fn new(inner: cl_mem) -> DeviceMem<T> {
        DeviceMem{ inner, _phantom: PhantomData }
    }

    pub(crate) unsafe fn ptr_to_cl_object(&self) -> *const cl_mem {
        &self.inner as *const cl_mem
    }
}

impl<T: Debug> fmt::Debug for DeviceMem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceMem<[inner: {:?}, mem_type: {:?}, size: {:?}, type_size: {:?}]>",
            self.inner,
            self.mem_type().unwrap(),
            self.size().unwrap(),
            std::mem::size_of::<T>(),
        )
    }
}

impl<T: Debug> DeviceMem<T> {
    fn info(&self, flag: MemInfo) -> Output<usize> {
        cl_get_mem_object_info(self, flag as u32)
    }
    
    pub fn mem_type(&self) -> Output<usize> {
        self.info(MemInfo::Type)
    }
    pub fn flags(&self) -> Output<usize> {
        self.info(MemInfo::Flags)
    }

    pub fn len(&self) -> Output<usize> {
        let mem_size_in_bytes = self.size()?;
        Ok(mem_size_in_bytes / std::mem::size_of::<T>())
    }

    pub fn size(&self) -> Output<usize> {
        self.info(MemInfo::Size)
    }
    pub fn host_ptr(&self) -> Output<usize> {
        self.info(MemInfo::HostPtr)
    }
    pub fn map_count(&self) -> Output<usize> {
        self.info(MemInfo::MapCount)
    }
    pub fn reference_count(&self) -> Output<usize> {
        self.info(MemInfo::ReferenceCount)
    }
    pub fn context(&self) -> Output<usize> {
        self.info(MemInfo::Context)
    }
    pub fn associated_memobject(&self) -> Output<usize> {
        self.info(MemInfo::AssociatedMemobject)
    }
    pub fn offset(&self) -> Output<usize> {
        self.info(MemInfo::Offset)
    }
    pub fn uses_svm_pointer(&self) -> Output<usize> {
        self.info(MemInfo::UsesSvmPointer)
    }
}




// pub mod kernel_capability {
//     use std::fmt::Debug;

//     #[derive(Debug)]
//     pub struct CanRead;
//     #[derive(Debug)]
//     pub struct CannotRead;

//     pub trait Reading: Debug + Sized {}

//     impl Reading for CanRead {}
//     impl Reading for CannotRead {}

//     pub trait Writing: Debug + Sized {}

//     #[derive(Debug)]
//     pub struct CanWrite;
//     #[derive(Debug)]
//     pub struct CannotWrite;

//     impl Writing for CanWrite{}
//     impl Writing for CannotWrite{}
// }

// use kernel_capability::*;

// #[derive(Debug)]
// pub struct DeviceMem<'a, T, R, W> where
//     R: Reading + Debug + Sized,
//     W: Writing + Debug + Sized,
//     T: 'a + Debug + Sized,
//     {
//         inner: cl_mem,
//         _phantom: PhantomData<T>,
//         reading: R,
//         writing: W,
// }

// /// No matter the kernel read/write capabilities all devices need the expose raw_cl_object.
// impl<'a, T, R, W> ClObject<cl_mem> for DeviceMem<'a, T, R, W> where T: 'a, R: Reading, W: Writing, {
//     unsafe fn raw_cl_object(&self) -> cl_mem {
//         self.inner
//     }
// }

// impl<'a, T, R, W> Drop for DeviceMem<'a, T, R, W> where T: 'a + Debug, R: Reading + Debug + Sized, W: Writing + Debug + Sized { 
//     fn drop(&mut self) {
//         println!("Dropping DeviceMem {:?}", self);
//         cl_release_mem(&unsafe{ self.raw_cl_object() });
//     }
// }

// pub type KernelReadOnlyMem<'a, T> = DeviceMem<'a, T, CanRead, CannotWrite>;
// pub type KernelWriteOnlyMem<'a, T> = DeviceMem<'a, T, CannotRead, CanWrite>;
// pub type KernelReadWriteMem<'a, T> = DeviceMem<'a, T, CanRead, CanWrite>;
// pub type AnyDeviceMem<'a, T> = DeviceMem<'a, T, dyn Reading, dyn Writing>;

// impl<'a, T, R, W> DeviceMem<'a, T, R, W>
// where R: Reading + Debug, W: Writing + Debug, T: 'a + Debug {
//     pub fn kernel_read_only(inner: cl_mem) -> KernelReadOnlyMem<'a, T> {
//         DeviceMem {
//             inner,
//             _phantom: PhantomData,
//             reading: CanRead{},
//             writing: CannotWrite{},
//         }
//     }    
// }

// impl<'a, T, R, W> DeviceMem<'a, T, R, W> where R: Reading + Debug, W: Writing + Debug, T: 'a + Debug {
//     pub fn kernel_write_only(inner: cl_mem) -> KernelWriteOnlyMem<'a, T> {
//         DeviceMem {
//             inner,
//             _phantom: PhantomData,
//             reading: CannotRead{},
//             writing: CanWrite{},
//         }
//     }    
// }


// impl<'a, T, R, W> DeviceMem<'a, T, R, W> where R: Reading + Debug, W: Writing + Debug, T: 'a + Debug {
//     pub fn kernel_read_write(inner: cl_mem) -> KernelReadWriteMem<'a, T> {
//         DeviceMem {
//             inner,
//             _phantom: PhantomData,
//             reading: CanRead{},
//             writing: CanWrite{},
//         }
//     }    
// }


// pub trait DeviceMem<T> where dyn DeviceMem<T>: ClObject<cl_mem> {

//     fn size_of_ptr() -> usize {
//         CL_MEM_POINTER_SIZE
//     }

//     fn byte_len(&self) -> usize {
//         cl_get_mem_object_info(self, MemInfo::Size as cl_mem_info)
//             .unwrap_or_else(|e| panic!("Failed to retreive byte_length for cl_mem object {:?}", e))
//     }

//     fn len(&self) -> usize {
//         self.byte_len() / std::mem::size_of::<T>()
//     }
// }

// /// Reading from the device memory means that we don't need to hand over a
// /// mutable pointer we can hand over a const pointer.
// pub trait ReadFromDeviceMem<T> {
//     fn as_mut_ptr(&self) -> *mut cl_mem;
// }

// /// Writing to the device memory means that the device needs a mutable.
// /// In OpenCL cl_mem objects are thread-safe so this API does take a `&mut`.
// pub trait WriteToDeviceMem<T> {
//     fn as_ptr(&self) -> *const cl_mem;
// }

// macro_rules! device_mem_type {
//     ($name:ident) => {

//         #[repr(C)]
//         #[derive(Debug, Eq, PartialEq)]
//         pub struct $name<T> {
//             mem_object: cl_mem,
//             phantom: PhantomData<T>,
//         }

//         impl<T> DeviceMem<T> for $name<T> {
//             fn cl_mem(&self) -> cl_mem {
//                 self.cl_object()
//             }
//         }

//         impl<T> $name<T> {
       
//             pub fn new(mem_object: cl_mem) -> $name<T> {
//                 $name {
//                     mem_object,
//                     phantom: PhantomData,
//                 }
//             }

//             pub unsafe fn id_ptr(&self) -> *const cl_mem {
//                 &self.mem_object as *const cl_mem
//             }
//         }

//         // impl<T> ReadFromDeviceMem<T> for $name<T> {
//         //     fn as_const_ptr(&self) -> *const cl_mem {
//         //         self.cl_object() as *const cl_mem
//         //     }
//         // }

//         // impl<T> ReadFromDeviceMem<T> for $name<&T> {
//         //     fn as_const_ptr(&self) -> *const cl_mem {
//         //         self.cl_object() as *const cl_mem
//         //     }
//         // }

//         // impl<T> ReadFromDeviceMem<T> for &$name<T> {
//         //     fn as_const_ptr(&self) -> *const cl_mem {
//         //         self.cl_object() as *const cl_mem
//         //     }
//         // }

//         // impl<T> WriteToDeviceMem<T> for $name<T> {
//         //     fn as_mut_ptr(&self) -> *mut cl_mem {
//         //         self.cl_object() as *mut cl_mem
//         //     }
//         // }

//         impl<T> ClObject<cl_mem> for $name<T> {
//             unsafe fn raw_cl_object(&self) -> cl_mem {
//                 self.mem_object
//             }
//         }

//           impl<T> ClObject<cl_mem> for &$name<T> {
//             unsafe fn raw_cl_object(&self) -> cl_mem {
//                 self.mem_object
//             }
//         }


//         impl<T> Drop for $name<T> {
//             fn drop(&mut self) {
//                 println!("Dropping device_mem here {:?}", self.cl_object());
//                 cl_release_mem(&self.cl_object());
//             }
//         }
//     };
// }

// /// Kernel cannot mutate.
// /// Host can mutate.
// device_mem_type!(KernelReadOnlyMem);

// impl<T> AsMutPointer for KernelReadOnlyMem<T> {
//     fn as_mut_pointer(&mut self) -> *mut libc::c_void {
//         self.cl_mem() as *mut libc::c_void
//     }
// }

// // /// Kernel cannot read, but can write.
// // /// Host can mutate.
// device_mem_type!(KernelWriteOnlyMem);

// // /// Kernel can both read and write.
// // /// Host can mutate.
// device_mem_type!(KernelReadWriteMem);

// crate::__codes_enum!(MemMigrationFlags, cl_mem_migration_flags, {
//     Host => (1 << 0),
//     ContentUndefined => (1 << 1)
// });

/* cl_mem_object_type */
crate::__codes_enum!(MemObjectType, cl_mem_object_type, {
    Buffer => 0x10F0,
    Image2D => 0x10F1,
    Image3D => 0x10F2,
    Image2DArray => 0x10F3,
    Image1D => 0x10F4,
    Image1DArray => 0x10F5,
    Image1DBuffer => 0x10F6,
    Pipe => 0x10F7
});

// * cl_mem_info */
crate::__codes_enum!(MemInfo, cl_mem_info, {
    Type => 0x1100,
    Flags => 0x1101,
    Size => 0x1102,
    HostPtr => 0x1103,
    MapCount => 0x1104,
    ReferenceCount => 0x1105,
    Context => 0x1106,
    AssociatedMemobject => 0x1107,
    Offset => 0x1108,
    UsesSvmPointer => 0x1109
});

/* cl_mem_migration_flags - bitfield */
crate::__codes_enum!(BufferCreateType, cl_buffer_create_type, {
    /* cl_buffer_create_type */
    CreateTypeRegion => 0x1220
});

// cl_mem_flag is a bitfield, but the definitions are mutually exclusive.
// Therefore MemFlag is *actually* a proper enum.
//
// CL_MEM_READ_WRITE
//
//   - The OpenCL Kernels will both read and write the buffer
//
// CL_MEM_WRITE_ONLY
//
//   - The OpenCL Kernels will only read the buffer
//
// CL_MEM_READ_ONLY
//
//   - The OpenCL Kernels will only write the buffer
//
//   - The above three flags are mutually exclusive. Only one should be
//     specified. If none are specified then CL_MEM_READ_WRITE is assumed. These
//     flags indicate to the OpenCL runtime how the buffer will be accessed from
//     the perspective of OpenCL C kernels running on the device. When read only
//     or write only is specified, some coherency operations may be skipped for
//     performance.

crate::__codes_enum!(MemFlag, cl_mem_flags, {
    ReadWrite => 1 << 0,
    WriteOnly => 1 << 1,
    ReadOnly => 1 << 2,
    AllocHostPtr => 1 << 4
    // // TODO: implement the mem types for the following flags
    // UseHostPtr => 1 << 3,
    
    // CopyHostPtr => 1 << 5,
    // HostWriteOnly => 1 << 7,
    // HostReadOnly => 1 << 8,
    // HostNoAccess => 1 << 9,
    // SvmFineGrainBuffer => 1 << 10,
    // SvmAtomics => 1 << 11,
    // KernelReadAndWrite => 1 << 12
});
