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
    auth::{role_control, Role},
    cl::{
        credential_definition_registry, schema_registry,
        types::{
            credential_definition::{CredentialDefinition, CredentialDefinitionCreatedEvent},
            credential_definition_id::CredentialDefinitionId,
            schema::{Schema, SchemaCreatedEvent},
            schema_id::SchemaId,
        },
    },
    did::{
        did_ethr_registry,
        types::{
            did::DID,
            did_doc::{
                DidDocument, DidResolutionAcceptType, DidResolutionOptions, VerificationKeyType,
            },
            did_doc_attribute::{DelegateType, DidDocAttribute, Validity},
            did_doc_builder::DidDocumentBuilder,
            did_events::{DidAttributeChanged, DidDelegateChanged, DidEvents, DidOwnerChanged},
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
