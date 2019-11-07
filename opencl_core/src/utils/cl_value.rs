use std::fmt::Debug;
use std::mem::size_of;
// use std::marker::PhantomData;
use libc::{size_t, c_void};

use crate::ffi::{
    cl_int,
    // cl_uint,
    cl_bool,
    // cl_ulong,
    cl_device_id,
    cl_device_fp_config,
    cl_device_exec_capabilities,
    cl_device_local_mem_type,
    cl_device_mem_cache_type,
    cl_device_partition_property,
    cl_device_affinity_domain,
    cl_device_type,
    cl_platform_id,
    cl_command_queue_properties,
    cl_command_queue,
    cl_context_properties,
    cl_context,    
    cl_build_status,
    cl_program_binary_type,
    cl_program,
    cl_command_type,
    cl_mem,
    cl_mem_object_type,
    cl_mem_flags,
    cl_mem_migration_flags,
    cl_mem_info,
    cl_buffer_create_type,
    cl_event,
    // ID3D10Resource
};

use crate::error::{Output, Error};

use crate::device::Device;
use crate::platform::Platform;
use crate::program::Program;
use crate::command_queue::CommandQueue;
use crate::context::Context;
use crate::device_mem::DeviceMem;
use crate::event::Event;
use crate::utils::cl_object::ClObject;

// flags
use crate::context::flags::{ContextProperties};
use crate::device::flags::{
    DeviceType,
    DeviceFpConfig,
    DeviceExecCapabilities,
    DeviceMemCacheType,
    DeviceLocalMemType,
    DevicePartitionProperty,
    DeviceAffinityDomain,
};
use crate::command_queue::flags::{CommandQueueProperties};
use crate::program::flags::{
    BuildStatus,
    ProgramBinaryType,
};
use crate::event::event_info::{CommandExecutionStatus, CommandType};
use crate::device_mem::flags::{
    MemFlags,
    MemMigrationFlags,
    MemObjectType,
    MemInfo,
    BufferCreateType,
};



/// An error related to an Event or WaitList.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ClValueError {
    #[fail(display = "OpenCL returned invalid utf8: {:?}", _0)]
    InvalidUTF8(String),

    #[fail(display = "OpenCL returned an invalid bitfield")]
    InvalidBitfield,

    #[fail(display = "Decoder failed to decode type {}", _0)]
    ClDecoderFailed(String),
}

impl From<ClValueError> for Error {
    fn from(e: ClValueError) -> Error {
        Error::ClValueError(e)
    }
}

pub trait ClValue<T> {
    fn into_rust_value(self) -> Output<T>;
}

pub trait ClDecoder<T> {
    unsafe fn cl_decode(self) -> T;
}

pub trait ClEncoder<T> {
    unsafe fn cl_encode(self) -> Output<T>;
}

#[repr(C)]
#[derive(Debug)]
pub struct ClReturn {
    size: size_t,
    ptr: *const c_void,
}


impl ClReturn {
    pub unsafe fn into_vec<R>(self) -> Vec<R> {
        Vec::from_raw_parts(self.ptr as *mut R, self.size, self.size)
    }

    pub unsafe fn new(size: size_t, ptr: *mut c_void) -> ClReturn {
        ClReturn { size, ptr, }
    }

    pub unsafe fn new_sized<T: Sized>(val: *mut c_void) -> ClReturn {
        ClReturn::new(size_of::<T>(), val)
    }

    pub unsafe fn from_vec<T: Debug>(mut vector: Vec<T>) -> ClReturn {
        &mut vector.shrink_to_fit(); // ensure capacity == size
        let size = vector.len();
        let ptr = vector.as_ptr() as *const c_void;
        std::mem::forget(vector);

        ClReturn {
            size,
            ptr,
        }
    }

    pub unsafe fn into_ref_count(self) -> u32 {
        // Casting this thing was a nightmare.
        Vec::from_raw_parts(self.ptr as *mut u32, 1, 1).remove(0)
    }
}

impl<'a, T: 'a + Debug> ClDecoder<DeviceMem<T>> for ClReturn {
    unsafe fn cl_decode(self) -> DeviceMem<T> {
        DeviceMem::new(self.ptr as cl_mem)
    }
}

impl<'a> ClDecoder<Vec<Device>> for ClReturn {
    unsafe fn cl_decode(self) -> Vec<Device> {
        self.into_vec::<cl_device_id>()
            .into_iter()
            .map(|device_id| Device::new(device_id))
            .collect()
    }
}

impl ClDecoder<Vec<size_t>> for ClReturn {
    unsafe fn cl_decode(self) -> Vec<size_t> {
        self.into_vec::<size_t>()
    }
}

impl ClDecoder<Vec<Platform>> for ClReturn {
    unsafe fn cl_decode(self) -> Vec<Platform> {
        self.into_vec::<cl_platform_id>()
            .into_iter()
            .map(|platform_id| Platform::new(platform_id))
            .collect()
    }
}

impl ClDecoder<Vec<DevicePartitionProperty>> for ClReturn {
    unsafe fn cl_decode(self) -> Vec<DevicePartitionProperty> {
        self.into_vec()
            .into_iter()
            .map(|props: ClReturn| props.cl_decode())
            .collect::<Vec<DevicePartitionProperty>>()
    }
}

