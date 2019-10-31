extern crate bindgen;
extern crate which;

use std::env;
use std::path::PathBuf;
use which::which;

fn main() {
    let llvm_config_path = which("llvm-config").unwrap().to_str().unwrap().to_string();
    env::set_var("LLVM_CONFIG_PATH", llvm_config_path);
    // Tell cargo to tell rustc to link the system -framework OpenCL shared library.
    // println!("cargo:rustc-link-lib=framework=OpenCL");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("opencl_headers/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("binding.rs"))
        .expect("Couldn't write bindings!");
}
