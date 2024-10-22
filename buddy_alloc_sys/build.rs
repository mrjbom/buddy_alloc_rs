use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let buddy_alloc_folder =
        Path::new(&manifest_dir).join("buddy_alloc_1.2.0_x86_64-elf-gcc_freestanding");
    println!(
        "cargo:rustc-link-search=native={}",
        buddy_alloc_folder.display()
    );
    #[cfg(debug_assertions)]
    println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_debug");
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-lib=static=buddy_alloc_1.2.0_release");

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