impl ClDecoder<Vec<ContextProperties>> for ClReturn {
    unsafe fn cl_decode(self) -> Vec<ContextProperties> {
        self.into_vec()
            .into_iter()
            .map(|props: ClReturn| props.cl_decode())
            .collect::<Vec<ContextProperties>>()
    }
}

impl ClDecoder<String> for ClReturn {
    unsafe fn cl_decode(self) -> String {
        let chars = self
            .into_vec::<u8>()
            .into_iter()
            .filter(|c| *c != 0u8)
            .collect();
        String::from_utf8(chars).unwrap_or_else(|e| {
            panic!("Failed to cl_decode a Vec<u8> to utf8 with error {:?}", e);  
        })
    }
}

impl ClDecoder<bool> for ClReturn {
    unsafe fn cl_decode(self) -> bool {
        debug_assert!(self.size  >= size_of::<cl_bool>());
        let val = self.ptr as *mut cl_bool;
        match *val {
            0 => false,
            1 => true,
            got => panic!("Failed to decode a cl_bool to a bool. Got: {:?}", got),
        }
    }
}

macro_rules! __impl_cl_decoder_for_handle_wrapper {
    ($cl_type:ty, $wrapper:ident) => {
        impl ClDecoder<$wrapper> for ClReturn {

            unsafe fn cl_decode(self) -> $wrapper {
                $wrapper::new(self.ptr as $cl_type)
            }
        }
    }
}



__impl_cl_decoder_for_handle_wrapper!(cl_event, Event);
__impl_cl_decoder_for_handle_wrapper!(cl_device_id, Device);
__impl_cl_decoder_for_handle_wrapper!(cl_platform_id, Platform);
__impl_cl_decoder_for_handle_wrapper!(cl_command_queue, CommandQueue);
__impl_cl_decoder_for_handle_wrapper!(cl_context, Context);
__impl_cl_decoder_for_handle_wrapper!(cl_program, Program);

macro_rules! __impl_cl_decoder_for_castable {
    ($rust_type:ty) => {
        impl ClDecoder<$rust_type> for ClReturn {
            unsafe fn cl_decode(self) -> $rust_type {
                self.ptr as *mut $rust_type as $rust_type
            }
        }
    }
}

__impl_cl_decoder_for_castable!(i32);
__impl_cl_decoder_for_castable!(u32);
__impl_cl_decoder_for_castable!(i64);
__impl_cl_decoder_for_castable!(u64);
__impl_cl_decoder_for_castable!(usize);

impl ClDecoder<[usize; 3]> for ClReturn {
    unsafe fn cl_decode(self) -> [usize; 3] {
        // NOTE: This seems very sketchy to me...
        let size = 3 * std::mem::size_of::<usize>();
        let v: Vec<usize> = Vec::from_raw_parts(self.ptr as *mut usize, size, size);
        [v[0], v[1], v[2]]
    }
}

macro_rules! __impl_cl_decoder_for_from_impl {
    ($cl_type:ty, $rust_type:ident) => {
        impl ClDecoder<$rust_type> for ClReturn {

            unsafe fn cl_decode(self) -> $rust_type {
                let val = self.ptr as *mut $cl_type;
                $rust_type::from(*val)
            }
        }
    }
}

__impl_cl_decoder_for_from_impl!(cl_device_mem_cache_type, DeviceMemCacheType);

__impl_cl_decoder_for_from_impl!(cl_device_partition_property, DevicePartitionProperty);
__impl_cl_decoder_for_from_impl!(cl_context_properties, ContextProperties);

__impl_cl_decoder_for_from_impl!(cl_device_local_mem_type, DeviceLocalMemType);

__impl_cl_decoder_for_from_impl!(cl_build_status, BuildStatus);
__impl_cl_decoder_for_from_impl!(cl_program_binary_type, ProgramBinaryType);
__impl_cl_decoder_for_from_impl!(cl_command_type, CommandType);
__impl_cl_decoder_for_from_impl!(cl_int, CommandExecutionStatus);
__impl_cl_decoder_for_from_impl!(cl_mem_migration_flags, MemMigrationFlags);
__impl_cl_decoder_for_from_impl!(cl_mem_object_type, MemObjectType);
__impl_cl_decoder_for_from_impl!(cl_mem_info, MemInfo);
__impl_cl_decoder_for_from_impl!(cl_buffer_create_type, BufferCreateType);



macro_rules! __impl_cl_decoder_for_bitflag {
    ($cl_type:ty, $rust_type:ident) => {
        impl ClDecoder<$rust_type> for ClReturn {

            unsafe fn cl_decode(self) -> $rust_type {
                let casted_ptr = self.ptr as *mut $cl_type;
                $rust_type::from_bits(*casted_ptr).ok_or_else(|| {
                    panic!("Failed to cl_decode bitflag {:?}", stringify!($rust_type))
                }).unwrap()
            }
        }
    }
}

__impl_cl_decoder_for_bitflag!(cl_mem_flags, MemFlags);
__impl_cl_decoder_for_bitflag!(cl_device_type, DeviceType);
__impl_cl_decoder_for_bitflag!(cl_command_queue_properties, CommandQueueProperties);
__impl_cl_decoder_for_bitflag!(cl_device_fp_config, DeviceFpConfig);
__impl_cl_decoder_for_bitflag!(cl_device_exec_capabilities, DeviceExecCapabilities);
__impl_cl_decoder_for_bitflag!(cl_device_affinity_domain, DeviceAffinityDomain);

pub type ClOutput = Output<ClReturn>;


