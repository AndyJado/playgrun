union Vac<'a, T> {
    i: &'a [T],
    o: &'a Option<T>,
}

impl<T> Vac<'_, T> {}

fn main() {
    let mut a = vec![1, 2, 3, 4];
    let b = &mut a[..];
    let c = b.last_mut();
    match c {
        Some(e) => e = None,
        None => todo!(),
    }
}
