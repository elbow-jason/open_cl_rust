use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ffi::{
    cl_mem,
    cl_context,
};
use crate::context::Context;
use crate::cl::{ClObject, ClPointer, ClObjectError};

pub mod low_level;
pub mod flags;

use low_level::{cl_retain_mem, cl_release_mem};

use flags::{
    MemObjectType,
};
use crate::error::{Error, Output};


/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum DeviceMemError {
    #[fail(display = "Given DeviceMem<T> has no associated cl_mem object")]
    NoAssociatedMemObject,

}

impl From<DeviceMemError> for Error {
    fn from(err: DeviceMemError) -> Error {
        Error::DeviceMemError(err)
    }
}


#[repr(C)]
#[derive(Eq, PartialEq, Hash)]
pub struct DeviceMem<T> where T: Debug {
    handle: cl_mem,
    _phantom: PhantomData<T>,
}

 
impl<T: Debug> Drop for DeviceMem<T> {
    fn drop(&mut self) {
        unsafe {
            cl_release_mem(&self.raw_cl_object()).unwrap_or_else(|e| {
                panic!("Failed to release cl_mem of {:?} due to {:?}", self, e);
            })
        }
    }
}


impl<T: Debug> Clone for DeviceMem<T> {
    fn clone(&self) -> DeviceMem<T> {
        unsafe {
            DeviceMem::new_retained(self.raw_cl_object()).unwrap_or_else(|e| {
                panic!("Failed to clone {:?} due to {:?}", self, e);
            })
        }
    }
}

impl<T: Debug> ClObject<cl_mem> for DeviceMem<T> {
    unsafe fn raw_cl_object(&self) -> cl_mem {
        self.handle
    }

    unsafe fn new(handle: cl_mem) -> Output<DeviceMem<T>> {
        if handle.is_null() {
            let error = ClObjectError::ClObjectCannotBeNull("DeviceMem<T>".to_string());
            return Err(error.into())
        }
        Ok(DeviceMem {
            handle,
            _phantom: PhantomData
        })
    }

    unsafe fn new_retained(handle: cl_mem) -> Output<DeviceMem<T>> {
        if handle.is_null() {
            let error = ClObjectError::ClObjectCannotBeNull("DeviceMem<T>".to_string());
            return Err(error.into())
        }

        let () = cl_retain_mem(&handle)?;
        Ok(DeviceMem {
            handle,
            _phantom: PhantomData
        })
    }
}



impl<T> DeviceMem<T>
where
    T: Debug,
{
    pub unsafe fn ptr_to_cl_object(&self) -> *const cl_mem {
        &self.handle as *const cl_mem
    }
}

impl<T: Debug> fmt::Debug for DeviceMem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceMem<[handle: {:?}, mem_type: {:?}, size: {:?}, type_size: {:?}]>",
            self.handle,
            self.mem_type().unwrap(),
            self.size().unwrap(),
            std::mem::size_of::<T>(),
        )
    }
}

use flags::{MemInfo, MemFlags};

impl<T: Debug> DeviceMem<T> {

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

    fn get_info<M>(&self, flag: MemInfo) -> Output<ClPointer<M>> where M: Debug + Copy {
        low_level::cl_get_mem_object_info::<T, M>(self, flag)
    }


    pub fn len(&self) -> Output<usize> {
        let mem_size_in_bytes = self.size()?;    
        Ok(mem_size_in_bytes / std::mem::size_of::<T>())
    }


    pub fn host_ptr(&self) -> Output<Option<Vec<T>>> where T: Copy {
        self.get_info::<T>(MemInfo::HostPtr).map(|ret| {
            unsafe {
                // let host_vec =
                if ret.is_null() {
                    return None;
                } 
                // if host_vec.as_ptr() as usize == 1 {
                //     return None;
                // } 
                Some( ret.into_many() )
            }
        })
    }

    pub fn associated_memobject(&self) -> Output<DeviceMem<T>> {
        self.get_info::<cl_mem>(MemInfo::AssociatedMemobject)
            .and_then(|ret| unsafe { ret.into_retained_wrapper::<DeviceMem<T>>() })
            .map_err(|e| {
                match e {
                    Error::ClObjectError(ClObjectError::ClObjectCannotBeNull(..)) => {
                        DeviceMemError::NoAssociatedMemObject.into()
                    }
                    other => other
                }
            })
    }

    pub fn context(&self) -> Output<Context> {
        self.get_info::<cl_context>(MemInfo::Context).and_then(|ret| unsafe { ret.into_retained_wrapper::<Context>() })
    }

