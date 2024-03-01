use crate::{contracts::did::types::did::DID, types::ContractParam, VdrError, VdrResult};

use crate::contracts::types::did::ParsedDid;
use serde_derive::{Deserialize, Serialize};
use sha3::Digest;

/// Wrapper structure for AnonCreds Credential Definition ID
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionId(String);

impl CredentialDefinitionId {
    const ID_PATH: &'static str = "anoncreds/v0/CLAIM_DEF";

    pub fn build(issuer_id: &DID, schema_id: &str, tag: &str) -> CredentialDefinitionId {
        CredentialDefinitionId::from(
            format!(
                "{}/{}/{}/{}",
                issuer_id.as_ref(),
                Self::ID_PATH,
                schema_id,
                tag
            )
            .as_str(),
        )
    }

    pub(crate) fn hash(&self) -> Vec<u8> {
        sha3::Keccak256::digest(self.0.as_bytes()).to_vec()
    }

    pub fn without_network(&self) -> VdrResult<CredentialDefinitionId> {
        ParsedCredentialDefinitionId::try_from(self)?.as_short_id()
    }
}

impl TryFrom<&CredentialDefinitionId> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &CredentialDefinitionId) -> Result<Self, Self::Error> {
        Ok(ContractParam::FixedBytes(value.hash()))
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct ParsedCredentialDefinitionId {
    pub(crate) issuer_id: DID,
    pub(crate) schema_id: String,
    pub(crate) tag: String,
    pub(crate) network: Option<String>,
}

impl ParsedCredentialDefinitionId {
    pub(crate) fn as_short_id(&self) -> VdrResult<CredentialDefinitionId> {
        Ok(CredentialDefinitionId::build(
            &self.issuer_id.without_network()?,
            &self.schema_id,
            &self.tag,
        ))
    }
}

impl TryFrom<&CredentialDefinitionId> for ParsedCredentialDefinitionId {
    type Error = VdrError;

    fn try_from(cred_def_id: &CredentialDefinitionId) -> Result<Self, Self::Error> {
        let parts = cred_def_id.as_ref().split('/').collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(VdrError::CommonInvalidData(
                "Invalid credential definition id provided".to_string(),
            ));
        }
        let issuer_id = DID::from(parts[0]);
        let parsed_issuer_id = ParsedDid::try_from(&issuer_id)?;
        Ok(ParsedCredentialDefinitionId {
            issuer_id: DID::from(parts[0]),
            schema_id: parts[3].to_string(),
            tag: parts[4].to_string(),
            network: parsed_issuer_id.network,
        })
    }
}
