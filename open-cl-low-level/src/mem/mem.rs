use std::fmt;

use crate::cl::{cl_mem, cl_mem_flags};
use crate::cl::{MemFlags, MemInfo, ObjectWrapper};
use crate::numbers::Number;
use crate::numbers::{NumberType, NumberTyped, NumberTypedT};
use crate::{Context, ContextPtr, Output};

use super::{functions, BufferBuilder, HostAccess, KernelAccess, MemConfig, MemLocation};

#[derive(Eq, PartialEq)]
pub struct Mem {
    inner: ObjectWrapper<cl_mem>,
    t: NumberType,
}

impl NumberTyped for Mem {
    fn number_type(&self) -> NumberType {
        self.t
    }
}

impl Mem {
    /// Instantiates a new Mem of type T.
    ///
    /// # Safety
    /// This function does not retain its cl_mem, but will release its cl_mem
    /// when it is dropped. Mismanagement of a cl_mem's lifetime.  Therefore,
    /// this function is unsafe.
    pub unsafe fn new<T: Number + NumberTypedT>(object: cl_mem) -> Output<Mem> {
        Ok(Mem {
            inner: ObjectWrapper::new(object),
            t: T::number_type(),
        })
    }

    pub fn create<T: Number + NumberTypedT, B: BufferBuilder>(
        context: &Context,
        buffer_creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation,
    ) -> Output<Mem> {
        unsafe {
            let mem_object = functions::create_buffer::<T, B>(
                context.context_ptr(),
                cl_mem_flags::from(host_access)
                    | cl_mem_flags::from(kernel_access)
                    | cl_mem_flags::from(mem_location),
                buffer_creator,
            )?;
            Mem::new::<T>(mem_object)
        }
    }

    /// Created a device memory buffer given the context, the buffer creator and some config.
    /// There are some buffer creators that are not valid for some MemConfigs. However, a
    /// mismatch of type and configuration between a buffer creator and the MemConfig will,
    /// at worst, result in this function call returning an error.
    ///
    /// # Safety
    /// Using an invalid context in this function call is undefined behavior.
    pub unsafe fn create_with_config<T: Number + NumberTypedT, B: BufferBuilder>(
        context: &Context,
        buffer_builder: B,
        mem_config: MemConfig,
    ) -> Output<Mem> {
        let mem_object = functions::create_buffer::<T, B>(
            context.context_ptr(),
            mem_config.into(),
            buffer_builder,
        )?;
        Mem::new::<T>(mem_object)
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
    /// Returns the len of the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn len(&self) -> Output<usize> {
        let mem_size_in_bytes = self.size()?;
        Ok(mem_size_in_bytes / self.number_type().size_of())
    }

    /// Determines if Mem is empty or not.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
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

    // /// Returns the associated_memobject of the Mem.
    // ///
    // /// # Safety
    // /// associated_memobject is unsafe because this method grants access to a
    // /// cl_mem object that already exists as an owned cl_mem object. Without
    // /// synchronized access, the use of these objects can lead to undefined
    // /// behavior.
    // unsafe fn associated_memobject(&self) -> Output<Mem<T>> {
    //     self.get_info::<cl_mem>(MemInfo::AssociatedMemobject)
    //         .map(|ret| {
    //             let mem_obj: cl_mem = ret.into_one();
    //             retain_mem(mem_obj);
    //             Mem::new(mem_obj)
    //         })
    //         .map_err(|e| match e {
    //             Error::ClObjectCannotBeNull => NO_ASSOCIATED_MEM_OBJECT,
    //             other => other,
    //         })
    // }

    /// Returns the ClContext of the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn context(&self) -> Output<Context> {
        functions::get_info_context(self.mem_ptr()).map(|c| Context::retain_new(c))
    }

    /// Returns the reference count info for the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn reference_count(&self) -> Output<u32> {
        functions::get_info_u32(self.mem_ptr(), MemInfo::ReferenceCount.into())
    }

    /// Returns the size info for the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn size(&self) -> Output<usize> {
        functions::get_info_usize(self.mem_ptr(), MemInfo::Size.into())
    }

    /// Returns the offset info for the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn offset(&self) -> Output<usize> {
        functions::get_info_usize(self.mem_ptr(), MemInfo::Offset.into())
    }

    /// Returns the MemFlag info for the Mem.
    ///
    /// # Safety
    /// Calling this function with an invalid Mem is invalid behavior.
    unsafe fn flags(&self) -> Output<MemFlags> {
        functions::get_info_flags(self.mem_ptr()).map(|b| MemFlags::from_bits(b).unwrap())
    }

    // // TODO: figure out what this is...
    // fn mem_type(&self) -> Output<MemType> {
    //     unsafe { self.get_info(MemInfo::Type).map(|ret| ret.into_one()) }
    // }
}

unsafe impl MemPtr for Mem {
    unsafe fn mem_ptr(&self) -> cl_mem {
        self.inner.cl_object()
    }

    unsafe fn mem_ptr_ref(&self) -> &cl_mem {
        self.inner.cl_object_ref()
    }
}

unsafe impl Send for Mem {}

impl fmt::Debug for Mem {
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
        let _mem: Mem =
            unsafe { Mem::create_with_config::<u32, usize>(&context, 10, mem_config).unwrap() };
    }

    #[test]
    fn mem_can_be_created_with_slice() {
        let (context, _devices) = ll_testing::get_context();
        let data: Vec<u32> = vec![0, 1, 2, 3, 4];
        let mem_config = MemConfig::for_data();
        let _mem: Mem = unsafe {
            Mem::create_with_config::<u32, &[u32]>(&context, &data[..], mem_config).unwrap()
        };
    }

    mod mem_ptr_trait {
        use crate::cl::MemFlags;
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
