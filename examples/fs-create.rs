use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    dir_without_prefix()
}

fn dir_without_prefix() {
    let a = PathBuf::from("foos");
    if !a.exists() {
        fs::create_dir(&a).unwrap()
    }
}

fn create2ok() {
    let p = Path::new("foos");
    let f_ok = fs::create_dir(p).ok();
    if f_ok.is_none() {
        println!("already therer!")
    } else {
        println!("see!")
    }
}
