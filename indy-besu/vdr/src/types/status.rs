/// Ledger status:  whether connected node and network are alive
#[derive(Debug, PartialEq, uniffi::Record)]
pub struct PingStatus {
    pub status: Status,
}

#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum Status {
    Ok,
    Err {
        msg: String
    },
}

impl PingStatus {
    pub fn ok() -> PingStatus {
        PingStatus { status: Status::Ok }
    }

    pub fn err(err: &str) -> PingStatus {
        PingStatus {
            status: Status::Err {
                msg: err.to_string()
            },
        }
    }
}
