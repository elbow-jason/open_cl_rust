pub mod flags;
pub mod helpers;
pub mod low_level;
pub mod info;

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc};
use std::mem::ManuallyDrop;
use std::fmt;
use std::fmt::Debug;

use num::Num;

use flags::{CommandQueueInfo, CommandQueueProperties};

use crate::ffi::{
    cl_command_queue,
    cl_command_queue_properties,
    cl_context,
    cl_device_id,
};

use crate::{utils, Context, Device, DevicePtr, DeviceMem, Event, Kernel, Output, Work};

use crate::cl::ClPointer;

use helpers::CommandQueueOptions;

pub unsafe fn release_command_queue(cq: cl_command_queue) {
    low_level::cl_release_command_queue(cq).unwrap_or_else(|e| {
        panic!("Failed to release cl_command_queue {:?} due to {:?}", cq, e);
    })
}

pub unsafe fn retain_command_queue(cq: cl_command_queue) {
    low_level::cl_retain_command_queue(cq).unwrap_or_else(|e| {
        panic!("Failed to retain cl_command_queue {:?} due to {:?}", cq, e);
    })
}


pub trait CommandQueueLock {
    unsafe fn write_lock(&self) -> RwLockWriteGuard<cl_command_queue>;
    unsafe fn read_lock(&self) -> RwLockReadGuard<cl_command_queue>;
    unsafe fn rw_lock(&self) -> &RwLock<cl_command_queue>;
}

pub trait CommandQueueRefCount: Sized {
    unsafe fn from_retained(cq: cl_command_queue) -> Output<Self>;
    unsafe fn from_unretained(cq: cl_command_queue) -> Output<Self>;
}


impl CommandQueueRefCount for CommandQueueObject {
    unsafe fn from_retained(cq: cl_command_queue) -> Output<CommandQueueObject> {
        utils::null_check(cq, "CommandQueueObject::from_retained")?;
        Ok(CommandQueueObject::unchecked_new(cq))
    }

    unsafe fn from_unretained(cq: cl_command_queue) -> Output<CommandQueueObject> {
        utils::null_check(cq, "DeviceObject::from_unretained")?;
        retain_command_queue(cq);
        Ok(CommandQueueObject::unchecked_new(cq))
    }
}

pub struct CommandQueueObject {
    object: Arc<RwLock<cl_command_queue>>,
    _unconstructable: (),
}

impl CommandQueueObject {
    unsafe fn unchecked_new(cq: cl_command_queue) -> CommandQueueObject {
        CommandQueueObject {
            object: Arc::new(RwLock::new(cq)),
            _unconstructable: (),
        }
    }
}

impl Drop for CommandQueueObject {
    fn drop(&mut self) {
        unsafe {
            let lock = self.write_lock();
            let cq = *lock;
            let rust_arc = Arc::strong_count(&self.object);
            let opencl_arc = info::reference_count(cq);
            
            debug!("cl_command_queue {:?} - CommandQueueObject::drop - start - rust_arc: {:?}, opencl_arc: {:?}", cq, rust_arc, opencl_arc);
            if let (1, Ok(1)) = (rust_arc, opencl_arc) {
                debug!("cl_command_queue {:?} - CommandQueueObject::drop - finishing", cq);
                low_level::cl_finish(cq).unwrap();
                debug!("cl_command_queue {:?} - CommandQueueObject::drop - finished", cq);
            }
            debug!("cl_command_queue {:?} - CommandQueueObject::drop - releasing", cq);
            match low_level::cl_release_command_queue(cq) {
                Ok(()) => {
                    debug!("cl_command_queue {:?} - CommandQueueObject::drop - released", cq);
                },
                Err(e) => {
                    std::mem::drop(lock);
                    error!("cl_command_queue {:?} - CommandQueueObject::drop - failure - error: {:?}", cq, e);
                    panic!("Failed to release cl_command_queue {:?} due to {:?}", cq, e);
                }
            }
        }
    }
}

impl Clone for CommandQueueObject {
    fn clone(&self) -> CommandQueueObject {
        unsafe {
            let lock = self.object.read().unwrap();
            retain_command_queue(*lock);
            std::mem::drop(lock);

            CommandQueueObject{
                object: self.object.clone(),
                _unconstructable: ()
            }
        }
    }
}

impl CommandQueueLock for CommandQueueObject {
    unsafe fn read_lock(&self) -> RwLockReadGuard<cl_command_queue> {
        self.object.read().unwrap()
    }

    unsafe fn write_lock(&self) -> RwLockWriteGuard<cl_command_queue> {
        self.object.write().unwrap()
    }
    unsafe fn rw_lock(&self) -> &RwLock<cl_command_queue> {
        &*self.object
    }
}

impl fmt::Debug for CommandQueueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let read_lock = unsafe { self.read_lock() };
        let address = *read_lock;

        write!(f, "CommandQueueObject{{{:?}}}", address)
    }
}

pub struct CommandQueue {
    inner: ManuallyDrop<CommandQueueObject>,
    _context: ManuallyDrop<Context>,
    _device: ManuallyDrop<Device>,
    _unconstructable: (),
}


impl CommandQueueLock for CommandQueue {
    unsafe fn read_lock(&self) -> RwLockReadGuard<cl_command_queue> {
        (*self.inner).read_lock()
    }
    unsafe fn write_lock(&self) -> RwLockWriteGuard<cl_command_queue> {
        (*self.inner).write_lock()
    }
    unsafe fn rw_lock(&self) -> &RwLock<cl_command_queue> {
        (*self.inner).rw_lock()
    }
}

