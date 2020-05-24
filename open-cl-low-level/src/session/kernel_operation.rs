use crate::{CommandQueueOptions, Dims, KernelArg, KernelError, Output, Work};
use libc::c_void;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ArgPtr<'a> {
    _phantom: PhantomData<&'a c_void>,
    _ptr: *mut c_void,
    _size: usize,
}

impl<'a> ArgPtr<'a> {
    pub fn new<T: KernelArg>(val: &'a T) -> ArgPtr<'a> {
        unsafe {
            ArgPtr::from_raw_parts(val.kernel_arg_ptr() as *mut c_void, val.kernel_arg_size())
        }
    }

    pub unsafe fn from_raw_parts(ptr: *mut c_void, size: usize) -> ArgPtr<'a> {
        ArgPtr {
            _phantom: PhantomData,
            _ptr: ptr,
            _size: size,
        }
    }
}

unsafe impl<'a> KernelArg for ArgPtr<'a> {
    fn kernel_arg_size(&self) -> usize {
        self._size
    }
    unsafe fn kernel_arg_ptr(&self) -> *const c_void {
        self._ptr as *const c_void
    }
    unsafe fn kernel_arg_mut_ptr(&mut self) -> *mut c_void {
        self._ptr
    }
}

#[derive(Debug)]
pub struct KernelOperation<'a> {
    _name: String,
    _args: Vec<ArgPtr<'a>>,
    _work: Option<Work>,
    pub command_queue_opts: Option<CommandQueueOptions>,
}

impl<'a> KernelOperation<'a> {
    pub fn new(name: &str) -> KernelOperation<'a> {
        KernelOperation {
            _name: name.to_owned(),
            _args: vec![],
            _work: None,
            command_queue_opts: None,
        }
    }

    pub fn name(&self) -> &str {
        &self._name[..]
    }

    pub fn command_queue_opts(&self) -> Option<CommandQueueOptions> {
        self.command_queue_opts.clone()
    }

    pub fn args(&self) -> &[ArgPtr<'a>] {
        &self._args[..]
    }

    pub fn mut_args(&mut self) -> &mut [ArgPtr<'a>] {
        &mut self._args[..]
    }

    pub fn with_dims<D: Into<Dims>>(mut self, dims: D) -> KernelOperation<'a> {
        self._work = Some(Work::new(dims.into()));
        self
    }

    pub fn with_work<W: Into<Work>>(mut self, work: W) -> KernelOperation<'a> {
        self._work = Some(work.into());
        self
    }

    pub fn add_arg<A: KernelArg>(mut self, arg: &'a A) -> KernelOperation<'a> {
        self._args.push(ArgPtr::new(arg));
        self
    }

    pub fn with_command_queue_options(mut self, opts: CommandQueueOptions) -> KernelOperation<'a> {
        self.command_queue_opts = Some(opts);
        self
    }

    pub fn argc(&self) -> usize {
        self._args.len()
    }

    #[inline]
    pub fn work(&self) -> Output<Work> {
        self._work
            .clone()
            .ok_or_else(|| KernelError::WorkIsRequired.into())
    }
}
