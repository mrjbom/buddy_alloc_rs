use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let buddy_alloc_folder =
        Path::new(&manifest_dir).join("buddy_alloc_1.2.0_x86_64-elf-gcc_freestanding");
    println!(
        "cargo:rustc-link-search=native={}",
        buddy_alloc_folder.display()
    );
    #[cfg(all(target_arch = "x86_64", target_vendor = "unknown", target_os = "none"))]
    {
        #[cfg(debug_assertions)]
        println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_debug");
        #[cfg(not(debug_assertions))]
        println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_release");
        println!("cargo:rustc-link-arg=-fno-stack-protector");
    }
    // For everything except the kernel, we try to compile and link library
    // This may not work correctly with sys binding, since it is generated for the kernel, but technically it should work on any x86_64 Linux and Windows machine
    #[cfg(not(all(target_arch = "x86_64", target_vendor = "unknown", target_os = "none")))]
    {
        let buddy_alloc_folder = Path::new(&manifest_dir).join("buddy_alloc_1.2.0_not_for_kernel");

        cc::Build::new()
            .file(
                buddy_alloc_folder
                    .join("buddy_alloc_1.2.0.c")
                    .display()
                    .to_string(),
            )
            .debug(true)
            .compile("buddy_alloc_1.2.0_debug_not_for_kernel");

        cc::Build::new()
            .file(
                buddy_alloc_folder
                    .join("buddy_alloc_1.2.0.c")
                    .display()
                    .to_string(),
            )
            .debug(false)
            .compile("buddy_alloc_1.2.0_release_not_for_kernel");

        #[cfg(debug_assertions)]
        println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_debug_not_for_kernel");
        #[cfg(not(debug_assertions))]
        println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_release_not_for_kernel");
    }

    /*
    // bindgen crate can't do this, but bindgen-cli can
    // Looks like bindgen crate bug, lol

    // Bindgen
    // Clang will use gcc headers
    let include_paths_string = env::var("CLANG_GCC_INCLUDE_PATH")
        .expect(
            concat!(
                "CLANG_GCC_INCLUDE_PATH env variable not present\n",
                "Example CLANG_GCC_INCLUDE_PATH=gcc_include_path1:gcc_include_path2"
            )
        );
    let include_paths: Vec<&str> = include_paths_string.split(':').collect();
    let include_paths_with_isystem: Vec<String> = include_paths.iter()
        .map(|path| {
            let mut s = String::from("-isystem ");
            s.push_str(path);
            s
        })
        .collect();

    let buddy_alloc_header_path = Path::new(&manifest_dir).join("buddy_alloc_1.2.0_x86_64-elf-gcc_freestanding").join("buddy_alloc_1.2.0.c.h");
    let builder = bindgen::Builder::default()
        .clang_args(["--target=x86_64-elf", "-ffreestanding", "-fno-builtin", "-nostdinc"])
        .clang_args(&include_paths_with_isystem)
        .header(buddy_alloc_header_path.display().to_string());

    builder.generate()
        .expect("Failed to generate binding");
    */
}
