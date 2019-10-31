use std::fmt::Debug;

use crate::command_queue::{CommandQueue, CommandQueueProperties};
use crate::device_mem::MemFlag;
use crate::ffi::{
    cl_command_queue_properties, cl_context, cl_context_info, cl_context_properties, cl_mem_flags,
};
use crate::open_cl::{
    cl_create_buffer, cl_create_command_queue, cl_create_program_with_binary,
    cl_create_program_with_source, cl_release_context,
};

use crate::{ClObject, Device, DeviceMem, Output, Program};

#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    inner: cl_context,
    _unconstructable: (),
}

impl ClObject<cl_context> for Context {
    unsafe fn raw_cl_object(&self) -> cl_context {
        // println!("Context getting raw_cl_object {:?}", self);
        self.inner
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // println!("Dropping program {:?}", self);
        unsafe {
            cl_release_context(&self.raw_cl_object());
        }
    }
}

impl Context {
    pub fn new(inner: cl_context) -> Context {
        Context {
            inner,
            _unconstructable: (),
        }
    }

    pub fn create_command_queue(
        &self,
        device: &Device,
        opt_props: Option<CommandQueueProperties>,
    ) -> Output<CommandQueue> {
        let properties = match opt_props {
            None => CommandQueueProperties::ProfilingEnable,
            Some(prop) => prop,
        };
        let command_queue = cl_create_command_queue(
            self,
            &device,
            properties.bits() as cl_command_queue_properties,
        )?;
        Ok(unsafe { CommandQueue::new(command_queue) })
    }

    /// The KernelReadOnlyMem struct holds a `cl_mem` pointer that can be
    /// written to by the host (you from Rust-land), but cannot be written to
    /// by a kernel. That is to say performing kernel operations on a KernelReadOnlyMem
    /// cannot cause the KernelReadOnlyMem's underlying data to be changed.
    pub fn create_read_only_buffer<T>(&self, buffer_len: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        // println!("create_read_only_buffer buffer_size {:?}", buffer_len);

        let buf = cl_create_buffer::<T>(
            self,
            buffer_len,
            MemFlag::ReadOnly as cl_mem_flags | MemFlag::AllocHostPtr as cl_mem_flags,
        )?;
        Ok(DeviceMem::new(buf))
    }

    /// The KernelWriteOnlyMem struct holds a `cl_mem` pointer that can be
    /// written to by the host (you from Rust-land), AND can be written to
    /// by a kernel. That is to say performing kernel operations on a
    /// KernelWriteOnlyMem can cause the KernelWriteOnlyMem's underlying data
    /// to be changed. This type is best used as the assignable target for
    /// an operation. For example in the operation `let x = a + b` the `x`
    /// variable is the "assignable target" that is written to.
    pub fn create_write_only_buffer<T>(&self, buffer_size: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        let buf = cl_create_buffer::<T>(
            self,
            buffer_size,
            MemFlag::WriteOnly as cl_mem_flags | MemFlag::AllocHostPtr as cl_mem_flags,
        )?;
        Ok(DeviceMem::new(buf))
    }

    /// The KernelReadWriteMem struct holds a `cl_mem` pointer that can be
    /// written to by the host (you from Rust-land), AND can be written to
    /// by a kernel. The kernel can also read from the memory. That is to
    /// say performing kernel operations on a KernelReadWriteMem can cause
    /// the KernelReadOnlyMem's underlying data to be changed. This type
    /// is best used as an intermediate target for an operation.
    /// The KernelReadWriteMem is essentially a mutable variable and is
    /// bet uses when output for a kernel operation is passed to another
    /// kernel as an arg.
    pub fn create_read_write_buffer<T>(&self, buffer_size: usize) -> Output<DeviceMem<T>>
    where
        T: Debug,
    {
        let buf = cl_create_buffer::<T>(
            self,
            buffer_size,
            MemFlag::ReadWrite as cl_mem_flags | MemFlag::AllocHostPtr as cl_mem_flags,
        )?;
        Ok(DeviceMem::new(buf))
    }

    pub fn create_program_with_source(&self, src: String) -> Output<Program> {
        cl_create_program_with_source(self, &src[..])
    }

    pub fn create_program_with_binary(&self, device: &Device, binary: String) -> Output<Program> {
        cl_create_program_with_binary(self, device, &binary[..])
    }
}

crate::__codes_enum!(ContextInfo, cl_context_info, {
    ReferenceCount => 0x1080,
    Devices => 0x1081,
    Properties => 0x1082,
    NumDevices => 0x1083
});

crate::__codes_enum!(ContextProperties, cl_context_properties, {
    Platform => 0x1084,
    InteropUserSync => 0x1085
});
