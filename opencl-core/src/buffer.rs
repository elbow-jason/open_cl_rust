use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ll::{ClContext, ClMem, MemFlags, MemPtr};

use crate::{BufferCreator, ClNumber, Context, HostAccess, KernelAccess, MemLocation, Output};

pub struct Buffer<T: ClNumber> {
    inner: Arc<RwLock<ClMem<T>>>,
    _context: Context,
}

unsafe impl<T: ClNumber> Send for Buffer<T> {}
unsafe impl<T: ClNumber> Sync for Buffer<T> {}

impl<T: ClNumber> Clone for Buffer<T> {
    fn clone(&self) -> Buffer<T> {
        Buffer {
            inner: self.inner.clone(),
            _context: self._context.clone(),
        }
    }
}

impl<T: ClNumber> Debug for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer{{{:?}}}", self.inner)
    }
}

impl<T: ClNumber> Buffer<T> {
    pub fn new(ll_mem: ClMem<T>, context: Context) -> Buffer<T> {
        Buffer {
            inner: Arc::new(RwLock::new(ll_mem)),
            _context: context,
        }
    }

    pub fn create<B: BufferCreator<T>>(
        context: &Context,
        creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<Buffer<T>> {
        let ll_mem = ClMem::create(
            context.low_level_context(),
            creator,
            host_access,
            kernel_access,
            mem_location,
        )?;
        Ok(Buffer::new(ll_mem, context.clone()))
    }

    pub fn create_from_low_level_context<B: BufferCreator<T>>(
        ll_context: &ClContext,
        creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<Buffer<T>> {
        let ll_mem = ClMem::create(
            ll_context,
            creator,
            host_access,
            kernel_access,
            mem_location,
        )?;
        let context = Context::from_low_level_context(ll_context)?;
        Ok(Buffer::new(ll_mem, context))
    }

    pub fn read_lock(&self) -> RwLockReadGuard<ClMem<T>> {
        self.inner.read().unwrap()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<ClMem<T>> {
        self.inner.write().unwrap()
    }

    pub fn context(&self) -> &Context {
        &self._context
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.read_lock().reference_count() }
    }

    pub fn size(&self) -> Output<usize> {
        unsafe { self.read_lock().size() }
    }

    /// A non-panicking version of len.
    pub fn length(&self) -> Output<usize> {
        let size = unsafe { self.read_lock().size() }?;
        Ok(size / std::mem::size_of::<T>())
    }

    /// A method for getting the len of the device memory buffer.
    /// Panics if the buffer size info returns an error.
    pub fn len(&self) -> usize {
        self.length().unwrap()
    }

    pub fn offset(&self) -> Output<usize> {
        unsafe { self.read_lock().offset() }
    }

    pub fn flags(&self) -> Output<MemFlags> {
        unsafe { self.read_lock().flags() }
    }
}

// unsafe impl<T: ClNumber> MemPtr<T> for Buffer<T> {
//     unsafe fn mem_ptr(&self) -> cl_mem {
//         self.inner.mem_ptr()
//     }
// }

// macro_rules! __impl_mem_info {
//     ($name:ident, $flag:ident, $output_t:ty) => {
//         impl<T> DeviceMem<T> where T: Debug + Sync + Send {
//             pub fn $name(&self) -> Output<$output_t> {
//                 self.get_info(MemInfo::$flag)
//                     .map(|ret| unsafe { ret.into_one() })
//             }
//         }
//     };
// }

// __impl_mem_info!(mem_type, Type, MemObjectType);
// __impl_mem_info!(flags, Flags, MemFlags);

#[cfg(test)]
mod tests {
    use crate::ll::*;
    use crate::*;

    #[test]
    fn buffer_can_be_created_with_a_length() {
        let context = testing::get_context();
        let _buffer = Buffer::<u32>::create(
            &context,
            10,
            HostAccess::ReadWrite,
            KernelAccess::ReadWrite,
            MemLocation::AllocOnDevice,
        )
        .unwrap();
    }

    #[test]
    fn buffer_can_be_created_with_a_slice_of_data() {
        let context = testing::get_context();
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let _buffer = Buffer::create(
            &context,
            &data[..],
            HostAccess::NoAccess,
            KernelAccess::ReadWrite,
            MemLocation::CopyToDevice,
        )
        .unwrap();
    }

    #[test]
    fn buffer_reference_count_works() {
        let buffer = testing::get_buffer::<u32>(10);

        let ref_count = buffer
            .reference_count()
            .expect("Failed to call buffer.reference_count()");
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn buffer_size_works() {
        let buffer = testing::get_buffer::<u32>(10);
        let size = buffer.size().expect("Failed to call buffer.size()");
        assert_eq!(size, 40);
    }

    // #[test]
    // fn device_mem_method_mem_type_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let _out: MemObjectType = device_mem
    //         .mem_type()
    //         .expect("Failed to call device_mem.mem_type()");
    // }

    #[test]
    fn buffer_flags_works() {
        let buffer = testing::get_buffer::<u32>(10);
        let flags = buffer.flags().expect("Failed to call buffer.flags()");
        assert_eq!(
            flags,
            MemFlags::KERNEL_READ_WRITE
                | MemFlags::ALLOC_HOST_PTR
                | MemFlags::READ_WRITE_ALLOC_HOST_PTR
        );
    }

    #[test]
    fn buffer_offset_works() {
        let buffer = testing::get_buffer::<u32>(10);
        let offset = buffer.offset().expect("Failed to call buffer.offset()");
        assert_eq!(offset, 0);
    }
}
