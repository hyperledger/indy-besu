use crate::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, uniffi::Record)]
pub struct CredentialDefinitionId {
    value: String,
}

impl CredentialDefinitionId {
    const ID_PATH: &'static str = "anoncreds/v0/CLAIM_DEF";

    pub fn new(id: &str) -> CredentialDefinitionId {
        let cred_def_id = CredentialDefinitionId {
            value: id.to_string()
        };

        trace!("Created new CredentialDefinitionId: {:?}", cred_def_id);

        cred_def_id
    }

    pub fn build(issuer_id: &DID, schema_id: &str, tag: &str) -> CredentialDefinitionId {
        let cred_def_id = CredentialDefinitionId::new(&format!(
            "{}/{}/{}/{}",
            issuer_id.value(),
            ID_PATH,
            schema_id,
            tag
        ));

        trace!("Created new CredentialDefinitionId: {:?}", cred_def_id);

        cred_def_id
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
