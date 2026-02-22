use bindgen;
use cc;
use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::builder()
        .headers(["cJSON.h", "error.h"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindgen.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .files(["cJSON.c", "error.c"])
        .compile("mylib");
}
