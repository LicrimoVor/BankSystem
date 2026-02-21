// build.rs

use bindgen;
use cc;
use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::builder()
        // Файл, для которого создаются байндинги
        .header("src/mylib.h")
        // Перезапуск сборки при изменении переданных файлов
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Сгенерировать байндинги
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        // Записать получившиеся байндинги в файл OUT_DIR/bindgen.rs
        .write_to_file(out_path.join("bindgen.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        // добавить src/mylib.c в выходную библиотеку
        .file("src/mylib.c")
        // скомпилировать C-код как библиотеку libmylib.a в папке OUT_DIR
        .compile("mylib");
}
