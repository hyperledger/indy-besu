use crate::{
    client::client::test::client,
    contracts::{
        auth::Role,
        cl::{
            schema_registry::test::create_schema,
            types::{credential_definition::test::credential_definition, schema::test::schema},
            CredentialDefinition, Schema,
        },
        did::{did_registry::test::create_did, types::did_doc::test::did_doc, DidDocument},
    },
    error::VdrResult,
    signer::basic_signer::{
        test::{basic_signer, TRUSTEE_ACC},
        BasicSigner,
    },
    types::{Address, Transaction},
    LedgerClient,
};
use std::ops::Deref;

async fn sign_and_submit_transaction(
    client: &LedgerClient,
    transaction: Transaction,
    signer: &BasicSigner,
) -> String {
    let sign_bytes = transaction.get_signing_bytes().unwrap();
    let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.deref()).unwrap();
    transaction.set_signature(signature);
    let block_hash = client.submit_transaction(&transaction).await.unwrap();
    client.get_receipt(&block_hash).await.unwrap()
}

mod did {
    use super::*;
    use crate::did_registry;

    pub(crate) async fn build_and_submit_create_did_doc_transaction(
        client: &LedgerClient,
        did_doc: &DidDocument,
        signer: &BasicSigner,
    ) -> String {
        let transaction =
            did_registry::build_create_did_transaction(&client, &TRUSTEE_ACC, did_doc)
                .await
                .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    #[async_std::test]
    async fn demo_build_and_submit_did_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // write
        let did_doc = did_doc(None);
        let receipt = build_and_submit_create_did_doc_transaction(&client, &did_doc, &signer).await;
        println!("Receipt: {}", receipt);

        // read
        let transaction = did_registry::build_resolve_did_transaction(&client, &did_doc.id)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let resolved_did_doc = did_registry::parse_resolve_did_result(&client, &result).unwrap();
        assert_eq!(did_doc, resolved_did_doc);

        Ok(())
    }
}

mod schema {
    use super::*;
    use crate::schema_registry;

    pub(crate) async fn build_and_submit_create_schema_transaction(
        client: &LedgerClient,
        schema: &Schema,
        signer: &BasicSigner,
    ) -> String {
        let transaction =
            schema_registry::build_create_schema_transaction(&client, &TRUSTEE_ACC, schema)
                .await
                .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    #[async_std::test]
    async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID Document
        let did_doc = create_did(&client, &signer).await;

        // write
        let schema = schema(&did_doc.id, None);
        let receipt = build_and_submit_create_schema_transaction(&client, &schema, &signer).await;
        println!("Receipt: {}", receipt);

        // read
        let transaction = schema_registry::build_resolve_schema_transaction(&client, &schema.id)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let resolved_schema =
            schema_registry::parse_resolve_schema_result(&client, &result).unwrap();
        assert_eq!(schema, resolved_schema);

        Ok(())
    }
}

mod credential_definition {
    use super::*;
    use crate::credential_definition_registry;

    pub(crate) async fn build_and_submit_create_cred_def_transaction(
        client: &LedgerClient,
        cred_def: &CredentialDefinition,
        signer: &BasicSigner,
    ) -> String {
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                cred_def,
            )
            .await
            .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    #[async_std::test]
    async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID Document and Schema
        let did_doc = create_did(&client, &signer).await;
        let schema = create_schema(&client, &did_doc.id, &signer).await;

        // write
        let credential_definition = credential_definition(&did_doc.id, &schema.id, None);
        let receipt =
            build_and_submit_create_cred_def_transaction(&client, &credential_definition, &signer)
                .await;
        println!("Receipt: {}", receipt);

        // read
        let transaction =
            credential_definition_registry::build_resolve_credential_definition_transaction(
                &client,
                &credential_definition.id,
            )
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let resolved_credential_definition =
            credential_definition_registry::parse_resolve_credential_definition_result(
                &client, &result,
            )
            .unwrap();
        assert_eq!(credential_definition, resolved_credential_definition);

        Ok(())
    }
}

mod role {
    use super::*;
    use crate::role_control;

