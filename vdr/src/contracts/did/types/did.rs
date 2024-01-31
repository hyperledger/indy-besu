use crate::{ContractParam, VdrError, VdrResult};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DID(String);

impl DID {
    pub const DID_PREFIX: &'static str = "did";

    pub fn build(method: &str, network: Option<&str>, id: &str) -> DID {
        if let Some(network) = network {
            DID(format!(
                "{}:{}:{}:{}",
                Self::DID_PREFIX,
                method,
                network,
                id
            ))
        } else {
            DID(format!("{}:{}:{}", Self::DID_PREFIX, method, id))
        }
    }

    pub fn method(&self) -> VdrResult<String> {
        let (method, _, _) = self.parts()?;
        Ok(method)
    }

    pub fn identifier(&self) -> VdrResult<String> {
        let (_, _, identifier) = self.parts()?;
        Ok(identifier)
    }

    fn parts(&self) -> VdrResult<(String, Option<String>, String)> {
        let parts = self.as_ref().split(':').collect::<Vec<&str>>();
        match parts.len() {
            3 => Ok((parts[1].to_string(), None, parts[2].to_string())),
            4 => Ok((
                parts[1].to_string(),
                Some(parts[2].to_string()),
                parts[3].to_string(),
            )),
            _ => Err(VdrError::ContractInvalidInputData),
        }
    }

    pub fn short(&self) -> VdrResult<DID> {
        let (method, _, id) = self.parts()?;
        Ok(DID::from(
            format!("{}:{}:{}", Self::DID_PREFIX, method, id).as_str(),
        ))
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

impl TryFrom<&DID> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &DID) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(value.to_string()))
    }
}
