use std::env;
use std::io::Error;
use std::path::PathBuf;

macro_rules! polyhook2_path {
    ($l:literal) => {
        concat!("polyhook2/", $l)
    };
}

fn link() -> Result<(), Error> {
    println!("cargo:rustc-link-search=native=polyhook2-sys/{}", polyhook2_path!("lib"));
    println!("cargo:rustc-link-lib=static=PolyHook_2");
    Ok(())
}

#[cfg(feature = "bindgen")]
fn generate_bindings() -> Result<(), Error> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .clang_arg(format!("-I{}", polyhook2_path!("include")))
        .opaque_type("std::.*")
        .allowlist_type("(PLH|plh)(u|U)?.*")
        .allowlist_function("(PLH|plh)(u|U)?.*")
        .allowlist_var("(PLH|plh)(u|U)?.*")
        .ctypes_prefix("cty")
        .use_core()
        .layout_tests(false)
        .size_t_is_usize(true)
        .enable_cxx_namespaces()
        .header(polyhook2_path!("wrapper.h"))
        .generate()
        .expect("Failed to generate bindings.")
        .write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings() -> Result<(), Error> {
    Ok(())
}

fn main() -> Result<(), Error> {
    generate_bindings()?;
    link()?;
    Ok(())
}