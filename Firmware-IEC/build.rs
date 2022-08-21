use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn memory_x_handler() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}

fn download_matiec(matiec_dir: &Path) {
    if !matiec_dir.is_dir() {
        assert!(Command::new("git")
            .arg("clone")
            .arg("--")
            .arg("https://github.com/beremiz/matiec.git")
            .arg(matiec_dir)
            .status()
            .unwrap()
            .success());

        assert!(Command::new("git")
            .arg("-C")
            .arg(matiec_dir)
            .arg("switch")
            .arg("--detach")
            .arg("ba00e2b18e7335c03c011e1c6b2a5d99fc3571c3")
            .status()
            .unwrap()
            .success());
    }

    if !matiec_dir.join("iec2c").is_file() {
        assert!(Command::new("autoreconf")
            .current_dir(matiec_dir)
            .arg("-i")
            .status()
            .unwrap()
            .success());

        assert!(Command::new("./configure")
            .current_dir(matiec_dir)
            .status()
            .unwrap()
            .success());

        assert!(Command::new("make")
            .current_dir(matiec_dir)
            .status()
            .unwrap()
            .success());
    }
}

fn build_iec_program(matiec_dir: &Path, iec_build_dir: &Path) {
    assert!(Command::new("rm")
        .arg("-rf")
        .arg(iec_build_dir)
        .status()
        .unwrap()
        .success());
    assert!(Command::new("mkdir")
        .arg(iec_build_dir)
        .status()
        .unwrap()
        .success());

    let source = PathBuf::from(".").canonicalize().unwrap().join("src");

    assert!(Command::new(matiec_dir.join("iec2c"))
        .current_dir(source)
        .arg("-I")
        .arg(matiec_dir.join("lib"))
        .arg("-T")
        .arg(iec_build_dir)
        .arg("-Ol")
        .arg("-n")
        .arg("main.st")
        .status()
        .unwrap()
        .success());

    cc::Build::new()
        .file(iec_build_dir.join("STD_CONF.c"))
        .file(iec_build_dir.join("STD_RESOURCE.c"))
        .include(matiec_dir.join("lib").join("C"))
        .warnings(false)
        .opt_level(0)
        .compile("iec_program");
}

fn main() {
    memory_x_handler();

    let matiec_dir = PathBuf::from(".").canonicalize().unwrap().join("matiec");
    let iec_build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap()
        .join("iec-build");

    download_matiec(&matiec_dir);
    build_iec_program(&matiec_dir, &iec_build_dir);
}
