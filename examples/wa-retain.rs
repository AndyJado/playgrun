fn main() {}

fn retain_vec() {
    let mut a: Vec<_> = vec![Some(1), Some(2), None];
    a.retain(|&c| c.is_some());
    dbg!(a);
}
