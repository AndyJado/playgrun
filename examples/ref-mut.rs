fn main() {
    foo()
}

fn foo() {
    let mut a = 3;
    let mut aa = &mut a;
    let aaa = &mut aa;
    println!("{aaa}")
}
