fn main() {
    let mut a = Some(Some(1));
    let b = a.take();
    println!("{:?}", b)
}