impl fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandQueue{{{:?}}}", unsafe { *self.read_lock() })
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        debug!("cl_command_queue {:?} - CommandQueue::drop", unsafe { *self.read_lock() });
        unsafe {
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
            inner: self.inner.clone(),
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
        let command_queue = low_level::cl_create_command_queue(
            context,
            &device,
            properties.bits() as cl_command_queue_properties,
        )?;
        unsafe { CommandQueue::new(command_queue, context.clone(), device.clone()) }
    }

    pub unsafe fn decompose(self) -> (cl_context, cl_device_id, cl_command_queue) {
        let cq: cl_command_queue = *self.read_lock();
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
        let command_queue_opts = CommandQueueOptions::default();
        low_level::cl_enqueue_write_buffer(self, device_mem, host_buffer, command_queue_opts)
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
        low_level::cl_enqueue_write_buffer(self, device_mem, host_buffer, command_queue_opts)
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &mut [T]) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send 
    {
        let command_queue_opts = CommandQueueOptions::default();
        unsafe {
            low_level::cl_enqueue_read_buffer(self, device_mem, host_buffer, command_queue_opts)
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
            low_level::cl_enqueue_read_buffer(self, device_mem, host_buffer, command_queue_opts)
        }
    }

    pub fn sync_enqueue_kernel_with_opts(
        &self,
        kernel: &Kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event> {
        let event = self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)?;
        low_level::cl_finish(unsafe { *self.read_lock() })?;
        Ok(event)
    }

    pub fn sync_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
        let command_queue_opts = CommandQueueOptions::default();
        let event = self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)?;
        low_level::cl_finish(unsafe { *self.read_lock() })?;
        Ok(event)
    }

    pub fn async_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
        let command_queue_opts = CommandQueueOptions::default();
        self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)
    }

    pub fn async_enqueue_kernel_with_opts(
        &self,
        kernel: &Kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event> {
        unsafe {
            low_level::cl_enqueue_nd_range_kernel(
                &self,
                kernel,
                work.work_dim(),
                work.global_work_offset(),
                work.global_work_size(),
                work.local_work_size(),
                command_queue_opts.wait_list,
            )
        }
    }

    pub fn info<T: Copy>(self, flag: CQInfo) -> Output<ClPointer<T>> {
        unsafe { 
            let cq_lock = self.read_lock();
            info::fetch::<T>(*cq_lock, flag)
        }
    }

    pub fn load_context(&self) -> Output<Context> {
        unsafe { info::load_context(*self.read_lock()) }
    }

    pub fn load_device(&self) -> Output<Device> {
        unsafe { info::load_device(*self.read_lock()) }
    }

    pub fn reference_count(&self) -> Output<u32> {
        unsafe { info::reference_count(*self.read_lock()) }
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        unsafe { info::properties(*self.read_lock()) }
    }
}

impl PartialEq for CommandQueue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { std::ptr::eq(*self.read_lock(), *other.read_lock()) }
    }
}

impl Eq for CommandQueue {}

#[cfg(test)]
mod tests {
    use super::flags::CommandQueueProperties;
    use crate::{Context, Output, Device, testing};
    const SRC: &'static str = "
    __kernel void test(__global int *i) {
        *i += 1;
    }";
    

    #[test]
    pub fn command_queue_method_context_works() {
        testing::init_logger();
        let session = testing::get_session(SRC);
        let _context: &Context = session.command_queue().context();
    }

    #[test]
    pub fn command_queue_method_load_context_works() {
        let session = testing::get_session(SRC);
        let result: Output<Context> = session.command_queue().load_context();
        result.unwrap_or_else(|e| panic!("Failed to load_context: {:?}", e));
    }

    #[test]
    pub fn command_queue_load_context_matches_kept_context() {
        let session = testing::get_session(SRC);
        let kept_context: &Context = session
            .command_queue()
            .context();
        let loaded_context: Context = session.command_queue().load_context().unwrap();
        assert_eq!(kept_context, &loaded_context); 
    }

    #[test]
    pub fn command_queue_method_device_works() {
        let session = testing::get_session(SRC);
        let _device: &Device = session.command_queue().device();
    }

    #[test]
    pub fn command_queue_method_load_device_works() {
        let session = testing::get_session(SRC);
        let result: Output<Device> = session.command_queue().load_device();
        result.unwrap_or_else(|e| panic!("Failed to load_device: {:?}", e));
    }

    #[test]
    pub fn command_queue_load_device_matches_kept_device() {
        let session = testing::get_session(SRC);
        let kept_device = session
            .command_queue()
            .device();
        let loaded_device = session.command_queue().load_device().unwrap();
        assert_eq!(kept_device, &loaded_device); 
    }

    #[test]
    pub fn command_queue_method_reference_count_works() {
        let session = testing::get_session(SRC);
        let ref_count: u32 = session
            .command_queue()
            .reference_count()
            .expect("CommandQueue method reference_count() failed");
        assert_eq!(ref_count, 1);
    }

    #[test]
    pub fn command_queue_method_properties_works() {
        let session = testing::get_session(SRC);
        let props: CommandQueueProperties = session
            .command_queue()
            .properties()
            .expect("CommandQueue method properties() failed");
        let bits = props.bits();
        let maybe_same_prop = CommandQueueProperties::from_bits(bits);
        if !maybe_same_prop.is_some() {
            panic!(
                "
                CommandQueue method properties returned \
                an invalid CommandQueueProperties bitflag {:?}\
                ",
                bits
            );
        }
    }

    #[test]
    pub fn command_queue_copy_new_works() {
        let session = testing::get_session(SRC);
        let cq2 = session.command_queue().new_copy().unwrap();
        assert!(&cq2 != session.command_queue());
    }
}
