fn main() {
    let a = 0;
    let b = Duh::a;
    match b {
        n @ Duh::a => println!("{:?}",n as u32),
        Duh::b => panic!(),
    }
}

enum Duh {
    a,
    b(i32),
}