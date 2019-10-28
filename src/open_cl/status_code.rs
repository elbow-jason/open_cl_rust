use crate::open_cl::ffi::cl_int;
use std::fmt;

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum StatusCode {
    Success,
    Failure(i32),
}

use StatusCode::*;

impl From<cl_int> for StatusCode {
    fn from(code: cl_int) -> StatusCode {
        match code {
            0 => Success,
            fail => Failure(fail as i32),
        }
    }
}

impl From<StatusCode> for cl_int {
    fn from(status_code: StatusCode) -> cl_int {
        match status_code {
            StatusCode::Success => 0,
            Failure(fail) => fail,
        }
    }
}

impl From<&StatusCode> for cl_int {
    fn from(status_code: &StatusCode) -> cl_int {
        match status_code {
            StatusCode::Success => 0,
            Failure(fail) => *fail as cl_int,
        }
    }
}

impl StatusCode {
    fn to_cl_name(&self) -> String {
        match &self {
            Success => "CL_SUCCESS".to_string(),
            Failure(-1) => "CL_DEVICE_NOT_FOUND".to_string(),
            Failure(-2) => "CL_DEVICE_NOT_AVAILABLE".to_string(),
            Failure(-3) => "CL_COMPILER_NOT_AVAILABLE".to_string(),
            Failure(-4) => "CL_MEM_OBJECT_ALLOCATION_FAILURE".to_string(),
            Failure(-5) => "CL_OUT_OF_RESOURCES".to_string(),
            Failure(-6) => "CL_OUT_OF_HOST_MEMORY".to_string(),
            Failure(-7) => "CL_PROFILING_INFO_NOT_AVAILABLE".to_string(),
            Failure(-8) => "CL_MEM_COPY_OVERLAP".to_string(),
            Failure(-9) => "CL_IMAGE_FORMAT_MISMATCH".to_string(),
            Failure(-10) => "CL_IMAGE_FORMAT_NOT_SUPPORTED".to_string(),
            Failure(-11) => "CL_BUILD_PROGRAM_FAILURE".to_string(),
            Failure(-12) => "CL_MAP_FAILURE".to_string(),
            Failure(-13) => "CL_MISALIGNED_SUB_BUFFER_OFFSET".to_string(),
            Failure(-14) => "CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST".to_string(),
            Failure(-15) => "CL_COMPILE_PROGRAM_FAILURE".to_string(),
            Failure(-16) => "CL_LINKER_NOT_AVAILABLE".to_string(),
            Failure(-17) => "CL_LINK_PROGRAM_FAILURE".to_string(),
            Failure(-18) => "CL_DEVICE_PARTITION_FAILED".to_string(),
            Failure(-19) => "CL_KERNEL_ARG_INFO_NOT_AVAILABLE".to_string(),
            Failure(-30) => "CL_INVALID_VALUE".to_string(),
            Failure(-31) => "CL_INVALID_DEVICE_TYPE".to_string(),
            Failure(-32) => "CL_INVALID_PLATFORM".to_string(),
            Failure(-33) => "CL_INVALID_DEVICE".to_string(),
            Failure(-34) => "CL_INVALID_CONTEXT".to_string(),
            Failure(-35) => "CL_INVALID_QUEUE_PROPERTIES".to_string(),
            Failure(-36) => "CL_INVALID_COMMAND_QUEUE".to_string(),
            Failure(-37) => "CL_INVALID_HOST_PTR".to_string(),
            Failure(-38) => "CL_INVALID_MEM_OBJECT".to_string(),
            Failure(-39) => "CL_INVALID_IMAGE_FORMAT_DESCRIPTOR".to_string(),
            Failure(-40) => "CL_INVALID_IMAGE_SIZE".to_string(),
            Failure(-41) => "CL_INVALID_SAMPLER".to_string(),
            Failure(-42) => "CL_INVALID_BINARY".to_string(),
            Failure(-43) => "CL_INVALID_BUILD_OPTIONS".to_string(),
            Failure(-44) => "CL_INVALID_PROGRAM".to_string(),
            Failure(-45) => "CL_INVALID_PROGRAM_EXECUTABLE".to_string(),
            Failure(-46) => "CL_INVALID_KERNEL_NAME".to_string(),
            Failure(-47) => "CL_INVALID_KERNEL_DEFINITION".to_string(),
            Failure(-48) => "CL_INVALID_KERNEL".to_string(),
            Failure(-49) => "CL_INVALID_ARG_INDEX".to_string(),
            Failure(-50) => "CL_INVALID_ARG_VALUE".to_string(),
            Failure(-51) => "CL_INVALID_ARG_SIZE".to_string(),
            Failure(-52) => "CL_INVALID_KERNEL_ARGS".to_string(),
            Failure(-53) => "CL_INVALID_WORK_DIMENSION".to_string(),
            Failure(-54) => "CL_INVALID_WORK_GROUP_SIZE".to_string(),
            Failure(-55) => "CL_INVALID_WORK_ITEM_SIZE".to_string(),
            Failure(-56) => "CL_INVALID_GLOBAL_OFFSET".to_string(),
            Failure(-57) => "CL_INVALID_EVENT_WAIT_LIST".to_string(),
            Failure(-58) => "CL_INVALID_EVENT".to_string(),
            Failure(-59) => "CL_INVALID_OPERATION".to_string(),
            Failure(-60) => "CL_INVALID_GL_OBJECT".to_string(),
            Failure(-61) => "CL_INVALID_BUFFER_SIZE".to_string(),
            Failure(-62) => "CL_INVALID_MIP_LEVEL".to_string(),
            Failure(-63) => "CL_INVALID_GLOBAL_WORK_SIZE".to_string(),
            Failure(-64) => "CL_INVALID_PROPERTY".to_string(),
            Failure(001) => "CL_PLATFORM_NOT_FOUND_KHR".to_string(),
            Failure(unk_code) => format!("UNKNOWN_ERROR_CODE_{:?}", unk_code),
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StatusCode({}, {:?})",
            self.to_cl_name(),
            cl_int::from(self) as i32
        )
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StatusCode({:?}, {:?})",
            self.to_cl_name(),
            cl_int::from(self) as i32
        )
    }
}
