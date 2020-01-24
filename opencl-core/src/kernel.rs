use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ll::*;

use crate::Program;
use crate::Context;

pub struct Kernel {
    program: ManuallyDrop<Program>,
    inner: ManuallyDrop<RwLock<ClKernel>>,
    _unconstructable: ()
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
        let inner_clone = self.read_lock().clone();
        Kernel {
            inner: ManuallyDrop::new(RwLock::new(inner_clone)),
            program: ManuallyDrop::new(self.program().clone()),
            _unconstructable: ()
        }
    }
}

impl Kernel {
    pub unsafe fn new(kernel: ClKernel, program: Program) -> Kernel {
        Kernel {
            program: ManuallyDrop::new(program),
            inner: ManuallyDrop::new(RwLock::new(kernel)),
            _unconstructable: ()
        }
    }

    pub fn create(program: &Program, name: &str) -> Output<Kernel> {
        let ll_kernel = unsafe { ClKernel::create(program.low_level_program(), name) }?;
        Ok(unsafe { Kernel::new(ll_kernel, program.clone()) })
    }

    pub fn set_arg<T>(&self, arg_index: usize, arg: &mut T) -> Output<()>
    where
        T: KernelArg + Debug,
    {
        unsafe{ self.write_lock().set_arg(arg_index, arg) }
    }

    pub fn function_name(&self) -> Output<String> {
        unsafe { self.read_lock().function_name() }
    }
    
    pub fn num_args(&self) -> Output<u32> {
        unsafe { self.read_lock().num_args() }
    }
    
    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.read_lock().reference_count() }
    }
    
    pub fn context(&self) -> &Context {
        self.program().context()
    }

    pub fn program(&self) -> &Program {
        &*self.program
    }
    
    pub fn attributes(&self) -> Output<String> {
        unsafe { self.read_lock().attributes() }
    }

    pub fn read_lock(&self) -> RwLockReadGuard<ClKernel> {
        self.inner.read().unwrap()
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<ClKernel> {
        self.inner.write().unwrap()
    }

    // pub fn low_level_kernel(&self) -> &ClKernel {
    //     &self.inner
    // }
}
