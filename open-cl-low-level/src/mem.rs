use std::fmt;
use libc::c_void;

use crate::ffi::{
    clCreateBuffer, clGetMemObjectInfo, cl_context, cl_int, cl_mem, cl_mem_flags, cl_mem_info,
};

use crate::cl_helpers::cl_get_info5;
use crate::{
    build_output, ClContext, ClPointer, ContextPtr, HostAccessMemFlags,
    KernelAccessMemFlags, MemFlags, MemInfo, MemLocationMemFlags, Output,
    ObjectWrapper,
};

use crate::numbers::{FFINumber, NumberType, NumberTyped};


/// Low-level helper for creating a cl_mem buffer from a context, mem flags, and a buffer creator.
///
/// # Safety
/// Use of a invalid cl_context in this function call is undefined behavior.
pub unsafe fn cl_create_buffer_with_creator<T: FFINumber, B: BufferCreator<T>>(
    context: cl_context,
    mem_flags: cl_mem_flags,
    buffer_creator: B,
) -> Output<cl_mem> {
    cl_create_buffer(
        context,
        mem_flags,
        buffer_creator.buffer_byte_size(),
        buffer_creator.buffer_ptr()
    )
}

/// Low level helper functin for creating cl_mem buffer.
///
/// # Safety
/// Calling this function with an invalid context, or an incorrect size in bytes,
/// or an invalid host pointer is undefined behavior.
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

pub fn cl_get_mem_object_info<T>(device_mem: cl_mem, flag: cl_mem_info) -> Output<ClPointer<T>>
where
    T: Copy,
{
    unsafe { cl_get_info5(device_mem, flag, clGetMemObjectInfo) }
}

pub trait BufferCreator<T: FFINumber>: Sized {
    /// The SizeAndPtr of a buffer creation arg.
    ///
    /// Currently the only 2 types that implement BufferCreator are
    /// `usize` representiing length/size and &[T] for ClNumber T representing data.
    fn buffer_byte_size(&self) -> usize;
    fn buffer_ptr(&self) -> *mut c_void;
    fn mem_config(&self) -> MemConfig;
}

impl<T: FFINumber> BufferCreator<T> for &[T] {
    fn buffer_byte_size(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    fn buffer_ptr(&self) -> *mut c_void {
        self.as_ptr() as *const _ as *mut c_void

    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}

impl<T: FFINumber> BufferCreator<T> for &mut [T] {
    fn buffer_byte_size(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    fn buffer_ptr(&self) -> *mut c_void {
        self.as_ptr() as *const _ as *mut c_void

    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_data()
    }
}


impl<T: FFINumber> BufferCreator<T> for usize {
    fn buffer_byte_size(&self) -> usize {
        std::mem::size_of::<T>() * *self
    }

    fn buffer_ptr(&self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn mem_config(&self) -> MemConfig {
        MemConfig::for_size()
    }
}

/// The MemPtr trait gives access to the cl_mem of a wrapping object and provides
/// functions for cl_mem info.
///
/// # Safety
/// This trait is unsafe because it allows access to an un-reference-counted raw pointer.
pub unsafe trait MemPtr: NumberTyped {
    /// Returns a copy to the cl_mem of the implementor.
    ///
    /// # Safety
    /// This function is unsafe because it returns an uncounted cl_mem
    /// object and gives access to a raw pointer.
    unsafe fn mem_ptr(&self) -> cl_mem;

    /// Returns a reference to the cl_mem of the implementor.
    ///
    /// # Safety
    /// This function is unsafe because it results in an uncounted copy of
    /// a cl_mem if the user dereferences the reference.
    unsafe fn mem_ptr_ref(&self) -> &cl_mem;

    /// Returns the ClPointer of the info type of a given MemInfo flag.
    ///
    /// # Safety
    /// Calling this function a mismatch between the MemInfo's expected type and T is
    /// undefined behavior.
    unsafe fn get_info<I: Copy>(&self, flag: MemInfo) -> Output<ClPointer<I>> {
        cl_get_mem_object_info::<I>(self.mem_ptr(), flag.into())
    }

    /// Returns the len of the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn len(&self) -> Output<usize> {
        let mem_size_in_bytes = self.size()?;
        Ok(mem_size_in_bytes / self.number_type().size_of_t())
    }

    /// Determines if ClMem is empty or not.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn is_empty(&self) -> Output<bool> {
        self.len().map(|l| l == 0)
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

    // /// Returns the associated_memobject of the ClMem.
    // ///
    // /// # Safety
    // /// associated_memobject is unsafe because this method grants access to a
    // /// cl_mem object that already exists as an owned cl_mem object. Without
    // /// synchronized access, the use of these objects can lead to undefined
    // /// behavior.
    // unsafe fn associated_memobject(&self) -> Output<ClMem<T>> {
    //     self.get_info::<cl_mem>(MemInfo::AssociatedMemobject)
    //         .map(|ret| {
    //             let mem_obj: cl_mem = ret.into_one();
    //             retain_mem(mem_obj);
    //             ClMem::new(mem_obj)
    //         })
    //         .map_err(|e| match e {
    //             Error::ClObjectCannotBeNull => NO_ASSOCIATED_MEM_OBJECT,
    //             other => other,
    //         })
    // }

    /// Returns the ClContext of the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn context(&self) -> Output<ClContext> {
        self.get_info::<cl_context>(MemInfo::Context)
            .and_then(|cl_ptr| ClContext::retain_new(cl_ptr.into_one()))
    }

    /// Returns the reference count info for the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn reference_count(&self) -> Output<u32> {
        self.get_info(MemInfo::ReferenceCount)
            .map(|ret| ret.into_one())
    }

    /// Returns the size info for the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn size(&self) -> Output<usize> {
        self.get_info(MemInfo::Size).map(|ret| ret.into_one())
    }

