/// The functions in this module MUST NOT RETURN cl_object pointers that
/// are not wrapped in Drop implementing wrapper structs. Exposing the
/// caller to any raw pointer completely negates the point of using Rust
/// as the language to interface with OpenCL. 

/// API Spec https://www.khronos.org/registry/OpenCL/specs/opencl-2.0.pdf

use std::fmt;
use std::fmt::Debug;

use std::iter::repeat;
use std::ptr;
use std::sync::Mutex;

use libc;

pub mod host_buffer;
pub mod status_code;
pub mod volume;
mod wait_list;
mod event_helpers;
pub mod cl_object;
pub mod events;


use crate::ffi::*;
// 
// pub use host_buffer::{Buffer, Vectorable, VectorBuffer, BufferBuilder, BufferOpConfig};
pub use host_buffer::BufferOpConfig;
pub use status_code::StatusCode;
pub use volume::Volume;
pub use cl_object::{ClObject, CopyClObject, MutClObject};
pub use events::*;

use crate::{
    WaitList, Event, Context, Device, Program, Platform,
};

use crate::{
    KernelArgSizeAndPointer,
    KernelError,
    EventError,
    CommandQueue,
    DeviceError,
    Kernel,
    KernelArg,
    DeviceMem,
};

// TODO: move this to the app level, move contextual errors (e.g. EmptyBitfield)
// 
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum Error {
    KernelError(KernelError),
    EventError(EventError),
    DeviceError(DeviceError),
    StatusCode(i32),
    EmptyBitfield,
    CStringInvalidSourceCode,
    CStringInvalidProgramBinary,
    CStringInvalidKernelName,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StatusCode(err_code) => {
                let status = StatusCode::from(*err_code as cl_int);
                write!(f, "Error::({:?})", status)
            },
            _ => write!(f, "{:?}", self)
        }
    }
}

pub type Output<T> = Result<T, Error>;

mod strings {
    use std::ffi::CString;
    use super::{Error, Output};

    pub fn to_c_string(string: &str, error: Error) -> Output<CString> {
        CString::new(string).or_else(|_| Err(error))
    }

    pub fn to_utf8_string(buf: Vec<u8>) -> String {
        let safe_vec = buf.into_iter().filter(|c| *c != 0u8).collect();
        String::from_utf8(safe_vec).unwrap()

    }

}


