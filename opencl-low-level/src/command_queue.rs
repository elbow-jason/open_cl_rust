use std::fmt::Debug;
use libc::c_void;

use crate::ffi::{
    clCreateCommandQueue, clEnqueueNDRangeKernel, clEnqueueReadBuffer, clEnqueueWriteBuffer,
    clFinish, clGetCommandQueueInfo, cl_bool, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_event, cl_kernel, cl_context, cl_device_id, cl_mem,
};

use crate::cl_helpers::cl_get_info5;
use crate::{
    Output, CommandQueueInfo, build_output, DevicePtr, WaitList,
    Volume, ClInput, SizeAndPtr, ClPointer,
};


pub struct CommandQueueOptions {
    pub is_blocking: bool,
    pub offset: usize,
    pub wait_list: WaitList,
}

impl Default for CommandQueueOptions {
    fn default() -> CommandQueueOptions {
        CommandQueueOptions {
            is_blocking: true,
            offset: 0,
            wait_list: WaitList::empty(),
        }
    }
}

unsafe impl<T> ClInput<*const c_void> for &[T] {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<*const c_void> {
        let size = std::mem::size_of::<T>() * self.len();
        let ptr = self.as_ptr() as *const c_void;
        SizeAndPtr(size, ptr)
    }
}

unsafe impl<T> ClInput<*mut c_void> for &mut [T] {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<*mut c_void> {
        let size = std::mem::size_of::<T>() * self.len();
        let ptr = self.as_ptr() as *mut c_void;
        SizeAndPtr(size, ptr)
    }
}

// fn buffer_mem_size_and_ptr<T>(buf: &[T]) -> (usize, *const c_void) {
//     (
//         std::mem::size_of::<T>() * buf.len(),
//         buf.as_ptr() as *const libc::c_void,
//     )
// }



// use crate::event::{Event, WaitList};
// use opencl_low_level::{DevicePtr, DeviceRefCount};

__release_retain!(command_queue, CommandQueue);

pub unsafe fn cl_create_command_queue(
    context: cl_context,
    device: cl_device_id,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    device.usability_check()?;
    let mut err_code = 0;
    let command_queue = clCreateCommandQueue(
        context,
        device,
        flags,
        &mut err_code,
    );
    build_output(command_queue, err_code)
}

pub unsafe fn cl_finish(command_queue: cl_command_queue) -> Output<()> {
    build_output((), clFinish(command_queue))
}

pub unsafe fn cl_enqueue_nd_range_kernel(
    queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: u8,
    global_work_offset: Option<[usize; 3]>,
    global_work_size: [usize; 3],
    local_work_size: Option<[usize; 3]>,
    wait_list: WaitList,
) -> Output<cl_event> {
    let mut tracking_event: cl_event = new_tracking_event();

    let SizeAndPtr(wait_list_len, wait_list_ptr) = wait_list.size_and_ptr();

    let global_work_offset_ptr = global_work_offset.map_or_else(|| Volume::empty_ptr(),|g| Volume::from(g).as_ptr());
    let global_work_size_ptr = Volume::from(global_work_size).as_ptr();
    let local_work_size_ptr = local_work_size.map_or_else(|| Volume::empty_ptr(),|g| Volume::from(g).as_ptr());

    let err_code = clEnqueueNDRangeKernel(
        queue,
        kernel,
        u32::from(work_dim),
        global_work_offset_ptr,
        global_work_size_ptr,
        local_work_size_ptr,
        wait_list_len as u32,
        wait_list_ptr,
        &mut tracking_event,
    );

    build_output((), err_code)?;
    cl_finish(queue)?;

    // TODO: Remove this check when Event checks for null pointer
    debug_assert!(!tracking_event.is_null());
    Ok(tracking_event)
}

fn new_tracking_event() -> cl_event {
    std::ptr::null_mut() as cl_event
}

// #[inline]
// fn into_event(err_code: cl_int, tracking_event: cl_event) -> Output<cl_event> {
//     build_output(tracking_event, err_code)?;
// }

pub unsafe fn cl_enqueue_read_buffer<T>(
    queue: cl_command_queue,
    mem: cl_mem,
    buffer: &mut [T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event> where T: Debug + Sync + Send {
    let mut tracking_event = new_tracking_event();

    let SizeAndPtr(wl_len, wl_ptr) = command_queue_opts.wait_list.size_and_ptr();

    let SizeAndPtr(buffer_size, buffer_ptr) = buffer.size_and_ptr();

    // TODO: Make this a Error returning check
    // debug_assert!(buffer.len() == device_mem.len());

    let err_code = clEnqueueReadBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_size,
        buffer_ptr,
        wl_len as u32,
        wl_ptr,
        &mut tracking_event,
    );
    build_output(tracking_event, err_code)
}

pub unsafe fn cl_enqueue_write_buffer<T>(
    queue: cl_command_queue,
    mem: cl_mem,
    buffer: &[T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event> where T: Debug + Sync + Send {
    let mut tracking_event = new_tracking_event();
    
    let SizeAndPtr(wlist_size, wlist_ptr) = command_queue_opts.wait_list.size_and_ptr();

    let SizeAndPtr(buffer_size, buffer_ptr) = buffer.size_and_ptr();
        
    let err_code = clEnqueueWriteBuffer(
        queue,
        mem,
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_size,
        buffer_ptr,
        wlist_size as u32,
        wlist_ptr,
        &mut tracking_event,
    );

    build_output(tracking_event, err_code)
}

pub unsafe fn cl_get_command_queue_info<T: Copy>(
    command_queue: cl_command_queue,
    flag: CommandQueueInfo,
) -> Output<ClPointer<T>> {
    cl_get_info5(
        command_queue,
        flag as cl_command_queue_info,
        clGetCommandQueueInfo,
    )
}
