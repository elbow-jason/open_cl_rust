use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ll::*;

use crate::{Context, Program, Buffer};

pub struct Kernel {
    program: ManuallyDrop<Program>,
    inner: ManuallyDrop<RwLock<ClKernel>>,
    _unconstructable: (),
}

unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}

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
            _unconstructable: (),
        }
    }
}

impl Kernel {
    pub unsafe fn new(kernel: ClKernel, program: Program) -> Kernel {
        Kernel {
            program: ManuallyDrop::new(program),
            inner: ManuallyDrop::new(RwLock::new(kernel)),
            _unconstructable: (),
        }
    }

    pub fn create(program: &Program, name: &str) -> Output<Kernel> {
        let ll_kernel = unsafe { ClKernel::create(program.low_level_program(), name) }?;
        Ok(unsafe { Kernel::new(ll_kernel, program.clone()) })
    }

    pub unsafe fn set_arg<T>(&self, arg_index: usize, arg: &mut T) -> Output<()>
    where
        T: KernelArg + Debug,
    {
        self.write_lock().set_arg(arg_index, arg)
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
}

pub enum KernelOpArg<T: ClNumber> {
    Num(T),
    Buffer(Buffer<T>),
}

impl<T: ClNumber> From<T> for KernelOpArg<T> {
    fn from(num: T) -> KernelOpArg<T> {
        KernelOpArg::Num(num)
    }
}

impl<T: ClNumber> From<Buffer<T>> for KernelOpArg<T> {
    fn from(buffer: Buffer<T>) -> KernelOpArg<T> {
        KernelOpArg::Buffer(buffer)
    }
}

impl<T: ClNumber> KernelOpArg<T> {
    pub fn into_buffer(self) -> Output<Buffer<T>> {
        if let KernelOpArg::Buffer(buffer) = self {
            Ok(buffer)
        } else {
            Err(KernelError::KernelOpArgWasNotMem.into())
        }
    }

    pub fn into_num(self) -> Output<T> {
        if let KernelOpArg::Num(num) = self {
            Ok(num)
        } else {
            Err(KernelError::KernelOpArgWasNotMem.into())
        }
    }
}


pub struct KernelOperation<T: ClNumber + KernelArg> {
    _name: String,
    _args: Vec<KernelOpArg<T>>,
    _work: Option<Work>,
    _returning: Option<usize>,
    pub command_queue_opts: Option<CommandQueueOptions>,
}

impl<T: ClNumber + KernelArg> KernelOperation<T> {
    pub fn new(name: &str) -> KernelOperation<T> {
        KernelOperation {
            _name: name.to_owned(),
            _args: vec![],
            _work: None,
            _returning: None,
            command_queue_opts: None,
        }
    }

    pub fn name(&self) -> &str {
        &self._name[..]
    }

    pub fn command_queue_opts(&self) -> Option<CommandQueueOptions> {
        self.command_queue_opts.clone()
    }

    pub fn args(&self) -> &[KernelOpArg<T>] {
        &self._args[..]
    }

    pub fn mut_args(&mut self) -> &mut [KernelOpArg<T>] {
        &mut self._args[..]
    }

    pub fn with_dims<D: Into<Dims>>(mut self, dims: D) -> KernelOperation<T> {
        self._work = Some(Work::new(dims.into()));
        self
    }

    pub fn with_work<W: Into<Work>>(mut self, work: W) -> KernelOperation<T> {
        self._work = Some(work.into());
        self
    }

    pub fn add_arg<A: Into<KernelOpArg<T>>>(mut self, arg: A) -> KernelOperation<T> {
        self._args.push(arg.into());
        self
    }

    pub fn with_command_queue_options(mut self, opts: CommandQueueOptions) -> KernelOperation<T> {
        self.command_queue_opts = Some(opts);
        self
    }

    pub fn returning_arg(mut self, arg_index: usize) -> KernelOperation<T> {
        self._returning = Some(arg_index);
        self
    }

    pub fn argc(&self) -> usize {
        self._args.len()
    }

    #[inline]
    pub fn return_value(&mut self) -> Output<Option<KernelOpArg<T>>> {
        match (self._returning, self.argc()) {
            (Some(argi), argc) if argi < argc => Ok(Some(self._args.remove(argi))),
            (Some(argi), argc) => {
                let oor_error = KernelError::ReturningArgIndexOutOfRange(argi, argc);
                Err(oor_error.into())
            }
            (None, _) => Ok(None),
        }
    }

    #[inline]
    pub fn work(&self) -> Output<Work> {
        self._work.clone().ok_or(KernelError::WorkIsRequired.into())
    }
}
