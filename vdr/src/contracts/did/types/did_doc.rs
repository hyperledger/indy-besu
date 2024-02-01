use crate::{error::VdrError, Block};

use crate::contracts::did::types::did::DID;
use log::warn;
use serde_derive::{Deserialize, Serialize};

pub const BASE_CONTEXT: &str = "https://www.w3.org/ns/did/v1";
pub const SECPK_CONTEXT: &str = "https://w3id.org/security/suites/secp256k1recovery-2020/v2";
pub const KEYS_CONTEXT: &str = "https://w3id.org/security/v3-unstable";
pub const DID_RESOLUTION_FORMAT: &str = "application/did+ld+json";


#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentWithMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_document: Option<DidDocument>,
    pub did_document_metadata: DidMetadata,
    pub did_resolution_metadata: DidResolutionMetadata,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidResolutionMetadata {
    pub content_type: Option<String>,
    pub error: Option<DidResolutionError>,
    pub message: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidResolutionOptions {
    pub accept: Option<String>,
    pub block_tag: Option<Block>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: StringOrVector,
    pub id: DID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<StringOrVector>,
    pub verification_method: Vec<VerificationMethod>,
    #[serde(default)]
    pub authentication: Vec<VerificationMethodOrReference>,
    #[serde(default)]
    pub assertion_method: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub capability_invocation: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub capability_delegation: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub key_agreement: Vec<VerificationMethodOrReference>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub service: Vec<Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<u64>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: VerificationKeyType,
    pub controller: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_multibase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base58: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base64: Option<String>,
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
    EcdsaSecp256k1VerificationKey2020,
    EcdsaSecp256k1RecoveryMethod2020,
}

impl ToString for VerificationKeyType {
    fn to_string(&self) -> String {
        match self {
            VerificationKeyType::Ed25519VerificationKey2018 => {
                "Ed25519VerificationKey2018".to_string()
            }
            VerificationKeyType::X25519KeyAgreementKey2019 => {
                "X25519KeyAgreementKey2019".to_string()
            }
            VerificationKeyType::Ed25519VerificationKey2020 => {
                "Ed25519VerificationKey2020".to_string()
            }
            VerificationKeyType::X25519KeyAgreementKey2020 => {
                "X25519KeyAgreementKey2020".to_string()
            }
            VerificationKeyType::JsonWebKey2020 => "JsonWebKey2020".to_string(),
            VerificationKeyType::EcdsaSecp256k1VerificationKey2019
            | VerificationKeyType::EcdsaSecp256k1VerificationKey2020 => {
                "EcdsaSecp256k1VerificationKey2019".to_string()
            }
            VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020 => {
                "EcdsaSecp256k1RecoveryMethod2020".to_string()
            }
        }
    }
}

impl TryFrom<&str> for VerificationKeyType {
    type Error = VdrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ed25519VerificationKey2018" => Ok(VerificationKeyType::Ed25519VerificationKey2018),
            "X25519KeyAgreementKey2019" => Ok(VerificationKeyType::X25519KeyAgreementKey2019),
            "Ed25519VerificationKey2020" => Ok(VerificationKeyType::Ed25519VerificationKey2020),
            "X25519KeyAgreementKey2020" => Ok(VerificationKeyType::X25519KeyAgreementKey2020),
            "JsonWebKey2020" => Ok(VerificationKeyType::JsonWebKey2020),
            "EcdsaSecp256k1RecoveryMethod2020" => {
                Ok(VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020)
            }
            "EcdsaSecp256k1VerificationKey2019" => {
                Ok(VerificationKeyType::EcdsaSecp256k1VerificationKey2020)
            }
            _type => Err({
                let vdr_error = VdrError::CommonInvalidData(format!(
                    "Unexpected verification key type {}",
                    _type
                ));

                warn!(
                    "Error: {} during converting VerificationKeyType from String: {} ",
                    vdr_error, value
                );

                vdr_error
            }),
        }
    }
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
        StringOrVector::Vector(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DidResolutionError {
    /*
     * The resolver has failed to construct the DID document.
     * This can be caused by a network issue, a wrong registry address or malformed logs while parsing the registry
     * history. Please inspect the `DIDResolutionMetadata.message` to debug further.
     */
    #[serde(rename = "notFound")]
    NotFound,
    /*
     * The resolver does not know how to resolve the given DID. Most likely it is not a `did:ethr`.
     */
    #[serde(rename = "invalidDid")]
    InvalidDid,
    /*
     * The resolver does not support the 'accept' format requested with `DidResolutionOptions`.
     */
    #[serde(rename = "representationNotSupported")]
    RepresentationNotSupported,
    /*
     * If an invalid DID URL is detected
     */
    #[serde(rename = "invalidDidUrl")]
    InvalidDidUrl,
    /*
     * DID method is not supported
     */
    #[serde(rename = "methodNotSupported")]
    MethodNotSupported,
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde_json::json;

    pub const TEST_DID_ETHR: &str = "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a";
    pub const ISSUER_ID: &str = "did:ethr:testnet:0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5";
    pub const SERVICE_ENDPOINT: &str = "http://example.com";
    pub const SERVICE_TYPE: &str = "Service";

    pub fn _service(id: &str) -> Service {
        Service {
            id: id.to_string(),
            type_: SERVICE_TYPE.to_string(),
            service_endpoint: ServiceEndpoint::String(SERVICE_ENDPOINT.to_string()),
        }
    }

    pub fn default_ethr_did_document(chain_id: Option<u64>) -> DidDocument {
        let chain_id = chain_id.unwrap_or(1);
        let blockchain_account_id = format!(
            "eip155:{}:0xb9c5714089478a327f09197987f16f9e5d936e8a",
            chain_id
        );
        serde_json::from_value(json!({
            "@context": [
                "https://www.w3.org/ns/did/v1",
                "https://w3id.org/security/suites/secp256k1recovery-2020/v2",
                "https://w3id.org/security/v3-unstable"
            ],
            "id": "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a",
            "verificationMethod": [
            {
                "id": "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a#controller",
                "type": "EcdsaSecp256k1RecoveryMethod2020",
                "controller": "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a",
                "blockchainAccountId": blockchain_account_id
            }
            ],
            "authentication": [
                "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a#controller"
            ],
            "assertionMethod": [
                "did:ethr:0xb9c5714089478a327f09197987f16f9e5d936e8a#controller"
            ]
        }))
        .unwrap()
    }
}
