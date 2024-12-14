use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use rand::{random, Rng};
use serde_json::json;
use log::info;

use crate::storage::db_config::query_task;

pub async fn greet(name: &str) -> String {
    info!("Receive data: {}", name);

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    // let random_number: u64 = random();
    // let event_id_hash = Sha256::digest(random_number.to_string().as_bytes());
    // let event_id = format!("{:x}", event_id_hash);
    //
    // let kind: u32 = rand::thread_rng().gen_range(1..=100);
    // let tags = json!([]);

    format!("Hello {}!", name)
}
