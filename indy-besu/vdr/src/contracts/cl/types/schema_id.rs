use crate::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaId {
    value: String,
}

impl SchemaId {
    const ID_PATH: &'static str = "anoncreds/v0/SCHEMA";

    pub fn build(issuer_id: &DID, name: &str, version: &str) -> SchemaId {
        let schema_id = SchemaId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.deref(),
                Self::ID_PATH,
                name,
                version
            )
            .as_str(),
        );

        trace!("Created new SchemaId: {:?}", schema_id);

        schema_id
    }
}

impl From<&str> for SchemaId {
    fn from(id: &str) -> Self {
        let schema_id = SchemaId {
            value: id.to_string(),
        };

        trace!("Created new SchemaId: {:?}", schema_id);

        schema_id
    }
}

impl Deref for SchemaId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
