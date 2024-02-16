#[allow(clippy::module_inception)]
mod client;
mod contracts;
mod error;
mod types;
mod utils;

#[cfg(feature = "basic_signer")]
mod signer;

#[cfg(feature = "migration")]
pub mod migration;

#[cfg(feature = "ledger_test")]
#[cfg(test)]
mod test;

pub use client::{Client, Contract, LedgerClient};
pub use contracts::{
    anoncreds::{
        credential_definition_registry, schema_registry,
        types::{
            credential_definition::CredentialDefinition,
            credential_definition_id::CredentialDefinitionId, schema::Schema, schema_id::SchemaId,
        },
    },
    auth::{role_control, Role},
    did::{
        did_ethr_registry, did_indy_registry, did_resolver,
        types::{
            did::DID,
            did_doc::{
                DidDocument, DidResolutionOptions, Service, ServiceEndpoint, ServiceEndpointObject,
                VerificationKeyType,
            },
            did_doc_attribute::{
                DelegateType, DidDocAttribute, PublicKeyAttribute, PublicKeyPurpose, PublicKeyType,
                ServiceAttribute, Validity,
            },
            did_doc_builder::DidDocumentBuilder,
            did_events::{DidAttributeChanged, DidDelegateChanged, DidEvents, DidOwnerChanged},
        },
    },
    migration::{
        legacy_mapping_registry,
        types::{
            did::{LegacyDid, LegacyVerkey},
            ed25519_signature::Ed25519Signature,
            resource_identifier::ResourceIdentifier,
        },
    },
    network::validator_control,
    StringOrVector,
};
pub use error::{VdrError, VdrResult};
pub use types::*;

pub use crate::client::QuorumConfig;
#[cfg(feature = "basic_signer")]
pub use signer::{BasicSigner, KeyPair};
