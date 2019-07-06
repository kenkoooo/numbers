use reqwest::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const END_POINT: &str = "http://www.takarakujinet.co.jp/ajax/numbers3/pastResultPage.do";

fn main() {
    let count: u32 = 9999;

    let mut params = HashMap::new();
    params.insert("searchway", "kaigou".to_owned());
    params.insert("year", "2019".to_owned());
    params.insert("month", "7".to_owned());
    params.insert("day", "6".to_owned());
    params.insert("kaigou", count.to_string());
    params.insert("howmany", count.to_string());

    let client = Client::new();
    let mut response = client.post(END_POINT).form(&params).send().unwrap();
    write(response.text().unwrap(), "data3.txt").unwrap();
}

fn write<S: ToString>(s: S, filepath: &str) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(s.to_string().as_bytes())?;
    Ok(())
}
