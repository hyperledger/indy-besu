use crate::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "uni_ffi", derive(uniffi::Record))]
pub struct SchemaId {
    value: String,
}

impl SchemaId {
    const ID_PATH: &'static str = "anoncreds/v0/SCHEMA";

    pub fn new(id: &str) -> SchemaId {
        let schema_id = SchemaId {
            value: id.to_string(),
        };

        trace!("Created new SchemaId: {:?}", schema_id);

        schema_id
    }

    pub fn build(issuer_id: &DID, name: &str, version: &str) -> SchemaId {
        let schema_id = SchemaId::new(&format!(
            "{}/{}/{}/{}",
            issuer_id.value(),
            ID_PATH,
            name,
            version
        ));

        trace!("Created new SchemaId: {:?}", schema_id);

        schema_id
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
