extern crate bindgen;
extern crate which;

use std::env;
use std::fs;
use std::path::PathBuf;
use which::which;

fn main() {
    // #[cfg(all(target_os = "macos", debug_assertions))]
    // build_dev(PathBuf::from("./dev_bindings/macos_dev_bindings.rs"));

    // #[cfg(not(debug_assertions))]
    build();
}

// fn build_dev(bindings_file: PathBuf) {
//     if !path_exists(bindings_file.to_str().unwrap()) {
//         setup_llvm();
//         link_opencl();
//         gen_and_save_bindings(bindings_file);
//     }
// }

fn build() {
    setup_llvm();
    link_opencl();
    let bindings_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    gen_and_save_bindings(bindings_file);
}

fn gen_and_save_bindings(bindings_file: PathBuf) {
    println!("cargo:rerun-if-changed=wrapper.h");
    bindgen::Builder::default()
        .header("opencl_headers/wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn link_opencl() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=OpenCL");
}

fn setup_llvm() {
    let llvm_config_path = which("llvm-config").unwrap().to_str().unwrap().to_string();
    env::set_var("LLVM_CONFIG_PATH", llvm_config_path);
}
