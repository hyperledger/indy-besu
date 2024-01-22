use crate::{
    client::client::test::{client, IDENTITY_ACC},
    contracts::{
        auth::Role,
        cl::{
            schema_registry,
            types::{credential_definition::test::credential_definition, schema::test::schema},
            CredentialDefinition, Schema, SchemaId,
        },
        did::{types::did_doc::test::did_doc, DidDocument, DID},
    },
    did_indy_registry,
    error::VdrResult,
    signer::basic_signer::{
        test::{basic_signer, TRUSTEE_ACC},
        BasicSigner,
    },
    types::{Address, Transaction},
    LedgerClient,
};

async fn create_did(client: &LedgerClient, signer: &crate::BasicSigner) -> DidDocument {
    let did_doc = did_doc(None);
    let transaction = did_indy_registry::build_create_did_transaction(
        &client,
        &TRUSTEE_ACC,
        &IDENTITY_ACC,
        &did_doc.id,
        &did_doc,
    )
    .await
    .unwrap();

    let sign_bytes = transaction.get_signing_bytes().unwrap();
    let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
    transaction.set_signature(signature);

    client.submit_transaction(&transaction).await.unwrap();
    did_doc
}

pub async fn create_schema(
    client: &LedgerClient,
    issuer_id: &DID,
    signer: &crate::BasicSigner,
) -> (SchemaId, Schema) {
    let (id, schema) = schema(issuer_id, None);
    let transaction =
        schema_registry::build_create_schema_transaction(&client, &TRUSTEE_ACC, &id, &schema)
            .await
            .unwrap();

    let sign_bytes = transaction.get_signing_bytes().unwrap();
    let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
    transaction.set_signature(signature);

    client.submit_transaction(&transaction).await.unwrap();
    (id, schema)
}

async fn sign_and_submit_transaction(
    client: &LedgerClient,
    transaction: Transaction,
    signer: &BasicSigner,
) -> String {
    let sign_bytes = transaction.get_signing_bytes().unwrap();
    let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
    transaction.set_signature(signature);
    let block_hash = client.submit_transaction(&transaction).await.unwrap();
    client.get_receipt(&block_hash).await.unwrap()
}

mod did {
    use super::*;
    use crate::{
        contracts::did::{
            did_ethr_registry,
            did_ethr_registry::test::{public_key, service, validity},
            types::did_doc_attribute::DidDocAttribute,
        },
        did_indy_registry, DID,
    };

    pub(crate) async fn build_and_submit_create_did_doc_transaction(
        client: &LedgerClient,
        did_doc: &DidDocument,
        signer: &BasicSigner,
    ) -> String {
        let transaction = did_indy_registry::build_create_did_transaction(
            &client,
            &TRUSTEE_ACC,
            &IDENTITY_ACC,
            &did_doc.id,
            did_doc,
        )
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
        let transaction = did_indy_registry::build_resolve_did_transaction(&client, &did_doc.id)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let resolved_did_doc =
            did_indy_registry::parse_resolve_did_result(&client, &result).unwrap();
        assert_eq!(did_doc, resolved_did_doc);

        Ok(())
    }

    #[async_std::test]
    async fn demo_build_and_submit_did_ethr_transaction_test() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // write
        let did = DID::from(format!("did:ethr:{}", TRUSTEE_ACC.clone().as_ref()).as_str());
        let transaction = did_ethr_registry::build_did_set_attribute_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &service(),
            &validity(),
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);

        // read event
        let transaction = did_ethr_registry::build_get_did_events_query(&client, &did, None, None)
            .await
            .unwrap();
        let events = client.query_events(&transaction).await.unwrap();
        let event = did_ethr_registry::parse_did_event_response(&client, &events[0]).unwrap();
        let _attribute: DidDocAttribute = event.try_into().unwrap();

        // read changed
        let transaction = did_ethr_registry::build_get_did_changed_transaction(&client, &did)
            .await
            .unwrap();
        let result = client.submit_transaction(&transaction).await.unwrap();
        let changed = did_ethr_registry::parse_did_changed_result(&client, &result).unwrap();
        assert!(!changed.is_none());

        // write
        let transaction = did_ethr_registry::build_did_set_attribute_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &public_key(),
            &validity(),
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);

        let did_doc = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        assert_eq!(1, did_doc.did_document.service.len());
        assert_eq!(2, did_doc.did_document.verification_method.len());

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_did_ethr_transaction_test() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();

        let (identity, _) = signer.create_key(None)?;

        // write
        let did = DID::from(format!("did:ethr:{}", identity.to_string()).as_str());
        let service = service();
        let validity = validity();

        let transaction_endorsing_data = did_ethr_registry::build_did_set_attribute_endorsing_data(
            &client, &did, &service, &validity,
        )
        .await
        .unwrap();

        let endorsing_sign_bytes = transaction_endorsing_data.get_signing_bytes()?;
        let signature = signer
            .sign(&endorsing_sign_bytes, &identity.to_string())
            .unwrap();

        let transaction = did_ethr_registry::build_did_set_attribute_signed_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &service,
            &validity,
            &signature,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);

        let did_doc = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        assert_eq!(1, did_doc.did_document.service.len());

        Ok(())
    }
}

mod schema {
    use super::*;
    use crate::{schema_registry, SchemaId};

    pub(crate) async fn build_and_submit_create_schema_transaction(
        client: &LedgerClient,
        id: &SchemaId,
        schema: &Schema,
        signer: &BasicSigner,
    ) -> String {
        let transaction =
            schema_registry::build_create_schema_transaction(&client, &TRUSTEE_ACC, id, schema)
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
        let (schema_id, schema) = schema(&did_doc.id, None);
        let receipt =
            build_and_submit_create_schema_transaction(&client, &schema_id, &schema, &signer).await;
        println!("Receipt: {}", receipt);

        // read
        let transaction = schema_registry::build_resolve_schema_transaction(&client, &schema_id)
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
    use crate::{credential_definition_registry, CredentialDefinitionId};

    pub(crate) async fn build_and_submit_create_cred_def_transaction(
        client: &LedgerClient,
        id: &CredentialDefinitionId,
        cred_def: &CredentialDefinition,
        signer: &BasicSigner,
    ) -> String {
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                id,
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
        let (schema_id, _) = create_schema(&client, &did_doc.id, &signer).await;

        // write
        let (credential_definition_id, credential_definition) =
            credential_definition(&did_doc.id, &schema_id, None);
        let receipt = build_and_submit_create_cred_def_transaction(
            &client,
            &credential_definition_id,
            &credential_definition,
            &signer,
        )
        .await;
        println!("Receipt: {}", receipt);

        // read
        let transaction =
            credential_definition_registry::build_resolve_credential_definition_transaction(
                &client,
                &credential_definition_id,
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
        let signature = signer.sign(&sign_bytes, TRUSTEE_ACC.as_ref()).unwrap();
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
