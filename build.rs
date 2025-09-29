extern crate bindgen;

use std::env;
use std::path::PathBuf;

// fn main() {}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native=lib3mf");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=static=lib3mf");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Detect target OS and profile
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Target profile dir (where your exe/dll lands)
    let target_dir = manifest_dir.join("target").join(&profile);

    // Pick the right dynamic library
    let lib_name = match target_os.as_str() {
        "windows" => "lib3mf.dll",
        "macos" => "lib3mf.dylib",
        "linux" => "lib3mf.so",
        other => panic!("Unsupported target OS: {}", other),
    };

    // Source path (put your prebuilt libs in a "deps" dir for example)
    let src_path = manifest_dir.join("lib3mf").join(lib_name);
    let dst_path = target_dir.join(lib_name);

    // Copy
    std::fs::copy(&src_path, &dst_path)
        .unwrap_or_else(|e| panic!("Failed to copy {:?} to {:?}: {}", src_path, dst_path, e));
}
