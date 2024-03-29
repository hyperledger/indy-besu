use crate::{types::ContractOutput, ContractParam, VdrError, VdrResult};
use once_cell::sync::Lazy;
use regex_lite::Regex;
use serde_derive::{Deserialize, Serialize};

pub const DID_PREFIX: &str = "did";



static DID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^did:(?:indybesu|ethr):(?:[a-zA-Z0-9]+:)*0x[a-fA-F0-9]{40}$").unwrap());
    
/// Wrapper structure for DID
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DID(String);

impl DID {
    pub fn build(method: &str, network: Option<&str>, id: &str) -> DID {
        if let Some(network) = network {
            DID(format!("{}:{}:{}:{}", DID_PREFIX, method, network, id))
        } else {
            DID(format!("{}:{}:{}", DID_PREFIX, method, id))
        }
    }

    pub fn without_network(&self) -> VdrResult<DID> {
        Ok(ParsedDid::try_from(self)?.as_short_did())
    }

    pub fn get_method_specefic_id(&self) -> &str {
        let (_, id) = self.0.rsplit_once(':').unwrap_or_default();

        id
    }

    pub(crate) fn validate(&self) -> VdrResult<()> {
        if !DID_REGEX.is_match(&self.0) {
            return Err(VdrError::InvalidDidDocument(format!("Incorrect DID: {}", &self.0)));
        };

        Ok(())
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

impl TryFrom<ContractOutput> for DID {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        Ok(DID::from(value.get_string(0)?.as_str()))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct ParsedDid {
    pub(crate) method: String,
    pub(crate) network: Option<String>,
    pub(crate) identifier: String,
}

impl ParsedDid {
    pub(crate) fn as_short_did(&self) -> DID {
        DID::from(format!("{}:{}:{}", DID_PREFIX, self.method, self.identifier).as_str())
    }
}

impl TryFrom<&DID> for ParsedDid {
    type Error = VdrError;

    fn try_from(did: &DID) -> Result<Self, Self::Error> {
        let parts = did.as_ref().split(':').collect::<Vec<&str>>();
        match parts.len() {
            3 => Ok(ParsedDid {
                method: parts[1].to_string(),
                network: None,
                identifier: parts[2].to_string(),
            }),
            4 => Ok(ParsedDid {
                method: parts[1].to_string(),
                network: Some(parts[2].to_string()),
                identifier: parts[3].to_string(),
            }),
            _ => Err(VdrError::ContractInvalidInputData),
        }
    }
}
