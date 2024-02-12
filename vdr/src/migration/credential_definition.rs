use crate::{
    contracts::did::types::did::DID,
    error::{VdrError, VdrResult},
    migration::{DID_METHOD, NETWORK},
    CredentialDefinition, CredentialDefinitionId, SchemaId,
};
use log::warn;
use log_derive::{logfn, logfn_inputs};
use serde_json::json;

use indy_data_types::{
    anoncreds::cred_def::{
        CredentialDefinition as IndyCredentialDefinition,
        CredentialDefinitionV1 as IndyCredentialDefinitionV1,
    },
    did::DidValue,
    CredentialDefinitionId as IndyCredentialDefinitionId, SchemaId as IndySchemaId,
};

impl CredentialDefinitionId {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_indy_format(id: &str) -> VdrResult<CredentialDefinitionId> {
        let parts: Vec<&str> = id.split(':').collect();
        let id = parts.get(0).ok_or_else(|| {
            let vdr_error = VdrError::CommonInvalidData("Invalid indy cred def id".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let schema_id = parts.get(3).ok_or_else(|| {
            let vdr_error =
                VdrError::CommonInvalidData("Invalid indy cred def schema id".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let tag = parts.get(4).ok_or_else(|| {
            let vdr_error = VdrError::CommonInvalidData("Invalid indy cred def tag".to_string());

            warn!(
                "Error: {:?} during converting CredentialDefinitionId from indy format",
                vdr_error
            );

            vdr_error
        })?;
        let issuer_did = DID::build(DID_METHOD, Some(NETWORK), id);

        let cred_def_id = CredentialDefinitionId::build(&issuer_did, schema_id, tag);
        Ok(cred_def_id)
    }
}

impl CredentialDefinition {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_indy_format_str(
        cred_def: &str,
        issuer_did: &str,
        schema_id: &str,
    ) -> VdrResult<CredentialDefinition> {
        let cred_def: IndyCredentialDefinition = serde_json::from_str(cred_def).map_err(|err| {
            VdrError::CommonInvalidData(format!(
                "Unable to parse indy credential definition. Err: {:?}",
                err
            ))
        })?;
        CredentialDefinition::from_indy_format(&cred_def, issuer_did, schema_id)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn from_indy_format(
        cred_def: &IndyCredentialDefinition,
        issuer_did: &str,
        schema_id: &str,
    ) -> VdrResult<CredentialDefinition> {
        match cred_def {
            IndyCredentialDefinition::CredentialDefinitionV1(cred_def) => {
                let besu_schema = CredentialDefinition {
                    issuer_id: DID::from(issuer_did),
                    schema_id: SchemaId::from(schema_id),
                    cred_def_type: cred_def.signature_type.clone(),
                    tag: cred_def.tag.to_string(),
                    value: json!(cred_def.value),
                };

                Ok(besu_schema)
            }
        }
    }
}

impl Into<IndyCredentialDefinition> for &CredentialDefinition {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn into(self) -> IndyCredentialDefinition {
        let value = serde_json::from_str(&self.value.to_string())
            .expect("Unable to parse credential definition data");

        IndyCredentialDefinition::CredentialDefinitionV1(IndyCredentialDefinitionV1 {
            id: IndyCredentialDefinitionId::new(
                &DidValue(self.issuer_id.to_string()),
                &IndySchemaId(self.schema_id.to_string()),
                &self.cred_def_type.to_str(),
                &self.tag,
            ),
            schema_id: IndySchemaId(self.schema_id.to_string()),
            signature_type: self.cred_def_type.clone(),
            tag: self.tag.to_string(),
            value,
        })
    }
}
