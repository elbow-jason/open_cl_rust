pub mod flags;
pub mod helpers;
pub mod low_level;

use std::fmt::Debug;

use num::Num;

use flags::{CommandQueueInfo, CommandQueueProperties};

use crate::ffi::{cl_command_queue, cl_command_queue_properties};

use crate::{Context, Device, DeviceMem, Event, Kernel, Output, Work};

use crate::cl::{ClObject, ClPointer};

use helpers::CommandQueueOptions;
use low_level::{cl_release_command_queue, cl_retain_command_queue};

__impl_unconstructable_cl_wrapper!(CommandQueue, cl_command_queue);
__impl_cl_object_for_wrapper!(
    CommandQueue,
    cl_command_queue,
    cl_retain_command_queue,
    cl_release_command_queue
);
__impl_clone_for_cl_object_wrapper!(CommandQueue, cl_retain_command_queue);
__impl_drop_for_cl_object_wrapper!(CommandQueue, cl_release_command_queue);

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

use CommandQueueInfo as CQInfo;

impl CommandQueue {
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
        unsafe { CommandQueue::new(command_queue) }
    }

    /// write_buffer is used to ,ove data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &[T]) -> Output<Event>
    where
        T: Sized + Debug + Num,
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
        T: Sized + Debug + Num,
    {
        low_level::cl_enqueue_write_buffer(self, device_mem, host_buffer, command_queue_opts)
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T>(&self, device_mem: &DeviceMem<T>, host_buffer: &mut [T]) -> Output<Event>
    where
        T: Sized + Debug + Num,
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
        T: Sized + Debug + Num,
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
        // The OpenCL context gives an non-reference counted pointer.
        // What an absolute joy.
        // Manually increase the reference count.
        self.info(CQInfo::Context)
            .and_then(|ret| unsafe { ret.into_retained_wrapper::<Context>() })
    }

    pub fn device(&self) -> Output<Device> {
        self.info(CQInfo::Device)
            .and_then(|ret| unsafe { ret.into_retained_wrapper::<Device>() })
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
        let device = Device::default();
        Session::create(device, src).expect("Failed to create Session")
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
