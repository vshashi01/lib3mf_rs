# lib3mf_rs

`lib3mf_rs` is a Rust wrapper for the [lib3mf](https://github.com/3MFConsortium/lib3mf) library, which provides tools for reading, writing, and manipulating 3MF files (the 3D Manufacturing Format). This crate enables Rust projects to interact with 3MF files using bindings generated via [bindgen](https://github.com/rust-lang/rust-bindgen) based on the C Bindings provided by the main library.

## Features

- Support for reading and writing 3MF files based on the C Bindings generated for the lib3mf project
- Automatic generation of Rust FFI bindings during build

## Current tested version

The library was last tested with the binaries from version 2.4.1

## Examples

Currently examples are not existent. Some tests are written in lib.rs which can be studied to understand how the API can be used.

Any contributions to extend the examples are welcome.

## Building

To build this crate, you need:

- Rust (latest stable recommended)
- The lib3mf binaries (lib3mf.lib & lib3mf dynamic libraries) placed in the `lib3mf` directory at the root of this crate. This can be downloaded from the release archive on the main lib3mf repository.

Build with:

```sh
cargo build
```

The build script (`build.rs`) will:

- Generate Rust bindings from the C headers using bindgen
- Copy the appropriate lib3mf dynamic library from `lib3mf/` to your build output directory (`target/debug` or `target/release`)

## Usage in Dependent Crates

If your crate depends on `lib3mf_rs`, you **must ensure the lib3mf dynamic library is available at runtime**.  
Rust's build system does **not** automatically copy the binaries to your dependent crate's output directory.

### How to Copy the Binary in Dependent Crates

1. **Manual Copy**  
   After building, copy the dynamic library from this crate's `lib3mf` directory to your binary's output directory (e.g., `target/debug` or `target/release`).

2. **Automated Copy via Build Script**  
   Add a `build.rs` to your dependent crate to copy the binary from the `lib3mf_rs` crate. Example:

   ```rust
   // build.rs in your dependent crate
   use std::env;
   use std::fs;
   use std::path::PathBuf;

   fn main() {
       let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
       // Adjust the path as needed to point to lib3mf_rs's lib3mf directory
       let lib3mf_path = PathBuf::from("../lib3mf_rs/lib3mf/lib3mf.dll"); // Windows example
       let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
       let dst = out_dir.join("lib3mf.dll");
       fs::copy(&lib3mf_path, &dst).expect("Failed to copy lib3mf.dll");
   }
   ```

   Make sure the path to `lib3mf.dll` is correct relative to your crate.

3. **Document the Requirement**  
   Inform users in your crate's README that they must copy the dynamic library to their binary directory.

## License

This crate is distributed under the MIT license.  
lib3mf itself is distributed under its own license; see [lib3mf's repository](https://github.com/3MFConsortium/lib3mf) for details.

## Notes

- This crate does **not** bundle the lib3mf dynamic library for crates.io publication.
- You must provide the correct version of the dynamic library for your platform.
- If you encounter issues with missing DLLs/SOs/DYLIBs at runtime, ensure the library is present in your binary's directory or in your system's library path.
