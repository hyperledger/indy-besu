use crate::{error::VdrError, types::{ContractOutput, ContractParam}, Address, VdrResult, SchemaId};
use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
    Address,
};
use std::collections::HashSet;
use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
    Address, SchemaId, VdrResult,
};

use crate::{contracts::did::types::did::DID, types::ContractEvent};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: HashSet<String>,
}

impl Schema {
    pub fn id(&self) -> SchemaId {
        SchemaId::build(&self.issuer_id, &self.name, &self.version)
    }

    pub fn matches_id(&self, expected_id: &SchemaId) -> VdrResult<()> {
        let actual_id = self.id();

        if expected_id.to_string() != actual_id.to_string() {
            return Err(VdrError::InvalidSchema(format!(
                "Id built from schema: {} != provided id: {}",
                actual_id.to_string(),
                expected_id.to_string()
            )));
        }

        Ok(())
    }

    pub fn validate(&self) -> VdrResult<()> {
        if self.name.is_empty() {
            return Err(VdrError::InvalidSchema("Name is not provided".to_string()));
        }

        if self.version.is_empty() {
            return Err(VdrError::InvalidSchema(
                "Version is not provided".to_string(),
            ));
        }

        if self.attr_names.is_empty() {
            return Err(VdrError::InvalidSchema(
                "Attributes are not provided".to_string(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<&Schema> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &Schema) -> Result<Self, Self::Error> {
        serde_json::to_vec(value)
            .map(ContractParam::Bytes)
            .map_err(|_| VdrError::ContractInvalidInputData)
    }
}

impl TryFrom<&ContractOutput> for Schema {
    type Error = VdrError;

    fn try_from(value: &ContractOutput) -> Result<Self, Self::Error> {
        serde_json::from_str(&value.get_string(0)?).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse Schema from the response. Err: {:?}",
                err
            ))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaCreatedEvent {
    pub id_hash: String,
    pub identity: Address,
    pub schema: Schema,
}

impl TryFrom<ContractEvent> for SchemaCreatedEvent {
    type Error = VdrError;

    fn try_from(log: ContractEvent) -> Result<Self, Self::Error> {
        let id = log.get_fixed_bytes(0)?;
        let identity = log.get_address(1)?;
        let schema_bytes = log.get_bytes(2)?;
        let schema = serde_json::from_slice(&schema_bytes).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse schema from contract event. Err: {:?}",
                err
            ))
        })?;

        Ok(SchemaCreatedEvent {
            id_hash: hex::encode(id),
            identity,
            schema,
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        contracts::{cl::types::schema_id::SchemaId, did::types::did_doc::test::ISSUER_ID},
        utils::rand_string,
    };

    pub const SCHEMA_ID: &str =
        "did:ethr:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0";
    pub const SCHEMA_NAME: &str = "F1DClaFEzi3t";
    pub const SCHEMA_VERSION: &str = "1.0.0";
    pub const SCHEMA_ATTRIBUTE_FIRST_NAME: &str = "First Name";

    pub fn schema_id(issuer_id: &DID, name: &str) -> SchemaId {
        SchemaId::build(issuer_id, name, SCHEMA_VERSION)
    }

    pub fn schema(issuer_id: &DID, name: Option<&str>) -> (SchemaId, Schema) {
        let name = name.map(String::from).unwrap_or_else(rand_string);
        let id = schema_id(issuer_id, name.as_str());
        let mut attr_names: HashSet<String> = HashSet::new();
        attr_names.insert(SCHEMA_ATTRIBUTE_FIRST_NAME.to_string());

        let schema = Schema {
            issuer_id: issuer_id.clone(),
            name,
            version: SCHEMA_VERSION.to_string(),
            attr_names,
        };
        (id, schema)
    }

    fn schema_param() -> ContractParam {
        let (_, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
        ContractParam::Bytes(serde_json::to_vec(&schema).unwrap())
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_schema_into_contract_param_test() {
            let (_, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            let param: ContractParam = (&schema).try_into().unwrap();
            assert_eq!(schema_param(), param);
        }
    }
}
