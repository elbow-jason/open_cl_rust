

use std::fmt::Debug;

use crate::ffi::{
    cl_buffer_create_type,
    cl_mem_flags,
    cl_mem_info,
    cl_mem_object_type,
    cl_mem_migration_flags,
};

crate::__codes_enum!(MemMigrationFlags, cl_mem_migration_flags, {
    Host => (1 << 0),
    ContentUndefined => (1 << 1)
});

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


bitflags! {
    /// NOTE: Due to the mutual exclusion of some of the combinations of the flags
    /// of MemFlags. There is a better way. We _could_ compose the desired result
    /// as a struct of enums (e.g. RW::ReadWrite, RW::WriteOnly, RW::ReadWrite),
    /// carrying around the valid state of the buffer creation and generating the
    /// cl_bitfield immediately before the call to clCreateBuffer.
    /// 
    /// Details: https://www.khronos.org/registry/OpenCL/sdk/1.0/docs/man/xhtml/clCreateBuffer.html
    pub struct MemFlags: cl_mem_flags {
        const READ_WRITE = 1 << 0;
        const WRITE_ONLY = 1 << 1;
        const READ_ONLY = 1 << 2;
        const ALLOC_HOST_PTR = 1 << 4;
        const USE_HOST_PTR = 1 << 3;
        const COPY_HOST_PTR = 1 << 5;
        const HOST_WRITE_ONLY = 1 << 7;
        const HOST_READ_ONLY = 1 << 8;
        const HOST_NO_ACCESS = 1 << 9;
        const SVM_FINE_GRAIN_BUFFER = 1 << 10;
        const SVM_ATOMICS = 1 << 11;
        const KERNEL_READ_AND_WRITE = 1 << 12;
        // a few useful custom MemFlags that are also examples.
        const READ_WRITE_ALLOC_HOST_PTR = Self::READ_WRITE.bits | Self::ALLOC_HOST_PTR.bits;
        const READ_ONLY_ALLOC_HOST_PTR = Self::READ_ONLY.bits | Self::ALLOC_HOST_PTR.bits;
        const WRITE_ONLY_ALLOC_HOST_PTR = Self::WRITE_ONLY.bits | Self::ALLOC_HOST_PTR.bits;
    }
}
