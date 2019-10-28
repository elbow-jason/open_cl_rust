use crate::open_cl::{
    cl_build_program,
    cl_create_kernel,
    cl_device_id,
    cl_get_program_build_log,
    cl_program,
    cl_program_build_info,
    // cl_program_binary_type,
    cl_program_info,
    cl_release_program,
};

use crate::{Device, Kernel, Output};

fn do_build_on_devices(prog: &cl_program, mut devices: Vec<cl_device_id>) -> Output<()> {
    cl_build_program(prog, &mut devices[..])
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Program(cl_program);

impl Drop for Program {
    fn drop(&mut self) {
        println!("Dropping program {:?}", self.cl_object());
        cl_release_program(&self.cl_object()).unwrap_or_else(|e| {
            panic!(
                "Failed to complete cl_release_program while dropping: {:?}",
                e
            )
        })
    }
}

impl Program {
    pub fn new(program: cl_program) -> Program {
        Program(program)
    }

    pub fn cl_object(&self) -> cl_program {
        self.0
    }
    pub fn build_on_many_devices(&self, devices: Vec<Device>) -> Output<()> {
        let cl_devices: Vec<cl_device_id> = devices.into_iter().map(|d| d.cl_object()).collect();
        do_build_on_devices(&self.cl_object(), cl_devices)
    }

    pub fn build_on_one_device(&self, device: &Device) -> Output<()> {
        let devices = vec![device.cl_object()];
        do_build_on_devices(&self.cl_object(), devices)
    }

    pub fn fetch_kernel(&self, name: &str) -> Output<Kernel> {
        match cl_create_kernel(&self.cl_object(), name) {
            Ok(kernel) => Ok(Kernel::new(kernel)),
            Err(err) => Err(err),
        }
    }

    pub fn get_log(program: &Program, device: &Device) -> Output<String> {
        cl_get_program_build_log(
            &program.cl_object(),
            &device.cl_object(),
            ProgramBuildInfo::LOG as cl_program_build_info,
        )
    }
}

// /* cl_program_build_info */
crate::__codes_enum!(ProgramBuildInfo, cl_program_build_info, {
    STATUS => 0x1181,
    OPTIONS => 0x1182,
    LOG => 0x1183,
    BINARYTYPE => 0x1184,
    GLOBALVARIABLETOTALSIZE => 0x1185
});

// /* cl_program_info */
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

// // /* cl_program_binary_type */
// crate::__codes_enum!(ProgramBinaryType, cl_program_binary_type, {
//     NONE => 0x0,
//     COMPILED_OBJECT => 0x1,
//     LIBRARY => 0x2,
//     EXECUTABLE => 0x4
// });
