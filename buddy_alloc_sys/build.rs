use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // For x86_64-unknown-none
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64"
        && env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "unknown"
        && env::var("CARGO_CFG_TARGET_OS").unwrap() == "none"
    {
        let buddy_alloc_folder = Path::new(&manifest_dir).join("buddy_alloc_kernel");
        println!(
            "cargo:rustc-link-search=native={}",
            buddy_alloc_folder.display()
        );

        if env::var("DEBUG").unwrap() == "true" {
            println!("cargo:rustc-link-lib=static=buddy_alloc_debug_kernel");
        } else {
            println!("cargo:rustc-link-lib=static=buddy_alloc_release_kernel");
        }
    } else {
        // For everything else, we try to compile and link library
        // This may not work correctly with sys binding, since it is generated for the kernel, but technically it should work on any x86_64 Linux and Windows machine
        println!("cargo:warning={}", "Building not for kernel! May not work!");
        let buddy_alloc_folder = Path::new(&manifest_dir).join("buddy_alloc_not_for_kernel");

        cc::Build::new()
            .file(
                buddy_alloc_folder
                    .join("buddy_alloc.c")
                    .display()
                    .to_string(),
            )
            .debug(true)
            .compile("buddy_alloc_debug_not_for_kernel");

        cc::Build::new()
            .file(
                buddy_alloc_folder
                    .join("buddy_alloc.c")
                    .display()
                    .to_string(),
            )
            .debug(false)
            .compile("buddy_alloc_release_not_for_kernel");

        if env::var("DEBUG").unwrap() == "true" {
            println!("cargo:rustc-link-lib=static=buddy_alloc_debug_not_for_kernel");
        } else {
            println!("cargo:rustc-link-lib=static=buddy_alloc_release_not_for_kernel");
        }
    }
}
