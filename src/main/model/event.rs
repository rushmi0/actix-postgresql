use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
}
