use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
    Address,
};

use crate::contracts::did::types::did::DID;
use log::trace;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const CONTEXT: &str = "https://www.w3.org/ns/did/v1";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidRecord {
    pub document: DidDocument,
    pub metadata: DidMetadata,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: StringOrVector,
    pub id: DID,
    pub controller: StringOrVector,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<VerificationMethodOrReference>,
    pub assertion_method: Vec<VerificationMethodOrReference>,
    pub capability_invocation: Vec<VerificationMethodOrReference>,
    pub capability_delegation: Vec<VerificationMethodOrReference>,
    pub key_agreement: Vec<VerificationMethodOrReference>,
    pub service: Vec<Service>,
    pub also_known_as: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DidMetadata {
    pub owner: Address,
    pub sender: Address,
    pub created: u64,
    pub updated: u64,
    pub deactivated: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: VerificationKeyType,
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_multibase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_jwk: Option<Value>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum VerificationKeyType {
    #[default]
    Ed25519VerificationKey2018,
    X25519KeyAgreementKey2019,
    Ed25519VerificationKey2020,
    X25519KeyAgreementKey2020,
    JsonWebKey2020,
    EcdsaSecp256k1VerificationKey2019,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VerificationMethodOrReference {
    String(String),
    VerificationMethod(VerificationMethod),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub service_endpoint: ServiceEndpoint,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    String(String),
    Object(ServiceEndpointObject),
    Set(Vec<ServiceEndpoint>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEndpointObject {
    pub uri: String,
    pub accept: Vec<String>,
    pub routing_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrVector {
    String(String),
    Vector(Vec<String>),
}

impl Default for StringOrVector {
    fn default() -> Self {
        let vector = StringOrVector::Vector(Vec::new());

        trace!("Created new StringOrVector::Vector: {:?}", vector);

        vector
    }
}

impl From<&DidDocument> for ContractParam {
    fn from(value: &DidDocument) -> Self {
        trace!(
            "DidDocument: {:?} convert into ContractParam has started",
            value
        );

        let did_doc_contract_param = ContractParam::String(json!(value).to_string());

        trace!(
            "DidDocument convert into ContractParam has finished. Result: {:?}",
            did_doc_contract_param
        );

        did_doc_contract_param
    }
}

impl TryFrom<&ContractOutput> for DidDocument {
    type Error = VdrError;

    fn try_from(value: &ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "DidDocument convert from ContractOutput: {:?} has started",
            value
        );

        let did_doc = serde_json::from_str(&value.get_string(0)?).map_err(|err| {
            VdrError::ContractInvalidResponseData(format!(
                "Unable to parse DID Document from the response. Err: {:?}",
                err
            ))
        })?;

        trace!(
            "DidDocument convert from ContractOutput has finished. Result: {:?}",
            did_doc
        );

        Ok(did_doc)
    }
}

impl TryFrom<ContractOutput> for DidMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "DidMetadata convert from ContractOutput: {:?} has started",
            value
        );

        let owner = value.get_address(0)?;
        let sender = value.get_address(1)?;
        let created = value.get_u128(2)? as u64;
        let updated = value.get_u128(3)? as u64;
        let deactivated = value.get_bool(4)?;

        let did_metadata = DidMetadata {
            owner,
            sender,
            created,
            updated,
            deactivated,
        };

        trace!(
            "DidMetadata convert from ContractOutput has finished. Result: {:?}",
            did_metadata
        );

        Ok(did_metadata)
    }
}

impl TryFrom<ContractOutput> for DidRecord {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "DidDocumentWithMeta convert from ContractOutput: {:?} has started",
            value
        );

        let output_tuple = value.get_tuple(0)?;
        let document = DidDocument::try_from(&output_tuple)?;
        let metadata = output_tuple.get_tuple(1)?;

        let did_doc_with_metadata = DidRecord {
            document,
            metadata: DidMetadata::try_from(metadata)?,
        };

        trace!(
            "DidDocumentWithMeta convert from ContractOutput has finished. Result: {:?}",
            did_doc_with_metadata
        );

        Ok(did_doc_with_metadata)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::utils::rand_bytes;

    pub const ISSUER_ID: &str = "did:indy2:testnet:3LpjszkgTmE3qThge25FZw";
    pub const CONTEXT: &str = "https://www.w3.org/ns/did/v1";
    pub const MULTIBASE_KEY: &str = "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf";
    pub const SERVICE_ENDPOINT: &str = "127.0.0.1:5555";
    pub const SERVICE_TYPE: &str = "DIDCommService";
    pub const KEY_1: &str = "KEY-1";

    pub fn verification_method(id: &str) -> VerificationMethod {
        VerificationMethod {
            id: format!("{}#{}", id, KEY_1),
            type_: VerificationKeyType::Ed25519VerificationKey2018,
            controller: id.to_string(),
            public_key_multibase: Some(MULTIBASE_KEY.to_string()),
            public_key_jwk: None,
        }
    }

    pub fn verification_relationship(id: &str) -> VerificationMethodOrReference {
        VerificationMethodOrReference::String(format!("{}#{}", id, KEY_1))
    }

    pub fn service(id: &str) -> Service {
        Service {
            id: id.to_string(),
            type_: SERVICE_TYPE.to_string(),
            service_endpoint: ServiceEndpoint::String(SERVICE_ENDPOINT.to_string()),
        }
    }

    pub fn new_id() -> String {
        format!(
            "did:indy2:testnet:{}",
            &bs58::encode(rand_bytes()).into_string()
        )
    }

    pub fn did_doc(id: Option<&str>) -> DidDocument {
        let id = id.map(String::from).unwrap_or_else(new_id);
        DidDocument {
            context: StringOrVector::Vector(vec![CONTEXT.to_string()]),
            id: DID::from(id.as_str()),
            controller: StringOrVector::Vector(vec![]),
            verification_method: vec![verification_method(&id)],
            authentication: vec![verification_relationship(&id)],
            assertion_method: vec![],
            capability_invocation: vec![],
            capability_delegation: vec![],
            key_agreement: vec![],
            service: vec![],
            also_known_as: Some(vec![]),
        }
    }

    fn did_doc_param() -> ContractParam {
        ContractParam::String(json!(did_doc(Some(ISSUER_ID))).to_string())
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_did_doc_into_contract_param_test() {
            let param: ContractParam = (&did_doc(Some(ISSUER_ID))).into();
            assert_eq!(did_doc_param(), param);
        }
    }

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_did_doc() {
            let data = ContractOutput::new(vec![did_doc_param()]);
            let converted = DidDocument::try_from(&data).unwrap();
            assert_eq!(did_doc(Some(ISSUER_ID)), converted);
        }
    }
}
