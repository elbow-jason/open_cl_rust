// use std::marker::PhantomData;
// use std::mem::ManuallyDrop;
// use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

// use libc::c_void;

// use crate::ll::Kernel as ClKernel;

use crate::{Buffer, CommandQueueOptions, Dims, Output, Work};

use crate::ll::{KernelArg as ClKernelArg, KernelArgPtr, KernelError, Number};

#[derive(Debug)]
pub enum KernelArg<'a> {
    Num(ClKernelArg<'a>),
    Buffer(&'a Buffer),
}

#[derive(Debug)]
pub struct KernelOperation<'a> {
    _name: String,
    _args: Vec<KernelArg<'a>>,
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

    pub fn args(&self) -> &[KernelArg<'a>] {
        &self._args[..]
    }

    pub fn mut_args(&mut self) -> &mut [KernelArg<'a>] {
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

    pub fn add_arg<A: Into<KernelArg<'a>>>(mut self, arg: A) -> KernelOperation<'a> {
        self._args.push(arg.into());
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

impl<'a> From<&'a Buffer> for KernelArg<'a> {
    fn from(buf: &'a Buffer) -> KernelArg<'a> {
        KernelArg::Buffer(buf)
    }
}

impl<'a, T> From<&'a T> for KernelArg<'a>
where
    T: KernelArgPtr + Number,
{
    fn from(num: &'a T) -> KernelArg<'a> {
        KernelArg::Num(ClKernelArg::new(num))
    }
}

// pub struct Kernel {
//     program: ManuallyDrop<Program>,
//     inner: ManuallyDrop<Arc<RwLock<ClKernel>>>,
//     _unconstructable: (),
// }

// unsafe impl Send for Kernel {}
// unsafe impl Sync for Kernel {}

// impl Drop for Kernel {
//     fn drop(&mut self) {
//         unsafe {
//             ManuallyDrop::drop(&mut self.inner);
//             ManuallyDrop::drop(&mut self.program);
//         }
//     }
// }

// impl Clone for Kernel {
//     fn clone(&self) -> Kernel {
//         let inner_clone = self.read_lock().clone();
//         Kernel {
//             inner: ManuallyDrop::new(Arc::new(RwLock::new(inner_clone))),
//             program: ManuallyDrop::new(self.program().clone()),
//             _unconstructable: (),
//         }
//     }
// }

// impl Kernel {
//     pub unsafe fn new(kernel: ClKernel, program: Program) -> Kernel {
//         Kernel {
//             program: ManuallyDrop::new(program),
//             inner: ManuallyDrop::new(Arc::new(RwLock::new(kernel))),
//             _unconstructable: (),
//         }
//     }

//     pub fn create(program: &Program, name: &str) -> Output<Kernel> {
//         let ll_kernel = unsafe { ClKernel::create(program.low_level_program(), name) }?;
//         Ok(unsafe { Kernel::new(ll_kernel, program.clone()) })
//     }

//     pub unsafe fn set_arg<T>(&self, arg_index: usize, arg: &mut T) -> Output<()>
//     where
//         T: KernelArg,
//     {
//         self.write_lock().set_arg(arg_index, arg)
//     }

//     pub fn function_name(&self) -> Output<String> {
//         unsafe { self.read_lock().function_name() }
//     }

//     pub fn num_args(&self) -> Output<u32> {
//         unsafe { self.read_lock().num_args() }
//     }

//     pub fn reference_count(&self) -> Output<u32> {
//         unsafe { self.read_lock().reference_count() }
//     }

//     pub fn context(&self) -> &Context {
//         self.program().context()
//     }

//     pub fn program(&self) -> &Program {
//         &*self.program
//     }

//     pub fn attributes(&self) -> Output<String> {
//         unsafe { self.read_lock().attributes() }
//     }

//     pub fn read_lock(&self) -> RwLockReadGuard<ClKernel> {
//         self.inner.read().unwrap()
//     }

//     pub fn write_lock(&self) -> RwLockWriteGuard<ClKernel> {
//         self.inner.write().unwrap()
//     }
// }

// // pub trait ToKernelOpArg<'a> {
// //     fn to_kernel_op_arg(&self) -> KernelOpArg<'a>;
// // }

// // impl<'a, T> ToKernelOpArg<'a> for T
// // where
// //     T: Number + AsPtr<T> + Sized,
// // {
// //     fn to_kernel_op_arg(&self) -> KernelOpArg<'a> {
// //         KernelOpArg::Num(NumArg::new(*self))
// //     }
// // }

// // impl<'a> ToKernelOpArg<'a> for &'a Buffer {
// //     fn to_kernel_op_arg(&self) -> KernelOpArg<'a> {
// //         KernelOpArg::Buffer(self)
// //     }
// // }

// // impl<'a> KernelOpArg<'a> {
// //     pub fn into_buffer(self) -> Output<&'a Buffer> {
// //         if let KernelOpArg::Buffer(buffer) = self {
// //             Ok(buffer)
// //         } else {
// //             let err = Into::into(KernelError::KernelOpArgWasNotMem);
// //             Err(err)
// //         }
// //     }

// //     pub fn into_num<T: NumberTypedT + Copy>(self) -> Output<T> {
// //         if let KernelOpArg::Num(num) = self {
// //             num.into_number::<T>()
// //         } else {
// //             let err = Into::into(KernelError::KernelOpArgWasNotMem);
// //             Err(err)
// //         }
// //     }
// // }

// pub struct KernelOperation<'a> {
//     _name: String,
//     _args: Vec<ArgPtr<'a>>,
//     _work: Option<Work>,
//     _returning: Option<usize>,
//     pub command_queue_opts: Option<CommandQueueOptions>,
// }

// impl<'a> KernelOperation<'a> {
//     pub fn new(name: &str) -> KernelOperation<'a> {
//         KernelOperation {
//             _name: name.to_owned(),
//             _args: vec![],
//             _work: None,
//             _returning: None,
//             command_queue_opts: None,
//         }
//     }

//     pub fn name(&self) -> &str {
//         &self._name[..]
//     }

//     pub fn command_queue_opts(&self) -> Option<CommandQueueOptions> {
//         self.command_queue_opts.clone()
//     }

//     pub fn args(&self) -> &[ArgPtr<'a>] {
//         &self._args[..]
//     }

//     pub fn mut_args(&mut self) -> &mut [ArgPtr<'a>] {
//         &mut self._args[..]
//     }

//     pub fn with_dims<D: Into<Dims>>(mut self, dims: D) -> KernelOperation<'a> {
//         self._work = Some(Work::new(dims.into()));
//         self
//     }

//     pub fn with_work<W: Into<Work>>(mut self, work: W) -> KernelOperation<'a> {
//         self._work = Some(work.into());
//         self
//     }

//     pub fn add_arg<A: KernelArg>(mut self, arg: &A) -> KernelOperation<'a> {
//         self._args.push(arg);
//         self
//     }

//     pub fn with_command_queue_options(mut self, opts: CommandQueueOptions) -> KernelOperation<'a> {
//         self.command_queue_opts = Some(opts);
//         self
//     }

//     pub fn with_returning_arg(mut self, arg_index: usize) -> KernelOperation<'a> {
//         self._returning = Some(arg_index);
//         self
//     }

//     pub fn argc(&self) -> usize {
//         self._args.len()
//     }

//     #[inline]
//     pub fn work(&self) -> Output<Work> {
//         let err = Into::into(KernelError::WorkIsRequired);
//         self._work.clone().ok_or(err)
//     }
// }
