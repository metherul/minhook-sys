use std::path::{ Path };
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=minhook/cmake/Debug/");

    if !Path::new("curl/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let _ = Command::new("cmake")
        .args(&["-S", "minhook/", "-B", "minhook/cmake/"])
        .status();

    let _ = Command::new("cmake")
        .args(&["--build", "minhook/cmake/"])
        .status();
}