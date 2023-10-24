mod client;
mod contracts;
mod error;
mod signer;
mod utils;

#[cfg(feature = "migration")]
pub mod migration;

pub use client::{Client, ContractConfig, LedgerClient, PingStatus, Status};
pub use contracts::{
    cl::{
        credential_definition_registry::CredentialDefinitionRegistry,
        schema_registry::SchemaRegistry,
        types::{
            credential_definition::CredentialDefinition,
            credential_definition_id::CredentialDefinitionId, schema::Schema, schema_id::SchemaId,
        },
    },
    did::{
        did_registry::DidRegistry,
        types::{
            did_doc::{DidDocument, VerificationKey, VerificationKeyType, DID},
            did_doc_builder::DidDocumentBuilder,
        },
    },
};
pub use signer::{BasicSigner, Signer};

#[cfg(feature = "ledger_test")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::test::client,
        contracts::{
            cl::{
                schema_registry::test::create_schema,
                types::{credential_definition::test::credential_definition, schema::test::schema},
            },
            did::{did_registry::test::create_did, types::did_doc::test::did_doc},
        },
        error::VdrResult,
        signer::signer::test::ACCOUNT,
    };

    mod did {
        use super::*;

        #[async_std::test]
        async fn demo_build_and_submit_did_transaction_test() -> VdrResult<()> {
            let client = client();

            // write
            let did_doc = did_doc(None);
            let transaction =
                DidRegistry::build_create_did_transaction(&client, ACCOUNT, &did_doc).unwrap();
            let signed_transaction = client.sign_transaction(&transaction).await.unwrap();
            let block_hash = client
                .submit_transaction(&signed_transaction)
                .await
                .unwrap();

            // get receipt
            let receipt = client.get_receipt(&block_hash).await.unwrap();
            println!("Receipt: {}", receipt);

            // read
            let transaction =
                DidRegistry::build_resolve_did_transaction(&client, &did_doc.id).unwrap();
            let result = client.submit_transaction(&transaction).await.unwrap();
            let resolved_did_doc = DidRegistry::parse_resolve_did_result(&client, &result).unwrap();
            assert_eq!(did_doc, resolved_did_doc);

            Ok(())
        }

        #[async_std::test]
        async fn demo_single_step_did_transaction_execution_test() -> VdrResult<()> {
            let client = client();

            // write
            let did_doc = did_doc(None);
            let _receipt = DidRegistry::create_did(&client, ACCOUNT, &did_doc)
                .await
                .unwrap();

            // read
            let resolved_did_doc = DidRegistry::resolve_did(&client, &did_doc.id)
                .await
                .unwrap();
            assert_eq!(did_doc, resolved_did_doc);

            Ok(())
        }
    }

    #[cfg(feature = "ledger_test")]
    mod schema {
        use super::*;

        #[async_std::test]
        async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
            let client = client();

            // create DID Document
            let did_doc = create_did(&client).await;

            // write
            let schema = schema(&did_doc.id, None);
            let transaction =
                SchemaRegistry::build_create_schema_transaction(&client, ACCOUNT, &schema).unwrap();
            let signed_transaction = client.sign_transaction(&transaction).await.unwrap();
            let block_hash = client
                .submit_transaction(&signed_transaction)
                .await
                .unwrap();
            let receipt = client.get_receipt(&block_hash).await.unwrap();
            println!("Receipt: {}", receipt);

            // read
            let transaction =
                SchemaRegistry::build_resolve_schema_transaction(&client, &schema.id).unwrap();
            let result = client.submit_transaction(&transaction).await.unwrap();
            let resolved_schema =
                SchemaRegistry::parse_resolve_schema_result(&client, &result).unwrap();
            assert_eq!(sschema, resolved_schema);

            Ok(())
        }

        #[async_std::test]
        async fn demo_single_step_transaction_execution_test() -> VdrResult<()> {
            let client = client();

            // create DID Document
            let did_doc = create_did(&client).await;

            // write
            let schema = schema(&did_doc.id, None);
            let _receipt = SchemaRegistry::create_schema(&client, ACCOUNT, &schema)
                .await
                .unwrap();

            // read
            let resolved_schema = SchemaRegistry::resolve_schema(&client, &schema.id)
                .await
                .unwrap();
            assert_eq!(schema, resolved_schema);

            Ok(())
        }
    }

    #[cfg(feature = "ledger_test")]
    mod credential_definition {
        use super::*;

        #[async_std::test]
        async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
            let client = client();

            // create DID Document and Schema
            let did_doc = create_did(&client).await;
            let schema = create_schema(&client, &did_doc.id).await;

            // write
            let credential_definition = credential_definition(&did_doc.id, &schema.id, None);
            let transaction =
                CredentialDefinitionRegistry::build_create_credential_definition_transaction(
                    &client,
                    ACCOUNT,
                    &credential_definition,
                )
                .unwrap();
            let signed_transaction = client.sign_transaction(&transaction).await.unwrap();
            let block_hash = client
                .submit_transaction(&signed_transaction)
                .await
                .unwrap();
            let receipt = client.get_receipt(&block_hash).await.unwrap();
            println!("Receipt: {}", receipt);

            // read
            let transaction =
                CredentialDefinitionRegistry::build_resolve_credential_definition_transaction(
                    &client,
                    &credential_definition.id,
                )
                .unwrap();
            let result = client.submit_transaction(&transaction).await.unwrap();
            let resolved_credential_definition =
                CredentialDefinitionRegistry::parse_resolve_credential_definition_result(
                    &client, &result,
                )
                .unwrap();
            assert_eq!(credential_definition, resolved_credential_definition);

            Ok(())
        }

        #[async_std::test]
        async fn demo_single_step_transaction_execution_test() -> VdrResult<()> {
            let client = client();

            // create DID Document
            let did_doc = create_did(&client).await;
            DidRegistry::resolve_did(&client, &did_doc.id)
                .await
                .unwrap();

            let schema = create_schema(&client, &did_doc.id).await;
            SchemaRegistry::resolve_schema(&client, &schema.id)
                .await
                .unwrap();

            // write
            let credential_definition = credential_definition(&did_doc.id, &schema.id, None);
            let _receipt = CredentialDefinitionRegistry::create_credential_definition(
                &client,
                ACCOUNT,
                &credential_definition,
            )
            .await
            .unwrap();

            // read
            let resolved_credential_definition =
                CredentialDefinitionRegistry::resolve_credential_definition(
                    &client,
                    &credential_definition.id,
                )
                .await
                .unwrap();
            assert_eq!(credential_definition, resolved_credential_definition);

            Ok(())
        }
    }
}
