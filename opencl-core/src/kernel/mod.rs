use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard};

pub mod flags;
pub mod kernel_arg;
pub mod low_level;

pub use kernel_arg::{KernelArg, KernelArgSizeAndPointer};

use crate::ffi::cl_kernel;

use crate::error::Output;
use crate::program::Program;
use crate::utils;

/// An error related to a `Kernel`.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum KernelError {
    #[fail(
        display = "The kernel name '{}' could not be represented as a CString.",
        _0
    )]
    CStringInvalidKernelName(String),
}


unsafe fn release_kernel(kernel: cl_kernel) {
    low_level::cl_release_kernel(kernel).unwrap_or_else(|e| {
        panic!("Failed to release cl_kernel {:?} due to {:?} ", kernel, e);
    });
}

unsafe fn retain_kernel(kernel: cl_kernel) {
    low_level::cl_retain_kernel(kernel).unwrap_or_else(|e| {
        panic!("Failed to retain cl_kernel {:?} due to {:?}", kernel, e);
    });
}

pub trait KernelPtr: Sized {
    unsafe fn kernel_ptr(&self) -> cl_kernel;
}

pub trait KernelLock<K> where Self: Sized, K: KernelPtr {
    unsafe fn rw_lock(&self) -> &RwLock<K>;
    unsafe fn write_lock(&self) -> RwLockWriteGuard<K>;
    unsafe fn read_lock(&self) -> RwLockReadGuard<K>;
}

pub struct KernelWrapper {
    inner: cl_kernel,
    _unconstructable: ()
}

impl KernelWrapper {
    pub fn unchecked_new(inner: cl_kernel) -> KernelWrapper {
        KernelWrapper { inner, _unconstructable: () }
    }
}

impl KernelPtr for KernelWrapper {
    unsafe fn kernel_ptr(&self) -> cl_kernel {
        self.inner
    }
}

impl Drop for KernelWrapper {
    fn drop(&mut self) {
        unsafe {
            // If memory issues / race conditions persist change this to a
            // write_lock or use a Mutex.
            release_kernel(self.inner);
        }
    }
}

impl Clone for KernelWrapper {
    fn clone(&self) -> KernelWrapper {
        unsafe {
            let kernel = self.kernel_ptr();
            // If memory issues / race conditions persist change this to a
            // write_lock or use a Mutex.
            retain_kernel(kernel);
            KernelWrapper::unchecked_new(kernel)
        }
        
    }
}

pub trait KernelRefCount: Sized {
    unsafe fn from_retained(cq: cl_kernel) -> Output<Self>;
    unsafe fn from_unretained(cq: cl_kernel) -> Output<Self>;
}


impl KernelRefCount for KernelWrapper {
    unsafe fn from_retained(kernel: cl_kernel) -> Output<KernelWrapper> {
        utils::null_check(kernel, "KernelWrapper::from_retained")?;
        Ok(KernelWrapper::unchecked_new(kernel))
    }

    unsafe fn from_unretained(kernel: cl_kernel) -> Output<KernelWrapper> {
        utils::null_check(kernel, "KernelWrapper::from_unretained")?;
        retain_kernel(kernel);
        Ok(KernelWrapper::unchecked_new(kernel))
    }
}


pub struct KernelObject {
    object: Arc<RwLock<KernelWrapper>>,
    _unconstructable: ()
}

impl KernelObject {
    pub unsafe fn from_kernel_wrapper(kernel_wrapper: KernelWrapper) -> KernelObject {
        KernelObject {
            object: Arc::new(RwLock::new(kernel_wrapper)),
            _unconstructable: ()
        }
    }
}

impl Clone for KernelObject {
    fn clone(&self) -> KernelObject {
        KernelObject {
            object: self.object.clone(),
            _unconstructable: ()
        }
    }
}

impl KernelRefCount for KernelObject {
    unsafe fn from_retained(kernel: cl_kernel) -> Output<KernelObject> {
        let wrapper = KernelWrapper::from_retained(kernel)?;
        Ok(KernelObject::from_kernel_wrapper(wrapper))
    }

    unsafe fn from_unretained(kernel: cl_kernel) -> Output<KernelObject> {
        let wrapper = KernelWrapper::from_unretained(kernel)?;
        Ok(KernelObject::from_kernel_wrapper(wrapper))
    }
}

impl KernelLock<KernelWrapper> for KernelObject {
    unsafe fn rw_lock(&self) -> &RwLock<KernelWrapper> {
        &*self.object
    }

    unsafe fn write_lock(&self) -> RwLockWriteGuard<KernelWrapper> {
        self.object.write().unwrap()
    }

    unsafe fn read_lock(&self) -> RwLockReadGuard<KernelWrapper> {
        self.object.read().unwrap()
    }
}

pub trait KernelRefCountWithProgram: Sized {
    unsafe fn from_retained(cq: cl_kernel, prog: Program) -> Output<Self>;
    unsafe fn from_unretained(cq: cl_kernel, prog: Program) -> Output<Self>;
}

pub struct Kernel {
    program: ManuallyDrop<Program>,
    inner: ManuallyDrop<KernelObject>,
    _unconstructable: ()
}

impl KernelRefCountWithProgram for Kernel {
    unsafe fn from_retained(kernel: cl_kernel, program: Program) -> Output<Kernel> {
        let inner = KernelObject::from_retained(kernel)?;
        Ok(Kernel {
            program: ManuallyDrop::new(program),
            inner: ManuallyDrop::new(inner),
            _unconstructable: ()
        })
    }

    unsafe fn from_unretained(kernel: cl_kernel, program: Program) -> Output<Kernel> {
        let inner = KernelObject::from_unretained(kernel)?;
        Ok(Kernel {
            program: ManuallyDrop::new(program),
            inner: ManuallyDrop::new(inner),
            _unconstructable: ()
        })
    }
}

impl KernelLock<KernelWrapper> for Kernel {
    unsafe fn rw_lock(&self) -> &RwLock<KernelWrapper> {
        self.inner.rw_lock()
    }

    unsafe fn write_lock(&self) -> RwLockWriteGuard<KernelWrapper> {
        self.inner.write_lock()
    }

    unsafe fn read_lock(&self) -> RwLockReadGuard<KernelWrapper> {
        self.inner.read_lock()
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.inner);
            ManuallyDrop::drop(&mut self.program);
        }
    }
}

impl Clone for Kernel {
    fn clone(&self) -> Kernel {
        Kernel {
            inner: ManuallyDrop::new((*self.inner).clone()),
            program: ManuallyDrop::new((*self.program).clone()),
            _unconstructable: ()
        }
    }
}

impl Kernel {
    pub fn create(program: &Program, name: &str) -> Output<Kernel> {
        low_level::cl_create_kernel(program, name)
    }

    pub fn set_arg<T>(&self, arg_index: usize, arg: &T) -> Output<()>
    where
        T: KernelArg + Debug,
    {
        low_level::cl_set_kernel_arg(self, arg_index, arg)
    }
}

unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}
