#![allow(dead_code)]
use std::path::Path;
use std::path::PathBuf;
use std::fs;

pub fn browse(dir:&Path) -> Vec<PathBuf> {
    let mut list = Vec::new();
    for r in fs::read_dir(dir).unwrap() {
        match r {
            Err(e) => eprintln!("{}: Failed to list directory because of an error: {:?}", dir.display(), e.kind()),
            Ok(e) => list.push(e.path()),
        };
    };
    list
}