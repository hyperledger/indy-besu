use crate::{
    client::{ContractOutput, ContractParam},
    error::{VdrError, VdrResult},
    DID_METHOD,
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const CONTEXT: &'static str = "https://www.w3.org/ns/did/v1";

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDocumentWithMeta {
    pub document: DidDocument,
    pub metadata: DidMetadata,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DID(String);

impl DID {
    pub const DID_PREFIX: &'static str = "did";

    pub fn new(did: &str) -> DID {
        DID(did.to_string())
    }

    pub fn build(network: &str, id: &str) -> DID {
        DID(format!("{}:{}:{}:{}", DID_PREFIX, DID_METHOD, network, id))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct DidMetadata {
    pub created: u128,
    pub updated: u128,
    pub deactivated: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: VerificationKeyType,
    pub controller: String,
    #[serde(flatten)]
    pub verification_key: VerificationKey,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VerificationKey {
    #[serde(rename_all = "camelCase")]
    Base58 { public_key_base58: String },
    #[serde(rename_all = "camelCase")]
    Multibase { public_key_multibase: String },
    #[serde(rename_all = "camelCase")]
    JWK { public_key_jwk: Value },
}

impl Default for VerificationKey {
    fn default() -> Self {
        VerificationKey::Base58 {
            public_key_base58: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum VerificationKeyType {
    Ed25519VerificationKey2018,
    X25519KeyAgreementKey2019,
    Ed25519VerificationKey2020,
    X25519KeyAgreementKey2020,
    JsonWebKey2020,
    EcdsaSecp256k1VerificationKey2019,
}

impl Default for VerificationKeyType {
    fn default() -> Self {
        VerificationKeyType::Ed25519VerificationKey2018
    }
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
            VerificationKeyType::EcdsaSecp256k1VerificationKey2019 => {
                "EcdsaSecp256k1VerificationKey2019".to_string()
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
            "EcdsaSecp256k1VerificationKey2019" => {
                Ok(VerificationKeyType::EcdsaSecp256k1VerificationKey2019)
            }
            _ => Err(VdrError::Unexpected),
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
#[serde(rename_all = "camelCase")]
pub struct VerificationRelationshipStruct {
    pub id: String,
    pub verification_method: VerificationMethod,
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

impl Into<ContractParam> for VerificationMethod {
    fn into(self) -> ContractParam {
        let public_key_jwk = match self.verification_key {
            VerificationKey::JWK { ref public_key_jwk } => json!(public_key_jwk).to_string(),
            _ => "".to_string(),
        };
        let public_key_multibase = match self.verification_key {
            VerificationKey::Multibase {
                public_key_multibase,
            } => public_key_multibase,
            _ => "".to_string(),
        };

        ContractParam::Tuple(vec![
            ContractParam::String(self.id.to_string()),
            ContractParam::String(self.type_.to_string()),
            ContractParam::String(self.controller.to_string()),
            ContractParam::String(public_key_jwk),
            ContractParam::String(public_key_multibase),
        ])
    }
}

impl TryFrom<ContractOutput> for VerificationMethod {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let public_key_jwk = value.get_string(3)?;
        let public_key_multibase = value.get_string(4)?;
        let verification_key = if !public_key_jwk.is_empty() {
            VerificationKey::JWK {
                public_key_jwk: serde_json::from_str::<Value>(&public_key_jwk)
                    .map_err(|_| VdrError::Unexpected)?,
            }
        } else if !public_key_multibase.is_empty() {
            VerificationKey::Multibase {
                public_key_multibase: public_key_multibase.to_string(),
            }
        } else {
            return Err(VdrError::Unexpected);
        };

        Ok(VerificationMethod {
            id: value.get_string(0)?,
            type_: VerificationKeyType::try_from(value.get_string(1)?.as_str())?,
            controller: value.get_string(2)?,
            verification_key,
        })
    }
}

impl VerificationMethod {
    pub fn empty() -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String("".to_string()),
            ContractParam::String("".to_string()),
            ContractParam::String("".to_string()),
            ContractParam::String("".to_string()),
            ContractParam::String("".to_string()),
        ])
    }
}

impl Into<ContractParam> for VerificationMethodOrReference {
    fn into(self) -> ContractParam {
        match self {
            VerificationMethodOrReference::String(reference) => ContractParam::Tuple(vec![
                ContractParam::String(reference.to_string()),
                VerificationMethod::empty(),
            ]),
            VerificationMethodOrReference::VerificationMethod(verification_method) => {
                ContractParam::Tuple(vec![
                    ContractParam::String(verification_method.id.to_string()),
                    verification_method.into(),
                ])
            }
        }
    }
}

impl TryFrom<ContractOutput> for VerificationMethodOrReference {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let id = value.get_string(0)?;
        let verification_method =
            VerificationMethod::try_from(value.get_tuple(1)?).unwrap_or_default();

        if !verification_method.id.is_empty() {
            Ok(VerificationMethodOrReference::VerificationMethod(
                verification_method,
            ))
        } else {
            Ok(VerificationMethodOrReference::String(id))
        }
    }
}

impl Into<ContractParam> for StringOrVector {
    fn into(self) -> ContractParam {
        match self {
            StringOrVector::String(ref value) => {
                ContractParam::Array(vec![ContractParam::String(value.to_string())])
            }
            StringOrVector::Vector(ref values) => ContractParam::Array(
                values
                    .iter()
                    .map(|value| ContractParam::String(value.to_string()))
                    .collect(),
            ),
        }
    }
}

impl Into<ContractParam> for Service {
    fn into(self) -> ContractParam {
        let (endpoint, accept, routing_keys) = match self.service_endpoint {
            ServiceEndpoint::String(endpoint) => (
                ContractParam::Array(vec![ContractParam::String(endpoint.to_string())]),
                ContractParam::Array(vec![]),
                ContractParam::Array(vec![]),
            ),
            ServiceEndpoint::Object(endpoint) => (
                ContractParam::Array(vec![ContractParam::String(endpoint.uri.to_string())]),
                ContractParam::Array(
                    endpoint
                        .accept
                        .iter()
                        .map(|accept| ContractParam::String(accept.to_string()))
                        .collect(),
                ),
                ContractParam::Array(
                    endpoint
                        .routing_keys
                        .iter()
                        .map(|routing_key| ContractParam::String(routing_key.to_string()))
                        .collect(),
                ),
            ),
            ServiceEndpoint::Set(_) => unimplemented!(),
        };

        ContractParam::Tuple(vec![
            ContractParam::String(self.id.to_string()),
            ContractParam::String(self.type_.to_string()),
            endpoint,
            accept,
            routing_keys,
        ])
    }
}

impl TryFrom<ContractOutput> for Service {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let uri = value.get_string(2)?;
        let accept = value.get_string_array(3)?;
        let routing_keys = value.get_string_array(4)?;

        Ok(Service {
            id: value.get_string(0)?,
            type_: value.get_string(1)?,
            service_endpoint: ServiceEndpoint::Object(ServiceEndpointObject {
                uri,
                accept,
                routing_keys,
            }),
        })
    }
}

impl Into<ContractParam> for DidDocument {
    fn into(self) -> ContractParam {
        let context: ContractParam = self.context.into();
        let id = ContractParam::String(self.id.value().to_string());
        let controller: ContractParam = self.controller.into();
        let verification_method: ContractParam = ContractParam::Array(
            self.verification_method
                .into_iter()
                .map(VerificationMethod::into)
                .collect(),
        );
        let authentication: ContractParam = ContractParam::Array(
            self.authentication
                .into_iter()
                .map(VerificationMethodOrReference::into)
                .collect(),
        );
        let assertion_method: ContractParam = ContractParam::Array(
            self.assertion_method
                .into_iter()
                .map(VerificationMethodOrReference::into)
                .collect(),
        );
        let capability_invocation: ContractParam = ContractParam::Array(
            self.capability_invocation
                .into_iter()
                .map(VerificationMethodOrReference::into)
                .collect(),
        );
        let capability_delegation: ContractParam = ContractParam::Array(
            self.capability_delegation
                .into_iter()
                .map(VerificationMethodOrReference::into)
                .collect(),
        );
        let key_agreement: ContractParam = ContractParam::Array(
            self.key_agreement
                .into_iter()
                .map(VerificationMethodOrReference::into)
                .collect(),
        );
        let service: ContractParam =
            ContractParam::Array(self.service.into_iter().map(Service::into).collect());
        let also_known_as: ContractParam = ContractParam::Array(
            self.also_known_as
                .unwrap_or_default()
                .into_iter()
                .map(|also_known_as| ContractParam::String(also_known_as))
                .collect(),
        );

        ContractParam::Tuple(vec![
            context,
            id,
            controller,
            verification_method,
            authentication,
            assertion_method,
            capability_invocation,
            capability_delegation,
            key_agreement,
            service,
            also_known_as,
        ])
    }
}

impl TryFrom<ContractOutput> for DidDocument {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let context = value.get_string_array(0)?;
        let id = value.get_string(1)?;
        let controller = value.get_string_array(2)?;
        let verification_method: Vec<VerificationMethod> = value
            .get_objects_array(3)?
            .into_iter()
            .map(VerificationMethod::try_from)
            .collect::<VdrResult<Vec<VerificationMethod>>>()?;
        let authentication: Vec<VerificationMethodOrReference> = value
            .get_objects_array(4)?
            .into_iter()
            .map(VerificationMethodOrReference::try_from)
            .collect::<VdrResult<Vec<VerificationMethodOrReference>>>()?;
        let assertion_method: Vec<VerificationMethodOrReference> = value
            .get_objects_array(5)?
            .into_iter()
            .map(VerificationMethodOrReference::try_from)
            .collect::<VdrResult<Vec<VerificationMethodOrReference>>>()?;
        let capability_invocation: Vec<VerificationMethodOrReference> = value
            .get_objects_array(6)?
            .into_iter()
            .map(VerificationMethodOrReference::try_from)
            .collect::<VdrResult<Vec<VerificationMethodOrReference>>>()?;
        let capability_delegation: Vec<VerificationMethodOrReference> = value
            .get_objects_array(7)?
            .into_iter()
            .map(VerificationMethodOrReference::try_from)
            .collect::<VdrResult<Vec<VerificationMethodOrReference>>>()?;
        let key_agreement: Vec<VerificationMethodOrReference> = value
            .get_objects_array(8)?
            .into_iter()
            .map(VerificationMethodOrReference::try_from)
            .collect::<VdrResult<Vec<VerificationMethodOrReference>>>()?;
        let service: Vec<Service> = value
            .get_objects_array(9)?
            .into_iter()
            .map(Service::try_from)
            .collect::<VdrResult<Vec<Service>>>()?;
        let also_known_as = value.get_string_array(10)?;

        Ok(DidDocument {
            context: StringOrVector::Vector(context),
            id: DID::new(&id),
            controller: StringOrVector::Vector(controller),
            verification_method,
            authentication,
            assertion_method,
            capability_invocation,
            capability_delegation,
            key_agreement,
            service,
            also_known_as: Some(also_known_as),
        })
    }
}

impl TryFrom<ContractOutput> for DidMetadata {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let created = value.get_u128(0)?;
        let updated = value.get_u128(1)?;
        let deactivated = value.get_bool(2)?;
        Ok(DidMetadata {
            created,
            updated,
            deactivated,
        })
    }
}

impl TryFrom<ContractOutput> for DidDocumentWithMeta {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        let document = value.get_tuple(0)?;
        let metadata = value.get_tuple(1)?;
        Ok(DidDocumentWithMeta {
            document: DidDocument::try_from(document)?,
            metadata: DidMetadata::try_from(metadata)?,
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::utils::rand_bytes;

    pub const ISSUER_ID: &'static str = "did:indy2:testnet:3LpjszkgTmE3qThge25FZw";
    pub const CONTEXT: &'static str = "https://www.w3.org/ns/did/v1";
    pub const MULTIBASE_KEY: &'static str = "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf";
    pub const SERVICE_ENDPOINT: &'static str = "https://example.com/path";
    pub const SERVICE_TYPE: &'static str = "DIDCommMessaging";
    pub const KEY_1: &'static str = "KEY-1";

    pub fn verification_method(id: &str) -> VerificationMethod {
        VerificationMethod {
            id: format!("{}#{}", id, KEY_1),
            type_: VerificationKeyType::Ed25519VerificationKey2018,
            controller: id.to_string(),
            verification_key: VerificationKey::Multibase {
                public_key_multibase: MULTIBASE_KEY.to_string(),
            },
        }
    }

    pub fn verification_relationship(id: &str) -> VerificationMethodOrReference {
        VerificationMethodOrReference::String(format!("{}#{}", id, KEY_1))
    }

    pub fn service(id: &str) -> Service {
        Service {
            id: format!("{}#didcomm-1", id),
            type_: SERVICE_TYPE.to_string(),
            service_endpoint: ServiceEndpoint::Object(ServiceEndpointObject {
                uri: SERVICE_ENDPOINT.to_string(),
                accept: vec![],
                routing_keys: vec![],
            }),
        }
    }

    pub fn new_id() -> String {
        format!(
            "did:indy2:testnet:{}",
            &bs58::encode(rand_bytes()).into_string()
        )
    }

    pub fn did_doc(id: Option<&str>) -> DidDocument {
        let id = id.map(String::from).unwrap_or_else(|| new_id());
        DidDocument {
            context: StringOrVector::Vector(vec![CONTEXT.to_string()]),
            id: DID::new(&id),
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
        ContractParam::Tuple(vec![
            ContractParam::Array(vec![ContractParam::String(CONTEXT.to_string())]),
            ContractParam::String(ISSUER_ID.to_string()),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![verification_method_param()]),
            ContractParam::Array(vec![verification_relationship_param()]),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
        ])
    }

    fn verification_relationship_param() -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String(format!("{}#KEY-1", ISSUER_ID)),
            ContractParam::Tuple(vec![
                ContractParam::String("".to_string()),
                ContractParam::String("".to_string()),
                ContractParam::String("".to_string()),
                ContractParam::String("".to_string()),
                ContractParam::String("".to_string()),
            ]),
        ])
    }

    fn verification_method_param() -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String(format!("{}#KEY-1", ISSUER_ID)),
            ContractParam::String(VerificationKeyType::Ed25519VerificationKey2018.to_string()),
            ContractParam::String(ISSUER_ID.to_string()),
            ContractParam::String("".to_string()),
            ContractParam::String(MULTIBASE_KEY.to_string()),
        ])
    }

    fn service_param() -> ContractParam {
        ContractParam::Tuple(vec![
            ContractParam::String(format!("{}#didcomm-1", ISSUER_ID)),
            ContractParam::String(SERVICE_TYPE.to_string()),
            ContractParam::String(SERVICE_ENDPOINT.to_string()),
            ContractParam::Array(vec![]),
            ContractParam::Array(vec![]),
        ])
    }

    mod convert_into_contract_param {
        use super::*;

        #[test]
        fn convert_did_doc_into_contract_param_test() {
            let param: ContractParam = did_doc(Some(ISSUER_ID)).into();
            assert_eq!(did_doc_param(), param);
        }

        #[test]
        fn convert_verification_method_into_contract_param_test() {
            let param: ContractParam = verification_method(ISSUER_ID).into();
            assert_eq!(verification_method_param(), param);
        }

        #[test]
        fn convert_verification_relationship_into_contract_param_test() {
            let param: ContractParam = verification_relationship(ISSUER_ID).into();
            assert_eq!(verification_relationship_param(), param);
        }

        #[test]
        fn convert_service_into_contract_param_test() {
            let param: ContractParam = service(ISSUER_ID).into();
            assert_eq!(service_param(), param);
        }
    }

    mod convert_into_object {
        use super::*;

        #[test]
        fn convert_contract_output_into_did_doc() {
            let data = ContractOutput::new(did_doc_param().into_tuple().unwrap());
            let converted = DidDocument::try_from(data).unwrap();
            assert_eq!(did_doc(Some(ISSUER_ID)), converted);
        }

        #[test]
        fn convert_contract_output_into_verification_method() {
            let data = ContractOutput::new(verification_method_param().into_tuple().unwrap());
            let converted = VerificationMethod::try_from(data).unwrap();
            assert_eq!(verification_method(ISSUER_ID), converted);
        }

        #[test]
        fn convert_contract_output_into_verification_relationship() {
            let data = ContractOutput::new(verification_relationship_param().into_tuple().unwrap());
            let converted = VerificationMethodOrReference::try_from(data).unwrap();
            assert_eq!(verification_relationship(ISSUER_ID), converted);
        }

        #[test]
        fn convert_contract_output_into_service() {
            let data = ContractOutput::new(service_param().into_tuple().unwrap());
            let converted = Service::try_from(data).unwrap();
            assert_eq!(service(ISSUER_ID), converted);
        }
    }
}
