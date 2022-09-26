use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Cursor, Result, Seek, SeekFrom, Write},
};

fn main() -> Result<()> {
    fs::write(
        "foo.md",
        b"
        ldkasjdlsajldjl
        dladjsadlkasdj
        aldjkdsakldj
        ",
    )?;
    let f = fs::read_to_string("foo.md")?;
    let mut cursor = Cursor::new(f);
    let mut buf = String::new();
    let num_bytes = cursor.read_line(&mut buf).expect("!read line");
    dbg!(buf, num_bytes);
    let mut buf = String::new();
    let num_bytes = cursor.read_line(&mut buf).expect("!read line");
    dbg!(buf, num_bytes);

    Ok(())
}
