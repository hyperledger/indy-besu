use crate::{contracts::did::types::did::DID, types::ContractParam, VdrError};

use serde_derive::{Deserialize, Serialize};
use sha3::Digest;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaId(String);

impl SchemaId {
    const ID_PATH: &'static str = "/anoncreds/v0/SCHEMA";

    pub fn build(issuer_id: &DID, name: &str, version: &str) -> SchemaId {
        SchemaId::from(
            format!(
                "{}{}/{}/{}",
                issuer_id.as_ref(),
                Self::ID_PATH,
                name,
                version
            )
            .as_str(),
        )
    }

    // A unique identifier for the schema needed for cred def
    // referencing to https://hyperledger.github.io/indy-did-method/#cred-def
    pub fn unique_id(&self) -> String {
        self.0.replace(Self::ID_PATH, "").replace('/', ":")
    }

    pub fn hash(&self) -> Vec<u8> {
        sha3::Keccak256::digest(self.0.as_bytes()).to_vec()
    }

    pub(crate) fn to_filter(&self) -> String {
        hex::encode(self.hash())
    }
}

impl TryFrom<&SchemaId> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &SchemaId) -> Result<Self, Self::Error> {
        Ok(ContractParam::FixedBytes(value.hash()))
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
