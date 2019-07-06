use reqwest::{Client, Result};
use std::collections::HashMap;

fn download_numbers_data(count: usize, url: &str) -> Result<String> {
    let mut params = HashMap::new();
    params.insert("searchway", "kaigou".to_owned());
    params.insert("year", "2019".to_owned());
    params.insert("month", "7".to_owned());
    params.insert("day", "6".to_owned());
    params.insert("kaigou", count.to_string());
    params.insert("howmany", count.to_string());
    let client = Client::new();
    client.post(url).form(&params).send()?.text()
}

pub fn download3(count: usize) -> Result<String> {
    let end_point = "http://www.takarakujinet.co.jp/ajax/numbers3/pastResultPage.do";
    download_numbers_data(count, end_point)
}
pub fn download4(count: usize) -> Result<String> {
    let end_point = "http://www.takarakujinet.co.jp/ajax/numbers4/pastResultPage.do";
    download_numbers_data(count, end_point)
}
