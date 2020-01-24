use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc};
use std::mem::ManuallyDrop;
use std::fmt;

use crate::{Context, Device, Kernel, Buffer};
// use crate::traits::Upcast;

use crate::ll::*;

pub trait CommandQueueLock<P> where P: CommandQueuePtr {
    unsafe fn write_lock(&self) -> RwLockWriteGuard<P>;
    unsafe fn read_lock(&self) -> RwLockReadGuard<P>;
    unsafe fn rw_lock(&self) -> &RwLock<P>;

    fn address(&self) -> String {
        unsafe {
            let read_lock = self.read_lock();
            let ptr = read_lock.command_queue_ptr();
            format!("{:?}", ptr)
        }
    }
}

pub struct CommandQueue {
    _queue: ManuallyDrop<Arc<RwLock<ClCommandQueue>>>,
    _context: ManuallyDrop<ClContext>,
    _device: ManuallyDrop<ClDeviceID>,
    _unconstructable: (),
}


impl CommandQueueLock<ClCommandQueue> for CommandQueue {
    unsafe fn read_lock(&self) -> RwLockReadGuard<ClCommandQueue> {
        (*self._queue).read().unwrap()
    }
    unsafe fn write_lock(&self) -> RwLockWriteGuard<ClCommandQueue> {
        (*self._queue).write().unwrap()
    }
    unsafe fn rw_lock(&self) -> &RwLock<ClCommandQueue> {
        &(*self._queue)
    }
}

impl fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandQueue{{{:?}}}", self.address())
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            debug!("cl_command_queue {:?} - CommandQueue::drop", self.address());
            ManuallyDrop::drop(&mut self._queue);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._device);
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> CommandQueue {
        CommandQueue {
            _queue: ManuallyDrop::new((*self._queue).clone()),
            _context: self._context.clone(),
            _device: self._device.clone(),
            _unconstructable: ()
        }
    }
}

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

impl CommandQueue {
    unsafe fn new(queue: ClCommandQueue, context: &Context, device: &Device) -> CommandQueue {
        CommandQueue::new_from_low_level(
            queue,
            context.low_level_context(),
            device.low_level_device(),
        )
    }

    unsafe fn new_from_low_level(queue: ClCommandQueue, context: &ClContext, device: &ClDeviceID) -> CommandQueue {
        CommandQueue{
            _queue: ManuallyDrop::new(Arc::new(RwLock::new(queue))),
            _context: ManuallyDrop::new(context.clone()),
            _device: ManuallyDrop::new(device.clone()),
            _unconstructable: ()
        }
    }

