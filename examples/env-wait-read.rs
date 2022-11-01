use std::io::{self, BufRead};


fn main() ->io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;
    println!("{:?}",handle.lines().next().expect("string from stdin locker"));
    Ok(())
}