    /// Returns the offset info for the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn offset(&self) -> Output<usize> {
        self.get_info(MemInfo::Offset).map(|ret| ret.into_one())
    }

    /// Returns the MemFlag info for the ClMem.
    ///
    /// # Safety
    /// Calling this function with an invalid ClMem is invalid behavior.
    unsafe fn flags(&self) -> Output<MemFlags> {
        self.get_info(MemInfo::Flags).map(|ret| ret.into_one())
    }

    // // TODO: figure out what this is...
    // fn mem_type(&self) -> Output<MemType> {
    //     unsafe { self.get_info(MemInfo::Type).map(|ret| ret.into_one()) }
    // }
}

#[derive(Eq, PartialEq)]
pub struct ClMem {
    inner: ObjectWrapper<cl_mem>,
    t: NumberType,
}

impl NumberTyped for ClMem {
    fn number_type(&self) -> NumberType {
        self.t
    }
}

impl ClMem {
    /// Instantiates a new ClMem of type T.
    ///
    /// # Safety
    /// This function does not retain its cl_mem, but will release its cl_mem
    /// when it is dropped. Mismanagement of a cl_mem's lifetime.  Therefore,
    /// this function is unsafe.
    pub unsafe fn new<T: FFINumber>(object: cl_mem) -> Output<ClMem> {
        Ok(ClMem {
            inner: ObjectWrapper::new(object)?,
            t: T::number_type()
        })
    }

    pub fn create<T: FFINumber, B: BufferCreator<T>>(
        context: &ClContext,
        buffer_creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<ClMem> {
        unsafe {
            let mem_object = cl_create_buffer_with_creator(
                context.context_ptr(),
                cl_mem_flags::from(host_access)
                    | cl_mem_flags::from(kernel_access)
                    | cl_mem_flags::from(mem_location),
                buffer_creator,
            )?;
            ClMem::new::<T>(mem_object)
        }
    }

    /// Created a device memory buffer given the context, the buffer creator and some config.
    /// There are some buffer creators that are not valid for some MemConfigs. However, a
    /// mismatch of type and configuration between a buffer creator and the MemConfig will,
    /// at worst, result in this function call returning an error.
    ///
    /// # Safety
    /// Using an invalid context in this function call is undefined behavior.
    pub unsafe fn create_with_config<T: FFINumber, B: BufferCreator<T>>(
        context: &ClContext,
        buffer_creator: B,
        mem_config: MemConfig,
    ) -> Output<ClMem> {
        let mem_object = cl_create_buffer_with_creator(
            context.context_ptr(),
            mem_config.into(),
            buffer_creator,
        )?;
        ClMem::new::<T>(mem_object)
    }
}

unsafe impl MemPtr for ClMem {
    unsafe fn mem_ptr(&self) -> cl_mem {
        self.inner.cl_object()
    }

    unsafe fn mem_ptr_ref(&self) -> &cl_mem {
        self.inner.cl_object_ref()
    }
}

unsafe impl Send for ClMem {}

impl fmt::Debug for ClMem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", unsafe { self.mem_ptr() })
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn mem_can_be_created_with_len() {
        let (context, _devices) = ll_testing::get_context();
        let mem_config = MemConfig::default();
        let _mem: ClMem =
            unsafe { ClMem::create_with_config::<u32, usize>(&context, 10, mem_config).unwrap() };
    }

    #[test]
    fn mem_can_be_created_with_slice() {
        let (context, _devices) = ll_testing::get_context();
        let data: Vec<u32> = vec![0, 1, 2, 3, 4];
        let mem_config = MemConfig::for_data();
        let _mem: ClMem =
            unsafe { ClMem::create_with_config(&context, &data[..], mem_config).unwrap() };
    }

    mod mem_ptr_trait {
        use crate::*;

        #[test]
        fn len_method_works() {
            let (_devices, _context, ll_mem) = ll_testing::get_mem::<u32>(10);
            let len = unsafe { ll_mem.len().unwrap() };
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
