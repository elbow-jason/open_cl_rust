use crate::ffi::{
    cl_uint,
    cl_device_id,
    cl_program,
    cl_program_build_info,
    clBuildProgram,
    clGetProgramBuildInfo,
    clCreateProgramWithBinary,
    clCreateProgramWithSource,
};

use crate::utils::{ClObject, StatusCode};
use crate::device::Device;
use crate::error::Output; 
use crate::context::Context;
use crate::utils::strings;

use super::{ProgramError, Program, flags};

__release_retain!(program, Program);

pub fn cl_build_program(program: &Program, devices: &[&Device]) -> Output<()> {
    let err_code = unsafe {
        // We'll see...
        let mut cl_devices: Vec<cl_device_id> = devices.iter().map(|d| d.raw_cl_object()).collect();

        clBuildProgram(
            program.raw_cl_object(),
            cl_devices.len() as cl_uint,
            cl_devices.as_mut_ptr(),
            std::ptr::null(),
            std::mem::transmute(std::ptr::null::<fn()>()), // pfn_notify
            std::ptr::null_mut(), // user_data
        )
    };
    StatusCode::into_output(err_code, ())
}

pub fn cl_get_program_build_log(
    program: &Program,
    device: &Device,
    build_info_flag: flags::ProgramBuildInfo,
) -> Output<String> {
    device.usability_check()?;
    let mut size = 0 as libc::size_t;
    // determine buffer size
    let mut err_code = unsafe {
        clGetProgramBuildInfo(
            program.raw_cl_object(),
            device.raw_cl_object(),
            build_info_flag as cl_program_build_info,
            0,
            std::ptr::null_mut(),
            &mut size,
        )
    };

    // check that the info can be retrieved
    size = StatusCode::into_output(err_code, size)?;

    // make a buffer of the size
    let mut buf: Vec<u8> = vec![0u8; size as usize];
    // get bytes from the device for the last compilation for this program.
    err_code = unsafe {
        clGetProgramBuildInfo(
            program.raw_cl_object(),
            device.raw_cl_object(),
            build_info_flag as cl_program_build_info,
            buf.len() as libc::size_t,
            buf.as_mut_ptr() as *mut libc::c_void,
            std::ptr::null_mut(),
        )
    };
    buf = StatusCode::into_output(err_code, buf)?;

    Ok(strings::to_utf8_string(buf))
}

pub fn cl_create_program_with_source(context: &Context, src: &str) -> Output<Program> {
    let src = strings::to_c_string(src).ok_or_else(|| ProgramError::CStringInvalidSourceCode)?;
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
            std::ptr::null(),
            &mut err_code,
        )
    };
    
    let checked_program = StatusCode::into_output(err_code, program)?;
    unsafe { Ok(Program::new(checked_program)) }
}

pub fn cl_create_program_with_binary(
    context: &Context,
    device: &Device,
    binary: &str,
) -> Output<Program> {
    device.usability_check()?;
    let src = strings::to_c_string(binary).ok_or_else(|| ProgramError::CStringInvalidProgramBinary)?;
    let mut err_code = 0;
    let program = unsafe {
        clCreateProgramWithBinary(
            context.raw_cl_object(),
            1,
            device.raw_cl_object() as *const cl_device_id,
            binary.len() as *const libc::size_t,
            src.as_ptr() as *mut *const u8,
            std::ptr::null_mut(),
            &mut err_code,
        )
    };
    let checked_program = StatusCode::into_output(err_code, program)?;
    debug_assert!(checked_program.is_null() == false);
    unsafe { Ok(Program::new(checked_program)) }
}
