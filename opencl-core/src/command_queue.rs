use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc};
use std::mem::ManuallyDrop;
use std::fmt;
use std::fmt::Debug;

use crate::ffi::{
    cl_command_queue,
    cl_command_queue_properties,
    cl_context,
    cl_device_id,
    cl_kernel,
};

use crate::{Context, Device, Kernel};
use crate::ll::*;

unsafe fn async_enqueue_kernel_with_opts(
        cq: cl_command_queue,
        kernel: cl_kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
) -> Output<Event> {
    low_level::cl_enqueue_nd_range_kernel(
        cq,
        kernel,
        work.work_dim(),
        work.global_work_offset(),
        work.global_work_size(),
        work.local_work_size(),
        command_queue_opts.wait_list,
    )
}

// pub trait CommandQueuePtr: Sized {
//     unsafe fn command_queue_ptr(&self) -> cl_command_queue;
//     fn address(&self) -> String {
//         format!("{:?}", unsafe { self.command_queue_ptr() })
//     }
// }

// pub trait CommandQueueRefCount: Sized {
//     unsafe fn from_retained(cq: cl_command_queue) -> Output<Self>;
//     unsafe fn from_unretained(cq: cl_command_queue) -> Output<Self>;
// }

// pub struct CommandQueueWrapper {
//     inner: cl_command_queue,
//     _unconstructable: ()
// }

// impl CommandQueueWrapper {
//     pub unsafe fn unchecked_new(cq: cl_command_queue) -> CommandQueueWrapper {
//         CommandQueueWrapper{
//             inner: cq,
//             _unconstructable: ()
//         }
//     }
// }

// impl CommandQueueRefCount for CommandQueueWrapper {
//     unsafe fn from_retained(cq: cl_command_queue) -> Output<CommandQueueWrapper> {
//         utils::null_check(cq)?;
//         Ok(CommandQueueWrapper::unchecked_new(cq))
//     }

//     unsafe fn from_unretained(cq: cl_command_queue) -> Output<CommandQueueWrapper> {
//         utils::null_check(cq)?;
//         retain_command_queue(cq);
//         Ok(CommandQueueWrapper::unchecked_new(cq))
//     }
// }

// impl CommandQueuePtr for CommandQueueWrapper {
//     unsafe fn command_queue_ptr(&self) -> cl_command_queue {
//         self.inner
//     }
// }


impl Drop for CommandQueueWrapper {
    fn drop(&mut self) {
        debug!("cl_command_queue {:?} - CommandQueueObject::drop", self.inner);
        unsafe {
            release_command_queue(self.inner);
        }
    }
}

impl Clone for CommandQueueWrapper {
    fn clone(&self) -> CommandQueueWrapper {
        unsafe {
            retain_command_queue(self.inner);
            CommandQueueWrapper::unchecked_new(self.inner)
        }
    }
}

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

pub struct CommandQueueObject {
    object: Arc<RwLock<CommandQueueWrapper>>,
    _unconstructable: (),
}

impl CommandQueueObject {
    unsafe fn unchecked_new(wrapper: CommandQueueWrapper) -> CommandQueueObject {
        CommandQueueObject {
            object: Arc::new(RwLock::new(wrapper)),
            _unconstructable: (),
        }
    }
}

impl CommandQueueRefCount for CommandQueueObject {
    unsafe fn from_retained(cq: cl_command_queue) -> Output<CommandQueueObject> {
        let cqw = CommandQueueWrapper::from_retained(cq)?;
        Ok(CommandQueueObject::unchecked_new(cqw))
    }

    unsafe fn from_unretained(cq: cl_command_queue) -> Output<CommandQueueObject> {
        let cqw = CommandQueueWrapper::from_unretained(cq)?;
        Ok(CommandQueueObject::unchecked_new(cqw))
    }
}

impl CommandQueueLock<CommandQueueWrapper> for CommandQueueObject {
    unsafe fn read_lock(&self) -> RwLockReadGuard<CommandQueueWrapper> {
        self.object.read().unwrap()
    }

    unsafe fn write_lock(&self) -> RwLockWriteGuard<CommandQueueWrapper> {
        self.object.write().unwrap()
    }
    unsafe fn rw_lock(&self) -> &RwLock<CommandQueueWrapper> {
        &*self.object
    }
}

impl fmt::Debug for CommandQueueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandQueueObject{{{:?}}}", self.address())
    }
}

impl Clone for CommandQueueObject {
    fn clone(&self) -> CommandQueueObject {
        CommandQueueObject{
            object: self.object.clone(),
            _unconstructable: ()
        }
    }
}

pub struct CommandQueue {
    inner: ManuallyDrop<CommandQueueObject>,
    _context: ManuallyDrop<Context>,
    _device: ManuallyDrop<Device>,
    _unconstructable: (),
}


