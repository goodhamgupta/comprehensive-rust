// Read and BufRead can abstract over u8 sources?

use std::io::{BufRead, BufReader, Read, Result};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn main() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let cur = std::env::current_exe()?;
    println!("{}", cur.display());
    let file = std::fs::File::open(cur)?;
    println!("lines in file: {}", count_lines(file));
    Ok(())
}
