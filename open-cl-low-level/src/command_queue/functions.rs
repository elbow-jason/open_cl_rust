use super::{functions, CommandQueueOptions};
use crate::cl::{
    clCreateCommandQueue, clEnqueueNDRangeKernel, clEnqueueReadBuffer, clEnqueueWriteBuffer,
    clFinish, clGetCommandQueueInfo, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_context, cl_device_id, cl_event, cl_kernel, cl_mem, ClObject,
    CommandQueueInfo, StatusCodeError,
};

use crate::{BufferBuilder, GlobalWorkSize, LocalWorkSize, Number, Output, Waitlist, Work};
use libc::c_void;

/// Creates a new cl_command_queue.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn create_command_queue(
    mut context: cl_context,
    mut device: cl_device_id,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    let mut status_code = 0;
    let command_queue = clCreateCommandQueue(
        context.as_mut_ptr(),
        device.as_mut_ptr(),
        flags,
        &mut status_code,
    );
    StatusCodeError::check(status_code)?;
    cl_command_queue::new(command_queue)
}

/// Blocks until all previously queued tasks are finished.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn finish(mut command_queue: cl_command_queue) -> Output<()> {
    StatusCodeError::check(clFinish(command_queue.as_mut_ptr()))
}

/// Queues an n-dimensionally ranged kernel to be executed.
///
/// Blocks until the kernel is finished.
///
/// # Safety
/// Usage of an invalid ClObject is undefined behavior.
pub unsafe fn enqueue_kernel<W: Waitlist>(
    mut queue: cl_command_queue,
    mut kernel: cl_kernel,
    work: &Work,
    waitlist: W,
) -> Output<cl_event> {
    let mut tracking_event: *mut c_void = std::ptr::null_mut();
    let event_waitlist = waitlist.new_waitlist();
    let wl = event_waitlist.as_slice();

    let gws: GlobalWorkSize = work.global_work_size()?;
    let lws: LocalWorkSize = work.local_work_size()?;
    let status_code = clEnqueueNDRangeKernel(
        queue.as_mut_ptr(),
        kernel.as_mut_ptr(),
        work.work_dims(),
        work.global_work_offset().as_ptr(),
        gws.as_ptr(),
        lws.as_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );

    StatusCodeError::check(status_code)?;
    functions::finish(queue)?;

    // TODO: Remove this check when Event checks for null pointer

    cl_event::new(tracking_event)
}

pub unsafe fn enqueue_read_buffer<T>(
    mut queue: cl_command_queue,
    mut mem: cl_mem,
    buffer: &mut [T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event>
where
    T: Number,
{
    let mut tracking_event = std::ptr::null_mut();
    let waitlist = command_queue_opts.new_waitlist();
    let wl = waitlist.as_slice();

    // TODO: Make this a Error returning check
    // debug_assert!(buffer.len() == device_mem.len());

    let status_code = clEnqueueReadBuffer(
        queue.as_mut_ptr(),
        mem.as_mut_ptr(),
        command_queue_opts.is_blocking as u32,
        command_queue_opts.offset,
        buffer.buffer_len() * std::mem::size_of::<T>(),
        buffer.buffer_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );

    StatusCodeError::check(status_code)?;
    cl_event::new(tracking_event)
}

pub unsafe fn enqueue_write_buffer<T>(
    mut queue: cl_command_queue,
    mut mem: cl_mem,
    buffer: &[T],
    command_queue_opts: CommandQueueOptions,
) -> Output<cl_event>
where
    T: Number,
{
    let mut tracking_event = std::ptr::null_mut();

    let waitlist = command_queue_opts.new_waitlist();
    let wl = waitlist.as_slice();
    let status_code = clEnqueueWriteBuffer(
        queue.as_mut_ptr(),
        mem.as_mut_ptr(),
        command_queue_opts.is_blocking as u32,
        command_queue_opts.offset,
        buffer.buffer_len() * std::mem::size_of::<T>(),
        buffer.buffer_ptr(),
        wl.waitlist_len(),
        wl.waitlist_ptr(),
        &mut tracking_event,
    );
    StatusCodeError::check(status_code)?;
    cl_event::new(tracking_event)
}

#[inline(always)]
pub unsafe fn get_context(cq: cl_command_queue) -> Output<cl_context> {
    cl_get_info!(
        One,
        cl_context,
        clGetCommandQueueInfo,
        cq,
        CommandQueueInfo::Context.into()
    )
}

#[inline(always)]
pub unsafe fn get_info_u32(cq: cl_command_queue, flag: cl_command_queue_info) -> Output<u32> {
    cl_get_info!(One, u32, clGetCommandQueueInfo, cq, flag)
}

#[inline(always)]
pub unsafe fn get_info_u64(cq: cl_command_queue, flag: cl_command_queue_info) -> Output<u64> {
    cl_get_info!(One, u64, clGetCommandQueueInfo, cq, flag)
}

#[inline(always)]
pub unsafe fn get_device(cq: cl_command_queue) -> Output<cl_device_id> {
    cl_get_info!(
        One,
        cl_device_id,
        clGetCommandQueueInfo,
        cq,
        CommandQueueInfo::Device.into()
    )
}
