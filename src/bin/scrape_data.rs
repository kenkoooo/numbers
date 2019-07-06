use numbers::load_data;

use std::fs::File;
use std::io::prelude::*;

use serde_json;

fn main() {
    let data3 = load_data("assets/data3.txt");
    write(serde_json::to_string(&data3).unwrap(), "assets/data3.json").unwrap();
    let data4 = load_data("assets/data4.txt");
    write(serde_json::to_string(&data4).unwrap(), "assets/data4.json").unwrap();
}

fn write<S: ToString>(s: S, filepath: &str) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(s.to_string().as_bytes())?;
    Ok(())
}
