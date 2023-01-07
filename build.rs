use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");

    // Tell cargo to tell rustc to link the system shared library.
    println!("cargo:rustc-link-lib=/librealsense2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=/home/darian/Documents/github/librealsense/include/librealsense2/rs.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .whitelist_type("/h")
        // The input header we would like to generate
        // bindings for.
        .header("/home/darian/Documents/github/librealsense/include/librealsense2/rs.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path =
        PathBuf::from(env::var("/home/darian/Documents/github/realsense_bindings").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
