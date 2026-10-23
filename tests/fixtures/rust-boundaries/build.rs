use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=ffi.c");
    if env::var_os("CARGO_FEATURE_STD").is_none() {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    let object = out.join("ffi.o");
    assert!(Command::new("cc")
        .args(["-std=c11", "-c", "ffi.c", "-o"])
        .arg(&object)
        .status()
        .expect("fixture requires cc")
        .success());
    assert!(Command::new("ar")
        .arg("crs")
        .arg(out.join("libfixture.a"))
        .arg(object)
        .status()
        .expect("fixture requires ar")
        .success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=fixture");
}
