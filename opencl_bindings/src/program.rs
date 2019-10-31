use crate::ffi::{
    cl_program,
    cl_program_build_info,
    // cl_program_binary_type,
    cl_program_info,
};

use crate::open_cl::{
    cl_build_program, cl_create_kernel, cl_get_program_build_log, cl_release_program, ClObject,
};

use crate::{Device, Kernel, Output};

fn do_build_on_devices(program: &Program, devices: &[&Device]) -> Output<()> {
    cl_build_program(program, devices)
}

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
impl ClObject<cl_program> for Program {
    unsafe fn raw_cl_object(&self) -> cl_program {
        self.inner
    }
}

impl Program {
    pub fn new(inner: cl_program) -> Program {
        Program {
            inner,
            _phantom: (),
        }
    }

    pub fn build_on_many_devices(&self, devices: &[&Device]) -> Output<()> {
        do_build_on_devices(self, devices)
    }

    pub fn build_on_one_device(&self, device: &Device) -> Output<()> {
        let devices = vec![device];
        do_build_on_devices(self, &devices)
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
            ProgramBuildInfo::LOG as cl_program_build_info,
        )
    }
}

crate::__codes_enum!(ProgramBuildInfo, cl_program_build_info, {
    STATUS => 0x1181,
    OPTIONS => 0x1182,
    LOG => 0x1183,
    BINARYTYPE => 0x1184,
    GLOBALVARIABLETOTALSIZE => 0x1185
});

crate::__codes_enum!(ProgramInfo, cl_program_info, {
    REFERENCE_COUNT => 0x1160,
    CONTEXT => 0x1161,
    NUM_DEVICES => 0x1162,
    DEVICES => 0x1163,
    SOURCE => 0x1164,
    BINARY_SIZES => 0x1165,
    BINARIES => 0x1166,
    NUM_KERNELS => 0x1167,
    KERNEL_NAMES => 0x1168,
    IL => 0x1169,
    SCOPE_GLOBAL_CTORS_PRESENT => 0x116A,
    SCOPE_GLOBAL_DTORS_PRESENT => 0x116B

});

// crate::__codes_enum!(ProgramBinaryType, cl_program_binary_type, {
//     NONE => 0x0,
//     COMPILED_OBJECT => 0x1,
//     LIBRARY => 0x2,
//     EXECUTABLE => 0x4
// });
