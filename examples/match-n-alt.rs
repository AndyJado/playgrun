fn main() {
    let a = [1, 2, 3, 4, 5];
    match a {
        [ref ga @ .., 5] => {}
        _ => {}
    }
}
