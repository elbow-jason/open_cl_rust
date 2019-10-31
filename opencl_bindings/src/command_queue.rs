#![allow(non_upper_case_globals)]
use std::fmt::Debug;

use num::Num;

use crate::ffi::{
    cl_command_queue,
    cl_command_queue_info,
    cl_command_queue_properties,
};

use crate::open_cl::{
    cl_enqueue_nd_range_kernel,
    cl_enqueue_read_buffer,
    cl_enqueue_write_buffer,
    cl_finish,
    cl_release_command_queue,
    ClObject,
};

use crate::{
    BufferOpConfig,
    DeviceMem,
    Event,
    Kernel,
    Output,
    WaitList,
    Work,
};

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CommandQueue {
    inner: cl_command_queue,
    _unconstructable: (),
}

impl ClObject<cl_command_queue> for CommandQueue {
    unsafe fn raw_cl_object(&self) -> cl_command_queue {
        self.inner
    }
}

impl CommandQueue {
    pub(crate) unsafe fn new(queue: cl_command_queue) -> CommandQueue {
        CommandQueue {
            inner: queue,
            _unconstructable: (),
        }
    }

    /// write_buffer is used to ,ove data from the host buffer (buffer: &[T]) to
    /// the OpenCL cl_mem pointer inside `d_mem: &DeviceMem<T>`.
    pub fn write_buffer<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &[T],
        event_list: WaitList,
        maybe_op_config: Option<BufferOpConfig>,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num,
    {
        let buffer_op_cfg: BufferOpConfig =
            maybe_op_config.unwrap_or_else(|| BufferOpConfig::default());

        cl_enqueue_write_buffer(self, device_mem, host_buffer, buffer_op_cfg, event_list)
    }

    /// read_buffer is used to move data from the `device_mem` (`cl_mem` pointer
    /// inside `&DeviceMem<T>`) into a `host_buffer` (`&mut [T]`).
    pub fn read_buffer<T>(
        &self,
        device_mem: &DeviceMem<T>,
        host_buffer: &mut [T],
        event_list: WaitList,
        maybe_op_config: Option<BufferOpConfig>,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num,
    {
        let buffer_op_cfg: BufferOpConfig =
            maybe_op_config.unwrap_or_else(|| BufferOpConfig::default());

        cl_enqueue_read_buffer(self, device_mem, host_buffer, buffer_op_cfg, event_list)
    }

    pub fn sync_enqueue_kernel(
        &self,
        kernel: &Kernel,
        work: Work,
        event_list: WaitList,
    ) -> Output<Event> {
        let event = self.async_enqueue_kernel(kernel, work, event_list, None)?;
        let () = cl_finish(self)?;
        Ok(event)
    }

    fn async_enqueue_kernel(
        &self,
        kernel: &Kernel,
        work: Work,
        event_list: WaitList,
        _execution_event: Option<Event>,
    ) -> Output<Event> {
        cl_enqueue_nd_range_kernel(
            &self,
            kernel,
            work.work_dim(),
            work.global_work_offset(),
            work.global_work_size(),
            work.local_work_size(),
            event_list,
        )
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe {
            cl_release_command_queue(&self.raw_cl_object());
        }
    }
}

bitflags! {
    pub struct CommandQueueProperties: cl_command_queue_properties {
        const OutOfOrderExecModeEnable = 1 << 0;
        const ProfilingEnable = 1 << 1;
        const OnDevice = 1 << 2;
        const OnDeviceDefault = 1 << 3;
    }
}

impl Default for CommandQueueProperties {
    fn default() -> CommandQueueProperties {
        CommandQueueProperties::ProfilingEnable
    }
}

crate::__codes_enum!(CommandQueueInfo, cl_command_queue_info, {
    Context => 0x1090,
    Device => 0x1091,
    ReferenceCount => 0x1092,
    Properties => 0x1093,
    Size => 0x1094,
    DeviceDefault => 0x1095
});
