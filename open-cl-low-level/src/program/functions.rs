use crate::cl::{
    clBuildProgram, clCreateProgramWithBinary, clCreateProgramWithSource, clGetProgramBuildInfo,
    clGetProgramInfo, cl_context, cl_device_id, cl_program, cl_program_build_info, cl_program_info,
    cl_uint, ClObject, ProgramInfo, StatusCodeError,
};
use crate::Output;
use libc::{c_void, size_t};
use std::ffi::CString;

/// A low-level helper function for calling the OpenCL FFI function clBuildProgram.
///
/// # Safety
/// if the devices or the program are in an invalid state this function call results in
/// undefined behavior.
#[allow(clippy::transmuting_null)]
#[allow(unused_mut)]
#[inline(always)]
pub unsafe fn build_program(program: cl_program, device_ids: &[cl_device_id]) -> Output<()> {
    let err_code = clBuildProgram(
        program.as_ptr() as *mut libc::c_void,
        1u32,
        device_ids.as_ptr() as *const *mut libc::c_void,
        std::ptr::null(),
        std::mem::transmute(std::ptr::null::<fn()>()), // pfn_notify
        std::ptr::null_mut(),                          // user_data
    );
    StatusCodeError::check(err_code)
}

/// Low level helper function for clGetProgramBuildInfo.
///
/// # Safety
/// If the program or device is in an invalid state this function call is undefined behavior.
#[inline(always)]
pub unsafe fn get_program_build_log(
    program: cl_program,
    device: cl_device_id,
    flag: cl_program_build_info,
) -> Output<String> {
    cl_get_info!(One, String, clGetProgramBuildInfo, program, device, flag)
}

/// Low level helper function for calling the OpenCL FFI function clCreateProgramWithSource.
///
/// # Safety
/// If the context or device is in an invalid state this function will cause undefined
/// behavior.
pub unsafe fn create_program_with_src(context: cl_context, src: CString) -> Output<cl_program> {
    let mut src_list = vec![src.as_ptr()];

    let mut err_code = 0;
    let raw_program: *mut libc::c_void = clCreateProgramWithSource(
        context.as_ptr() as *mut libc::c_void,
        src_list.len() as cl_uint,
        // const char **strings
        // mut pointer to const pointer of char. Great.
        src_list.as_mut_ptr() as *mut *const libc::c_char,
        // null pointer here indicates that all strings in the src
        // are NULL-terminated.
        std::ptr::null(),
        &mut err_code,
    );
    StatusCodeError::check(err_code)?;
    cl_program::new(raw_program)
}

/// Low level helper function for calling the OpenCL FFI function clCreateProgramWithBinary.
///
/// # Safety
/// If the context or device is in an invalid state this function will cause undefined
/// behavior. WRT the clippy::cast_ptr_alignment below the dereferncing of the pointer
/// happens on the _other_ _side_ of the C FFI. So it cannot be any more unsafe that
/// in already is...
#[allow(clippy::cast_ptr_alignment)]
pub unsafe fn create_program_with_binary(
    context: cl_context,
    device: cl_device_id,
    binary: &[u8],
) -> Output<cl_program> {
    let mut status_code = 0;
    let raw_program = clCreateProgramWithBinary(
        context.as_ptr() as *mut c_void,
        1,
        device.as_ptr() as *const *mut c_void,
        binary.len() as *const libc::size_t,
        binary.as_ptr() as *mut *const u8,
        std::ptr::null_mut(),
        &mut status_code,
    );
    StatusCodeError::check(status_code)?;
    cl_program::new(raw_program)
}

/// Low level helper function for the FFI call to clGetProgramInfo with u32 expected
///
/// # Safety
/// Calling this function with a cl_program that is not in a valid state is
/// undefined behavior.
#[inline(always)]
pub unsafe fn get_program_info_u32(program: cl_program, flag: cl_program_info) -> Output<u32> {
    cl_get_info!(
        One,
        u32,
        clGetProgramInfo,
        program,
        Into::<cl_program_info>::into(flag)
    )
}

#[inline(always)]
pub unsafe fn get_program_info_string(
    program: cl_program,
    flag: cl_program_info,
) -> Output<String> {
    cl_get_info!(One, String, clGetProgramInfo, program, flag)
}

#[inline(always)]
pub unsafe fn get_program_info_vec_usize(
    program: cl_program,
    flag: cl_program_info,
) -> Output<Vec<usize>> {
    cl_get_info!(Many, usize, clGetProgramInfo, program, flag)
}

#[inline(always)]
pub unsafe fn get_program_info_bytes(
    program: cl_program,
    flag: cl_program_info,
) -> Output<Vec<u8>> {
    cl_get_info!(Many, u8, clGetProgramInfo, program, flag)
}

#[inline(always)]
pub unsafe fn get_program_info_usize(program: cl_program, flag: cl_program_info) -> Output<usize> {
    cl_get_info!(One, usize, clGetProgramInfo, program, flag)
}

#[inline(always)]
pub unsafe fn get_program_info_vec_device(program: cl_program) -> Output<Vec<cl_device_id>> {
    cl_get_info!(
        Many,
        cl_device_id,
        clGetProgramInfo,
        program,
        ProgramInfo::Devices.into()
    )
}

#[inline(always)]
pub unsafe fn get_program_info_context(program: cl_program) -> Output<cl_context> {
    cl_get_info!(
        One,
        cl_context,
        clGetProgramInfo,
        program,
        ProgramInfo::Context.into()
    )
}
