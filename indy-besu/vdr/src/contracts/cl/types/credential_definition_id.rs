use crate::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionId {
    value: String,
}

impl CredentialDefinitionId {
    const ID_PATH: &'static str = "anoncreds/v0/CLAIM_DEF";

    pub fn build(issuer_id: &DID, schema_id: &str, tag: &str) -> CredentialDefinitionId {
        let cred_def_id = CredentialDefinitionId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.deref(),
                Self::ID_PATH,
                schema_id,
                tag
            )
            .as_str(),
        );

        trace!("Created new CredentialDefinitionId: {:?}", cred_def_id);

        cred_def_id
    }
}

impl From<&str> for CredentialDefinitionId {
    fn from(id: &str) -> Self {
        let cred_def_id = CredentialDefinitionId {
            value: id.to_string(),
        };

        trace!("Created new CredentialDefinitionId: {:?}", cred_def_id);

        cred_def_id
    }
}

impl Deref for CredentialDefinitionId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
