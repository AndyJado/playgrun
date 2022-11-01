fn main() {
    let a = La {
        a: "a".to_string(),
        b: "b".to_owned(),
    };
    let La { a, b } = a;
    println!("{a},{b}")
}

struct La {
    a: String,
    b: String,
}
