use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersData {
    pub id: u32,
    pub date: String,
    pub result: String,
    pub straight_count: Option<u32>,
    pub box_count: Option<u32>,
}
