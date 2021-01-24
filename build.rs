extern crate cc;

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let out_dir = PathBuf::from(env::var("OUT_DIR").ok().expect("OUT_DIR not set"));
    let target = env::var("TARGET").ok().expect("TARGET not set");
    let host = env::var("HOST").ok().expect("HOST not set");
    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("libfixmath/libfixmath");

    let mut cconfig = cc::Build::new();
    cconfig
        .target(&target)
        .host(&host)
        .warnings(false)
        .opt_level(2)
        .include(&source_dir)
        .file(source_dir.join("fix16.c"))
        .file(source_dir.join("fix16_exp.c"))
        .file(source_dir.join("fix16_sqrt.c"))
        .file(source_dir.join("fix16_str.c"))
        .file(source_dir.join("fix16_trig.c"))
        .file(source_dir.join("fract32.c"))
        .file(source_dir.join("uint32.c"))
        .compile("fixmath");


    let bindings = bindgen::Builder::default()
        .header(source_dir.join("fixmath.h").to_str().expect("Unable to turn fixmath.h path into string"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
