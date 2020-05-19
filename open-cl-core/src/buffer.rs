use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ll::{Context as ClContext, Mem as ClMem, MemPtr};
use crate::{
    BufferBuilder, ClObject, Context, HostAccess, KernelAccess, MemConfig, MemFlags, MemLocation,
    Number, NumberType, NumberTyped, NumberTypedT, Output,
};

pub struct Buffer {
    _t: NumberType,
    _mem: Arc<RwLock<ClMem>>,
    _context: Context,
}

impl NumberTyped for Buffer {
    fn number_type(&self) -> NumberType {
        self._t
    }
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl Clone for Buffer {
    fn clone(&self) -> Buffer {
        Buffer {
            _t: self._t,
            _mem: self._mem.clone(),
            _context: self._context.clone(),
        }
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer{{{:?}}}", self._mem)
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let left = self._mem.read().unwrap().mem_ptr().as_ptr();
            let right = other._mem.read().unwrap().mem_ptr().as_ptr();
            std::ptr::eq(left, right)
        }
    }
}

impl Buffer {
    pub fn new(ll_mem: ClMem, context: Context) -> Buffer {
        Buffer {
            _t: ll_mem.number_type(),
            _mem: Arc::new(RwLock::new(ll_mem)),
            _context: context,
        }
    }

    pub fn create<T: Number + NumberTypedT, B: BufferBuilder>(
        context: &Context,
        creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<Buffer> {
        let ll_mem = ClMem::create::<T, B>(
            context.low_level_context(),
            creator,
            host_access,
            kernel_access,
            mem_location,
        )?;
        Ok(Buffer::new(ll_mem, context.clone()))
    }

    pub fn create_with_len<T: Number>(context: &Context, len: usize) -> Output<Buffer> {
        Buffer::create_from::<T, usize>(context, len)
    }

    pub fn create_from_slice<T: Number>(context: &Context, data: &[T]) -> Output<Buffer> {
        Buffer::create_from::<T, &[T]>(context, data)
    }

    pub fn create_from<T: Number, B: BufferBuilder>(
        context: &Context,
        creator: B,
    ) -> Output<Buffer> {
        let mem_config = { creator.mem_config() };
        Buffer::create_with_config::<T, B>(context, creator, mem_config)
    }

    pub fn create_with_config<T: Number, B: BufferBuilder>(
        context: &Context,
        creator: B,
        mem_config: MemConfig,
    ) -> Output<Buffer> {
        Buffer::create::<T, B>(
            context,
            creator,
            mem_config.host_access,
            mem_config.kernel_access,
            mem_config.mem_location,
        )
    }

    pub fn create_from_low_level_context<T: Number, B: BufferBuilder>(
        ll_context: &ClContext,
        creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<Buffer> {
        let ll_mem = ClMem::create::<T, B>(
            ll_context,
            creator,
            host_access,
            kernel_access,
            mem_location,
        )?;
        let context = Context::from_low_level_context(ll_context)?;
        Ok(Buffer::new(ll_mem, context))
    }

    pub fn read_lock(&self) -> RwLockReadGuard<ClMem> {
        self._mem.read().unwrap()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<ClMem> {
        self._mem.write().unwrap()
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
        unsafe { self.read_lock().len() }
        // Ok(size / std::mem::size_of::<T>())
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

#[cfg(test)]
mod tests {
    use crate::ll::*;
    use crate::*;

    #[test]
    fn buffer_can_be_created_with_a_length() {
        let context = testing::get_context();
        let _buffer = Buffer::create_with_len::<u32>(&context, 10).unwrap();
    }

    #[test]
    fn buffer_can_be_created_with_a_slice_of_data() {
        let context = testing::get_context();
        let data = vec![0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let _buffer = Buffer::create::<i32, &[i32]>(
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
    //     let buffer = testing::get_buffer::<u32>(10);
    //     let _out: MemObjectType = buffer.mem_type()
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
