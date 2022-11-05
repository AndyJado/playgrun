fn main() {
    let b: Option<String> = None;
    let ga: u32 = match Some(b).unwrap() {
        Some(_) => 34,
        None => continue,
    }
}