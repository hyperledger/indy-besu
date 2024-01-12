use crate::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaId(String);

impl SchemaId {
    const ID_PATH: &'static str = "anoncreds/v0/SCHEMA";

    pub fn build(issuer_id: &DID, name: &str, version: &str) -> SchemaId {
        let schema_id = SchemaId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.as_ref(),
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
        SchemaId(id.to_string())
    }
}

impl AsRef<str> for SchemaId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for SchemaId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
