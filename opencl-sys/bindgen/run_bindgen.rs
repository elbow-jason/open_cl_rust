extern crate bindgen;

use std::path::PathBuf;

fn main() {
    let bindings_file: PathBuf = PathBuf::from("./opencl_bindings.rs");
    bindgen::Builder::default()
        .header("../opencl_headers/wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}


