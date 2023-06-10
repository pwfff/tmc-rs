use std::path::PathBuf;

fn main() {
    // tmc repo
    let tmc_path = PathBuf::from("thirdparty/TMC-API")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let ic_path = tmc_path.clone().join("tmc/ic/TMC2240");
    let ic_c_path = ic_path.join("TMC2240.c");

    let ramp_path = tmc_path.clone().join("tmc/ramp");
    let ramp_c_path = ramp_path.join("LinearRamp1.c");
    let linear_ramp_c_path = ramp_path.join("Ramp.c");

    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=TMC2240");
    //println!("cargo:rustc-link-lib=Ramp");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // cc::Build::new()
    //     .include(tmc_path.clone())
    //     .file(ic_c_path)
    //     .compile("Ramp");

    cc::Build::new()
        .include(tmc_path.clone())
        .file(ic_c_path)
        .file(ramp_c_path)
        .file(linear_ramp_c_path)
        .compile("TMC2240");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // for no_std
        .use_core()
        .raw_line("extern crate tmc_rs_macros;")
        .raw_line("use tmc_rs_macros::generate_fields;")
        .raw_line("")
        .raw_line("#[generate_fields]")
        .enable_cxx_namespaces()
        .ctypes_prefix("cty")
        .clang_arg(format!("-I{}", tmc_path.clone().display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
