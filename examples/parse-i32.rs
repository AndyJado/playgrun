fn main() {
    u64_bytes();
    u8_into()
}

fn char_vec() {
    let a = [b'9'; 10];
    // let mut xrs: Vec<char> = a.into_iter().map(|c| c as char).collect();
    let b = String::from_utf8_lossy(&a);
    println!("{b}")
}

fn u64_u8_char() {
    let a = 9999999999u64;
    let b = a.to_string();
    let c = b.bytes().next().unwrap();
    println!("{}", c as char)
}

fn u64_bytes() {
    let mut a = [b'9'; 10];
    let b = String::from_utf8_lossy(&a);
    let c: u64 = b.parse().unwrap();
    let d = (c - 1).to_string();
    let mut iter = d.bytes();
    for i in 0..10 {
        a[i] = iter.next().unwrap()
    }
    dbg!(String::from_utf8_lossy(&a));
}

fn u8_into() {
    let mut a = [b' '; 10];
    a[10] = b'3';
    // let b: String = a.into();
}
