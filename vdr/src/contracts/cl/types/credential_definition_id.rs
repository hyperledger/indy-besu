use crate::{contracts::did::types::did::DID, types::ContractParam};
use log::trace;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionId(String);

impl CredentialDefinitionId {
    const ID_PATH: &'static str = "anoncreds/v0/CLAIM_DEF";

    pub fn build(issuer_id: &DID, schema_id: &str, tag: &str) -> CredentialDefinitionId {
        let cred_def_id = CredentialDefinitionId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.as_ref(),
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

impl From<&CredentialDefinitionId> for ContractParam {
    fn from(id: &CredentialDefinitionId) -> Self {
        ContractParam::String(id.to_string())
    }
}

impl From<&str> for CredentialDefinitionId {
    fn from(id: &str) -> Self {
        CredentialDefinitionId(id.to_string())
    }
}

impl AsRef<str> for CredentialDefinitionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for CredentialDefinitionId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
