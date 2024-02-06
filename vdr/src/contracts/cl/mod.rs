pub mod credential_definition_registry;
pub mod schema_registry;
pub mod types;

pub use types::{
    schema::{Schema, SchemaCreatedEvent},
    schema_id::SchemaId,
};

pub use types::{
    credential_definition::{CredentialDefinition, CredentialDefinitionCreatedEvent},
    credential_definition_id::CredentialDefinitionId,
};
