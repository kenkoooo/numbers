use chrono::prelude::*;
use numbers;
use numbers::NumbersData;
use serde_json;
use std::collections::BTreeMap;

const COPY_EXCLUDED: [&str; 1] = ["id"];

fn main() {
    let s = numbers::util::read_file("assets/data3.json").unwrap();
    let mut data: Vec<NumbersData> = serde_json::from_str(&s).unwrap();
    data.sort_by_key(|data| data.id);

    let mut maps = data
        .into_iter()
        .map(|data| {
            let mut map = BTreeMap::new();
            copy_data(&data, &mut map);
            process_date(&data, &mut map);
            process_result(&data, &mut map);
            map
        })
        .collect::<Vec<_>>();

    let k = 100;
    for i in (k..maps.len()).rev() {
        for k in 1..k {
            let prev = maps[i - k]
                .iter()
                .filter(|(k, _)| !COPY_EXCLUDED.contains(&k.as_str()))
                .map(|(key, value)| {
                    let key = format!("prev_{k}_{key}", k = k, key = key);
                    (key, *value)
                })
                .collect::<Vec<_>>();
            maps[i].extend(prev);
        }
    }

    let maps = maps
        .into_iter()
        .map(|map| post_process(map))
        .collect::<Vec<_>>();

    numbers::util::write_file(
        serde_json::to_string(&maps).unwrap(),
        "./preprocess_data3.json",
    )
    .unwrap();
}

fn copy_data(data: &NumbersData, out: &mut BTreeMap<String, f64>) {
    out.insert("id".to_string(), data.id as f64);
    if let Some(straight_count) = data.straight_count {
        out.insert("leak_straight_count".to_string(), straight_count as f64);
    }
    if let Some(box_count) = data.box_count {
        out.insert("leak_box_count".to_string(), box_count as f64);
    }
}

fn post_process(map: BTreeMap<String, f64>) -> BTreeMap<String, f64> {
    map.into_iter()
        .filter(|(key, _)| !key.starts_with("leak_"))
        .collect()
}

fn process_result(data: &NumbersData, out: &mut BTreeMap<String, f64>) {
    let result: usize = data.result.parse().unwrap();
    let d0 = result / 1000;
    let d1 = (result / 100) % 10;
    let d2 = (result / 10) % 10;
    let d3 = result % 10;

    out.insert("d1".to_string(), d1 as f64);
    out.insert("d2".to_string(), d2 as f64);
    out.insert("d3".to_string(), d3 as f64);

    let mut count = vec![0; 10];
    if data.result.len() == 3 {
        // Numbers3
        assert_eq!(d0, 0);
    } else {
        // Numbers4
        count[d0] += 1;
        out.insert("d0".to_string(), d0 as f64);
    }
    count[d1] += 1;
    count[d2] += 1;
    count[d3] += 1;
    for i in 0..10 {
        let label = format!("leak_count_{}", i);
        out.insert(label, count[i] as f64);
    }
}

fn process_date(data: &NumbersData, out: &mut BTreeMap<String, f64>) {
    let (year, month, day) = {
        let mut iter = data.date.split(" ");
        let year = iter.next().unwrap().parse::<i32>().unwrap();
        let mut date = iter.next().unwrap().split("/");
        let month = date.next().unwrap().parse::<u32>().unwrap();
        let day = date.next().unwrap().parse::<u32>().unwrap();
        (year, month, day)
    };

    let date = Utc.ymd(year, month, day);
    let this_year = Utc.ymd(year, 1, 1);
    let next_year = Utc.ymd(year + 1, 1, 1);
    let date_ratio =
        (date - this_year).num_seconds() as f64 / (next_year - this_year).num_seconds() as f64;
    let num_days_from_monday = date.weekday().num_days_from_monday();
    let day_of_week_ratio = num_days_from_monday as f64 / 7.0;

    out.insert("year".to_string(), year as f64);
    out.insert("month".to_string(), month as f64);
    out.insert("day".to_string(), day as f64);
    out.insert("date_ratio".to_string(), date_ratio);
    out.insert("date_ratio_sin".to_string(), make_sin(date_ratio));
    out.insert("date_ratio_cost".to_string(), make_cos(date_ratio));
    out.insert(
        "num_days_from_monday".to_string(),
        num_days_from_monday as f64,
    );
    out.insert("day_of_week_ratio".to_string(), day_of_week_ratio);
    out.insert(
        "day_of_week_ratio_sin".to_string(),
        make_sin(day_of_week_ratio),
    );
    out.insert(
        "day_of_week_ratio_cos".to_string(),
        make_cos(day_of_week_ratio),
    );
}

fn make_sin(ratio: f64) -> f64 {
    assert!(0.0 <= ratio && ratio <= 1.0, "{}", ratio);
    (ratio * std::f64::consts::PI * 2.0).sin()
}

fn make_cos(ratio: f64) -> f64 {
    assert!(0.0 <= ratio && ratio <= 1.0, "{}", ratio);
    (ratio * std::f64::consts::PI * 2.0).cos()
}
