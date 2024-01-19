use crate::ContractParam;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DID(String);

impl DID {
    pub const DID_PREFIX: &'static str = "did";

    pub fn build(method: &str, network: &str, id: &str) -> DID {
        DID::from(format!("{}:{}:{}:{}", Self::DID_PREFIX, method, network, id).as_str())
    }
}

impl From<&DID> for ContractParam {
    fn from(id: &DID) -> Self {
        ContractParam::String(id.to_string())
    }
}

impl From<&str> for DID {
    fn from(did: &str) -> Self {
        DID(did.to_string())
    }
}

impl AsRef<str> for DID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for DID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