// This mutex is used to work around weak OpenCL implementations.
// On some implementations concurrent calls to clGetPlatformIDs
// will cause the implantation to return invalid status.
lazy_static! {
    pub static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

fn with_platform_access<T, F: FnOnce() -> T>(op: F) -> T {
    let platform_lock = PLATFORM_ACCESS.lock();
    let output = op();
    std::mem::drop(platform_lock);
    output
}

#[inline]
fn into_result<T>(err_code: cl_int, result: T) -> Output<T> {
    match StatusCode::from(err_code) {
        StatusCode::Success => Ok(result),
        StatusCode::Failure(not_success) => Err(Error::StatusCode(not_success)),
    }
}

pub fn cl_get_platforms_count() -> Output<u32> {
    let mut num_platforms: cl_uint = 0;
    let err_code = with_platform_access(|| unsafe {
        clGetPlatformIDs(0, ptr::null_mut(), &mut num_platforms)
    });
    into_result(err_code, num_platforms)
}

pub fn cl_get_platforms_ids() -> Output<Vec<cl_platform_id>> {
    let mut num_platforms: cl_uint = 0;
    // transactional access to the platform Mutex
    with_platform_access(|| {
        let e1 = unsafe { clGetPlatformIDs(0, ptr::null_mut(), &mut num_platforms) };
        num_platforms = into_result(e1, num_platforms)?;

        let mut ids: Vec<cl_platform_id> = repeat(0 as cl_platform_id)
            .take(num_platforms as usize)
            .collect();

        let e2 = unsafe { clGetPlatformIDs(num_platforms, ids.as_mut_ptr(), &mut num_platforms) };
        into_result(e2, ids)
    })
}

pub fn cl_get_platform_info(
    platform: &Platform,
    platform_info: cl_platform_info,
) -> Output<String> {
    let mut size = 0 as libc::size_t;
    let mut err_code = unsafe {
        clGetPlatformInfo(
            platform.raw_cl_object(),
            platform_info,
            0,
            ptr::null_mut(),
            &mut size
        )
    };
    size = into_result(err_code, size)?;
    let mut buf: Vec<u8> = repeat(0u8).take(size as usize).collect();
    err_code = unsafe {
        clGetPlatformInfo(
            platform.raw_cl_object(),
            platform_info,
            size,
            buf.as_mut_ptr() as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    buf = into_result(err_code, buf)?;
    let info = unsafe { String::from_utf8_unchecked(buf) };
    Ok(info)
}

pub fn cl_get_device_count(
    platform: &Platform,
    device_type: cl_device_type,
) -> Output<u32> {
    let mut num_devices = 0;
    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type,
            0,
            ptr::null_mut(),
            &mut num_devices,
        )
    };
    into_result(err_code, num_devices)
}

pub fn cl_get_device_ids(
    platform: &Platform,
    device_type: cl_device_type,
) -> Output<Vec<cl_device_id>> {
    let mut num_devices: u32 = cl_get_device_count(platform, device_type)?;
    let mut ids: Vec<cl_device_id> = repeat(0 as cl_device_id)
        .take(num_devices as usize)
        .collect();

    let err_code = unsafe {
        clGetDeviceIDs(
            platform.raw_cl_object(),
            device_type,
            ids.len() as cl_uint,
            ids.as_mut_ptr(),
            &mut num_devices,
        )
    };
    into_result(err_code, ids)
}

pub fn cl_get_device_info(device: &Device, device_info: cl_device_info) -> Output<String> {
    device.usability_check()?;
    let mut size = 0 as libc::size_t;
    let err_code =
        unsafe { clGetDeviceInfo(
            device.raw_cl_object(),
            device_info,
            0,
            ptr::null_mut(),
            &mut size
        )
    };
    size = into_result(err_code, size)?;
    let mut buf: Vec<u8> = repeat(0u8).take(size as usize).collect();
    device.usability_check()?;
    let err_code = unsafe {
        clGetDeviceInfo(
            device.raw_cl_object(),
            device_info,
            size,
            buf.as_mut_ptr() as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    buf = into_result(err_code, buf)?;
    Ok(strings::to_utf8_string(buf))
}

pub fn cl_create_context(device: &Device) -> Output<cl_context> {
    // println!("Starting cl_create_context with device {:?}", device);
    device.usability_check()?;
    let mut err_code = 0;
    let ctx = unsafe {
        clCreateContext(
            ptr::null(),
            1,
            &device.raw_cl_object() as *const cl_device_id,
            std::mem::transmute(ptr::null::<fn()>()),
            ptr::null_mut(),
            &mut err_code,
        )
    };
    into_result(err_code, ctx)
}

pub fn cl_create_buffer<T>(
    context: &Context,
    len: usize,
    mem_flags: cl_mem_flags,
) -> Output<cl_mem> {
    let mut err_code = 0;
    let mut buf = unsafe {
        clCreateBuffer(
            context.raw_cl_object(),
            mem_flags,
            (len * std::mem::size_of::<T>()) as libc::size_t,
            ptr::null_mut(),
            &mut err_code,
        )
    };
    buf = into_result(err_code, buf)?;
    Ok(buf)
}

pub fn cl_get_mem_object_info<T>(
    device_mem: &DeviceMem<T>,
    mem_info_flag: cl_mem_info
) -> Output<usize>
    where T: Debug,

{
    let mut size: libc::size_t = 0;
    let err_code = unsafe {
        clGetMemObjectInfo(
            device_mem.raw_cl_object(),
            mem_info_flag,
            std::mem::size_of::<libc::size_t>() as libc::size_t,
            (&mut size as *mut libc::size_t) as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    into_result(err_code, size as usize)
}

pub fn cl_set_kernel_arg<T>(
        kernel: &Kernel,
        arg_index: usize,
        arg: &T,
    ) -> Output<()> where T: KernelArg + Debug {
    
        let err_code = unsafe {
            let (arg_size, arg_ptr): KernelArgSizeAndPointer = arg.as_kernel_arg();
            debug_assert!(!kernel.raw_cl_object().is_null());
            println!("
            clSetKernelArg starting call...
            arg: {:?}
            arg_size: {:?}
            arg_ptr: {:?}
            arg_ptr_deref: {:?}
            ",
            arg,
            arg_size,
            arg_ptr,
            *arg_ptr,
            );
            clSetKernelArg(
                kernel.raw_cl_object(),
                arg_index as cl_uint,
                arg_size,
                arg_ptr,
            )
        };
        println!("call to clSetKernelArg succeeded without crashing {:?}", err_code);
        into_result(err_code, ())
}

pub fn cl_create_kernel(program: &Program, name: &str) -> Output<cl_kernel> {
    let mut err_code = 0;
    let c_name = strings::to_c_string(name, Error::CStringInvalidKernelName)?;
    let kernel = unsafe {
        clCreateKernel(
            program.raw_cl_object(),
            c_name.as_ptr(),
            &mut err_code
        )
    };
    into_result(err_code, kernel)
}

pub fn cl_build_program(program: &Program, devices: &[&Device]) -> Output<()> {
    let err_code = unsafe {
        // We'll see...
        let mut cl_devices: Vec<cl_device_id> = devices.iter().map(|d| d.raw_cl_object()).collect();

        clBuildProgram(
            program.raw_cl_object(),
            cl_devices.len() as cl_uint,
            cl_devices.as_mut_ptr(),
            ptr::null(),
            std::mem::transmute(ptr::null::<fn()>()), // pfn_notify
            ptr::null_mut(),                          // user_data
        )
    };
    into_result(err_code, ())
}

pub fn cl_get_program_build_log(
    program: &Program,
    device: &Device,
    build_info_type: cl_program_build_info,
) -> Output<String> {
    device.usability_check()?;
    let mut size = 0 as libc::size_t;
    // determine buffer size
    let mut err_code = unsafe {
        clGetProgramBuildInfo(
            program.raw_cl_object(),
            device.raw_cl_object(),
            build_info_type,
            0,
            ptr::null_mut(),
            &mut size,
        )
    };

    // check that the info can be retrieved
    size = into_result(err_code, size)?;

    // make a buffer of the size
    let mut buf: Vec<u8> = vec![0u8; size as usize];
    // get bytes from the device for the last compilation for this program.
    err_code = unsafe {
        clGetProgramBuildInfo(
            program.raw_cl_object(),
            device.raw_cl_object(),
            build_info_type,
            buf.len() as libc::size_t,
            buf.as_mut_ptr() as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    buf = into_result(err_code, buf)?;

    Ok(strings::to_utf8_string(buf))
}



pub fn cl_get_event_profiling_info(event: &cl_event, info: cl_profiling_info) -> Output<u64> {
    let mut time: cl_ulong = 0;
    let err_code = unsafe {
        clGetEventProfilingInfo(
            *event,
            info,
            std::mem::size_of::<cl_ulong>() as libc::size_t,
            (&mut time as *mut u64) as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    into_result(err_code, time as u64)
}


pub fn cl_wait_for_events(event_list: WaitList) -> Output<()> {
    let err_code = unsafe {
        let (
            wait_list_len,
            wait_list_ptr_ptr
        ) = wait_list::len_and_ptr_ptr(event_list.cl_object());
    
        clWaitForEvents(
            wait_list_len,
            wait_list_ptr_ptr,
        )
    };
    into_result(err_code, ())
}

pub fn cl_finish(queue: &CommandQueue) -> Output<()> {
    // println!("cl_finish starting...");
    let out = into_result(unsafe { clFinish(queue.raw_cl_object()) }, ())?;
    // println!("cl_finish completed");
    Ok(out)
}

/// global_work_size[0] * ... * global_work_size[work_dim–1].
pub fn cl_enqueue_nd_range_kernel(
    queue: &CommandQueue,
    kernel: &Kernel,
    work_dim: u8,
    global_work_offset: Option<[usize; 3]>,
    global_work_size: [usize; 3],
    local_work_size: Option<[usize; 3]>,
    event_list: WaitList,

) -> Output<Event> {
    let mut tracking_event: cl_event = new_tracking_event();
    let err_code = unsafe {
        let (
            wait_list_len,
            wait_list_ptr_ptr
        ) = wait_list::len_and_ptr_ptr(event_list.cl_object());

        let global_work_offset_ptr = volume::option_to_ptr(global_work_offset);
        let global_work_size_ptr = volume::to_ptr(global_work_size);
        let local_work_size_ptr = volume::option_to_ptr(local_work_size);
        // println!(" clEnqueueNDRangeKernel called with
        // queue: {:?}
        // kernel: {:?}
        // work_dim: {:?}
        // global_work_offset: {:?}
        // global_work_offset_ptr: {:?}
        // global_work_size: {:?}
        // global_work_size_ptr: {:?}
        // local_work_size: {:?}
        // local_work_size_ptr: {:?}
        // event_list: {:?}
        // wait_list_len: {:?}
        // wait_list_ptr_ptr: {:?}
        // tracking_event: {:?}
        // ",
        // queue,
        // kernel,
        // work_dim,
        // global_work_offset,
        // global_work_offset_ptr,
        // global_work_size,
        // global_work_size_ptr,
        // local_work_size,
        // local_work_size_ptr,
        // event_list,
        // wait_list_len,
        // wait_list_ptr_ptr,
        // tracking_event,
        // );
        clEnqueueNDRangeKernel(
            queue.raw_cl_object(),
            kernel.raw_cl_object(),
            work_dim as cl_uint,
            global_work_offset_ptr,
            global_work_size_ptr,
            local_work_size_ptr,
            wait_list_len,
            wait_list_ptr_ptr,
            &mut tracking_event,
        )
    };

    // println!("got past clEnqueueNDRangeKernel");

    let () = into_result(err_code, ())?;
    let () = cl_finish(queue)?;
    Ok(Event::new(tracking_event))    
}

fn buffer_mem_size_and_ptr<T>(buf: &[T]) -> (usize, *const libc::c_void) {
    (std::mem::size_of::<T>() * buf.len(), buf.as_ptr() as *const libc::c_void)
}


fn new_tracking_event() -> cl_event {
    std::ptr::null_mut() as cl_event
}

#[inline]
fn into_event(err_code: cl_int, tracking_event: cl_event) -> Output<Event> {
    let () = into_result(err_code, ())?;
    Ok(Event::new(tracking_event))
}


// pub fn cl_enqueue_read_buffer<T: Num>(
//         command_queue: &CommandQueue,
//         device_mem: &DeviceMem<T>,
//         buffer: &mut [T],
//         buffer_op_config: BufferOpConfig,
//         event_list: WaitList,
//     ) -> Output<Event> where
//         T: Sized + Debug,
//     {
//         let mut tracking_event = new_tracking_event();
//         let err_code = unsafe {
//             let (
//                 wait_list_len,
//                 wait_list_ptr_ptr
//             ) = wait_list::len_and_ptr_ptr(event_list.cl_object());
//             let (
//                 buffer_mem_size,
//                 buffer_ptr,
//             ) = buffer_mem_size_and_ptr(buffer);
            
//             println!("calling clEnqueueReadBuffer...
//             buffer {:?}
//             buffer_mem_size {:?}
//             buffer_ptr {:?}
//             ",
//             buffer,
//             buffer_mem_size,
//             buffer_ptr,
//             );
//             let err_code = clEnqueueReadBuffer(
//                 command_queue.raw_cl_object(),
//                 device_mem.raw_cl_object(),
//                 // buffer_op_config.is_blocking as cl_bool,
//                 1 as cl_bool,
//                 buffer_op_config.offset,
//                 buffer_mem_size,
//                 buffer_ptr as *mut libc::c_void,
//                 wait_list_len,
//                 wait_list_ptr_ptr,
//                 &mut tracking_event,
//             );
//             println!("called clEnqueueReadBuffer without crashing");
//             err_code
//         };
//         into_event(err_code, tracking_event)
// }

pub fn cl_enqueue_read_buffer<T>(
        queue: &CommandQueue,
        device_mem: &DeviceMem<T>,
        buffer: &mut [T],
        buffer_op_config: BufferOpConfig,
        event_list: WaitList,
    ) -> Output<Event> where T: Debug {
        let mut tracking_event = new_tracking_event();
        let err_code = unsafe {
            let (
                wait_list_len,
                wait_list_ptr_ptr
            ) = wait_list::len_and_ptr_ptr(event_list.cl_object());

            let (
                buffer_mem_size,
                buffer_ptr,
            ) = buffer_mem_size_and_ptr(buffer);
            debug_assert!(buffer.len() == device_mem.len().unwrap());
            println!(" clEnqueueReadBuffer called with
            buffer: {:?}
            ",
            buffer,
            );
            // queue: {:?}
            // device_mem: {:?}
            // event_list: {:?}
            // wait_list_len: {:?}
            // wait_list_ptr_ptr: {:?}
            // buffer_mem_size: {:?}
            // buffer_ptr {:?}
            // tracking_event: {:?}
            // ",
            // queue,
            // device_mem,
            // event_list,
            // wait_list_len,
            // wait_list_ptr_ptr,
            // buffer_mem_size,
            // buffer_ptr,
            // tracking_event,
            // );
            let err = clEnqueueReadBuffer(
                queue.raw_cl_object(),
                device_mem.raw_cl_object(),
                buffer_op_config.is_blocking as cl_bool,
                buffer_op_config.offset,
                buffer_mem_size,
                buffer_ptr as *mut libc::c_void,
                wait_list_len,
                wait_list_ptr_ptr,
                &mut tracking_event,
            );

            println!("clEnqueueReadBuffer after...
            buffer: {:?}
            ",
            buffer,
            );
            // println!("NEW VERSION clEnqueueReadBuffer called without crashing...");
            err
        };
        into_event(err_code, tracking_event)
}

pub fn cl_enqueue_write_buffer<T>(
        command_queue: &CommandQueue,
        device_mem: &DeviceMem<T>,
        buffer: &[T],
        buffer_op_config: BufferOpConfig,
        event_list: WaitList,
    ) -> Output<Event> where T: Debug {
        let mut tracking_event = new_tracking_event();
        let err_code = unsafe {
            let (
                wait_list_len,
                wait_list_ptr_ptr
            ) = wait_list::len_and_ptr_ptr(event_list.cl_object());

            let (
                buffer_mem_size,
                buffer_ptr,
            ) = buffer_mem_size_and_ptr(buffer);
            println!("clEnqueueWriteBuffer...");
            let err = clEnqueueWriteBuffer(
                command_queue.raw_cl_object(),
                device_mem.raw_cl_object(),
                buffer_op_config.is_blocking as cl_bool,
                buffer_op_config.offset,
                buffer_mem_size,
                buffer_ptr,
                // (buffer.len() * std::mem::size_of::<T>()) as libc::size_t,
                // buffer.as_ptr() as *mut libc::c_void,
                wait_list_len,
                wait_list_ptr_ptr,
                &mut tracking_event,
            );
            // println!("clEnqueueWriteBuffer called without crashing...");
            err
        };
        into_event(err_code, tracking_event)
}

pub fn cl_create_program_with_source(context: &Context, src: &str) -> Output<Program> {
    let src = strings::to_c_string(src, Error::CStringInvalidSourceCode)?;
    let mut src_list = vec![src.as_ptr()];

    let mut err_code = 0;
    let program: cl_program = unsafe {
        clCreateProgramWithSource(
            context.raw_cl_object(),
            // the count that _literally_ has no description in the docs.
            1,
            // const char **strings
            // mut pointer to const pointer of char. Great.
            src_list.as_mut_ptr() as *mut *const libc::c_char,
            // null pointer here indicates that all strings in the src
            // are NULL-terminated.
            ptr::null(),
            &mut err_code,
        )
    };
    into_result(err_code, Program::new(program))
}

pub fn cl_create_program_with_binary(
    context: &Context,
    device: &Device,
    binary: &str,
) -> Output<Program> {
    device.usability_check()?;
    let src = strings::to_c_string(binary, Error::CStringInvalidProgramBinary)?;
    let mut err_code = 0;
    let program = unsafe {
        clCreateProgramWithBinary(
            context.raw_cl_object(),
            1,
            device.raw_cl_object() as *const cl_device_id,
            binary.len() as *const libc::size_t,
            src.as_ptr() as *mut *const u8,
            ptr::null_mut(),
            &mut err_code,
        )
    };
    into_result(err_code, Program::new(program))
}

pub fn cl_create_command_queue(
    context: &Context,
    device: &Device,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    device.usability_check()?;
    let mut err_code = 0;
    let command_queue = unsafe {
        clCreateCommandQueue(
            context.raw_cl_object(),
            device.raw_cl_object(),
            flags,
            &mut err_code
        )
    };
    into_result(err_code, command_queue)
}




// all cl_release_* and cl_retain_* functions take a raw reference to the
// cl object they pertain to.

macro_rules! release_retain {
    ($snake:ident, $pascal:ident) => {
        paste::item! {
            pub unsafe fn [<cl_release_ $snake>](cl_obj: &[<cl_ $snake>]) {
                let status = [<clRelease $pascal>](*cl_obj);
                if let Err(e) = into_result(status, ()) {
                    panic!(
                        "Failed to release {} OpenCL object {:?} due to {:?}",
                        stringify!($snake),
                        cl_obj,
                        e
                    );
                }
            }

            pub unsafe fn [<cl_retain_ $snake>](cl_obj: &[<cl_ $snake>]) {
                let status = [<clRetain $pascal>](*cl_obj);
                if let Err(e) = into_result(status, ()) {
                    panic!(
                        "Failed to retain {} OpenCL object {:?} due to {:?}",
                        stringify!($snake),
                        cl_obj,
                        e
                    );
                }
            }
        }
    }
}

release_retain!(command_queue, CommandQueue);
release_retain!(event, Event);
release_retain!(program, Program);
release_retain!(context, Context);
release_retain!(mem, MemObject);
release_retain!(kernel, Kernel);



#[test]
fn test_cl_get_platforms_count() {
    let count = cl_get_platforms_count()
        .map_err(|e| panic!("get_platform_count failed with {:?}", e))
        .unwrap();
    assert!(count > 0);
}

#[test]
fn test_cl_get_platforms_ids() {
    // use types::PlatformID;
    let platform_ids_result: Output<Vec<cl_platform_id>> = cl_get_platforms_ids();
    assert!(platform_ids_result.is_ok());
    let platform_ids = platform_ids_result.unwrap();
    assert!(platform_ids.len() > 0);
}
