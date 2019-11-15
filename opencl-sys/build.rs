extern crate bindgen;
// extern crate which;

use std::env;
// use std::fs;
use std::path::PathBuf;
use which::which;

fn main() {
    println!("skipped");
    // gen_and_save_bindings(PathBuf::from("./bindgen/opencl_bindings.rs"));
}


fn gen_and_save_bindings(bindings_file: PathBuf) {
    bindgen::Builder::default()
        .header("opencl_headers/wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}