    pub fn reference_count(&self) -> Output<u32> {
        self.get_info(MemInfo::ReferenceCount).map(|ret| unsafe{ ret.into_one() })
    }
    pub fn size(&self) -> Output<usize> {
        self.get_info(MemInfo::Size).map(|ret| unsafe { ret.into_one() })
    }

    pub fn offset(&self) -> Output<usize> {
        self.get_info(MemInfo::Offset).map(|ret| {
            unsafe { ret.into_one() }
        })
    }
    pub fn map_count(&self) -> Output<u32> {
        self.get_info(MemInfo::MapCount).map(|ret| {
            unsafe { ret.into_one()  }
        })
    }
}

macro_rules! __impl_mem_info {
    ($name:ident, $flag:ident, $output_t:ty) => {
        impl<T: Debug> DeviceMem<T> {
            pub fn $name(&self) -> Output<$output_t> {
                self.get_info(MemInfo::$flag).map(|ret| {
                    unsafe { ret.into_one() }
                })
            }
        }
    }
}

__impl_mem_info!(mem_type, Type, MemObjectType);
__impl_mem_info!(flags, Flags, MemFlags);



#[cfg(test)]
mod tests {
    use super::{DeviceMem, DeviceMemError};
    use crate::{Context, Session, Output, Error};
    use super::flags::{
        MemFlags,
        MemObjectType,
    };


    fn get_device_mem() -> (Session, DeviceMem<usize>) {
        let session = Session::default();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let dmem = DeviceMem::create_read_write_from(session.context(), &data[..])
            .expect("Failed to create_read_write_from one to nine");
        (session, dmem)
    }

    #[test]
    fn device_mem_method_len_works() {
        let (_sess, device_mem) = get_device_mem();
        let out = device_mem.len().expect("Failed to call device_mem.len()");
        assert_eq!(out, 9);
    }
    #[test]
    fn device_mem_method_host_ptr_works() {
        let (_sess, device_mem) = get_device_mem();
        let out = device_mem.host_ptr().expect("Failed to call device_mem.host_ptr()");
        match out {
            Some(host_vec) => {
                assert_eq!(host_vec.len(), 0);
            },
            None => (),
        }
    }

    #[test]
    fn device_mem_method_associated_memobject_works() {
        let (_sess, device_mem) = get_device_mem();
        let result: Output<DeviceMem<usize>> = device_mem.associated_memobject();
        match result {
            Ok(_dmem) => (),
            Err(Error::DeviceMemError(DeviceMemError::NoAssociatedMemObject)) => (),
            Err(e) => panic!("Call device_mem.associated_memobject() encountered an unexpected Error: {:?}", e),
        }
    }

    #[test]
    fn device_mem_method_reference_count_works() {
        let (_sess, device_mem) = get_device_mem();
        let out = device_mem.reference_count()
            .expect("Failed to call device_mem.reference_count()");
        assert!(out == 1);
    }
    #[test]
    fn device_mem_method_size_works() {
        let (_sess, device_mem) = get_device_mem();
        let out = device_mem.size().expect("Failed to call device_mem.size()");
        let size_t_in_bytes = std::mem::size_of::<usize>();
        let len = out / size_t_in_bytes;
        assert_eq!(len, 9);
    }
    #[test]
    fn device_mem_method_mem_type_works() {
        let (_sess, device_mem) = get_device_mem();
        let _out: MemObjectType = device_mem.mem_type()
            .expect("Failed to call device_mem.mem_type()");
    }
    #[test]
    fn device_mem_method_flags_works() {
        let (_sess, device_mem) = get_device_mem();
        let _out: MemFlags = device_mem.flags().expect("Failed to call device_mem.flags()");
    }
    #[test]
    fn device_mem_method_map_count_works() {
        let (_sess, device_mem) = get_device_mem();
        let out = device_mem.map_count().expect("Failed to call device_mem.map_count()");
        assert_eq!(out, 0);
    }
    #[test]
    fn device_mem_method_context_works() {
        let (_sess, device_mem) = get_device_mem();
        let _out: Context = device_mem.context()
            .expect("Failed to call device_mem.context()");
    }
    #[test]
    fn device_mem_method_offset_works() {
        let (_sess, device_mem) = get_device_mem();
        let out: usize = device_mem.offset().expect("Failed to call device_mem.offset()");
        assert_eq!(out, 0);
    }

}