// let..else stable today!!
fn main() {
    let b = Some(3);
    let Some(a) = b else {
        panic!();
    };
    println!("{a}")
}
