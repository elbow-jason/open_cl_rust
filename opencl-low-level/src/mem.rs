use std::fmt;
use std::marker::PhantomData;

use libc::c_void;

use crate::ffi::{
    clCreateBuffer, clGetMemObjectInfo, cl_context, cl_int, cl_mem, cl_mem_flags, cl_mem_info,
};

use crate::cl_helpers::cl_get_info5;
use crate::{
    build_output, ClContext, ClNumber, ClPointer, ContextPtr, Error, HostAccessMemFlags,
    KernelAccessMemFlags, MemFlags, MemInfo, MemLocationMemFlags, Output, SizeAndPtr,
};

__release_retain!(mem, MemObject);

unsafe fn release_mem(mem: cl_mem) {
    cl_release_mem(mem).unwrap_or_else(|e| {
        panic!("Failed to release cl_mem {:?} due to {:?}", mem, e);
    })
}

unsafe fn retain_mem(mem: cl_mem) {
    cl_retain_mem(mem).unwrap_or_else(|e| {
        panic!("Failed to retain cl_mem {:?} due to {:?}", mem, e);
    })
}

/// An error related to DeviceMems.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum MemError {
    #[fail(display = "Given ClMem has no associated cl_mem object")]
    NoAssociatedMemObject,
}

pub const NO_ASSOCIATED_MEM_OBJECT: Error = Error::MemError(MemError::NoAssociatedMemObject);

pub unsafe fn cl_create_buffer_with_creator<B, T>(
    context: cl_context,
    mem_flags: cl_mem_flags,
    mut buffer_creator: B,
) -> Output<cl_mem>
where
    T: ClNumber,
    B: BufferCreator<T>,
{
    let SizeAndPtr(buf_size, buf_ptr) = buffer_creator.buffer_size_and_ptr();
    cl_create_buffer(context, mem_flags, buf_size, buf_ptr)
}

pub unsafe fn cl_create_buffer(
    context: cl_context,
    mem_flags: cl_mem_flags,
    size_in_bytes: usize,
    ptr: *mut c_void,
) -> Output<cl_mem> {
    let mut err_code: cl_int = 0;
    let cl_mem_object: cl_mem =
        clCreateBuffer(context, mem_flags, size_in_bytes, ptr, &mut err_code);
    build_output(cl_mem_object, err_code)
}

// NOTE: Fix this cl_mem_info arg
pub fn cl_get_mem_object_info<T>(device_mem: cl_mem, flag: cl_mem_info) -> Output<ClPointer<T>>
where
    T: Copy,
{
    unsafe { cl_get_info5(device_mem, flag, clGetMemObjectInfo) }
}

pub trait BufferCreator<T: ClNumber>: Sized {
    unsafe fn buffer_size_and_ptr(&mut self) -> SizeAndPtr<*mut c_void>;
    fn mem_config(&self) -> MemConfig;
}

impl<T: ClNumber> BufferCreator<T> for &[T] {
    unsafe fn buffer_size_and_ptr(&mut self) -> SizeAndPtr<*mut c_void> {
        SizeAndPtr(
            std::mem::size_of::<T>() * self.len(),
            self.as_ptr() as *mut c_void,
        )
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}

impl<T: ClNumber> BufferCreator<T> for usize {
    unsafe fn buffer_size_and_ptr(&mut self) -> SizeAndPtr<*mut c_void> {
        SizeAndPtr(
            (std::mem::size_of::<T>() * *self) as usize,
            std::ptr::null_mut(),
        )
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_size()
    }
}

pub unsafe trait MemPtr<T: ClNumber> {
    unsafe fn mem_ptr(&self) -> cl_mem;
    unsafe fn mem_ptr_ref(&self) -> &cl_mem;

    unsafe fn get_info<I: Copy>(&self, flag: MemInfo) -> Output<ClPointer<I>> {
        cl_get_mem_object_info::<I>(self.mem_ptr(), flag.into())
    }

    // len will panic if the mem is not in a valid state.
    fn len(&self) -> Output<usize> {
        let mem_size_in_bytes = unsafe { self.size() }?;
        Ok(mem_size_in_bytes / std::mem::size_of::<T>())
    }