    pub fn create(
        context: &Context,
        device: &Device,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<CommandQueue> {
        unsafe {
            let ll_queue = ClCommandQueue::create(
            context.low_level_context(),
            device.low_level_device(),
            opt_props,
            )?;
            Ok(CommandQueue::new(ll_queue, context, device))
        }
    }

    pub fn create_copy(&self) -> Output<CommandQueue> {
        unsafe { 
            let props = self.properties()?;
            let ll_queue = ClCommandQueue::create_from_raw_pointers(
                (*self._context).context_ptr(),
                (*self._device).device_ptr(),
                props.into()            
            )?;
            Ok(CommandQueue::new_from_low_level(ll_queue, &self._context, &self._device))
        }
    }

    pub fn low_level_context(&self) -> &ClContext {
        &*self._context
    }

    pub fn low_level_device(&self) -> &ClDeviceID {
        &*self._device
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &Buffer<T>`.
    pub fn write_buffer<T>(&self, device_buffer: &Buffer<T>, host_buffer: &[T], opts: Option<CommandQueueOptions>) -> Output<()>
    where
        T: ClNumber,
    {
        unsafe {
            let mut qlock = self.write_lock();
            let mut buf_lock = device_buffer.write_lock();
            let event = qlock.write_buffer(&mut *buf_lock, host_buffer, opts.into())?;
            event.wait()
        }
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T: ClNumber>(&self, device_buffer: &Buffer<T>, host_buffer: &mut [T], opts: Option<CommandQueueOptions>) -> Output<Option<Vec<T>>> {
        unsafe {
            let mut qlock = self.write_lock();
            let buf_lock = device_buffer.read_lock();
            let mut event = qlock.read_buffer(&*buf_lock, host_buffer, opts)?;
            event.wait()
        }
    }

    pub fn enqueue_kernel(
        &self,
        kernel: Kernel,
        work: &Work,
        opts: Option<CommandQueueOptions>,
    ) -> Output<()> {
        unsafe {
            let kernel_lock = kernel.write_lock();
            let mut qlock = self.write_lock();
            let event = qlock.enqueue_kernel(
                &*kernel_lock,
                work,
                opts
            )?;
            event.wait()
        }
        
    }

    pub fn finish(&self) -> Output<()> {
        unsafe {
            let mut lock = self.write_lock();
            lock.finish()
        }
    }

    // pub fn sync_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
    //     self.sync_enqueue_kernel_with_opts(kernel, work, CommandQueueOptions::default())
    // }

    // pub fn async_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
    //     self.async_enqueue_kernel_with_opts(kernel, work, CommandQueueOptions::default())
    // }

    // pub fn async_enqueue_kernel_with_opts(
    //     &self,
    //     kernel: &Kernel,
    //     work: &Work,
    //     command_queue_opts: CommandQueueOptions,
    // ) -> Output<Event> {
    //     unsafe {
    //         let kernel_lock = kernel.write_lock();
    //         let cq_lock = self.write_lock();
    //         async_enqueue_kernel_with_opts(
    //             cq_lock.command_queue_ptr(),
    //             kernel_lock.kernel_ptr(),
    //             work,
    //             command_queue_opts
    //         )
    //     }
    // }

    // pub fn info<T: Copy>(self, flag: CQInfo) -> Output<ClPointer<T>> {
    //     unsafe { 
    //         let cq_lock = self.read_lock().info();
    //         info::fetch::<T>(cq_lock.command_queue_ptr(), flag)
    //     }
    // }

    // pub fn load_context(&self) -> Output<Context> {
    //     unsafe { info::load_context(self.read_lock().command_queue_ptr()) }
    // }

    // pub fn load_device(&self) -> Output<Device> {
    //     unsafe { info::load_device(self.read_lock().command_queue_ptr()) }
    // }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { self.read_lock().reference_count() }
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        unsafe { self.read_lock().properties() }
    }
}

impl PartialEq for CommandQueue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(self.read_lock().command_queue_ptr(), other.read_lock().command_queue_ptr()) }
    }
}

impl Eq for CommandQueue {}

// #[cfg(test)]
// mod tests {
//     use super::flags::CommandQueueProperties;
//     use crate::{Context, Output, Device, testing};
//     const SRC: &'static str = "
//     __kernel void test(__global int *i) {
//         *i += 1;
//     }";
    

//     #[test]
//     pub fn command_queue_method_context_works() {
//         testing::init_logger();
//         let session = testing::get_session(SRC);
//         let _context: &Context = session.command_queue().context();
//     }

//     #[test]
//     pub fn command_queue_method_load_context_works() {
//         let session = testing::get_session(SRC);
//         let result: Output<Context> = session.command_queue().load_context();
//         result.unwrap_or_else(|e| panic!("Failed to load_context: {:?}", e));
//     }

//     #[test]
//     pub fn command_queue_load_context_matches_kept_context() {
//         let session = testing::get_session(SRC);
//         let kept_context: &Context = session
//             .command_queue()
//             .context();
//         let loaded_context: Context = session.command_queue().load_context().unwrap();
//         assert_eq!(kept_context, &loaded_context); 
//     }

//     #[test]
//     pub fn command_queue_method_device_works() {
//         let session = testing::get_session(SRC);
//         let _device: &Device = session.command_queue().device();
//     }

//     #[test]
//     pub fn command_queue_method_load_device_works() {
//         let session = testing::get_session(SRC);
//         let result: Output<Device> = session.command_queue().load_device();
//         result.unwrap_or_else(|e| panic!("Failed to load_device: {:?}", e));
//     }

//     #[test]
//     pub fn command_queue_load_device_matches_kept_device() {
//         let session = testing::get_session(SRC);
//         let kept_device = session
//             .command_queue()
//             .device();
//         let loaded_device = session.command_queue().load_device().unwrap();
//         assert_eq!(kept_device, &loaded_device); 
//     }

//     #[test]
//     pub fn command_queue_method_reference_count_works() {
//         let session = testing::get_session(SRC);
//         let ref_count: u32 = session
//             .command_queue()
//             .reference_count()
//             .expect("CommandQueue method reference_count() failed");
//         assert_eq!(ref_count, 1);
//     }

//     #[test]
//     pub fn command_queue_method_properties_works() {
//         let session = testing::get_session(SRC);
//         let props: CommandQueueProperties = session
//             .command_queue()
//             .properties()
//             .expect("CommandQueue method properties() failed");
//         let bits = props.bits();
//         let maybe_same_prop = CommandQueueProperties::from_bits(bits);
//         if !maybe_same_prop.is_some() {
//             panic!(
//                 "
//                 CommandQueue method properties returned \
//                 an invalid CommandQueueProperties bitflag {:?}\
//                 ",
//                 bits
//             );
//         }
//     }

//     #[test]
//     pub fn command_queue_copy_new_works() {
//         let session = testing::get_session(SRC);
//         let cq2 = session.command_queue().new_copy().unwrap();
//         assert!(&cq2 != session.command_queue());
//     }
// }
