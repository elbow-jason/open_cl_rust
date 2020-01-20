use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::Context;
use crate::ll::{
    Output, ClNumber, ClMem, MemFlags, BufferCreator, MemPtr, MemConfig, KernelAccess,
    HostAccess, MemLocation,
};
use crate::ffi::{cl_mem};

pub struct Buffer<T: ClNumber> {
    inner: ClMem<T>,
    _context: Context,
    _phantom: PhantomData<T>,
}

unsafe impl<T: ClNumber> Send for Buffer<T> {}

impl<T: ClNumber> Clone for Buffer<T> {
    fn clone(&self) -> Buffer<T> {
        Buffer{
            inner: self.inner.clone(),
            _context: self._context.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: ClNumber> Debug for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer{{{:?}}}",self.inner)
    }
}

impl<T: ClNumber> Buffer<T> {
    pub fn new(ll_mem: ClMem<T>, context: Context) -> Buffer<T> {
        Buffer{
            inner: ll_mem,
            _context: context,
            _phantom: PhantomData,
        }
    }

    pub fn create<B: BufferCreator<T>>(
        context: &Context,
        creator: B,
        host_access: HostAccess,
        kernel_access: KernelAccess,
        mem_location: MemLocation
    ) -> Output<Buffer<T>> {
        let ll_mem = ClMem::create(context.low_level_context(), creator, host_access, kernel_access, mem_location)?;
        Ok(Buffer::new(ll_mem, context.clone()))
    }
}

unsafe impl<T: ClNumber> MemPtr<T> for Buffer<T> {
    unsafe fn mem_ptr(&self) -> cl_mem {
        self.inner.mem_ptr()
    }
}

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
    use crate::*;
    use crate::ll::*;

    // fn get_session() -> Session {
    //     let src = "__kernel void test(__global int *i) { *i += 1; }";
    //     // let device = Device::default();
    //     let devices = vec![device];
    //     Session::create_sessions(&devices[..], src).expect("Failed to create Session").remove(0)
    // }

    // fn get_device_mem() -> (Session, DeviceMem<usize>) {
    //     let session = get_session();
    //     let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    //     let dmem = DeviceMem::create_read_write_from(session.context(), &data[..])
    //         .expect("Failed to create_read_write_from one to nine");
    //     (session, dmem)
    // }

    #[test]
    fn buffer_can_be_created_with_a_length() {
        let context = testing::get_context();
        let _buffer = Buffer::<u32>::create(
            &context,
            10,
            HostAccess::ReadWrite,
            KernelAccess::ReadWrite,
            MemLocation::AllocOnDevice
        ).unwrap();
    }

    // #[test]
    // fn buffer_can_be_created_with_a_slice_of_data() {
    //     let context = testing::get_context();
    //     let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    //     let _buffer = Buffer::create(&context, &data[..]).unwrap();
    // }


    // #[test]
    // fn device_mem_method_associated_memobject_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let result: Output<DeviceMem<usize>> = device_mem.associated_memobject();
    //     match result {
    //         Ok(_dmem) => (),
    //         Err(Error::DeviceMemError(DeviceMemError::NoAssociatedMemObject)) => (),
    //         Err(e) => panic!(
    //             "Call device_mem.associated_memobject() encountered an unexpected Error: {:?}",
    //             e
    //         ),
    //     }
    // }

    // #[test]
    // fn device_mem_method_reference_count_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let out = device_mem
    //         .reference_count()
    //         .expect("Failed to call device_mem.reference_count()");
    //     assert!(out == 1);
    // }
    // #[test]
    // fn device_mem_method_size_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let out = device_mem.size().expect("Failed to call device_mem.size()");
    //     let size_t_in_bytes = std::mem::size_of::<usize>();
    //     let len = out / size_t_in_bytes;
    //     assert_eq!(len, 9);
    // }
    // #[test]
    // fn device_mem_method_mem_type_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let _out: MemObjectType = device_mem
    //         .mem_type()
    //         .expect("Failed to call device_mem.mem_type()");
    // }
    // #[test]
    // fn device_mem_method_flags_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let _out: MemFlags = device_mem
    //         .flags()
    //         .expect("Failed to call device_mem.flags()");
    // }
    // #[test]
    // fn device_mem_method_map_count_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let out = device_mem
    //         .map_count()
    //         .expect("Failed to call device_mem.map_count()");
    //     assert_eq!(out, 0);
    // }
    // #[test]
    // fn device_mem_method_context_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let _out: Context = device_mem
    //         .context()
    //         .expect("Failed to call device_mem.context()");
    // }
    // #[test]
    // fn device_mem_method_offset_works() {
    //     let (_sess, device_mem) = get_device_mem();
    //     let out: usize = device_mem
    //         .offset()
    //         .expect("Failed to call device_mem.offset()");
    //     assert_eq!(out, 0);
    // }
}
