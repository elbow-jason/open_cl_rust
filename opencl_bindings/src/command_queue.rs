#![allow(non_upper_case_globals)]
use std::fmt::Debug;

use num::Num;

use crate::ffi::{
    cl_command_queue,
    cl_command_queue_info,
    cl_command_queue_properties,
    // cl_mem,
};

use crate::open_cl::{
    cl_enqueue_nd_range_kernel,
    cl_enqueue_read_buffer,
    cl_enqueue_write_buffer,
    // cl_event,
    cl_finish,
    cl_release_command_queue,
    ClObject,
};

use crate::{
    // Count,
    // AsMutPointer,
    // MemSize,
    // Offset,
    DeviceMem,
    // Event,
    Event,
    // UserEvent,
    WaitList,
    Kernel,
    BufferOpConfig,
    Output,
    // ReadFromDeviceMem,
    Work,
    // AsPointer,
    // WriteToDeviceMem,
    // HostBuffer,
};

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CommandQueue{
    inner: cl_command_queue,
    _unconstructable: ()
}

impl ClObject<cl_command_queue> for CommandQueue {
    unsafe fn raw_cl_object(&self) -> cl_command_queue {
        self.inner
    }
}

impl CommandQueue {
    pub(crate) unsafe fn new(queue: cl_command_queue) -> CommandQueue {
        CommandQueue{
            inner: queue,
            _unconstructable: ()
        }
    }


    /// Move data from the HostBuffer (probably a Vec<T> or &[T]) to the OpenCL cl_mem pointer.
    pub fn write_buffer<T>(
        &self,
        d_mem: &DeviceMem<T>,
        buffer: &[T],
        event_list: WaitList,
        maybe_op_config: Option<BufferOpConfig>,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num,
    {
        let buffer_op_cfg: BufferOpConfig = maybe_op_config.unwrap_or_else(|| BufferOpConfig::default());
        println!("write_buffer enqueuing {:?} to {:?}", buffer, d_mem);
        cl_enqueue_write_buffer(
            self,
            d_mem,
            buffer,
            buffer_op_cfg,
            event_list,
        )
    }

    /// Move data from the OpenCL cl_mem pointer to the HostBuffer.
    pub fn read_buffer<T>(
        &self,
        device_mem: &DeviceMem<T>,
        buffer: &mut [T],
        event_list: WaitList,
        maybe_op_config: Option<BufferOpConfig>,
    ) -> Output<Event>
    where
        T: Sized + Debug + Num,
    {
        // println!("
        // read_buffer
        // device_mem: {:?}
        // buffer: {:?}
        // event_list: {:?}
        // ",
        // device_mem,
        // buffer,
        // event_list,

        // );
    
        let buffer_op_cfg: BufferOpConfig = maybe_op_config.unwrap_or_else(|| BufferOpConfig::default());

        cl_enqueue_read_buffer(
            self,
            device_mem,
            buffer,
            buffer_op_cfg,
            event_list,
        )
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
        // println!(
        //     "async_enqueue_kernel (self: {:?}, kernel: {:?}, work: {:?}, event_list: {:?}",
        //     self, kernel, work, event_list
        // );
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
        // println!("Dropping command_queue {:?}", self);
        unsafe { cl_release_command_queue(&self.raw_cl_object()); }
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
