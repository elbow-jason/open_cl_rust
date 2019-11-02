use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ffi::cl_mem;
use crate::context::Context;
use crate::error::Output;

pub mod low_level;
pub mod flags;
use low_level::{cl_retain_mem, cl_release_mem};

__impl_unconstructable_cl_wrapper!(DeviceMem<T>, cl_mem);
__impl_cl_object_for_wrapper!(DeviceMem<T>, cl_mem);
__impl_clone_for_cl_object_wrapper!(DeviceMem<T>, cl_retain_mem);
__impl_drop_for_cl_object_wrapper!(DeviceMem<T>, cl_release_mem);

impl<T> DeviceMem<T>
where
    T: Debug,
{
    pub unsafe fn ptr_to_cl_object(&self) -> *const cl_mem {
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

use flags::{MemInfo, MemFlags};

impl<T: Debug> DeviceMem<T> {
    fn info(&self, flag: MemInfo) -> Output<usize> {
        low_level::cl_get_mem_object_info(self, flag as u32)
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

    pub fn create_with_len(context: &Context, flags: MemFlags, len: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        let device_mem: DeviceMem<T> = low_level::cl_create_buffer_with_len::<T>(
            context,
            flags,
            len,
        )?;
        Ok(device_mem)
    }

    pub fn create_from(context: &Context, flags: MemFlags, slice: &[T]) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        let device_mem: DeviceMem<T> = low_level::cl_create_buffer_from_slice::<T>(
            context,
            flags,
            slice
        )?;
        Ok(device_mem)
    }

    pub fn create_read_only(context: &Context, len: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {   
        DeviceMem::create_with_len(
            context,
            MemFlags::READ_ONLY_ALLOC_HOST_PTR,
            len,
        )
    }

    pub fn create_write_only(context: &Context, len: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
         DeviceMem::create_with_len(
            context,
            MemFlags::WRITE_ONLY_ALLOC_HOST_PTR,
            len,
        )
    }

    pub fn create_read_write(context: &Context, len: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        DeviceMem::create_with_len(
            context,
            MemFlags::READ_WRITE_ALLOC_HOST_PTR,
            len,
        )
    }

    pub fn create_read_write_from(context: &Context, data: &[T]) -> Output<DeviceMem<T>> where T: Debug {
        DeviceMem::create_from(
            context,
            MemFlags::COPY_HOST_PTR | MemFlags::READ_WRITE_ALLOC_HOST_PTR,
            data
        )
    }

    pub fn create_read_only_from(context: &Context, data: &[T]) -> Output<DeviceMem<T>> where T: Debug {
        DeviceMem::create_from(
            context,
            MemFlags::COPY_HOST_PTR | MemFlags::READ_ONLY_ALLOC_HOST_PTR,
            data
        )
    }

    pub fn create_write_only_from(context: &Context, data: &[T]) -> Output<DeviceMem<T>> where T: Debug {
        DeviceMem::create_from(
            context,
            MemFlags::COPY_HOST_PTR | MemFlags::WRITE_ONLY_ALLOC_HOST_PTR,
            data
        )
    }
}

