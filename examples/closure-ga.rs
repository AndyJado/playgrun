fn main() {
    let ko = || {};
}

struct La {
    la: Box<dyn FnMut() -> ()>,
}

impl La {
    fn new() -> Self {
        let la = Box::new(|| {});
        La { la }
    }
}
