use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub enum ThreeNResponse {
    Solve { from: u64, to: u64 },
    Ok,
}

pub struct ThreeNState {
    pub from: u64,
    pub to: u64,
    pub increment: u64,
}

pub type ClientName = String;
pub type ThreeNResult = String;

pub type Timestamp = i64;

pub fn current_timestamp() -> Timestamp {
    Utc::now().timestamp()
}
