use std::convert::TryInto;
use std::fmt;

use crate::ffi::{
    clBuildProgram, clCreateProgramWithBinary, clCreateProgramWithSource, clGetProgramBuildInfo,
    clGetProgramInfo, cl_device_id, cl_program, cl_program_build_info, cl_program_info,
    cl_context
};
use crate::cl_helpers::{cl_get_info5, cl_get_info6, };
use crate::{
    build_output, ClPointer, Error, Output, strings, DevicePtr, utils, 
    ProgramInfo, ClContext, ContextPtr, ClDeviceID, ProgramBuildInfo,
};

pub const DEVICE_LIST_CANNOT_BE_EMPTY: Error = Error::ProgramError(ProgramError::CannotBuildProgramWithEmptyDevicesList);

__release_retain!(program, Program);

pub unsafe fn release_program(program: cl_program) {
    cl_release_program(program).unwrap_or_else(|e| {
        panic!("Failed to release cl_program {:?} due to {:?}", program, e);
    });
}

pub unsafe fn retain_program(program: cl_program) {
    cl_retain_program(program).unwrap_or_else(|e| {
        panic!("Failed to retain cl_program {:?} due to {:?}", program, e);
    });
}

/// An error related to Program.
#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum ProgramError {
    #[fail(display = "The given source code was not a valid CString")]
    CStringInvalidSourceCode,

    #[fail(display = "The given program binary was not a valid CString")]
    CStringInvalidProgramBinary,

    #[fail(display = "Cannot build a program with an empty list of devices")]
    CannotBuildProgramWithEmptyDevicesList,
}

#[allow(clippy::transmuting_null)]
#[allow(unused_mut)]
pub unsafe fn cl_build_program(program: cl_program, device_ids: &[cl_device_id]) -> Output<()> {
    // let device_id: *const cl_device_id = 
    let err_code = clBuildProgram(
        program,
        1u32,
        device_ids.as_ptr() as *const cl_device_id,
        std::ptr::null(),
        std::mem::transmute(std::ptr::null::<fn()>()), // pfn_notify
        std::ptr::null_mut(),                          // user_data
    );
    build_output((), err_code)
}
    
pub unsafe fn cl_get_program_build_log(
    program: cl_program,
    device: cl_device_id,
    info_flag: cl_program_build_info,
) -> Output<ClPointer<u8>> {
    device.usability_check()?;    
    cl_get_info6(
        program,
        device,
        info_flag,
        clGetProgramBuildInfo,
    )
}


pub unsafe fn cl_create_program_with_source(context: cl_context, src: &str) -> Output<cl_program> {
    let src = strings::to_c_string(src).ok_or_else(|| ProgramError::CStringInvalidSourceCode)?;
    let mut src_list = vec![src.as_ptr()];

    let mut err_code = 0;
    let program: cl_program = clCreateProgramWithSource(
        context,
        src_list.len().try_into().unwrap(),
        // const char **strings
        // mut pointer to const pointer of char. Great.
        src_list.as_mut_ptr() as *mut *const libc::c_char,
        // null pointer here indicates that all strings in the src
        // are NULL-terminated.
        std::ptr::null(),
        &mut err_code,
    );
    build_output(program, err_code)
}

// the dereferncing of the pointer happens on the _other_ _side_ of the C FFI.
// So it cannot be any more unsafe that in already is...
#[allow(clippy::cast_ptr_alignment)]
pub unsafe fn cl_create_program_with_binary(
    context: cl_context,
    device: cl_device_id,
    binary: &[u8],
) -> Output<cl_program> {
    device.usability_check()?;
    let mut err_code = 0;
    let program = clCreateProgramWithBinary(
        context,
        1,
        device as *const cl_device_id,
        binary.len() as *const libc::size_t,
        binary.as_ptr() as *mut *const u8,
        std::ptr::null_mut(),
        &mut err_code,
    );
    build_output(program, err_code)
}

pub unsafe fn cl_get_program_info<T: Copy>(program: cl_program, flag: cl_program_info) -> Output<ClPointer<T>> {
    cl_get_info5(
        program,
        flag,
        clGetProgramInfo,
    )
}

pub struct ClProgram {
    object: cl_program,

    _unconstructable: ()
}

impl ClProgram {
    pub unsafe fn create_with_source(context: &ClContext, src: &str) -> Output<ClProgram> {
        let prog = cl_create_program_with_source(context.context_ptr(), src)?;
        Ok(ClProgram::unchecked_new(prog))
    }

    pub unsafe fn create_with_binary(context: &ClContext, device: &ClDeviceID, bin: &[u8]) -> Output<ClProgram> {
        let prog = cl_create_program_with_binary(
            context.context_ptr(),
            device.device_ptr(),
            bin,
        )?;
        Ok(ClProgram::unchecked_new(prog))
    }

    pub unsafe fn unchecked_new(program: cl_program) -> ClProgram {
        ClProgram {
            object: program,
            _unconstructable: (),
        }
    }

