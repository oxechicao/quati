use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

pub fn write_file(filename: &str, content: &str) -> Result<()> {
    println!("writing file");
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
}

pub fn rm_file(filename: &str) -> Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}
