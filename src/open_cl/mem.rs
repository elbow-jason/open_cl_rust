
use std::fmt;
use super::ffi::cl_event;
use std::default::Default;

pub trait Offset {
    fn offset(&self) -> usize {
        0
    }
}

pub trait Count {
    fn count(&self) -> usize;
}

pub trait AsPointer<T> {
    fn as_pointer(&self) -> *const T;
}

pub trait AsMutPointer<T> {
    fn as_mut_pointer(&mut self) -> *mut T;
}

pub trait MemSize<T> {
    fn mem_size(&self) -> usize;
}

pub struct HostBuffer<T> {
    data: Vec<T>,
    _offset: usize,
}

impl<T> HostBuffer<T> {
    pub fn new(data: Vec<T>) -> HostBuffer<T> {
        HostBuffer {
            data,
            _offset: 0
        }
    }

    pub fn with_offset(mut self, offset: usize) -> HostBuffer<T> {
        self._offset = offset;
        self
    }

    pub fn into_data(self) -> Vec<T> {
        self.data
    }
}


impl<T> AsPointer<T> for HostBuffer<T> {
    fn as_pointer(&self) -> *const T {
        self.data.as_ptr()
    }
}

impl<T> AsMutPointer<T> for HostBuffer<T> {
    fn as_mut_pointer(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

impl<T> Offset for HostBuffer<T> {
    fn offset(&self) -> usize {
        self._offset
    }
}
impl<T> MemSize<T> for HostBuffer<T> {
    fn mem_size(&self) -> usize {
        self.data.len() * std::mem::size_of::<T>()
    }
}

impl<T> AsPointer<T> for &HostBuffer<T> {
    fn as_pointer(&self) -> *const T {
        self.data.as_ptr()
    }
}

impl<T> Offset for &HostBuffer<T> {
    fn offset(&self) -> usize {
        self._offset
    }
}
impl<T> MemSize<T> for &HostBuffer<T> {
    fn mem_size(&self) -> usize {
        self.data.len() * std::mem::size_of::<T>()
    }
}


impl<T> AsPointer<T> for &mut HostBuffer<T> {
    fn as_pointer(&self) -> *const T {
        self.data.as_ptr()
    }
}

impl<T> AsMutPointer<T> for &mut HostBuffer<T> {
    fn as_mut_pointer(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

impl<T> Offset for &mut HostBuffer<T> {
    fn offset(&self) -> usize {
        self._offset
    }
}
impl<T> MemSize<T> for &mut HostBuffer<T> {
    fn mem_size(&self) -> usize {
        self.data.len() * std::mem::size_of::<T>()
    }
}


impl<T> fmt::Debug for HostBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "HostBuffer<T>(offset: {:?}, mem_size: {:?}, pointer: {:?})",
            self.offset(),
            self.mem_size(),
            self.as_pointer(),
        )
    }

}

// host_buffer!(ReadOnlyHostBuffer);

macro_rules! def_mem_type {
    ($ptr_t:ty) => {

        impl AsPointer<$ptr_t> for $ptr_t {
            fn as_pointer(&self) -> *const $ptr_t {
                self as *const $ptr_t
            }
        }

        
        impl AsMutPointer<$ptr_t> for $ptr_t {
            fn as_mut_pointer(&mut self) -> *mut $ptr_t {
                self as *mut $ptr_t
            }
        }
        
        impl MemSize<$ptr_t> for $ptr_t {
            fn mem_size(&self) -> usize {
                std::mem::size_of::<$ptr_t>()
            }
        }
    }
}

def_mem_type!(isize);
def_mem_type!(usize);
def_mem_type!(u32);
def_mem_type!(u64);
def_mem_type!(i32);
def_mem_type!(i64);
def_mem_type!(f32);
def_mem_type!(f64);

pub struct OpConfig {
    pub is_blocking: Option<bool>,
    pub writing_event: Option<cl_event>,
}

impl Default for OpConfig {
    fn default() -> OpConfig {
        OpConfig {
            is_blocking: None,
            writing_event: None,
        }
    }
}
