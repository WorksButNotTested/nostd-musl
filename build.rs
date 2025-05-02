use std::{env, fs, path::PathBuf};

fn compile(file: &str, output: &str) {
    cc::Build::new()
        .define("_GNU_SOURCE", None)
        .opt_level(3)
        .flag("-Werror")
        .flag("-fno-stack-protector")
        .flag("-U_FORTIFY_SOURCE")
        .flag("-D_FORTIFY_SOURCE=0")
        .flag("-ffunction-sections")
        .flag("-Wa,--noexecstack")
        .include("include/")
        .file(file)
        .compile(output);
}

fn build_bindings() {
    let mut builder = bindgen::Builder::default()
        .use_core()
        .blocklist_type("max_align_t");
    let header_dir = PathBuf::from("include");
    for entry in fs::read_dir(&header_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("h") {
            builder = builder.header(path.to_string_lossy());
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn main() {
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=src");
    let target = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    compile("src/memcmp.c", "memcmp");

    let mut memcpy = "src/memcpy.c";
    let mut memmove = "src/memmove.c";
    let mut memset = "src/memset.c";

    if cfg!(feature = "optimized-assembly") {
        match target.as_str() {
            "aarch64" => {
                memcpy = "src/aarch64/memcpy.S";
                memset = "src/aarch64/memset.S";
            }
            "arm" => {
                memcpy = "src/arm/memcpy.S";
            }
            "x86" => {
                memcpy = "src/i386/memcpy.s";
                memmove = "src/i386/memmove.s";
                memset = "src/i386/memset.s";
            }
            "x86_64" => {
                memcpy = "src/x86_64/memcpy.s";
                memmove = "src/x86_64/memmove.s";
                memset = "src/x86_64/memset.s";
            }
            _ => {}
        }
    }

    compile(memcpy, "memcpy");
    compile(memmove, "memmove");
    compile(memset, "memset");

    compile("src/strlen.c", "strlen");

    build_bindings();
}
