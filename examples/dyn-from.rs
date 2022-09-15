struct Struct2 {
    field: Box<dyn Trait2>,
}

struct Struct {
    field: Box<dyn Trait>,
}

trait Trait {}

trait Trait2 {}

fn main() {
    println!("helo, watching ?")
}