    pub unsafe fn new(prog: cl_program) -> Output<ClProgram> {
        utils::null_check(prog)?;
        Ok(ClProgram::unchecked_new(prog))
    }

    pub fn build<D>(&mut self, devices: &[D]) -> Output<()> where D: DevicePtr {
        if devices.is_empty() {
            return Err(DEVICE_LIST_CANNOT_BE_EMPTY);
        }
        unsafe {
            let device_ptrs: Vec<cl_device_id> = devices
                .iter()
                .map(|d| d.device_ptr())
                .collect();
            cl_build_program(self.program_ptr(), &device_ptrs[..])
        }   
    }

    pub fn get_log<D: DevicePtr>(&self, device: &D) -> Output<String> {
        unsafe { 
            cl_get_program_build_log(
                self.program_ptr(),
                device.device_ptr(),
                ProgramBuildInfo::Log.into()
            )
            .map(|ret| ret.into_string())
        }
    }
}

impl Drop for ClProgram {
    fn drop(&mut self) {
        unsafe { release_program(self.object) };
    }
}  

impl Clone for ClProgram {
    fn clone(&self) -> ClProgram {
        unsafe {
            retain_program(self.object);
            ClProgram::unchecked_new(self.object)
        }
    }
}

impl ProgramPtr for ClProgram {
    unsafe fn program_ptr(&self) -> cl_program {
        self.object
    }
}

impl fmt::Debug for ClProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClProgram{{{:?}}}", self.object)
    }
}

impl PartialEq for ClProgram {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.object, other.object)
    }
}

impl Eq for ClProgram {}

fn get_info<T: Copy, P: ProgramPtr>(program: &P, flag: ProgramInfo) -> Output<ClPointer<T>> {
    unsafe { cl_get_program_info(program.program_ptr(), flag.into()) }
}

pub trait ProgramPtr: Sized {
    unsafe fn program_ptr(&self) -> cl_program;

    fn reference_count(&self) -> Output<u32> {
        get_info(self, ProgramInfo::ReferenceCount)
            .map(|ret| unsafe { ret.into_one() })
    }

    fn num_devices(&self) -> Output<usize> {
        get_info(self, ProgramInfo::NumDevices)
            .map(|ret| unsafe { 
                let num32: u32 = ret.into_one();
                num32 as usize
            })
    }

    fn source(&self) -> Output<String> {
        get_info(self, ProgramInfo::Source)
            .map(|ret| unsafe { ret.into_string() })
    }
    fn binary_sizes(&self) -> Output<Vec<usize>> {
        get_info(self, ProgramInfo::BinarySizes)
            .map(|ret| unsafe { ret.into_vec() })
    }

    fn binaries(&self) -> Output<Vec<u8>> {
        get_info(self, ProgramInfo::Binaries)
            .map(|ret| unsafe { ret.into_vec() })
    }

    fn num_kernels(&self) -> Output<usize> {
        get_info(self, ProgramInfo::NumKernels)
            .map(|ret| unsafe { ret.into_one() })
    }

    fn kernel_names(&self) -> Output<Vec<String>> {
        get_info(self, ProgramInfo::KernelNames).map(|ret| {
            let kernels: String = unsafe { ret.into_string() };
            kernels.split(';').map(|s| s.to_string()).collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SRC: &'static str = "
    __kernel void test123(__global int *i) {
        *i += 1;
    }";

    #[test]
    fn program_ptr_reference_count() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let ref_count = prog.reference_count().unwrap();
        assert_eq!(ref_count, 1);
    }

    #[test]
    fn cloning_increments_reference_count() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let prog2 = prog.clone();
        let prog3 = prog.clone();
        let ref_count = prog.reference_count().unwrap();
        assert_eq!(ref_count, 3);
        assert_eq!(prog, prog2);
        assert_eq!(prog, prog3);
    }

    #[test]
    fn program_ptr_num_devices() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let num_devices = prog.num_devices().unwrap();
        assert!(num_devices > 0);
    }


    #[test]
    fn num_devices_matches_devices_len() {
        let (prog, devices, _context) = ll_testing::get_program(SRC);
        let num_devices = prog.num_devices().unwrap();
        assert_eq!(num_devices, devices.len());
    }

    #[test]
    fn program_ptr_source_matches_creates_src() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let prog_src = prog.source().unwrap();
        assert_eq!(prog_src, SRC.to_string());
    }

    #[test]
    fn program_ptr_num_kernels() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let num_kernels = prog.num_kernels().unwrap();
        assert_eq!(num_kernels, 1);
    }

    #[test]
    fn program_ptr_kernel_names() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let kernel_names = prog.kernel_names().unwrap();
        assert_eq!(kernel_names, vec!["test123"]);
    }

    #[test]
    fn num_kernels_matches_kernel_names_len() {
        let (prog, _devices, _context) = ll_testing::get_program(SRC);
        let kernel_names = prog.kernel_names().unwrap();
        let num_kernels = prog.num_kernels().unwrap();
        assert_eq!(num_kernels, kernel_names.len());
    }
}