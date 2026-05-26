use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    //println!("cargo:rustc-link-lib=static=c++");
    let lean_project = PathBuf::from("/Users/pingfanh/project/lnmai-core-ffi");

    // let status = Command::new("lake")
    //     .arg("build")
    //     .current_dir(&lean_project)
    //     .status()
    //     .expect("failed to run lake build");
    // if !status.success() {
    //     panic!("lake build failed");
    // }

    let _lean_include = PathBuf::from("/Users/pingfanh/.elan/toolchains/leanprover--lean4---v4.30.0-rc2/include");

    // cc::Build::new()
    //     .file("/Users/pingfanh/project/lnmai-core-ffi/include/lnmai_ffi.h")
    //     .file("/Users/pingfanh/project/lnmai-core-ffi/include/lnmai_session.h")
    //     .include(&_lean_include)
    //     .compile("lean_wrapper");

    let lake_rsp = lean_project.join(".lake/build/bin/lnmai-core-ffi.rsp");

    let rsp_content = fs::read_to_string(&lake_rsp)
        .unwrap_or_else(|e| panic!("failed to read Lake rsp {:?}: {}", lake_rsp, e));

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let rsp_path = out_dir.join("link.rsp");

    let mut combined = String::new();
    let lines: Vec<&str> = rsp_content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line == "\"-fuse-ld=lld\"" || line == "-fuse-ld=lld" {
            i += 1;
            continue;
        }
        if line == "\"--sysroot\"" || line == "--sysroot" {
            i += 2;
            continue;
        }
        if line.contains("Main.c.o.export") {

            i += 1;

            continue;

        }
        combined.push_str(lines[i]);
        combined.push('\n');
        i += 1;
    }
    fs::write(&rsp_path, &combined).expect("failed to write rsp file");

    println!("cargo:rustc-link-arg=@{}", rsp_path.display());

    // let lean_lib = lean_project.join(".lake/build/lib/lean");
    //
    // combined.push_str(&format!("-L{}\n", lean_lib.display()));
    //
    // combined.push_str("-lleanrt\n");
    //
    // combined.push_str("-lleancore\n"); // 很多情况下必须
    //
    // combined.push_str("\"-lleanshared\"\n");

    println!("cargo:rerun-if-changed=/Users/pingfanh/project/lnmai-core-ffi/lakefile.toml");
    println!("cargo:rerun-if-changed=/Users/pingfanh/project/lnmai-core-ffi/LnmaiCore");
    println!("cargo:rerun-if-changed=/Users/pingfanh/project/lnmai-core-ffi/Proofs");
    println!("cargo:include=/Users/pingfanh/project/lnmai-core-ffi/include");

    println!("cargo:rustc-link-search=/Users/pingfanh/project/lnmai-core-ffi/lib");
}
 