    // /// This is SUPER unsafe. Leave this out.
    // /// Someone: "But elbow-jason you can use this to make a slice!"
    // /// Me: "A slice with what lifetime? Is it safe to read?"
    // /// Me: "If you want the underlying data read the buffer like a human being."
    // fn host_ptr(&self) -> Output<Option<Vec<T>>>
    // where
    //     T: Copy,
    // {
    //     unsafe {
    //         self.get_info::<T>(MemInfo::HostPtr).map(|ret| {
    //             // let host_vec =
    //             if ret.is_null() {
    //                 return None;
    //             }
    //             // if host_vec.as_ptr() as usize == 1 {
    //             //     return None;
    //             // }
    //             Some(ret.into_vec())
    //         })
    //     }
    // }

    /// associated_memobject is unsafe because this method grants access to a
    /// cl_mem object that already exists as an owned cl_mem object. Without
    /// synchronized access, the use of these objects can lead to undefined
    /// behavior.
    unsafe fn associated_memobject(&self) -> Output<ClMem<T>> {
        self.get_info::<cl_mem>(MemInfo::AssociatedMemobject)
            .map(|ret| {
                let mem_obj: cl_mem = ret.into_one();
                retain_mem(mem_obj);
                ClMem::new(mem_obj)
            })
            .map_err(|e| match e {
                Error::ClObjectCannotBeNull => NO_ASSOCIATED_MEM_OBJECT,
                other => other,
            })
    }

    unsafe fn context(&self) -> Output<ClContext> {
        self.get_info::<cl_context>(MemInfo::Context)
            .and_then(|cl_ptr| ClContext::retain_new(cl_ptr.into_one()))
    }

    unsafe fn reference_count(&self) -> Output<u32> {
        self.get_info(MemInfo::ReferenceCount)
            .map(|ret| ret.into_one())
    }

    unsafe fn size(&self) -> Output<usize> {
        self.get_info(MemInfo::Size).map(|ret| ret.into_one())
    }

    unsafe fn offset(&self) -> Output<usize> {
        self.get_info(MemInfo::Offset).map(|ret| ret.into_one())
    }

    unsafe fn flags(&self) -> Output<MemFlags> {
        self.get_info(MemInfo::Flags).map(|ret| ret.into_one())
    }

    // // TODO: figure out what this is...
    // fn mem_type(&self) -> Output<MemType> {
    //     unsafe { self.get_info(MemInfo::Type).map(|ret| ret.into_one()) }
    // }
}

#[derive(Eq, PartialEq, Hash)]
pub struct ClMem<T: ClNumber> {
    object: cl_mem,
    _phantom: PhantomData<T>,
}

impl<T: ClNumber> ClMem<T> {
    pub unsafe fn new(object: cl_mem) -> ClMem<T> {
        ClMem {
            object,
            _phantom: PhantomData,
        }
    }

    pub fn create<B>(
        context: &ClContext,
        buffer_creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<ClMem<T>>
    where
        B: BufferCreator<T>,
    {
        unsafe {
            let mem_object = cl_create_buffer_with_creator(
                context.context_ptr(),
                cl_mem_flags::from(host_access)
                    | cl_mem_flags::from(kernel_access)
                    | cl_mem_flags::from(mem_location),
                buffer_creator,
            )?;
            Ok(ClMem::new(mem_object))
        }
    }
    pub unsafe fn create_with_config<B>(
        context: &ClContext,
        buffer_creator: B,
        mem_config: MemConfig,
    ) -> Output<ClMem<T>>
    where
        B: BufferCreator<T>,
    {
        let mem_object = cl_create_buffer_with_creator(
            context.context_ptr(),
            mem_config.into(),
            buffer_creator,
        )?;
        Ok(ClMem::new(mem_object))
    }
}

unsafe impl<T: ClNumber> MemPtr<T> for ClMem<T> {
    unsafe fn mem_ptr(&self) -> cl_mem {
        self.object
    }

    unsafe fn mem_ptr_ref(&self) -> &cl_mem {
        &self.object
    }
}

impl<T: ClNumber> Drop for ClMem<T>
where
    T: ClNumber,
{
    fn drop(&mut self) {
        unsafe {
            release_mem(self.object);
        }
    }
}

impl<T: ClNumber> Clone for ClMem<T> {
    fn clone(&self) -> ClMem<T> {
        unsafe {
            retain_mem(self.object);
            ClMem::new(self.object)
        }
    }
}

unsafe impl<T: ClNumber> Send for ClMem<T> {}

impl<T: ClNumber> fmt::Debug for ClMem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClMem{{{:?}}}", self.object)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn mem_can_be_created_with_len() {
        let (context, _devices) = ll_testing::get_context();
        let mem_config = MemConfig::default();
        let _mem: ClMem<u32> =
            unsafe { ClMem::create_with_config(&context, 10, mem_config).unwrap() };
    }

