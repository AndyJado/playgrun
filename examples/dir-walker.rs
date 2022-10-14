use std::path::PathBuf;

fn main() {
    let dir_boy = DirWalker::new();
    println!("{:?}", dir_boy.key_paf_vec(PathBuf::from(".")))
}

type WalkDir = Box<dyn Fn(std::path::PathBuf) -> std::fs::ReadDir>;

pub struct DirWalker {
    iter: WalkDir,
}

impl DirWalker {
    pub fn new() -> Self {
        let iter = Box::new(|p| std::fs::read_dir(p).expect("dir should exists"));
        DirWalker { iter }
    }
    pub fn key_paf_vec(&self, p: std::path::PathBuf) -> Vec<std::path::PathBuf> {
        let a = &self.iter;
        let b = a(p);
        b.filter_map(|c| c.ok()).map(|e| e.path()).collect()
    }
}
