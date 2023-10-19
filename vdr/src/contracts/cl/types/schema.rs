use crate::{
    client::{ContractOutput, ContractParam},
    error::VdrError,
};

use crate::{contracts::cl::types::schema_id::SchemaId, DID};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaWithMeta {
    pub schema: Schema,
    pub metadata: SchemaMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub id: SchemaId,
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaMetadata {
    pub created: u128,
}

impl Into<ContractParam> for Schema {
    fn into(self) -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String(self.id.value().to_string()),
            ContractParam::String(self.issuer_id.value().to_string()),
            ContractParam::String(self.name.to_string()),
            ContractParam::String(self.version.to_string()),
            ContractParam::Array(
                self.attr_names
                    .iter()
                    .map(|attr_name| ContractParam::String(attr_name.to_string()))
                    .collect(),
            ),
        ])
    }
}

impl TryFrom<ContractOutput> for Schema {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        Ok(Schema {
            id: SchemaId::new(&value.get_string(0)?),
            issuer_id: DID::new(&value.get_string(1)?),
            name: value.get_string(2)?,
            version: value.get_string(3)?,
            attr_names: value.get_string_array(4)?,
        })
    }
}

impl TryFrom<ContractOutput> for SchemaMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let created = value.get_u128(0)?;
        Ok(SchemaMetadata { created })
    }
}

impl TryFrom<ContractOutput> for SchemaWithMeta {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let schema = value.get_tuple(0)?;
        let metadata = value.get_tuple(1)?;
        Ok(SchemaWithMeta {
            schema: Schema::try_from(schema)?,
            metadata: SchemaMetadata::try_from(metadata)?,
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{contracts::did::did_doc::test::ISSUER_ID, utils::rand_string};

    pub const SCHEMA_ID: &'static str =
        "did:indy2:testnet:3LpjszkgTmE3qThge25FZw/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0";
    pub const SCHEMA_NAME: &'static str = "F1DClaFEzi3t";
    pub const SCHEMA_VERSION: &'static str = "1.0.0";
    pub const SCHEMA_ATTRIBUTE_FIRST_NAME: &'static str = "First Name";
    pub const SCHEMA_ATTRIBUTE_LAST_NAME: &'static str = "Last Name";

    pub fn schema_id(issuer_id: &DID, name: &str) -> SchemaId {
        SchemaId::build(issuer_id, name, SCHEMA_VERSION)
    }

    pub fn schema(issuer_id: &DID, name: Option<&str>) -> Schema {
        let name = name.map(String::from).unwrap_or_else(|| rand_string());
        Schema {
            id: schema_id(&issuer_id, name.as_str()),
            issuer_id: issuer_id.clone(),
            name,
            version: SCHEMA_VERSION.to_string(),
            attr_names: vec![
                SCHEMA_ATTRIBUTE_FIRST_NAME.to_string(),
                SCHEMA_ATTRIBUTE_LAST_NAME.to_string(),
            ],
        }
    }

    fn schema_param() -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String(
                schema_id(&DID::new(ISSUER_ID), SCHEMA_NAME)
                    .value()
                    .to_string(),
            ),
            ContractParam::String(ISSUER_ID.to_string()),
            ContractParam::String(SCHEMA_NAME.to_string()),
            ContractParam::String(SCHEMA_VERSION.to_string()),
            ContractParam::Array(vec![
                ContractParam::String(SCHEMA_ATTRIBUTE_FIRST_NAME.to_string()),
                ContractParam::String(SCHEMA_ATTRIBUTE_LAST_NAME.to_string()),
            ]),
        ])
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_schema_into_contract_param_test() {
            let param: ContractParam = schema(&DID::new(ISSUER_ID), Some(SCHEMA_NAME)).into();
            assert_eq!(schema_param(), param);
        }
    }

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_schema() {
            let data = ContractOutput::new(schema_param().into_tuple().unwrap());
            let converted = Schema::try_from(data).unwrap();
            assert_eq!(schema(&DID::new(ISSUER_ID), Some(SCHEMA_NAME)), converted);
        }
    }
}
