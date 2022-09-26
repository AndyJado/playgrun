use std::str::Chars;

fn main() {
    let a = "123456789";
    find_keyword(a.chars())
}

fn find_keyword(mut chars: Chars) {
    let mut h = || chars.next();
    match h() {
        Some(x) => match x {
            '5' => match h() {
                Some(x_bhd5) => match x_bhd5 {
                    '6' => println!("{}", h().unwrap()),
                    _ => {}
                },
                None => todo!(),
            },
            _ => find_keyword(chars),
        },
        None => todo!(),
    }
}
