use log::warn;
use log_derive::{logfn, logfn_inputs};

use crate::{
    contracts::did::types::did_doc::{
        Service, ServiceEndpoint, StringOrVector, VerificationMethod,
        VerificationMethodOrReference, BASE_CONTEXT,
    },
    error::{VdrError, VdrResult},
    DidDocument, VerificationKeyType, DID,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DidDocumentBuilder {
    pub context: StringOrVector,
    pub id: DID,
    pub controller: Option<StringOrVector>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<VerificationMethodOrReference>,
    pub assertion_method: Vec<VerificationMethodOrReference>,
    pub capability_invocation: Vec<VerificationMethodOrReference>,
    pub capability_delegation: Vec<VerificationMethodOrReference>,
    pub key_agreement: Vec<VerificationMethodOrReference>,
    pub service: Vec<Service>,
    pub also_known_as: Option<Vec<String>>,
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
    pub fn add_context(mut self, context: &str) -> DidDocumentBuilder {
        match self.context {
            StringOrVector::String(ref base_context) => {
                self.context =
                    StringOrVector::Vector(vec![base_context.to_string(), context.to_string()])
            }
            StringOrVector::Vector(ref mut contexts) => contexts.push(context.to_string()),
        };
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_id(mut self, id: &DID) -> DidDocumentBuilder {
        self.id = id.to_owned();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_controller(mut self, controller: &str) -> DidDocumentBuilder {
        self.controller = Some(StringOrVector::String(controller.to_string()));
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_verification_method(
        mut self,
        id: Option<&str>,
        type_: &VerificationKeyType,
        controller: &DID,
        blockchain_account_id: Option<&str>,
        public_key_multibase: Option<&str>,
        public_key_hex: Option<&str>,
        public_key_base58: Option<&str>,
        public_key_base64: Option<&str>,
    ) -> DidDocumentBuilder {
        let id = id.map(String::from).unwrap_or_else(|| {
            format!(
                "{}+delegate-{}",
                self.id.as_ref(),
                self.verification_method.len()
            )
        });
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
        self.verification_method.push(verification_method.clone());
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_authentication_reference(mut self, index: usize) -> VdrResult<DidDocumentBuilder> {
        let kid = self
            .verification_method
            .get(index)
            .ok_or_else(|| {
                let vdr_error =
                    VdrError::CommonInvalidData("Missing verification method".to_string());

                warn!(
                    "Error: {} during getting verification method by index: {} from DidDocumentBuilder: {:?}",
                    vdr_error, index, self
                );

                vdr_error
            })?
            .id
            .to_string();
        let auth_reference = VerificationMethodOrReference::String(kid);
        self.authentication.push(auth_reference.clone());

        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_assertion_method_reference(mut self, index: usize) -> VdrResult<DidDocumentBuilder> {
        let kid = Self::get_kid_by_index(&self, index)?;
        let assertion_reference = VerificationMethodOrReference::String(kid);
        self.assertion_method.push(assertion_reference.clone());

        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_capability_invocation_reference(
        mut self,
        index: usize,
    ) -> VdrResult<DidDocumentBuilder> {
        let kid = Self::get_kid_by_index(&self, index)?;
        let capability_invocation_reference = VerificationMethodOrReference::String(kid);
        self.capability_invocation
            .push(capability_invocation_reference.clone());

        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_capability_delegation_reference(
        mut self,
        index: usize,
    ) -> VdrResult<DidDocumentBuilder> {
        let kid = Self::get_kid_by_index(&self, index)?;
        let capability_delegation_reference = VerificationMethodOrReference::String(kid);
        self.capability_delegation
            .push(capability_delegation_reference.clone());

        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_key_agreement_reference(mut self, index: usize) -> VdrResult<DidDocumentBuilder> {
        let kid = Self::get_kid_by_index(&self, index)?;
        let key_agreement_reference = VerificationMethodOrReference::String(kid);
        self.key_agreement.push(key_agreement_reference.clone());

        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn verification_keys(&self) -> usize {
        self.verification_method.len()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_service(
        mut self,
        id: Option<&str>,
        type_: &str,
        endpoint: &ServiceEndpoint,
    ) -> DidDocumentBuilder {
        let id = id
            .map(String::from)
            .unwrap_or_else(|| format!("#{}#service-{}", self.id.as_ref(), self.service.len() + 1));
        let service = Service {
            id,
            type_: type_.to_string(),
            service_endpoint: endpoint.clone(),
        };
        self.service.push(service.clone());

        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn build(self) -> DidDocument {
        DidDocument {
            context: self.context,
            id: self.id,
            controller: self.controller,
            verification_method: self.verification_method,
            authentication: self.authentication,
            assertion_method: self.assertion_method,
            capability_invocation: self.capability_invocation,
            capability_delegation: self.capability_delegation,
            key_agreement: self.key_agreement,
            service: self.service,
            also_known_as: self.also_known_as,
        }
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_kid_by_index(&self, index: usize) -> VdrResult<String> {
        let kid = self
            .verification_method
            .get(index)
            .ok_or_else(|| {
                let vdr_error =
                    VdrError::CommonInvalidData(
                        "Missing verification method".to_string()
                    );

                warn!(
                "Error: {} during getting verification method by index: {} from DidDocumentBuilder: {:?}",
                vdr_error, index, self
            );

                vdr_error
            })?
            .id
            .to_string();

        Ok(kid)
    }
}
