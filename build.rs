use std::path::{ Path };
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=deps/minhook/Debug/");

    if !Path::new("curl/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let _ = Command::new("cmake")
        .args(&["-S", "minhook/", "-B", "minook/"])
        .status();

    let _ = Command::new("cmake")
        .args(&["--build", "minhook/"])
        .status();
}