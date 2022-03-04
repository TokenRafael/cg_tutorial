extern crate walkdir;

use std::env;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

    let exe_path = locate_target(Path::new(&out_dir))
        .expect("failed to find target dir")
        .join(env::var("PROFILE").unwrap());

    copy(
        &manifest_dir.join("shaders"),
        &exe_path.join("shaders")
    )
}

fn locate_target(mut target_dir: &Path) -> Option<&Path> {
    loop {
        if target_dir.ends_with("target") {
            return Some(target_dir);
        }

        target_dir = match target_dir.parent() {
            Some(parent) => parent,
            None => break,
        };
    }

    None
}

fn copy(from: &Path, to: &Path) {
    let from: PathBuf = from.into();
    let to: PathBuf = to.into();
    for entry in WalkDir::new(&from) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from) {
            let target_path = to.join(rel_path);
            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("failed to create target dir");
            } else {
                std::fs::copy(entry.path(), &target_path).expect("failed to copy file");
            }
        }

    }
}