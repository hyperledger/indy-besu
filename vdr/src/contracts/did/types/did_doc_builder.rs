use log::warn;
use log_derive::{logfn, logfn_inputs};
use std::collections::HashMap;

use crate::{
    contracts::did::{
        types::did_doc::{
            Service, ServiceEndpoint, StringOrVector, VerificationMethod,
            VerificationMethodOrReference, BASE_CONTEXT,
        },
        KEYS_CONTEXT, SECPK_CONTEXT,
    },
    error::{VdrError, VdrResult},
    Address, DidDocument, VerificationKeyType, DID,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DidDocumentBuilder {
    pub context: StringOrVector,
    pub id: DID,
    pub controller: Option<StringOrVector>,
    pub verification_method: HashMap<String, VerificationMethod>,
    pub authentication: HashMap<String, VerificationMethodOrReference>,
    pub assertion_method: HashMap<String, VerificationMethodOrReference>,
    pub capability_invocation: HashMap<String, VerificationMethodOrReference>,
    pub capability_delegation: HashMap<String, VerificationMethodOrReference>,
    pub key_agreement: HashMap<String, VerificationMethodOrReference>,
    pub service: HashMap<String, Service>,
    pub also_known_as: Option<Vec<String>>,
    pub deactivated: bool,
    pub key_index: u32,
    pub service_index: u32,
}

impl DidDocumentBuilder {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn new() -> DidDocumentBuilder {
        DidDocumentBuilder {
            context: StringOrVector::String(BASE_CONTEXT.to_string()),
            ..DidDocumentBuilder::default()
        }
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn base_for_did(did: &DID, chain_id: u64) -> VdrResult<DidDocumentBuilder> {
        let identity = Address::try_from(did)?;
        let mut did_doc_builder = DidDocumentBuilder::new();
        did_doc_builder.add_context(SECPK_CONTEXT);
        did_doc_builder.add_context(KEYS_CONTEXT);
        did_doc_builder.set_id(did);
        did_doc_builder.add_verification_method(
            "controller",
            Some(format!("{}#controller", did.as_ref()).as_str()),
            &VerificationKeyType::EcdsaSecp256k1RecoveryMethod2020,
            did,
            Some(identity.as_blockchain_id(chain_id).as_str()),
            None,
            None,
            None,
            None,
        );
        did_doc_builder.add_authentication_reference("controller")?;
        did_doc_builder.add_assertion_method_reference("controller")?;
        Ok(did_doc_builder)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn deactivated_for_did(did: &DID) -> DidDocument {
        let mut did_doc_builder = DidDocumentBuilder::new();
        did_doc_builder.set_id(did);
        did_doc_builder.build()
    }

    pub fn increment_key_index(&mut self) {
        self.key_index += 1;
    }

    pub fn increment_service_index(&mut self) {
        self.service_index += 1;
    }

    #[logfn_inputs(Trace)]
    pub fn add_context(&mut self, context: &str) {
        match self.context {
            StringOrVector::String(ref base_context) => {
                self.context =
                    StringOrVector::Vector(vec![base_context.to_string(), context.to_string()])
            }
            StringOrVector::Vector(ref mut contexts) => contexts.push(context.to_string()),
        };
    }

    #[logfn_inputs(Trace)]
    pub fn set_id(&mut self, id: &DID) {
        self.id = id.to_owned();
    }

    #[logfn_inputs(Trace)]
    pub fn set_controller(&mut self, controller: &str) {
        self.controller = Some(StringOrVector::String(controller.to_string()));
    }

    #[logfn_inputs(Trace)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_verification_method(
        &mut self,
        key: &str,
        id: Option<&str>,
        type_: &VerificationKeyType,
        controller: &DID,
        blockchain_account_id: Option<&str>,
        public_key_multibase: Option<&str>,
        public_key_hex: Option<&str>,
        public_key_base58: Option<&str>,
        public_key_base64: Option<&str>,
    ) {
        let id = id
            .map(String::from)
            .unwrap_or_else(|| format!("{}#delegate-{}", self.id.as_ref(), self.key_index));
        let verification_method = VerificationMethod {
            id,
            type_: type_.to_owned(),
            controller: controller.to_string(),
            blockchain_account_id: blockchain_account_id.map(String::from),
            public_key_multibase: public_key_multibase.map(String::from),
            public_key_hex: public_key_hex.map(String::from),
            public_key_base58: public_key_base58.map(String::from),
            public_key_base64: public_key_base64.map(String::from),
        };
        self.verification_method
            .insert(key.to_string(), verification_method);
    }

    #[logfn_inputs(Trace)]
    pub fn remove_verification_method(&mut self, key: &str) {
        self.verification_method.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_authentication_reference(&mut self, key: &str) -> VdrResult<()> {
        let verification_method = Self::get_verification_method(self, key)?;
        let auth_reference =
            VerificationMethodOrReference::String(verification_method.id.to_string());
        self.authentication.insert(key.to_string(), auth_reference);
        Ok(())
    }

    #[logfn_inputs(Trace)]
    pub fn remove_authentication_reference(&mut self, key: &str) {
        self.authentication.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_assertion_method_reference(&mut self, key: &str) -> VdrResult<()> {
        let verification_method = Self::get_verification_method(self, key)?;
        let assertion_reference =
            VerificationMethodOrReference::String(verification_method.id.to_string());
        self.assertion_method
            .insert(key.to_string(), assertion_reference);
        Ok(())
    }

    #[logfn_inputs(Trace)]
    pub fn remove_assertion_method_reference(&mut self, key: &str) {
        self.assertion_method.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_capability_invocation_reference(&mut self, key: &str) -> VdrResult<()> {
        let verification_method = Self::get_verification_method(self, key)?;
        let capability_invocation_reference =
            VerificationMethodOrReference::String(verification_method.id.to_string());
        self.capability_invocation
            .insert(key.to_string(), capability_invocation_reference);
        Ok(())
    }

    #[logfn_inputs(Trace)]
    pub fn remove_capability_invocation_reference(&mut self, key: &str) {
        self.capability_invocation.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_capability_delegation_reference(&mut self, key: &str) -> VdrResult<()> {
        let verification_method = Self::get_verification_method(self, key)?;
        let capability_delegation_reference =
            VerificationMethodOrReference::String(verification_method.id.to_string());
        self.capability_delegation
            .insert(key.to_string(), capability_delegation_reference);
        Ok(())
    }

    #[logfn_inputs(Trace)]
    pub fn remove_capability_delegation_reference(&mut self, key: &str) {
        self.capability_delegation.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_key_agreement_reference(&mut self, key: &str) -> VdrResult<()> {
        let verification_method = Self::get_verification_method(self, key)?;
        let key_agreement_reference =
            VerificationMethodOrReference::String(verification_method.id.to_string());
        self.key_agreement
            .insert(key.to_string(), key_agreement_reference);
        Ok(())
    }

    #[logfn_inputs(Trace)]
    pub fn remove_key_agreement_reference(&mut self, key: &str) {
        self.key_agreement.remove(key);
    }

    #[logfn_inputs(Trace)]
    pub fn add_service(
        &mut self,
        key: &str,
        id: Option<&str>,
        type_: &str,
        endpoint: &ServiceEndpoint,
    ) {
        let id = id
            .map(String::from)
            .unwrap_or_else(|| format!("{}#service-{}", self.id.as_ref(), self.service_index));
        let service = Service {
            id,
            type_: type_.to_string(),
            service_endpoint: endpoint.clone(),
        };
        self.service.insert(key.to_string(), service);
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_verification_method(&self, key: &str) -> VdrResult<&VerificationMethod> {
        self
            .verification_method
            .get(key)
            .ok_or_else(|| {
                let vdr_error =
                    VdrError::CommonInvalidData(
                        "Missing verification method".to_string()
                    );

                warn!(
                "Error: {} during getting verification method by key: {} from DidDocumentBuilder: {:?}",
                vdr_error, key, self
            );

                vdr_error
            })
    }

    #[logfn_inputs(Trace)]
    pub fn remove_service(&mut self, key: &str) {
        self.service.remove(key);
    }

    pub fn deactivated(&mut self) {
        self.deactivated = true;
    }

    #[logfn(Trace)]
    pub fn build(self) -> DidDocument {
        if self.deactivated {
            return DidDocumentBuilder::deactivated_for_did(&self.id);
        }

        DidDocument {
            context: self.context,
            id: self.id,
            controller: self.controller,
            verification_method: self.verification_method.values().cloned().collect(),
            authentication: self.authentication.values().cloned().collect(),
            assertion_method: self.assertion_method.values().cloned().collect(),
            capability_invocation: self.capability_invocation.values().cloned().collect(),
            capability_delegation: self.capability_delegation.values().cloned().collect(),
            key_agreement: self.key_agreement.values().cloned().collect(),
            service: self.service.values().cloned().collect(),
            also_known_as: self.also_known_as,
        }
    }
}
