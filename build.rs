use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");

    // Tell cargo to tell rustc to link the system shared library.
    println!("cargo:rustc-link-lib=dylib=realsense2");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!(
        "cargo:rerun-if-changed=/home/darian/Documents/github/realsense_wrapper/src/bindings.rs"
    );

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("/home/darian/Documents/github/librealsense/include/librealsense2/rs.h")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .header("/home/darian/Documents/github/librealsense/include/librealsense2/h/rs_pipeline.h")
        .header("/home/darian/Documents/github/librealsense/include/librealsense2/h/rs_frame.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path =
    //     PathBuf::from(env::var("/home/darian/Documents/github/realsense_wrapper").unwrap());
    bindings
        .write_to_file("/home/darian/Documents/github/realsense_wrapper/src/bindings.rs")
        .expect("Couldn't write bindings!");
}