impl CommandQueueLock<CommandQueueWrapper> for CommandQueue {
    unsafe fn read_lock(&self) -> RwLockReadGuard<CommandQueueWrapper> {
        (*self.inner).read_lock()
    }
    unsafe fn write_lock(&self) -> RwLockWriteGuard<CommandQueueWrapper> {
        (*self.inner).write_lock()
    }
    unsafe fn rw_lock(&self) -> &RwLock<CommandQueueWrapper> {
        (*self.inner).rw_lock()
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
            ManuallyDrop::drop(&mut self.inner);
            ManuallyDrop::drop(&mut self._context);
            ManuallyDrop::drop(&mut self._device);
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> CommandQueue {
        CommandQueue {
            _device: self._device.clone(),
            _context: self._context.clone(),
            inner: ManuallyDrop::new((*self.inner).clone()),
            _unconstructable: ()
        }
    }
}

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

use CommandQueueInfo as CQInfo;

impl CommandQueue {
    unsafe fn new(queue: cl_command_queue, context: Context, device: Device) -> Output<CommandQueue> {
        let mut man_drop_context = ManuallyDrop::new(context);
        let mut man_drop_device = ManuallyDrop::new(device);

        match CommandQueueObject::from_retained(queue) {
            Ok(cq_object) => {
                Ok(CommandQueue {
                    inner: ManuallyDrop::new(cq_object),
                    _context: man_drop_context,
                    _device: man_drop_device,
                    _unconstructable: (),
                })
            },
            Err(e) => {
                // if an error has occurred we must drop context then device
                ManuallyDrop::drop(&mut man_drop_context);
                ManuallyDrop::drop(&mut man_drop_device);
                Err(e)
            }
        }
        
    }

    pub fn create(
        context: &Context,
        device: &Device,
        opt_props: Option<flags::CommandQueueProperties>,
    ) -> Output<CommandQueue> {
        let properties = match opt_props {
            None => flags::CommandQueueProperties::ProfilingEnable,
            Some(prop) => prop,
        };
        unsafe { 
            let command_queue = low_level::cl_create_command_queue(
                context,
                device,
                properties.bits() as cl_command_queue_properties,
            )?;
            CommandQueue::new(command_queue, context.clone(), device.clone())
        }
    }

    pub unsafe fn decompose(self) -> (cl_context, cl_device_id, cl_command_queue) {
        let cq: cl_command_queue = self.read_lock().command_queue_ptr();
        let parts = (self.context().context_ptr(), self.device().device_ptr(), cq);
        std::mem::forget(self);
        parts
    }

    pub fn new_copy(&self) -> Output<CommandQueue> {
        let props = self.properties()?;
        CommandQueue::create(
            self.context(),
            self.device(),
            Some(props)
        )
    }

    pub fn context(&self) -> &Context {
        &*self._context
    }

    pub fn device(&self) -> &Device {
        &*self._device
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &[T]) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send,
    {
        unsafe {
            let lock = self.write_lock();
            low_level::cl_enqueue_write_buffer(
                lock.command_queue_ptr(),
                device_mem,
                host_buffer,
                CommandQueueOptions::default()
            )
        }
    }

    /// write_buffer is used to move data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer_with_opts<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &[T],
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send 
    {
        unsafe {
            let lock = self.write_lock();
            low_level::cl_enqueue_write_buffer(lock.command_queue_ptr(), device_mem, host_buffer, command_queue_opts)
        }
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &mut [T]) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send 
    {
        let command_queue_opts = CommandQueueOptions::default();
        unsafe {
            let lock = self.write_lock();
            low_level::cl_enqueue_read_buffer(lock.command_queue_ptr(), device_mem, host_buffer, command_queue_opts)
        }
    }

    pub fn read_buffer_with_opts<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &mut [T],
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send,
    {
        unsafe {
            let lock = self.write_lock();
            low_level::cl_enqueue_read_buffer(lock.command_queue_ptr(), device_mem, host_buffer, command_queue_opts)
        }
    }

    pub fn sync_enqueue_kernel_with_opts(
        &self,
        kernel: &Kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event> {
        unsafe {
            let kernel_lock = kernel.write_lock();
            let cq_lock = self.write_lock();
            let cq: cl_command_queue = cq_lock.command_queue_ptr();
            let event = async_enqueue_kernel_with_opts(
                cq,
                kernel_lock.kernel_ptr(),
                work,
                command_queue_opts
            )?;
            low_level::cl_finish(cq)?;
            Ok(event)
        }
        
    }

    pub fn finish(&self) -> Output<()> {
        unsafe {
            let lock = self.write_lock();
            low_level::cl_finish(lock.command_queue_ptr())
        }
    }

    pub fn sync_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
        self.sync_enqueue_kernel_with_opts(kernel, work, CommandQueueOptions::default())
    }

    pub fn async_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
        self.async_enqueue_kernel_with_opts(kernel, work, CommandQueueOptions::default())
    }

    pub fn async_enqueue_kernel_with_opts(
        &self,
        kernel: &Kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event> {
        unsafe {
            let kernel_lock = kernel.write_lock();
            let cq_lock = self.write_lock();
            async_enqueue_kernel_with_opts(
                cq_lock.command_queue_ptr(),
                kernel_lock.kernel_ptr(),
                work,
                command_queue_opts
            )
        }
    }

    pub fn info<T: Copy>(self, flag: CQInfo) -> Output<ClPointer<T>> {
        unsafe { 
            let cq_lock = self.read_lock();
            info::fetch::<T>(cq_lock.command_queue_ptr(), flag)
        }
    }

    // pub fn load_context(&self) -> Output<Context> {
    //     unsafe { info::load_context(self.read_lock().command_queue_ptr()) }
    // }

    // pub fn load_device(&self) -> Output<Device> {
    //     unsafe { info::load_device(self.read_lock().command_queue_ptr()) }
    // }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { info::reference_count(self.read_lock().command_queue_ptr()) }
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        unsafe { info::properties(self.read_lock().command_queue_ptr()) }
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
