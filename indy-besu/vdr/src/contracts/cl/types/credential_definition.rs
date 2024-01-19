use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
};

use crate::contracts::{cl::types::schema_id::SchemaId, did::types::did::DID};
use log::trace;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinitionRecord {
    pub credential_definition: CredentialDefinition,
    pub metadata: CredentialDefinitionMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinition {
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    #[serde(rename = "schemaId")]
    pub schema_id: SchemaId,
    #[serde(rename = "credDefType")]
    pub cred_def_type: CredentialDefinitionTypes,
    pub tag: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum CredentialDefinitionTypes {
    #[default]
    CL,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionMetadata {
    pub created: u64,
}

impl From<&CredentialDefinition> for ContractParam {
    fn from(value: &CredentialDefinition) -> Self {
        trace!(
            "CredentialDefinition: {:?} convert into ContractParam has started",
            value
        );

        let cred_def_contract_param = ContractParam::String(json!(value).to_string());

        trace!(
            "CredentialDefinition: {:?} convert into ContractParam has finished. Result: {:?}",
            value,
            cred_def_contract_param
        );

        cred_def_contract_param
    }
}

impl TryFrom<&ContractOutput> for CredentialDefinition {
    type Error = VdrError;

    fn try_from(value: &ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "CredentialDefinition convert from ContractOutput: {:?} has started",
            value
        );

        let cred_def = serde_json::from_str(&value.get_string(0)?).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse CredentialDefinition from the response. Err: {:?}",
                err
            ))
        })?;

        trace!(
            "CredentialDefinition convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            cred_def
        );

        Ok(cred_def)
    }
}

impl TryFrom<ContractOutput> for CredentialDefinitionMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "CredentialDefinitionMetadata convert from ContractOutput: {:?} has started",
            value
        );

        let created = value.get_u128(0)?;
        let cred_def_metadata = CredentialDefinitionMetadata {
            created: created as u64,
        };

        trace!(
            "CredentialDefinitionMetadata convert from ContractOutput: {:?} has finished. Result: {:?}",
            value, cred_def_metadata
        );

        Ok(cred_def_metadata)
    }
}

impl TryFrom<ContractOutput> for CredentialDefinitionRecord {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "CredentialDefinitionWithMeta convert from ContractOutput: {:?} has started",
            value
        );

        let output_tuple = value.get_tuple(0)?;
        let credential_definition = CredentialDefinition::try_from(&output_tuple)?;
        let metadata = output_tuple.get_tuple(1)?;

        let cred_def_with_metadata = CredentialDefinitionRecord {
            credential_definition,
            metadata: CredentialDefinitionMetadata::try_from(metadata)?,
        };

        trace!(
            "CredentialDefinitionWithMeta convert from ContractOutput: {:?} has finished. Result: {:?}",
            value, cred_def_with_metadata
        );

        Ok(cred_def_with_metadata)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        contracts::{
            cl::types::{
                credential_definition_id::CredentialDefinitionId, schema::test::SCHEMA_ID,
            },
            did::types::did_doc::test::ISSUER_ID,
        },
        utils::rand_string,
    };
    use serde_json::json;

    pub const _CREDENTIAL_DEFINITION_ID: &str = "did:indy2:testnet:3LpjszkgTmE3qThge25FZw/anoncreds/v0/CLAIM_DEF/did:indy2:testnet:3LpjszkgTmE3qThge25FZw/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0/default";
    pub const CREDENTIAL_DEFINITION_TAG: &str = "default";

    pub fn credential_definition_id(
        issuer_id: &DID,
        schema_id: &SchemaId,
        tag: &str,
    ) -> CredentialDefinitionId {
        CredentialDefinitionId::build(issuer_id, schema_id.as_ref(), tag)
    }

    fn credential_definition_value() -> serde_json::Value {
        json!({
            "n": "779...397",
            "rctxt": "774...977",
            "s": "750..893",
            "z":
            "632...005"
        })
    }

    pub fn credential_definition(
        issuer_id: &DID,
        schema_id: &SchemaId,
        tag: Option<&str>,
    ) -> (CredentialDefinitionId, CredentialDefinition) {
        let tag = tag.map(String::from).unwrap_or_else(rand_string);
        let id = credential_definition_id(issuer_id, schema_id, tag.as_str());
        let cred_def = CredentialDefinition {
            issuer_id: issuer_id.clone(),
            schema_id: SchemaId::from(schema_id.as_ref()),
            cred_def_type: CredentialDefinitionTypes::CL,
            tag: tag.to_string(),
            value: credential_definition_value(),
        };
        (id, cred_def)
    }

    fn cred_def_param() -> ContractParam {
        let (_, cred_def) = credential_definition(
            &DID::from(ISSUER_ID),
            &SchemaId::from(SCHEMA_ID),
            Some(CREDENTIAL_DEFINITION_TAG),
        );
        ContractParam::String(json!(cred_def).to_string())
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_cred_def_into_contract_param_test() {
            let (_, credential_definition) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            let param: ContractParam = (&credential_definition).into();
            assert_eq!(cred_def_param(), param);
        }
    }

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_cred_def() {
            let data = ContractOutput::new(vec![cred_def_param()]);
            let converted = CredentialDefinition::try_from(&data).unwrap();
            let (_, credential_definition) = credential_definition(
                &DID::from(ISSUER_ID),
                &SchemaId::from(SCHEMA_ID),
                Some(CREDENTIAL_DEFINITION_TAG),
            );
            assert_eq!(credential_definition, converted);
        }
    }
}