    pub(crate) async fn build_and_submit_assign_role_transaction(
        client: &LedgerClient,
        assignee_account: &Address,
        role_to_assign: &Role,
        signer: &BasicSigner,
    ) -> String {
        let transaction = role_control::build_assign_role_transaction(
            client,
            &TRUSTEE_ACC,
            role_to_assign,
            assignee_account,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    async fn build_and_submit_revoke_role_transaction(
        client: &LedgerClient,
        revokee_account: &Address,
        role_to_revoke: &Role,
        signer: &BasicSigner,
    ) -> String {
        let transaction = role_control::build_revoke_role_transaction(
            client,
            &TRUSTEE_ACC,
            role_to_revoke,
            revokee_account,
        )
        .await
        .unwrap();

        let sign_bytes = transaction.get_signing_bytes().unwrap();
        let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.deref()).unwrap();
        transaction.set_signature(signature);

        let block_hash = client.submit_transaction(&transaction).await.unwrap();

        client.get_receipt(&block_hash).await.unwrap()
    }

    async fn build_and_submit_get_role_transaction(
        client: &LedgerClient,
        assignee_account: &Address,
    ) -> Role {
        let transaction = role_control::build_get_role_transaction(client, assignee_account)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        role_control::parse_get_role_result(&client, &result).unwrap()
    }

    async fn build_and_submit_has_role_transaction(
        client: &LedgerClient,
        role: &Role,
        assignee_account: &Address,
    ) -> bool {
        let transaction = role_control::build_has_role_transaction(client, role, assignee_account)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        role_control::parse_has_role_result(&client, &result).unwrap()
    }

    #[async_std::test]
    async fn demo_build_and_submit_assign_and_remove_role_transactions_test() -> VdrResult<()> {
        let signer = basic_signer();
        let (assignee_account, _) = signer.create_account(None).unwrap();
        let client = client();
        let role_to_assign = Role::Endorser;

        let receipt = build_and_submit_assign_role_transaction(
            &client,
            &assignee_account,
            &role_to_assign,
            &signer,
        )
        .await;
        println!("Receipt: {}", receipt);

        let assigned_role = build_and_submit_get_role_transaction(&client, &assignee_account).await;
        assert_eq!(role_to_assign, assigned_role);

        let receipt = build_and_submit_revoke_role_transaction(
            &client,
            &assignee_account,
            &role_to_assign,
            &signer,
        )
        .await;
        println!("Receipt: {}", receipt);

        let has_role =
            build_and_submit_has_role_transaction(&client, &role_to_assign, &assignee_account)
                .await;
        assert!(!has_role);

        Ok(())
    }
}

mod validator {
    use crate::{
        contracts::network::ValidatorAddresses, signer::basic_signer::test::basic_signer,
        validator_control,
    };

    use super::*;

    async fn build_and_submit_get_validators_transaction(
        client: &LedgerClient,
    ) -> ValidatorAddresses {
        let transaction = validator_control::build_get_validators_transaction(&client)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();

        validator_control::parse_get_validators_result(&client, &result).unwrap()
    }

    async fn build_and_submit_add_validator_transaction(
        client: &LedgerClient,
        new_validator_address: &Address,
        signer: &BasicSigner,
    ) -> String {
        let transaction = validator_control::build_add_validator_transaction(
            &client,
            &TRUSTEE_ACC,
            new_validator_address,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    async fn build_and_submit_remove_validator_transaction(
        client: &LedgerClient,
        validator_address: &Address,
        signer: &BasicSigner,
    ) -> String {
        // write
        let transaction = validator_control::build_remove_validator_transaction(
            &client,
            &TRUSTEE_ACC,
            validator_address,
        )
        .await
        .unwrap();
        sign_and_submit_transaction(client, transaction, signer).await
    }

    #[async_std::test]
    async fn demo_build_and_submit_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let (new_validator_address, _) = signer.create_account(None).unwrap();
        let client = client();
        role::build_and_submit_assign_role_transaction(
            &client,
            &TRUSTEE_ACC,
            &Role::Steward,
            &signer,
        )
        .await;

        let receipt =
            build_and_submit_add_validator_transaction(&client, &new_validator_address, &signer)
                .await;
        println!("Receipt: {}", receipt);

        let validator_list = build_and_submit_get_validators_transaction(&client).await;
        assert_eq!(validator_list.len(), 5);
        assert!(validator_list.contains(&new_validator_address));

        let receipt =
            build_and_submit_remove_validator_transaction(&client, &new_validator_address, &signer)
                .await;
        println!("Receipt: {}", receipt);

        let validator_list = build_and_submit_get_validators_transaction(&client).await;
        assert_eq!(validator_list.len(), 4);
        assert!(!validator_list.contains(&new_validator_address));

        Ok(())
    }
}
