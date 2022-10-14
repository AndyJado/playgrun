fn main() {
    let ko = || {};
}

struct La<T: Fn() -> ()> {
    la: Box<T>,
}

impl<T: Fn() -> ()> La<T> {
    fn new(ga: T) -> Self {
        let la = Box::new(ga);
        La { la }
    }
}
