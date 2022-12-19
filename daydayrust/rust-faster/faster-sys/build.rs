// Credit to <https://github.com/rust-rocksdb/rust-rocksdb/blob/master/librocksdb-sys/build.rs>
use std::path::{Path, PathBuf};
use std::borrow::Borrow;
use std::process::Command;
use std::ffi::OsStr;
use std::env;

fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S]) 
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>
{
    let cmd = if cmd.as_ref().is_relative() && cmd.as_ref().components().count() > 1 {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("Can canonicalize")
    } else {
        PathBuf::from(cmd.as_ref())
    };

    println!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );

    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),

    }
}

fn bindgen_faster() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("faster/cc/src/core/faster-c.h")
        // <https://github.com/rust-lang/rust-bindgen/issues/550>
        .blocklist_type("max_align")
        .ctypes_prefix("libc")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Write faster bindings");       
}

// cmake build
fn build_faster() {
    let mut config = cmake::Config::new("faster/cc");

    config
        // .define("CMAKE_BUILD_TYPE", "Release")
        .cflag("--std=c++11");

    println!("Configuring and compiling faster");
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/{}", dst.display(), "build");
    println!("cargo:rustc-link-lib=static=faster");
}

fn main() {
    if !Path::new("./faster/LICENSE").exists() {
        run_command_or_fail("../../", "git", &["submodule", "update", "--init"]);
    }

    println!("cargo:rerun-if-changed=build.rs");
    // Tell cargo to invalidate the built crate whenever the faster changes
    println!("cargo:rerun-if-changed=faster/");

    bindgen_faster();
    build_faster();

    println!("cargo:rustc-link-lib=stdc++fs");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=tbb");
    println!("cargo:rustc-link-lib=gcc");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=aio");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");

    // Allow dependent crates to locate the sources and output directory of
    // this crate. Notably, this allows a dependent crate to locate the faster
    // sources and built archive artifacts provided by this crate.
    // println!(
    //     "cargo:cargo_manifest_dir={}",
    //     env::var("CARGO_MANIFEST_DIR").unwrap()
    // );
    // println!("cargo:out_dir={}", env::var("OUT_DIR").unwrap());
}