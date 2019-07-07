use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn write_file<S: ToString>(s: S, filepath: &str) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(s.to_string().as_bytes())?;
    Ok(())
}

pub fn read_file(filepath: &str) -> std::io::Result<String> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}
