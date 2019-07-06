use std::fs::File;
use std::io::prelude::*;

pub fn write<S: ToString>(s: S, filepath: &str) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(s.to_string().as_bytes())?;
    Ok(())
}
