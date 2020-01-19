use std::fmt::Debug;

use crate::utils::volume;

use crate::cl::{cl_get_info5, ClObject, ClPointer};

use super::flags::CommandQueueInfo;
use super::CommandQueueOptions;
use crate::event::{Event, WaitList};
use crate::ffi::{
    clCreateCommandQueue, clEnqueueNDRangeKernel, clEnqueueReadBuffer, clEnqueueWriteBuffer,
    clFinish, clGetCommandQueueInfo, cl_bool, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_event, cl_int, cl_kernel
};
use crate::utils::StatusCode;
use crate::{Context, Device, DeviceMem, Output};
use opencl_low_level::{DevicePtr, DeviceRefCount};

__release_retain!(command_queue, CommandQueue);

pub unsafe fn cl_create_command_queue(
    context: &Context,
    device: &Device,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    device.usability_check()?;
    let mut err_code = 0;
    let command_queue = clCreateCommandQueue(
        context.context_ptr(),
        device.device_ptr(),
        flags,
        &mut err_code,
    );
    StatusCode::build_output(err_code, command_queue)
}

pub unsafe fn cl_finish(command_queue: cl_command_queue) -> Output<()> {
    StatusCode::build_output(clFinish(command_queue), ())
}

pub unsafe fn cl_enqueue_nd_range_kernel(
    queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: u8,
    global_work_offset: Option<[usize; 3]>,
    global_work_size: [usize; 3],
    local_work_size: Option<[usize; 3]>,
    wait_list: WaitList,
) -> Output<Event> {
    let mut tracking_event: cl_event = new_tracking_event();

    let (wait_list_len, wait_list_ptr_ptr) = wait_list.len_and_ptr_ptr();

    let global_work_offset_ptr = volume::option_to_ptr(global_work_offset);
    let global_work_size_ptr = volume::to_ptr(global_work_size);
    let local_work_size_ptr = volume::option_to_ptr(local_work_size);

    let err_code = clEnqueueNDRangeKernel(
        queue,
        kernel,
        u32::from(work_dim),
        global_work_offset_ptr,
        global_work_size_ptr,
        local_work_size_ptr,
        wait_list_len,
        wait_list_ptr_ptr,
        &mut tracking_event,
    );

    StatusCode::build_output(err_code, ())?;

    cl_finish(queue)?;

    // TODO: Remove this check when Event checks for null pointer
    debug_assert!(!tracking_event.is_null());
    Event::new(tracking_event)
}

fn buffer_mem_size_and_ptr<T>(buf: &[T]) -> (usize, *const libc::c_void) {
    (
        std::mem::size_of::<T>() * buf.len(),
        buf.as_ptr() as *const libc::c_void,
    )
}

fn new_tracking_event() -> cl_event {
    std::ptr::null_mut() as cl_event
}

#[inline]
fn into_event(err_code: cl_int, tracking_event: cl_event) -> Output<Event> {
    StatusCode::build_output(err_code, ())?;
    unsafe { Event::new(tracking_event) }
}

pub unsafe fn cl_enqueue_read_buffer<T>(
    queue: cl_command_queue,
    device_mem: &DeviceMem<T>,
    buffer: &mut [T],
    command_queue_opts: CommandQueueOptions,
) -> Output<Event> where T: Debug + Sync + Send {
    let mut tracking_event = new_tracking_event();

    let (wait_list_len, wait_list_ptr_ptr) = command_queue_opts.wait_list.len_and_ptr_ptr();

    let (buffer_mem_size, buffer_ptr) = buffer_mem_size_and_ptr(buffer);

    // TODO: Make this a Error returning check
    debug_assert!(buffer.len() == device_mem.len());

    let err_code = clEnqueueReadBuffer(
        queue,
        device_mem.raw_cl_object(),
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_mem_size,
        buffer_ptr as *mut libc::c_void,
        wait_list_len,
        wait_list_ptr_ptr,
        &mut tracking_event,
    );
    into_event(err_code, tracking_event)
}

pub unsafe fn cl_enqueue_write_buffer<T>(
    queue: cl_command_queue,
    device_mem: &DeviceMem<T>,
    buffer: &[T],
    command_queue_opts: CommandQueueOptions,
) -> Output<Event> where T: Debug + Sync + Send {
    let mut tracking_event = new_tracking_event();
    
    let (wait_list_len, wait_list_ptr_ptr) = command_queue_opts.wait_list.len_and_ptr_ptr();

    let (buffer_mem_size, buffer_ptr) = buffer_mem_size_and_ptr(buffer);
        
    let err_code = clEnqueueWriteBuffer(
        queue,
        device_mem.raw_cl_object(),
        command_queue_opts.is_blocking as cl_bool,
        command_queue_opts.offset,
        buffer_mem_size,
        buffer_ptr,
        wait_list_len,
        wait_list_ptr_ptr,
        &mut tracking_event,
    );

    into_event(err_code, tracking_event)
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
