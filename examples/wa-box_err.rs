use std::error::Error;

fn main() {}

fn foo() -> Result<u8, Box<dyn Error>> {
    let (a, b): (u8, u32);
    Ok(3)
}
