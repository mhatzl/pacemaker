use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("ffi")
        .canonicalize()
        .expect("cannot canonicalize path");
    let headers_path = libdir_path.join("wrapper.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");
    let obj_path = libdir_path.join("wrapper.o");
    let lib_path = libdir_path.join("libwrapper.a");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=wrapper");

    if !std::process::Command::new("clang")
        .arg("-target")
        .arg("thumbv7em-none-eabihf")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("wrapper.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        panic!("could not compile object file");
    }

    if !std::process::Command::new("llvm-ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `llvm-ar`")
        .status
        .success()
    {
        panic!("could not emit library file");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
