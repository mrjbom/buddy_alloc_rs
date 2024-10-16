use std::io::Write;
use std::path::PathBuf;

const BUDDY_ALLOC_HEADER_FILE_PATH: &str = "../buddy_alloc-1.2.0/buddy_alloc.h";

fn main() {
    //println!("cargo:rerun-if-changed=buddy_alloc-1.2.0/buddy_alloc.h");

    // Link
    println!("cargo:rustc-link-lib=static=buddy_alloc");

    let buddy_alloc_header_file_path = PathBuf::from("../buddy_alloc-1.2.0/buddy_alloc.h");
    if !buddy_alloc_header_file_path.exists() {
        panic!("buddy_alloc.h not found!");
    }
    // Compilation
    // cc don't compile .h file
    // create copy with .c extension
    let buddy_alloc_c_file_path = buddy_alloc_header_file_path.with_extension("c");
    std::fs::write(
        &buddy_alloc_c_file_path,
        std::fs::read_to_string(buddy_alloc_header_file_path)
            .expect("Failed to read buddy_alloc.h"),
    )
    .expect("Failed to write buddy_alloc.c file");

    // Compile .c file
    cc::Build::new()
        .file(&buddy_alloc_c_file_path)
        .define("BUDDY_ALLOC_IMPLEMENTATION", None)
        .compile("buddy_alloc");

    // Generate binding and write to buffer
    let mut bindings_content = Vec::<u8>::new();
    bindgen::Builder::default()
        .header(BUDDY_ALLOC_HEADER_FILE_PATH)
        .allowlist_item("buddy.*")
        .allowlist_item("BUDDY.*")
        .use_core() // No std
        .generate()
        .expect("Failed to generate binding")
        .write(Box::new(&mut bindings_content))
        .expect("Failed to write binding to buffer");

    // Write binding content from buffer to lib.rs
    // Disable warnings for lib.rs
    let mut librs_file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open("src/lib.rs")
        .expect("Failed to open lib.rs");

    // No std
    librs_file
        .write("#![no_std]\n".as_ref())
        .expect("Failed to write to lib.rs");

    // Disable all warnings in lib.rs
    librs_file
        .write("#![allow(warnings)]\n\n".as_ref())
        .expect("Failed to write to lib.rs");

    // Comment
    librs_file
        .write("// Bindgen FFI binding to buddy_alloc\n\n".as_ref())
        .expect("Failed to write to lib.rs");

    // Write binding content
    librs_file
        .write(bindings_content.as_ref())
        .expect("Failed to write to lib.rs");
}
