#![allow(non_upper_case_globals)]


use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ffi::{cl_buffer_create_type, cl_mem, cl_mem_flags, cl_mem_info, cl_mem_object_type};
use crate::open_cl::{cl_get_mem_object_info, cl_release_mem, ClObject, Output};

#[derive(Eq, PartialEq)]
pub struct DeviceMem<T>
where
    T: Debug,
{
    inner: cl_mem,
    _phantom: PhantomData<T>,
}

/// No matter the kernel read/write capabilities all devices
/// need to expose raw_cl_object.
impl<T> ClObject<cl_mem> for DeviceMem<T>
where
    T: Debug,
{
    unsafe fn raw_cl_object(&self) -> cl_mem {
        self.inner
    }
}

impl<T> ClObject<cl_mem> for &DeviceMem<T>
where
    T: Debug,
{
    unsafe fn raw_cl_object(&self) -> cl_mem {
        self.inner
    }
}

impl<T> Drop for DeviceMem<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        unsafe {
            cl_release_mem(&self.raw_cl_object());
        }
    }
}

impl<T> DeviceMem<T>
where
    T: Debug,
{
    pub(crate) fn new(inner: cl_mem) -> DeviceMem<T> {
        DeviceMem {
            inner,
            _phantom: PhantomData,
        }
    }

    pub(crate) unsafe fn ptr_to_cl_object(&self) -> *const cl_mem {
        &self.inner as *const cl_mem
    }
}

impl<T: Debug> fmt::Debug for DeviceMem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceMem<[inner: {:?}, mem_type: {:?}, size: {:?}, type_size: {:?}]>",
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

// NOTE: Version for cl_mem_migration_flags?
// crate::__codes_enum!(MemMigrationFlags, cl_mem_migration_flags, {
//     Host => (1 << 0),
//     ContentUndefined => (1 << 1)
// });

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

crate::__codes_enum!(BufferCreateType, cl_buffer_create_type, {
    /* cl_buffer_create_type */
    CreateTypeRegion => 0x1220
});

// cl_mem_flag is a bitfield, but some definitions are mutually exclusive.
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
