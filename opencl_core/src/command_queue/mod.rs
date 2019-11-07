
pub mod low_level;
pub mod flags;
pub mod helpers;

use std::fmt::Debug;

use num::Num;

use flags::{
    CommandQueueInfo,
    CommandQueueProperties
};

use crate::ffi::{
    cl_command_queue,
    cl_command_queue_properties,
};

use crate::{
    DeviceMem,
    Device,
    Context,
    Event,
    Kernel,
    Output,
    Work,
};

use crate::utils::cl_value::{ClDecoder, ClReturn};

use low_level::{cl_release_command_queue, cl_retain_command_queue};
use helpers::CommandQueueOptions;

__impl_unconstructable_cl_wrapper!(CommandQueue, cl_command_queue);
__impl_cl_object_for_wrapper!(CommandQueue, cl_command_queue);
__impl_clone_for_cl_object_wrapper!(CommandQueue, cl_retain_command_queue);
__impl_drop_for_cl_object_wrapper!(CommandQueue, cl_release_command_queue);

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
        Ok(unsafe { CommandQueue::new(command_queue) })
    }

    /// write_buffer is used to ,ove data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &[T]
    ) -> Output<Event>
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
    pub fn read_buffer<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &mut [T],
        
    ) -> Output<Event>
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
        let () = low_level::cl_finish(self)?;
        Ok(event)
    }

    pub fn sync_enqueue_kernel(
        &self,
        kernel: &Kernel,
        work: &Work,
        
    ) -> Output<Event> {
        let command_queue_opts = CommandQueueOptions::default();
        let event = self.async_enqueue_kernel_with_opts(kernel, work, command_queue_opts)?;
        let () = low_level::cl_finish(self)?;
        Ok(event)
    }

    pub fn async_enqueue_kernel(
        &self,
        kernel: &Kernel,
        work: &Work,
    ) -> Output<Event> {
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
    fn info(&self, flag: CommandQueueInfo) -> Output<ClReturn> {
        low_level::cl_get_command_queue_info(self, flag)
    }

    pub fn context(&self) -> Output<Context> {
        let cl_ret = self.info(CommandQueueInfo::Context)?;
        Ok(unsafe { cl_ret.cl_decode() })
    }

    pub fn device(&self) -> Output<Device> {
        let cl_ret = self.info(CommandQueueInfo::Device)?;
        Ok(unsafe { cl_ret.cl_decode() })
        
    }

    pub fn reference_count(&self) -> Output<usize> {
        let cl_ret = self.info(CommandQueueInfo::ReferenceCount)?;
        Ok(unsafe { cl_ret.cl_decode() })
    }

    pub fn properties(&self) -> Output<CommandQueueProperties> {
        let cl_ret = self.info(CommandQueueInfo::Properties)?;
        Ok(unsafe { cl_ret.cl_decode() })
    }
    
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn command_queue_can_be_created_test() {
    //     unimplemented!()
    // }
}