    #[test]
    fn mem_can_be_created_with_slice() {
        let (context, _devices) = ll_testing::get_context();
        let mut data: Vec<u32> = vec![0, 1, 2, 3, 4];
        let mem_config = MemConfig::for_data();
        let _mem: ClMem<u32> =
            unsafe { ClMem::create_with_config(&context, &data[..], mem_config).unwrap() };
    }

    mod mem_ptr_trait {
        use crate::*;

        #[test]
        fn len_method_works() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let len = ll_mem.len().unwrap();
            assert_eq!(len, 10);
        }

        #[test]
        fn reference_count_method_works() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let ref_count = unsafe { ll_mem.reference_count().unwrap() };
            assert_eq!(ref_count, 1);
        }

        #[test]
        fn size_method_returns_size_in_bytes() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let bytes_size = unsafe { ll_mem.size().unwrap() };
            assert_eq!(bytes_size, 10 * std::mem::size_of::<u32>());
        }

        #[test]
        fn offset_method_works() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let offset = unsafe { ll_mem.offset().unwrap() };
            assert_eq!(offset, 0);
        }

        #[test]
        fn flags_method_works() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let flags = unsafe { ll_mem.flags().unwrap() };
            assert_eq!(flags, MemFlags::READ_WRITE_ALLOC_HOST_PTR);
        }
    }
}

pub enum KernelAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<KernelAccess> for KernelAccessMemFlags {
    fn from(kernel_access: KernelAccess) -> KernelAccessMemFlags {
        match kernel_access {
            KernelAccess::ReadOnly => KernelAccessMemFlags::READ_ONLY,
            KernelAccess::WriteOnly => KernelAccessMemFlags::WRITE_ONLY,
            KernelAccess::ReadWrite => KernelAccessMemFlags::READ_WRITE,
        }
    }
}

impl From<KernelAccess> for MemFlags {
    fn from(kernel_access: KernelAccess) -> MemFlags {
        MemFlags::from(KernelAccessMemFlags::from(kernel_access))
    }
}

impl From<KernelAccess> for cl_mem_flags {
    fn from(kernel_access: KernelAccess) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(kernel_access))
    }
}

pub enum HostAccess {
    ReadOnly,
    WriteOnly,
    NoAccess,
    ReadWrite,
}

impl From<HostAccess> for HostAccessMemFlags {
    fn from(host_access: HostAccess) -> HostAccessMemFlags {
        match host_access {
            HostAccess::ReadOnly => HostAccessMemFlags::READ_ONLY,
            HostAccess::WriteOnly => HostAccessMemFlags::WRITE_ONLY,
            HostAccess::NoAccess => HostAccessMemFlags::NO_ACCESS,
            HostAccess::ReadWrite => HostAccessMemFlags::READ_WRITE,
        }
    }
}

impl From<HostAccess> for MemFlags {
    fn from(host_access: HostAccess) -> MemFlags {
        MemFlags::from(HostAccessMemFlags::from(host_access))
    }
}

impl From<HostAccess> for cl_mem_flags {
    fn from(host_access: HostAccess) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(host_access))
    }
}

/// The enumeration of how memory allocation (or not) can be directed.
///
/// This forum post has some good explanations:
///   https://software.intel.com/en-us/forums/opencl/topic/708049
pub enum MemLocation {
    KeepInPlace,
    AllocOnDevice,
    CopyToDevice,
    ForceCopyToDevice,
}

impl From<MemLocation> for MemLocationMemFlags {
    fn from(mem_location: MemLocation) -> MemLocationMemFlags {
        match mem_location {
            MemLocation::KeepInPlace => MemLocationMemFlags::KEEP_IN_PLACE,
            MemLocation::AllocOnDevice => MemLocationMemFlags::ALLOC_ON_DEVICE,
            MemLocation::CopyToDevice => MemLocationMemFlags::COPY_TO_DEVICE,
            MemLocation::ForceCopyToDevice => MemLocationMemFlags::FORCE_COPY_TO_DEVICE,
        }
    }
}

