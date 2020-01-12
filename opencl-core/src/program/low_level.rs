use crate::ffi::{
    clBuildProgram, clCreateProgramWithBinary, clCreateProgramWithSource, clGetProgramBuildInfo,
    clGetProgramInfo, cl_device_id, cl_program, cl_program_build_info, cl_program_info,
    cl_context
};

use crate::cl::{cl_get_info5, cl_get_info6, ClPointer};
use crate::context::{Context, ContextRefCount};
use crate::device::{Device, DevicePtr};
use crate::error::{Error, Output};
use crate::utils::strings;
use crate::utils::StatusCode;

use super::{flags, UnbuiltProgram, ProgramError, Program, ProgramPtr};
use flags::ProgramInfo;

__release_retain!(program, Program);

pub const DEVICE_LIST_CANNOT_BE_EMPTY: Error = Error::ProgramError(ProgramError::CannotBuildProgramWithEmptyDevicesList);

#[allow(clippy::transmuting_null)]
#[allow(unused_mut)]
pub unsafe fn cl_build_program<D>(mut unbuilt: UnbuiltProgram, device: &D) -> Output<Program> where D: DevicePtr {
    let device_id: *const cl_device_id = &device.device_ptr() as *const cl_device_id;
    
    let err_code = clBuildProgram(
        unbuilt.program_ptr(),
        1u32,
        device_id,
        std::ptr::null(),
        std::mem::transmute(std::ptr::null::<fn()>()), // pfn_notify
        std::ptr::null_mut(),                          // user_data
    );
    StatusCode::build_output(err_code, ())?;
    let program_device: Device = Device::new(device.device_ptr())?;
    let (context_ptr, program_ptr): (cl_context, cl_program) = unbuilt.decompose();
    let program_context: Context = Context::from_retained(context_ptr)?;
    let built_program: Program = Program::new(
        program_ptr,
        program_context,
        program_device,
    );
    Ok(built_program)
}
    
pub fn cl_get_program_build_log(
    program: &Program,
    device: &Device,
    info_flag: flags::ProgramBuildInfo,
) -> Output<ClPointer<u8>> {
    device.usability_check()?;
    unsafe {
        cl_get_info6(
            program.program_ptr(),
            device.device_ptr(),
            info_flag as cl_program_build_info,
            clGetProgramBuildInfo,
        )
    }
}


pub fn cl_create_program_with_source(context: &Context, src: &str) -> Output<UnbuiltProgram> {
    let src = strings::to_c_string(src).ok_or_else(|| ProgramError::CStringInvalidSourceCode)?;
    let mut src_list = vec![src.as_ptr()];

    let mut err_code = 0;
    let program: cl_program = unsafe {
        clCreateProgramWithSource(
            context.context_ptr(),
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

    println!("cl_build_program created {:?}", program);

    StatusCode::build_output(err_code, ())?;
    Ok(unsafe { UnbuiltProgram::new(program, context.clone()) })
}

// the dereferncing of the pointer happens on the _other_ _side_ of the C FFI.
// So it cannot be any more unsafe that in already is...
#[allow(clippy::cast_ptr_alignment)]
pub fn cl_create_program_with_binary(
    context: &Context,
    device: &Device,
    binary: &str,
) -> Output<UnbuiltProgram> {
    device.usability_check()?;
    let src =
        strings::to_c_string(binary).ok_or_else(|| ProgramError::CStringInvalidProgramBinary)?;
    let mut err_code = 0;
    let program = unsafe {
        clCreateProgramWithBinary(
            context.context_ptr(),
            1,
            device.device_ptr() as *const cl_device_id,
            binary.len() as *const libc::size_t,
            src.as_ptr() as *mut *const u8,
            std::ptr::null_mut(),
            &mut err_code,
        )
    };
    let checked_program = StatusCode::build_output(err_code, program)?;
    debug_assert!(!checked_program.is_null());
    Ok(unsafe { UnbuiltProgram::new(checked_program, context.clone()) })
}

pub fn cl_get_program_info<T: Copy, P: ProgramPtr>(program: &P, flag: ProgramInfo) -> Output<ClPointer<T>> {
    unsafe {
        cl_get_info5(
            program.program_ptr(),
            flag as cl_program_info,
            clGetProgramInfo,
        )
    }
}
