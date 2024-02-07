use crate::{SchemaId, types::{ContractOutput, ContractParam}, VdrError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier(String);

impl Identifier {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.to_string())
    }
}

impl From<&SchemaId> for Identifier {
    fn from(value: &SchemaId) -> Self {
        Identifier::from(value.as_ref())
    }
}

impl From<&indy_data_types::SchemaId> for Identifier {
    fn from(value: &indy_data_types::SchemaId) -> Self {
        Identifier::from(value.0.as_str())
    }
}

impl TryFrom<&Identifier> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &Identifier) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(value.0.to_string()))
    }
}

impl TryFrom<ContractOutput> for Identifier {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let string = value.get_string(0)?;
        Ok(Identifier(string))
    }
}
