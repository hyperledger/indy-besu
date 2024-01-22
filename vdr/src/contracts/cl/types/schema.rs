use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
};

use crate::contracts::did::types::did::DID;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRecord {
    pub schema: Schema,
    pub metadata: SchemaMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaMetadata {
    pub created: u64,
}

impl TryFrom<&Schema> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &Schema) -> Result<Self, Self::Error> {
        Ok(ContractParam::String(json!(value).to_string()))
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

impl TryFrom<ContractOutput> for SchemaMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let created = value.get_u128(0)?;
        let schema_metadata = SchemaMetadata {
            created: created as u64,
        };
        Ok(schema_metadata)
    }
}

impl TryFrom<ContractOutput> for SchemaRecord {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let output_tuple = value.get_tuple(0)?;
        let schema = Schema::try_from(&output_tuple)?;
        let metadata = output_tuple.get_tuple(1)?;

        let schema_with_meta = SchemaRecord {
            schema,
            metadata: SchemaMetadata::try_from(metadata)?,
        };
        Ok(schema_with_meta)
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
        "did:indy2:testnet:3LpjszkgTmE3qThge25FZw/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0";
    pub const SCHEMA_NAME: &str = "F1DClaFEzi3t";
    pub const SCHEMA_VERSION: &str = "1.0.0";
    pub const SCHEMA_ATTRIBUTE_FIRST_NAME: &str = "First Name";
    pub const SCHEMA_ATTRIBUTE_LAST_NAME: &str = "Last Name";

    pub fn schema_id(issuer_id: &DID, name: &str) -> SchemaId {
        SchemaId::build(issuer_id, name, SCHEMA_VERSION)
    }

    pub fn schema(issuer_id: &DID, name: Option<&str>) -> (SchemaId, Schema) {
        let name = name.map(String::from).unwrap_or_else(rand_string);
        let id = schema_id(issuer_id, name.as_str());
        let schema = Schema {
            issuer_id: issuer_id.clone(),
            name,
            version: SCHEMA_VERSION.to_string(),
            attr_names: vec![
                SCHEMA_ATTRIBUTE_FIRST_NAME.to_string(),
                SCHEMA_ATTRIBUTE_LAST_NAME.to_string(),
            ],
        };
        (id, schema)
    }

    fn schema_param() -> ContractParam {
        let (_, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
        ContractParam::String(json!(schema).to_string())
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

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_schema() {
            let data = ContractOutput::new(vec![schema_param()]);
            let converted = Schema::try_from(&data).unwrap();
            let (_, schema) = schema(&DID::from(ISSUER_ID), Some(SCHEMA_NAME));
            assert_eq!(schema, converted);
        }
    }
}
