/// Info helper functions. These are unsafe because calling an `info` function
/// with a cl_command_queue that has reached a reference_count of 0 is undefined
/// behavior.

use super::low_level;
use super::flags::CommandQueueInfo as CQInfo;
use super::flags::CommandQueueProperties as CQProps;
use crate::cl::ClPointer;
use crate::ffi::{cl_command_queue};
use crate::{Output, Context, ContextRefCount, Device, DeviceRefCount};

#[inline]
pub unsafe fn fetch<T: Copy>(cq: cl_command_queue, flag: CQInfo) -> Output<ClPointer<T>> {
    low_level::cl_get_command_queue_info(cq, flag)
}

#[inline]
pub unsafe fn load_context(cq: cl_command_queue) -> Output<Context> {
    fetch(cq, CQInfo::Context).and_then(|cl_ptr| Context::from_unretained(cl_ptr.into_one()) )
}

#[inline]
pub unsafe fn load_device(cq: cl_command_queue) -> Output<Device> {
    fetch(cq, CQInfo::Device).and_then(|ret| Device::from_unretained(ret.into_one()) )
}

#[inline]
pub unsafe fn reference_count(cq: cl_command_queue) -> Output<u32> {
    fetch(cq, CQInfo::ReferenceCount).map(|ret| ret.into_one())
}

#[inline]
pub unsafe fn properties(cq: cl_command_queue) -> Output<CQProps> {
    fetch(cq, CQInfo::Properties).map(|ret| ret.into_one() )
}

