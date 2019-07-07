use crate::util;
use crate::NumbersData;

use regex::Regex;

fn parse_numbers_file(filename: &str) -> Vec<Vec<String>> {
    let re = Regex::new(r"<tr.*?/tr>").unwrap();
    let td = Regex::new(r"<td.*?/td>").unwrap();
    let br = Regex::new(r"<br\s*/>").unwrap();
    let td_c = Regex::new(r"^<td.*?>(.*?)</td>$").unwrap();
    let space = Regex::new(r"[\r\s]+").unwrap();

    let s = util::read_file(filename).unwrap().replace("\n", " ");
    let mut result = vec![];
    let mut pos = 0;
    while let Some(m) = re.find_at(&s, pos) {
        let mut one = vec![];
        for td in td.find_iter(m.as_str()) {
            let s = br.replace_all(td.as_str(), " ");
            let s = td_c.replace_all(&s, "$1");
            let s = space.replace_all(&s, " ");

            one.push(s.trim().to_string());
        }
        if !one.is_empty() {
            result.push(one);
        }
        pos = m.end();
    }
    result
}

pub fn load_data(filepath: &str) -> Vec<NumbersData> {
    let parsed_result = parse_numbers_file(filepath);
    parsed_result
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, row)| {
            let straight_count = String::from_utf8(
                row[3]
                    .chars()
                    .take_while(|&c| c.is_numeric())
                    .map(|c| c as u8)
                    .collect::<Vec<_>>(),
            )
            .ok()
            .and_then(|s| s.parse::<u32>().ok());

            let box_count = String::from_utf8(
                row[4]
                    .chars()
                    .take_while(|&c| c.is_numeric())
                    .map(|c| c as u8)
                    .collect::<Vec<_>>(),
            )
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
            NumbersData {
                id: row[0].parse().unwrap(),
                date: row[1].clone(),
                result: row[2].clone(),
                straight_count,
                box_count,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        let data = load_data("assets/data3.txt");
        let d1 = data.iter().find(|d| d.id == 1).unwrap();
        assert_eq!(d1.result, "191".to_string());
        let d1 = data.iter().find(|d| d.id == 5214).unwrap();
        assert_eq!(d1.result, "782".to_string());

        let data = load_data("assets/data4.txt");
        let d1 = data.iter().find(|d| d.id == 5214).unwrap();
        assert_eq!(d1.result, "2571".to_string());
    }
}
