use numbers;

use serde_json;

fn main() {
    eprintln!("Downloading Numbers3 data...");
    let contents = numbers::download3(9999).unwrap();
    numbers::util::write(contents, "assets/data3.txt").unwrap();
    let data = numbers::load_data("assets/data3.txt");
    numbers::util::write(serde_json::to_string(&data).unwrap(), "assets/data3.json").unwrap();

    eprintln!("Downloading Numbers4 data...");
    let contents = numbers::download4(9999).unwrap();
    numbers::util::write(contents, "assets/data4.txt").unwrap();
    let data = numbers::load_data("assets/data4.txt");
    numbers::util::write(serde_json::to_string(&data).unwrap(), "assets/data4.json").unwrap();
    eprintln!("Done");
}
