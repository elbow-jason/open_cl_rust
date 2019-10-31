use crate::ffi::{
    cl_program,
    cl_program_build_info,
    // cl_program_binary_type,
    cl_program_info,
};

use crate::open_cl::{
    cl_create_program_with_binary,
    cl_create_program_with_source,
    cl_build_program,
    cl_create_kernel,
    cl_get_program_build_log,
    cl_release_program,
    cl_retain_program,
    ClObject,
};

use crate::{Device, Kernel, Output, Context};

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Program {
    inner: cl_program,
    _phantom: (),
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { cl_release_program(&self.raw_cl_object()) };
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        unsafe {
            let cl_object: cl_program = self.raw_cl_object();
            cl_retain_program(&cl_object);
            Program::new(cl_object)
        }
    }
}


impl ClObject<cl_program> for Program {
    unsafe fn raw_cl_object(&self) -> cl_program {
        self.inner
    }
}

impl Program {
    pub unsafe fn new(inner: cl_program) -> Program {
        Program {
            inner,
            _phantom: (),
        }
    }

    pub fn create_with_source(context: &Context, src: String) -> Output<Program> {
        cl_create_program_with_source(context, &src[..])
    }

    pub fn create_program_with_binary(context: &Context, device: &Device, binary: String) -> Output<Program> {
        cl_create_program_with_binary(context, device, &binary[..])
    }

    pub fn build_on_many_devices(&self, devices: &[&Device]) -> Output<()> {
        cl_build_program(self, devices)
    }

    pub fn build_on_one_device(&self, device: &Device) -> Output<()> {
        cl_build_program(self, &vec![device])
    }

    pub fn fetch_kernel(&self, name: &str) -> Output<Kernel> {
        match cl_create_kernel(self, name) {
            Ok(kernel) => Ok(Kernel::new(kernel)),
            Err(err) => Err(err),
        }
    }

    pub fn get_log(program: &Program, device: &Device) -> Output<String> {
        cl_get_program_build_log(
            program,
            device,
            ProgramBuildInfo::Log as cl_program_build_info,
        )
    }
}

crate::__codes_enum!(ProgramBuildInfo, cl_program_build_info, {
    Status => 0x1181,
    Options => 0x1182,
    Log => 0x1183,
    // NOTE: Version for BinaryType?
    // BinaryType => 0x1184,
    GlobalVariableTotalSize => 0x1185
});

crate::__codes_enum!(ProgramInfo, cl_program_info, {
    ReferenceCount => 0x1160,
    Context => 0x1161,
    NumDevices => 0x1162,
    Devices => 0x1163,
    Source => 0x1164,
    BinarySizes => 0x1165,
    Binaries => 0x1166,
    NumKernels => 0x1167,
    KernelNames => 0x1168,
    Il => 0x1169,
    ScopeGlobalCtorsPresent => 0x116A,
    ScopeGlobalDtorsPresent => 0x116B

});

// NOTE: Version for cl_program_binary_type?
// crate::__codes_enum!(ProgramBinaryType, cl_program_binary_type, {
//     NONE => 0x0,
//     COMPILED_OBJECT => 0x1,
//     LIBRARY => 0x2,
//     EXECUTABLE => 0x4
// });