impl From<MemLocation> for MemFlags {
    fn from(mem_location: MemLocation) -> MemFlags {
        MemFlags::from(MemLocationMemFlags::from(mem_location))
    }
}

impl From<MemLocation> for cl_mem_flags {
    fn from(mem_location: MemLocation) -> cl_mem_flags {
        cl_mem_flags::from(MemFlags::from(mem_location))
    }
}

pub struct MemConfig {
    pub host_access: HostAccess,
    pub kernel_access: KernelAccess,
    pub mem_location: MemLocation,
}

impl MemConfig {
    pub fn build(
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> MemConfig {
        MemConfig {
            host_access,
            kernel_access,
            mem_location,
        }
    }
}

impl From<MemConfig> for MemFlags {
    fn from(mem_config: MemConfig) -> MemFlags {
        unsafe { MemFlags::from_bits_unchecked(cl_mem_flags::from(mem_config)) }
    }
}

impl From<MemConfig> for cl_mem_flags {
    fn from(mem_config: MemConfig) -> cl_mem_flags {
        cl_mem_flags::from(mem_config.host_access)
            | cl_mem_flags::from(mem_config.kernel_access)
            | cl_mem_flags::from(mem_config.mem_location)
    }
}

impl Default for MemConfig {
    fn default() -> MemConfig {
        MemConfig {
            host_access: HostAccess::ReadWrite,
            kernel_access: KernelAccess::ReadWrite,
            mem_location: MemLocation::AllocOnDevice,
        }
    }
}

impl MemConfig {
    pub fn for_data() -> MemConfig {
        MemConfig {
            mem_location: MemLocation::CopyToDevice,
            ..MemConfig::default()
        }
    }

    pub fn for_size() -> MemConfig {
        MemConfig {
            mem_location: MemLocation::AllocOnDevice,
            ..MemConfig::default()
        }
    }
}

#[cfg(test)]
mod mem_flags_tests {
    use super::*;
    use crate::KernelAccessMemFlags;

    #[test]
    fn kernel_access_read_only_conversion_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::ReadOnly;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::READ_ONLY
        );
    }

    #[test]
    fn kernel_access_write_only_conversion_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::WriteOnly;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::WRITE_ONLY
        );
    }

    #[test]
    fn kernel_access_convert_read_write_into_kernel_access_mem_flag() {
        let kernel_access = KernelAccess::ReadWrite;
        assert_eq!(
            KernelAccessMemFlags::from(kernel_access),
            KernelAccessMemFlags::READ_WRITE
        );
    }

    #[test]
    fn host_access_read_only_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::ReadOnly;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::READ_ONLY
        );
    }

    #[test]
    fn host_access_write_only_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::WriteOnly;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::WRITE_ONLY
        );
    }

    #[test]
    fn host_access_read_write_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::ReadWrite;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::READ_WRITE
        );
    }

    #[test]
    fn host_access_no_access_conversion_into_host_access_mem_flag() {
        let host_access = HostAccess::NoAccess;
        assert_eq!(
            HostAccessMemFlags::from(host_access),
            HostAccessMemFlags::NO_ACCESS
        );
    }

    #[test]
    fn mem_location_keep_in_place_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::KeepInPlace;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::KEEP_IN_PLACE
        );
    }

    #[test]
    fn mem_location_alloc_on_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::AllocOnDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::ALLOC_ON_DEVICE
        );
    }

    #[test]
    fn mem_location_copy_to_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::CopyToDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_location_force_copy_to_device_conversion_into_mem_location_mem_flag() {
        let mem_location = MemLocation::ForceCopyToDevice;
        assert_eq!(
            MemLocationMemFlags::from(mem_location),
            MemLocationMemFlags::FORCE_COPY_TO_DEVICE
        );
    }

    #[test]
    fn mem_config_conversion_into_cl_mem_flags() {
        let mem_location = MemLocation::AllocOnDevice;
        let host_access = HostAccess::ReadWrite;
        let kernel_access = KernelAccess::ReadWrite;
        let mem_config = MemConfig {
            mem_location,
            host_access,
            kernel_access,
        };
        let expected = MemFlags::ALLOC_HOST_PTR.bits()
            | MemFlags::HOST_READ_WRITE.bits()
            | MemFlags::KERNEL_READ_WRITE.bits();

        assert_eq!(cl_mem_flags::from(mem_config), expected);
    }
}
