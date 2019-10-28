/// API Spec https://www.khronos.org/registry/OpenCL/specs/opencl-2.0.pdf
use std::fmt;

use std::iter::repeat;
use std::ptr;
use std::sync::Mutex;

use libc;

pub mod ffi;
pub mod mem;
pub mod status_code;
pub mod volume;
mod wait_list;
mod event_helpers;
pub mod cl_object;

pub use ffi::*;
pub use mem::*;
pub use status_code::StatusCode;
pub use volume::Volume;
pub use cl_object::ClObject;

use crate::{EventList, ClEvent};

use crate::{KernelError, EventError, CommandQueue};

// TODO: move this to the app level, move contextual errors (e.g. EmptyBitfield)
// 
#[derive(Debug, Fail, PartialEq, Clone)]
pub enum Error {
    KernelError(KernelError),
    EventError(EventError),
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

// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Error::StatusCode(err_code) => {
//                 let status = StatusCode::from(*err_code as cl_int);
//                 write!(f, "Error::({:?})", status)
//             },
//             _ => write!(f, "{:?}", self)
//         }
//     }
// }

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
    platform_id: &cl_platform_id,
    platform_info: cl_platform_info,
) -> Output<String> {
    let mut size = 0 as libc::size_t;
    let mut err_code =
        unsafe { clGetPlatformInfo(*platform_id, platform_info, 0, ptr::null_mut(), &mut size) };
    size = into_result(err_code, size)?;
    let mut buf: Vec<u8> = repeat(0u8).take(size as usize).collect();
    err_code = unsafe {
        clGetPlatformInfo(
            *platform_id,
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
    platform_id: &cl_platform_id,
    device_type: cl_device_type,
) -> Output<u32> {
    let mut num_devices = 0;
    let err_code = unsafe {
        clGetDeviceIDs(
            *platform_id,
            device_type,
            0,
            ptr::null_mut(),
            &mut num_devices,
        )
    };
    into_result(err_code, num_devices)
}

pub fn cl_get_device_ids(
    platform_id: &cl_platform_id,
    device_type: cl_device_type,
) -> Output<Vec<cl_device_id>> {
    let mut num_devices: u32 = cl_get_device_count(platform_id, device_type)?;
    let mut ids: Vec<cl_device_id> = repeat(0 as cl_device_id)
        .take(num_devices as usize)
        .collect();

    let err_code = unsafe {
        clGetDeviceIDs(
            *platform_id,
            device_type,
            ids.len() as cl_uint,
            ids.as_mut_ptr(),
            &mut num_devices,
        )
    };
    into_result(err_code, ids)
}

pub fn cl_get_device_info(device_id: &cl_device_id, device_info: cl_device_info) -> Output<String> {
    let mut size = 0 as libc::size_t;

    let err_code =
        unsafe { clGetDeviceInfo(*device_id, device_info, 0, ptr::null_mut(), &mut size) };
    size = into_result(err_code, size)?;
    let mut buf: Vec<u8> = repeat(0u8).take(size as usize).collect();

    let err_code = unsafe {
        clGetDeviceInfo(
            *device_id,
            device_info,
            size,
            buf.as_mut_ptr() as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    buf = into_result(err_code, buf)?;
    Ok(strings::to_utf8_string(buf))
}

pub fn cl_create_context(device_id: &cl_device_id) -> Output<cl_context> {
    let mut err_code = 0;
    let ctx = unsafe {
        clCreateContext(
            ptr::null(),
            1,
            device_id,
            std::mem::transmute(ptr::null::<fn()>()),
            ptr::null_mut(),
            &mut err_code,
        )
    };
    into_result(err_code, ctx)
}

pub fn cl_create_buffer<T>(
    context: &cl_context,
    len: usize,
    mem_flags: cl_mem_flags,
) -> Output<cl_mem> {
    let mut err_code = 0;
    let mut buf = unsafe {
        clCreateBuffer(
            *context,
            mem_flags,
            (len * std::mem::size_of::<T>()) as libc::size_t,
            ptr::null_mut(),
            &mut err_code,
        )
    };
    buf = into_result(err_code, buf)?;
    Ok(buf)
}

pub fn cl_get_mem_object_info(mem_id: &cl_mem, mem_info_flag: cl_mem_info) -> Output<usize> {
    let mut size: libc::size_t = 0;
    let err_code = unsafe {
        clGetMemObjectInfo(
            *mem_id,
            mem_info_flag,
            std::mem::size_of::<libc::size_t>() as libc::size_t,
            (&mut size as *mut libc::size_t) as *mut libc::c_void,
            ptr::null_mut(),
        )
    };
    into_result(err_code, size as usize)
}





pub fn cl_set_kernel_arg<T>(
    kernel: cl_kernel,
    arg_index: usize,
    arg_size: usize,
    arg_ptr: *const T,
) -> Output<()> {
    into_result(
        unsafe { clSetKernelArg(
            kernel,
            arg_index as cl_uint,
            arg_size as libc::size_t,
            arg_ptr as *const libc::c_void
        ) },
        (),
    )
}

pub fn cl_create_kernel(program: &cl_program, name: &str) -> Output<cl_kernel> {
    let mut err_code = 0;
    let c_name = strings::to_c_string(name, Error::CStringInvalidKernelName)?;
    let kernel = unsafe { clCreateKernel(*program, c_name.as_ptr(), &mut err_code) };
    into_result(err_code, kernel)
}

pub fn cl_build_program(program: &cl_program, devices: &mut [cl_device_id]) -> Output<()> {
    let err_code = unsafe {
        clBuildProgram(
            *program,
            devices.len() as cl_uint,
            devices.as_mut_ptr(),
            ptr::null(),
            std::mem::transmute(ptr::null::<fn()>()), // pfn_notify
            ptr::null_mut(),                          // user_data
        )
    };
    into_result(err_code, ())
}

pub fn cl_get_program_build_log(
    program: &cl_program,
    device_id: &cl_device_id,
    build_info_type: cl_program_build_info,
) -> Output<String> {
    let mut size = 0 as libc::size_t;
    // determine buffer size
    let mut err_code = unsafe {
        clGetProgramBuildInfo(
            *program,
            *device_id,
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
            *program,
            *device_id,
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


pub fn cl_wait_for_events(event_list: EventList) -> Output<()> {
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
    println!("cl_finish starting...");
    let out = into_result(unsafe { clFinish(queue.raw_cl_object()) }, ())?;
    println!("cl_finish completed");
    Ok(out)
}

/// global_work_size[0] * ... * global_work_size[work_dim–1].
pub fn cl_enqueue_nd_range_kernel(
    queue: &CommandQueue,
    kernel: &cl_kernel,
    work_dim: u8,
    global_work_offset: Option<[usize; 3]>,
    global_work_size: [usize; 3],
    local_work_size: Option<[usize; 3]>,
    event_list: EventList,

) -> Output<Option<ClEvent>> {
    // let mut output_event: *mut cl_event = event_helpers::null_mut_ptr();
    let mut output_event: cl_event = ptr::null_mut();
    let err_code = unsafe {
        let (
            wait_list_len,
            wait_list_ptr_ptr
        ) = wait_list::len_and_ptr_ptr(event_list.cl_object());

        let global_work_offset_ptr = volume::option_to_ptr(global_work_offset);
        let global_work_size_ptr = volume::to_ptr(global_work_size);
        let local_work_size_ptr = volume::option_to_ptr(local_work_size);
        println!(" clEnqueueNDRangeKernel called with
        queue: {:?}
        kernel: {:?}
        work_dim: {:?}
        global_work_offset: {:?}
        global_work_offset_ptr: {:?}
        global_work_size: {:?}
        global_work_size_ptr: {:?}
        local_work_size: {:?}
        local_work_size_ptr: {:?}
        event_list: {:?}
        wait_list_len: {:?}
        wait_list_ptr_ptr: {:?}
        output_event: {:?}
        ",
        queue,
        kernel,
        work_dim,
        global_work_offset,
        global_work_offset_ptr,
        global_work_size,
        global_work_size_ptr,
        local_work_size,
        local_work_size_ptr,
        event_list,
        wait_list_len,
        wait_list_ptr_ptr,
        output_event,
        );
        clEnqueueNDRangeKernel(
            queue.raw_cl_object(),
            *kernel,
            work_dim as cl_uint,
            global_work_offset_ptr,
            global_work_size_ptr,
            local_work_size_ptr,
            wait_list_len,
            wait_list_ptr_ptr,
            &mut output_event,
        )
    };

    println!("got past clEnqueueNDRangeKernel");

    output_event = into_result(err_code, output_event)?;
    let () = cl_finish(queue)?;
    if event_helpers::is_null_mut(&output_event) {
        Ok(None)
    } else {
        let wrapped_event: ClEvent = ClEvent::new(output_event).map_err(|e: EventError| Error::from(e))?;
        Ok(Some(wrapped_event))
    }    
}


pub fn cl_enqueue_read_buffer<T, M>(
        command_queue: &CommandQueue,
        mem_obj: M,
        buffer: &mut HostBuffer<T>,
        is_blocking_read: bool,
        event_list: EventList,
        event_that_wrote: Option<cl_event>,
    ) -> Output<()> where
        T: Sized,
        M: ClObject<cl_mem>
    {
        let err_code = unsafe {
            let (
                wait_list_len,
                wait_list_ptr_ptr
            ) = wait_list::len_and_ptr_ptr(event_list.cl_object());
            clEnqueueReadBuffer(
                command_queue.raw_cl_object(),
                mem_obj.raw_cl_object(),
                is_blocking_read as cl_bool,
                buffer.offset(),
                buffer.mem_size(),
                buffer.as_mut_pointer() as *mut libc::c_void,
                wait_list_len,
                wait_list_ptr_ptr,
                match event_that_wrote {
                    Some(cl_event_obj) => cl_event_obj as *mut cl_event,
                    None => ptr::null_mut(),
                },
            )
        };
        into_result(err_code, ())
}

pub fn cl_enqueue_write_buffer<T, B>(
        command_queue: &CommandQueue,
        mem_id: cl_mem,
        buffer: B,
        is_blocking_write: bool,
        event_list: EventList,
        event_that_wrote: Option<cl_event>,
    ) -> Output<()> where
        B: AsPointer<T> + MemSize<T> + Offset,
    {
        let err_code = unsafe {
            let (
                wait_list_len,
                wait_list_ptr_ptr
            ) = wait_list::len_and_ptr_ptr(event_list.cl_object());
        
            clEnqueueWriteBuffer(
                command_queue.raw_cl_object(),
                mem_id,
                is_blocking_write as cl_bool,
                buffer.offset(),
                buffer.mem_size(),
                buffer.as_pointer() as *mut libc::c_void,
                wait_list_len,
                wait_list_ptr_ptr,
                match event_that_wrote {
                    Some(cl_event_obj) => cl_event_obj as *mut cl_event,
                    None => ptr::null_mut(),
                },
            )
        };
        into_result(err_code, ())
}

pub fn cl_create_program_with_source(context: &cl_context, src: &str) -> Output<cl_program> {
    let src = strings::to_c_string(src, Error::CStringInvalidSourceCode)?;
    let mut src_list = vec![src.as_ptr()];

    let mut err_code = 0;
    let program: cl_program = unsafe {
        clCreateProgramWithSource(
            *context,
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
    into_result(err_code, program)
}

pub fn cl_create_program_with_binary(
    context: &cl_context,
    device: &cl_device_id,
    binary: &str,
) -> Output<cl_program> {
    let src = strings::to_c_string(binary, Error::CStringInvalidProgramBinary)?;
    let mut err_code = 0;
    let program = unsafe {
        clCreateProgramWithBinary(
            *context,
            1,
            *device as *const cl_device_id,
            binary.len() as *const libc::size_t,
            src.as_ptr() as *mut *const u8,
            ptr::null_mut(),
            &mut err_code,
        )
    };
    into_result(err_code, program)
}

pub fn cl_create_command_queue(
    context: &cl_context,
    device: &cl_device_id,
    flags: cl_command_queue_properties,
) -> Output<cl_command_queue> {
    let mut err_code = 0;

    let command_queue = unsafe { clCreateCommandQueue(*context, *device, flags, &mut err_code) };
    into_result(err_code, command_queue)
}

// all cl_release_* and cl_retain_* functions take a raw reference to the
// cl object they pertain to.


pub fn cl_release_command_queue(queue: &cl_command_queue) -> Output<()> {
    into_result(unsafe { clReleaseCommandQueue(*queue) }, ())
}

pub fn cl_retain_command_queue(queue: &cl_command_queue) -> Output<()> {
    into_result(unsafe { clRetainCommandQueue(*queue) }, ())
}

pub fn cl_release_event(event: &cl_event) -> Output<()> {
    into_result(unsafe { clReleaseEvent(*event) }, ())
}

pub fn cl_retain_event(event: &cl_event) -> Output<()> {
    into_result(unsafe { clRetainEvent(*event) }, ())
}

pub fn cl_release_program(program: &cl_program) -> Output<()> {
    into_result(unsafe { clReleaseProgram(*program) }, ())
}

pub fn cl_retain_program(program: &cl_program) -> Output<()> {
    into_result(unsafe { clRetainProgram(*program) }, ())
}

pub fn cl_release_context(context: &cl_context) -> Output<()> {
    into_result(unsafe { clReleaseContext(*context) }, ())
}

pub fn cl_retain_context(context: &cl_context) -> Output<()> {
    into_result(unsafe { clRetainContext(*context) }, ())
}

/// This function decrements the OpenCL memobjreference count.
///
/// After the memobjreference count becomes zero and commands queued for
/// execution on a command-queue(s) that use memobjhave finished, the
/// memory object is deleted.
pub fn cl_release_mem_object(memobj: &cl_mem) -> Output<()> {
    into_result(unsafe { clReleaseMemObject(*memobj) }, ())
}

/// This function increments the memobjreference count.
pub fn cl_retain_mem_object(memobj: &cl_mem) -> Output<()> {
    into_result(unsafe { clRetainMemObject(*memobj) }, ())
}

pub fn cl_release_kernel(kernel: &cl_kernel) -> Output<()> {
    into_result(unsafe { clReleaseKernel(*kernel) }, ())
}

pub fn cl_retain_kernel(kernel: &cl_kernel) -> Output<()> {
    into_result(unsafe { clRetainKernel(*kernel) }, ())
}




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
