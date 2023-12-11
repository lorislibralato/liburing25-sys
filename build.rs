use std::{env, path::Path, process::Command};

fn main() {
    let out = Path::new(&env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap();

    let manifest = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();

    if out.join("liburing").is_dir() {
        std::fs::remove_dir_all(out.join("liburing")).unwrap();
    }

    copy_dir(manifest.join("liburing"), out.join("liburing")).unwrap();

    Command::new("sh")
        .current_dir(out.join("liburing"))
        .arg("-c")
        .arg("./configure --cc=clang --cxx=clang++ --use-libc && cd src && make V=1 CFLAGS=\"-g -O3 -Wall -Wextra -fno-stack-protector -flto=thin\" liburing-ffi.a")
        .status()
        .unwrap();

    std::fs::copy(
        out.join("liburing/src/liburing-ffi.a"),
        out.join("liburing25-sys.a"),
    )
    .unwrap();

    println!("cargo:rustc-link-lib=static:+verbatim=liburing25-sys.a");
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let dst = dst.as_ref();
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let subpath = entry.file_name();
        if entry.file_type()?.is_dir() {
            copy_dir(entry.path(), dst.join(subpath))?;
        } else {
            std::fs::copy(entry.path(), dst.join(subpath))?;
        }
    }
    Ok(())
}
