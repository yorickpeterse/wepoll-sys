extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

const SRC_DIR: &'static str = "wepoll";

fn main() {
    let src_dir = Path::new(&SRC_DIR);

    let out_env_var =
        env::var("OUT_DIR").expect("Failed to obtain the OUT_DIR variable");

    let out_dir = Path::new(&out_env_var);
    let build_dir = out_dir.join("wepoll-build");

    if let Err(err) = fs::remove_dir_all(&build_dir) {
        if err.kind() != io::ErrorKind::NotFound {
            panic!("Failed to remove the build directory: {}", err);
        }
    }

    fs::create_dir(&build_dir).expect("Failed to create the build directory");

    for file in &["wepoll.c", "wepoll.h"] {
        fs::copy(src_dir.join(file), build_dir.join(file))
            .expect(&format!("Failed to copy {} to the build directory", file));
    }

    if cfg!(feature = "compile-wepoll") {
        cc::Build::new()
            .include(&build_dir)
            .out_dir(&build_dir)
            .file(&build_dir.join("wepoll.c"))
            .compile("wepoll");

        println!("cargo:rustc-link-lib=static=wepoll");
        println!("cargo:rustc-link-search={}", &build_dir.display());
    }

    bindgen::Builder::default()
        .header(build_dir.join("wepoll.h").display().to_string())
        .generate()
        .expect("Failed to generate wepoll Rust bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write the Rust bindings");
}
