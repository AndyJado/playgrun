use std::fmt;

struct DD;

impl fmt::Debug for DD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

fn main() {
    let a = DD;
    println!("{}", a.to_string())
}