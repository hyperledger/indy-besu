use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
};

use crate::contracts::did::types::did::DID;
use log::trace;
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

impl From<&Schema> for ContractParam {
    fn from(value: &Schema) -> Self {
        trace!("Schema: {:?} convert into ContractParam has started", value);

        let schema_contract_param = ContractParam::String(json!(value).to_string());

        trace!(
            "Schema: {:?} convert into ContractParam has finished. Result: {:?}",
            value,
            schema_contract_param
        );

        schema_contract_param
    }
}

impl TryFrom<&ContractOutput> for Schema {
    type Error = VdrError;

    fn try_from(value: &ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "Schema convert from ContractOutput: {:?} has started",
            value
        );

        let schema = serde_json::from_str(&value.get_string(0)?).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse Schema from the response. Err: {:?}",
                err
            ))
        })?;

        trace!(
            "Schema convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            schema
        );

        Ok(schema)
    }
}

impl TryFrom<ContractOutput> for SchemaMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "SchemaMetadata convert from ContractOutput: {:?} has started",
            value
        );

        let created = value.get_u128(0)?;
        let schema_metadata = SchemaMetadata {
            created: created as u64,
        };

        trace!(
            "SchemaMetadata convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            schema_metadata
        );

        Ok(schema_metadata)
    }
}

impl TryFrom<ContractOutput> for SchemaRecord {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "SchemaWithMeta convert from ContractOutput: {:?} has started",
            value
        );

        let output_tuple = value.get_tuple(0)?;
        let schema = Schema::try_from(&output_tuple)?;
        let metadata = output_tuple.get_tuple(1)?;

        let schema_with_meta = SchemaRecord {
            schema,
            metadata: SchemaMetadata::try_from(metadata)?,
        };

        trace!(
            "SchemaWithMeta convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            schema_with_meta
        );

        Ok(schema_with_meta)
    }
}

pub mod migration {
    use super::*;
    use crate::{error::VdrResult, NETWORK};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct IndySchemaIdFormat(String);

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct IndySchemaFormat {
        pub id: String,
        pub name: String,
        pub version: String,
        #[serde(rename = "attrNames")]
        pub attr_names: Vec<String>,
        #[serde(rename = "seqNo")]
        pub seq_no: Option<u64>,
        #[serde(default)]
        pub ver: String,
    }

    impl SchemaId {
        pub fn from_legacy_form(id: &str) -> SchemaId {
            let parts: Vec<&str> = id.split(':').collect();
            let issuer_did = DID::build(NETWORK, parts[0]);
            SchemaId::build(&issuer_did, parts[2], parts[3])
        }
    }

    impl Schema {
        pub fn from_indy_schema_format(schema: &str) -> VdrResult<Schema> {
            let indy_schema: IndySchemaFormat =
                serde_json::from_str(&schema).map_err(|_err| VdrError::Unexpected)?;
            Schema::try_from(indy_schema)
        }
    }

    impl TryFrom<IndySchemaFormat> for Schema {
        type Error = VdrError;

        fn try_from(schema: IndySchemaFormat) -> Result<Self, Self::Error> {
            let parts: Vec<&str> = schema.id.split(':').collect();
            let issuer_id = DID::build(NETWORK, parts[0]);
            Ok(Schema {
                id: SchemaId::build(&issuer_id, &schema.name, &schema.version),
                issuer_id,
                name: schema.name.to_string(),
                version: schema.version.to_string(),
                attr_names: schema.attr_names,
            })
        }
    }

    impl Into<IndySchemaFormat> for Schema {
        fn into(self) -> IndySchemaFormat {
            IndySchemaFormat {
                id: format!(
                    "{}:2:{}:{}",
                    self.issuer_id.value(),
                    self.name,
                    self.version
                ),
                name: self.name.to_string(),
                version: self.version.to_string(),
                attr_names: self.attr_names,
                seq_no: None,
                ver: "1.0".to_string(),
            }
        }
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
            let param: ContractParam = (&schema).into();
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
