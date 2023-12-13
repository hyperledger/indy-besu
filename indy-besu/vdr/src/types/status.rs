use serde_derive::{Deserialize, Serialize};

/// Ledger status:  whether connected node and network are alive
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PingStatus {
    pub status: Status,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Ok,
    Err(String),
}

impl PingStatus {
    pub fn ok() -> PingStatus {
        PingStatus { status: Status::Ok }
    }

    pub fn err(err: &str) -> PingStatus {
        PingStatus {
            status: Status::Err(err.to_string()),
        }
    }
}
