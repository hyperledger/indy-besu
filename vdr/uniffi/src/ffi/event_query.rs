use indy2_vdr::{Address, Block, EventLog as EventLog_, EventQuery as EventQuery_};

#[derive(uniffi::Object)]
pub struct EventQuery {
    pub query: EventQuery_,
}

#[uniffi::export]
impl EventQuery {
    #[uniffi::constructor]
    pub fn new(
        address: String,
        from_block: Option<u64>,
        to_block: Option<u64>,
        event_signature: Option<String>,
        event_filter: Option<String>,
    ) -> EventQuery {
        EventQuery {
            query: EventQuery_ {
                address: Address::from(address.as_str()),
                from_block: from_block.map(Block::from),
                to_block: to_block.map(Block::from),
                event_signature,
                event_filter,
            },
        }
    }
}

impl From<EventQuery_> for EventQuery {
    fn from(query: EventQuery_) -> Self {
        EventQuery { query }
    }
}

#[derive(uniffi::Record)]
pub struct EventLog {
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl From<EventLog_> for EventLog {
    fn from(log: EventLog_) -> Self {
        EventLog {
            topics: log
                .topics
                .into_iter()
                .map(|topic| topic.0.to_vec())
                .collect(),
            data: log.data,
        }
    }
}

impl Into<EventLog_> for EventLog {
    fn into(self) -> EventLog_ {
        EventLog_::new(self.topics, self.data)
    }
}
