use crate::{error::VdrError, types::ContractParam, Address, CredentialDefinitionId, VdrResult};

use crate::{
    contracts::{cl::types::schema_id::SchemaId, did::types::did::DID},
    types::ContractEvent,
};
use serde_derive::{Deserialize, Serialize};

pub use indy_data_types::anoncreds::cred_def::SignatureType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialDefinition {
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    #[serde(rename = "schemaId")]
    pub schema_id: SchemaId,
    #[serde(rename = "credDefType")]
    pub cred_def_type: SignatureType,
    pub tag: String,
    pub value: serde_json::Value,
}




impl CredentialDefinition {
    pub fn id(&self) -> CredentialDefinitionId {
        CredentialDefinitionId::build(&self.issuer_id, &self.schema_id.to_string(), &self.tag)
    }

    pub fn matches_id(&self, expected_id: &CredentialDefinitionId) -> VdrResult<()> {
        let actual_id = self.id();

        if expected_id.to_string() != actual_id.to_string() {
            return Err(VdrError::InvalidCredentialDefinition(format!(
                "Id built from cred_def: {} != provided id: {}",
                actual_id.to_string(),
                expected_id.to_string()
            )));
        }

        Ok(())
    }

    pub fn validate(&self) -> VdrResult<()> {
        if self.cred_def_type != CredentialDefinitionTypes::CL {
            return Err(VdrError::InvalidCredentialDefinition(format!(
                "Unsupported type: {}",
                self.cred_def_type.as_ref()
            )));
        }

        if self.tag.is_empty() {
            return Err(VdrError::InvalidCredentialDefinition(
                "Tag is not provided".to_string(),
            ));
        }

        if self.value.is_null() {
            return Err(VdrError::InvalidCredentialDefinition(
                "Value is not provided".to_string(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<&CredentialDefinition> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &CredentialDefinition) -> Result<Self, Self::Error> {
        serde_json::to_vec(value)
            .map(ContractParam::Bytes)
            .map_err(|_| VdrError::ContractInvalidInputData)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialDefinitionCreatedEvent {
    pub id_hash: String,
    pub identity: Address,
    pub cred_def: CredentialDefinition,
}

impl TryFrom<ContractEvent> for CredentialDefinitionCreatedEvent {
    type Error = VdrError;

    fn try_from(log: ContractEvent) -> Result<Self, Self::Error> {
        let id = log.get_fixed_bytes(0)?;
        let identity = log.get_address(1)?;
        let schema_bytes = log.get_bytes(2)?;
        let cred_def = serde_json::from_slice(&schema_bytes).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse credential definition from contract event. Err: {:?}",
                err
            ))
        })?;

        Ok(CredentialDefinitionCreatedEvent {
            id_hash: hex::encode(id),
            identity,
            cred_def,
        })
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

    pub const _CREDENTIAL_DEFINITION_ID: &str = "did:ethr:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/CLAIM_DEF/did:ethr:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5/anoncreds/v0/SCHEMA/F1DClaFEzi3t/1.0.0/default";
    pub const CREDENTIAL_DEFINITION_TAG: &str = "default";

    pub fn credential_definition_id(
        issuer_id: &DID,
        schema_id: &SchemaId,
        tag: &str,
    ) -> CredentialDefinitionId {
        CredentialDefinitionId::build(issuer_id, schema_id, tag)
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
            cred_def_type: SignatureType::CL,
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
        ContractParam::Bytes(serde_json::to_vec(&cred_def).unwrap())
    }

    #[test]
    fn cred_def_matches_id_test() {
        let (id, cred_def) = credential_definition(
            &DID::from(ISSUER_ID),
            &SchemaId::from(SCHEMA_ID),
            Some(CREDENTIAL_DEFINITION_TAG),
        );

        cred_def.matches_id(&id).unwrap();
    }

    #[test]
    fn cred_def_not_matches_id_test() {
        let (_, cred_def) = credential_definition(
            &DID::from(ISSUER_ID),
            &SchemaId::from(SCHEMA_ID),
            Some(CREDENTIAL_DEFINITION_TAG),
        );

        let (id, _) = credential_definition(
            &DID::from(ISSUER_ID),
            &SchemaId::from(SCHEMA_ID),
            Some("NotDefault"),
        );

        let err = cred_def.matches_id(&id).unwrap_err();
        assert!(matches!(err, VdrError::InvalidCredentialDefinition { .. }))
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
            let param: ContractParam = (&credential_definition).try_into().unwrap();
            assert_eq!(cred_def_param(), param);
        }
    }
}
