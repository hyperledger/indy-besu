pub mod credential_definition_registry;
pub mod schema_registry;
pub mod types;

pub use types::{
    credential_definition::CredentialDefinition, credential_definition_id::CredentialDefinitionId,
    schema::Schema, schema_id::SchemaId,
};
