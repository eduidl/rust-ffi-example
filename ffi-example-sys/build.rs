use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .file("src/c/array.c")
        .file("src/c/string.c")
        .include("src/c")
        .compile("libffi_tips.a");

    let bindings = bindgen::Builder::default()
        .header("src/c/all.h")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
