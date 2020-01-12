pub mod flags;
pub mod helpers;
pub mod low_level;
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

use crate::{Context, ContextRefCount, Device, DevicePtr, DeviceRefCount, DeviceMem, Event, Kernel, Output, Work};

use crate::cl::ClPointer;
use crate::cl::ClObjectError;
use crate::error::Error;

use helpers::CommandQueueOptions;
use low_level::{cl_release_command_queue}; //, cl_retain_command_queue};

pub trait CommandQueuePtr {
    fn command_queue_ptr(&self) -> cl_command_queue;
}


pub struct CommandQueue {
    inner: ManuallyDrop<cl_command_queue>,
    context: Context,
    device: Device,
    _unconstructable: (),
}


impl CommandQueuePtr for CommandQueue {
    fn command_queue_ptr(&self) -> cl_command_queue {
        *self.inner
    }
}

impl fmt::Debug for CommandQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandQueue{{{:?}}}", self.command_queue_ptr())
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            cl_release_command_queue(self.command_queue_ptr()).unwrap_or_else(|e| {
                panic!("Failed to release cl_command_queue {:?} {:?}", self, e);
            })
        }
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> CommandQueue {
        let props = self.properties().unwrap_or_else(|e| {
                panic!("Failed to retrieve existing command queue properties! {:?} {:?}", self, e);
            });
        CommandQueue::create(
            &self.context,
            &self.device,
            Some(props)
        ).unwrap_or_else(|e| {
            panic!("Failed to clone CommandQueue {:?} {:?}", self, e);
        })

    }
}


// impl ClObject<cl_command_queue> for CommandQueue {
    // unsafe fn raw_cl_object(&self) -> cl_command_queue {
    //     self.inner
    // }

    // unsafe fn new_retained(cl_object: $cl_object_type) -> Output<$wrapper> {
    //     if cl_object.is_null() {
    //         use crate::cl::ClObjectError;
    //         use crate::error::Error;
    //         let wrapper_name = stringify!($wrapper).to_string();
    //         let e = Error::ClObjectError(ClObjectError::ClObjectCannotBeNull(wrapper_name));
    //         return Err(e);
    //     }
    //     let () = $retain_func(cl_object)?;
    //     Ok($wrapper {
    //         inner: cl_object,
    //         _unconstructable: (),
    //     })
    // }
// }

// __impl_unconstructable_cl_wrapper!(CommandQueue, cl_command_queue);
// __impl_default_debug_for!(CommandQueue);
// __impl_cl_object_for_wrapper!(
//     CommandQueue,
//     cl_command_queue,
//     cl_retain_command_queue,
//     cl_release_command_queue
// );
// __impl_clone_for_cl_object_wrapper!(CommandQueue, cl_retain_command_queue);
// __impl_drop_for_cl_object_wrapper!(CommandQueue, cl_release_command_queue);

unsafe impl Send for CommandQueue {}
// unsafe impl Sync for CommandQueue {}

use CommandQueueInfo as CQInfo;

impl CommandQueue {
     unsafe fn new(queue: cl_command_queue, context: Context, device: Device) -> Output<CommandQueue> {
        if queue.is_null() {
            let e = Error::ClObjectError(ClObjectError::ClObjectCannotBeNull("CommandQueue".to_string()));
            return Err(e);
        }
        Ok(CommandQueue {
            inner: ManuallyDrop::new(queue),
            context,
            device,
            _unconstructable: (),
        })
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
        let parts = (self.context.context_ptr(), self.device.device_ptr(), *self.inner);
        std::mem::forget(self);
        parts
    }

    /// write_buffer is used to ,ove data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &[T]) -> Output<Event>
    where
        T: Sized + Debug + Num + Sync + Send,
    {
        let command_queue_opts = CommandQueueOptions::default();
        low_level::cl_enqueue_write_buffer(self, device_mem, host_buffer, command_queue_opts)
    }

    /// write_buffer is used to ,ove data from the host buffer (buffer: &[T]) to
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
        low_level::cl_enqueue_read_buffer(self, device_mem, host_buffer, command_queue_opts)
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
        low_level::cl_enqueue_read_buffer(self, device_mem, host_buffer, command_queue_opts)
    }

    pub fn sync_enqueue_kernel_with_opts(
        &self,
        kernel: &Kernel,
        work: &Work,
        command_queue_opts: CommandQueueOptions,
    ) -> Output<Event> {
        let event = self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)?;
        low_level::cl_finish(self)?;
        Ok(event)
    }

    pub fn sync_enqueue_kernel(&self, kernel: &Kernel, work: &Work) -> Output<Event> {
        let command_queue_opts = CommandQueueOptions::default();
        let event = self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)?;
        low_level::cl_finish(self)?;
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
    fn info<T: Copy>(&self, flag: CQInfo) -> Output<ClPointer<T>> {
        low_level::cl_get_command_queue_info(self, flag)
    }

    pub fn context(&self) -> Output<Context> {
        self.info(CQInfo::Context).and_then(|cl_ptr| unsafe {
            Context::from_unretained(cl_ptr.into_one())
        })
    }

    pub fn device(&self) -> Output<Device> {
        self.info(CQInfo::Device)
            .and_then(|ret| unsafe { Device::from_unretained(ret.into_one()) })
    }

    pub fn reference_count(&self) -> Output<u32> {
        self.info(CQInfo::ReferenceCount)
            .map(|ret| unsafe { ret.into_one() })
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        self.info(CQInfo::Properties)
            .map(|ret| unsafe { ret.into_one() })
    }
}

#[cfg(test)]
mod tests {
    use super::flags::CommandQueueProperties;
    use crate::{Context, Device, Session};

    fn get_session() -> Session {
        let src = "__kernel void test(__global int *i) { *i += 1; }";
        let devices = [Device::default()];
        Session::create_sessions(&devices, src).expect("Failed to create Session").remove(0)
    }

    #[test]
    pub fn command_queue_method_context_works() {
        let session = get_session();
        let _ctx: Context = session
            .command_queue()
            .context()
            .expect("CommandQueue method context() failed");
    }

    #[test]
    pub fn command_queue_method_device_works() {
        let session = get_session();
        let _device: Device = session
            .command_queue()
            .device()
            .expect("CommandQueue method device() failed");
    }

    #[test]
    pub fn command_queue_method_reference_count_works() {
        let session = get_session();
        let ref_count: u32 = session
            .command_queue()
            .reference_count()
            .expect("CommandQueue method reference_count() failed");
        assert_eq!(ref_count, 1);
    }

    #[test]
    pub fn command_queue_method_properties_works() {
        let session = get_session();
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
}
