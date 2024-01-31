use crate::{
    client::client::test::{client, TEST_NETWORK},
    contracts::{
        auth::Role,
        cl::types::{credential_definition::test::credential_definition, schema::test::schema},
        did::{PublicKeyAttribute, ETHR_DID_METHOD},
    },
    error::VdrResult,
    signer::basic_signer::{
        test::{basic_signer, TRUSTEE_ACC},
        BasicSigner,
    },
    types::{Address, Transaction},
    LedgerClient,
};

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
        contracts::{
            did::{
                did_ethr_registry,
                did_ethr_registry::test::{public_key, service, validity},
                types::did_doc_attribute::DidDocAttribute,
                DID,
            },
            PublicKeyPurpose, PublicKeyType,
        },
        Address, LedgerClient, Validity, VdrResult,
    };
    use serde_json::json;

    pub(crate) fn did(address: &Address) -> DID {
        DID::build(ETHR_DID_METHOD, Some(TEST_NETWORK), address.as_ref())
    }

    async fn endorse_set_did_attribute(
        client: &LedgerClient,
        identity: &Address,
        did: &DID,
        attribute: &DidDocAttribute,
        validity: &Validity,
        signer: &BasicSigner,
    ) {
        let transaction_endorsing_data = did_ethr_registry::build_did_set_attribute_endorsing_data(
            client, did, attribute, validity,
        )
        .await
        .unwrap();

        let endorsing_sign_bytes = transaction_endorsing_data.get_signing_bytes().unwrap();
        let signature = signer
            .sign(&endorsing_sign_bytes, &identity.to_string())
            .unwrap();

        let transaction = did_ethr_registry::build_did_set_attribute_signed_transaction(
            client,
            &TRUSTEE_ACC,
            did,
            attribute,
            validity,
            &signature,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);
    }

    async fn endorse_revoke_did_attribute(
        client: &LedgerClient,
        identity: &Address,
        did: &DID,
        attribute: &DidDocAttribute,
        signer: &BasicSigner,
    ) {
        let transaction_endorsing_data =
            did_ethr_registry::build_did_revoke_attribute_endorsing_data(client, did, attribute)
                .await
                .unwrap();

        let endorsing_sign_bytes = transaction_endorsing_data.get_signing_bytes().unwrap();
        let signature = signer
            .sign(&endorsing_sign_bytes, &identity.to_string())
            .unwrap();

        let transaction = did_ethr_registry::build_did_revoke_attribute_signed_transaction(
            client,
            &TRUSTEE_ACC,
            did,
            attribute,
            &signature,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);
    }

    #[async_std::test]
    async fn demo_create_did_ethr() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // write service
        let did = super::did::did(&TRUSTEE_ACC.clone());
        let transaction = did_ethr_registry::build_did_set_attribute_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &service(),
            &validity(),
        )
        .await
        .unwrap();
        let _receipt = sign_and_submit_transaction(&client, transaction, &signer).await;

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
        let _receipt = sign_and_submit_transaction(&client, transaction, &signer).await;

        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document.service.len());
        assert_eq!(2, did_document.verification_method.len());
        assert_eq!(
            false,
            did_doc_with_meta.did_document_metadata.deactivated.unwrap()
        );

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_did_ethr() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did::did(&identity);

        // endorse service attribute
        let service = service();
        let validity = validity();
        endorse_set_did_attribute(&client, &identity, &did, &service, &validity, &signer).await;

        // resolve DID
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document.service.len());
        assert_eq!(
            false,
            did_doc_with_meta.did_document_metadata.deactivated.unwrap()
        );

        Ok(())
    }

    #[async_std::test]
    async fn demo_did_ethr_deactivate() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did::did(&identity);

        // endorse service attribute
        let service = service();
        let validity = validity();
        endorse_set_did_attribute(&client, &identity, &did, &service, &validity, &signer).await;

        // deactivate DID
        let new_owner = Address::null();
        let transaction_endorsing_data =
            did_ethr_registry::build_did_change_owner_endorsing_data(&client, &did, &new_owner)
                .await
                .unwrap();

        let signature = signer
            .sign(
                &transaction_endorsing_data.get_signing_bytes()?,
                &identity.to_string(),
            )
            .unwrap();

        let transaction = did_ethr_registry::build_did_change_owner_signed_transaction(
            &client,
            &TRUSTEE_ACC,
            &did,
            &new_owner,
            &signature,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Change owner Receipt: {}", receipt);

        // Resole DID
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document = did_doc_with_meta.did_document.unwrap();

        assert!(did_doc_with_meta.did_document_metadata.deactivated.unwrap());

        assert_eq!(did.short().unwrap(), did_document.id);
        assert_eq!(0, did_document.service.len());
        assert_eq!(0, did_document.verification_method.len());
        assert_eq!(0, did_document.authentication.len());
        assert_eq!(0, did_document.assertion_method.len());

        Ok(())
    }

    #[async_std::test]
    async fn demo_did_add_remove_attribute() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        let did = super::did::did(&identity);

        // endorse service attribute
        let service = service();
        let validity = validity();
        endorse_set_did_attribute(&client, &identity, &did, &service, &validity, &signer).await;

        // endorse first key attribute
        let public_key = public_key();
        endorse_set_did_attribute(&client, &identity, &did, &public_key, &validity, &signer).await;

        // endorse second key attribute
        let public_key_2 = DidDocAttribute::PublicKey(PublicKeyAttribute {
            purpose: PublicKeyPurpose::Enc,
            type_: PublicKeyType::Ed25519VerificationKey2020,
            public_key_base58: Some("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".to_string()),
            ..PublicKeyAttribute::default()
        });
        endorse_set_did_attribute(&client, &identity, &did, &public_key_2, &validity, &signer)
            .await;

        // resolve DID
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_before_remove = did_doc_with_meta.did_document.unwrap();
        assert_eq!(1, did_document_before_remove.service.len());
        assert_eq!(3, did_document_before_remove.verification_method.len());
        assert_eq!(2, did_document_before_remove.key_agreement.len());
        assert_eq!(1, did_document_before_remove.authentication.len());
        assert_eq!(1, did_document_before_remove.assertion_method.len());

        // remove service and second ley
        endorse_revoke_did_attribute(&client, &identity, &did, &public_key, &signer).await;
        endorse_revoke_did_attribute(&client, &identity, &did, &service, &signer).await;

        // resolve DID
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_after_remove = did_doc_with_meta.did_document.unwrap();
        assert_eq!(0, did_document_after_remove.service.len());
        assert_eq!(2, did_document_after_remove.verification_method.len());
        assert_eq!(1, did_document_after_remove.key_agreement.len());
        assert_eq!(1, did_document_after_remove.authentication.len());
        assert_eq!(1, did_document_after_remove.assertion_method.len());

        // add one more key
        let public_key_3 = DidDocAttribute::PublicKey(PublicKeyAttribute {
            purpose: PublicKeyPurpose::VeriKey,
            type_: PublicKeyType::EcdsaSecp256k1VerificationKey2020,
            public_key_hex: Some(
                "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71".to_string(),
            ),
            ..PublicKeyAttribute::default()
        });
        endorse_set_did_attribute(&client, &identity, &did, &public_key_3, &validity, &signer)
            .await;

        // resolve DID
        let did_doc_with_meta = did_ethr_registry::resolve_did(&client, &did, None)
            .await
            .unwrap();
        let did_document_after_add = did_doc_with_meta.did_document.unwrap();
        assert_eq!(0, did_document_after_add.service.len());
        assert_eq!(3, did_document_after_add.verification_method.len());
        assert_eq!(1, did_document_after_add.key_agreement.len());
        assert_eq!(1, did_document_after_add.authentication.len());
        assert_eq!(2, did_document_after_add.assertion_method.len());

        Ok(())
    }
}

mod schema {
    use super::*;
    use crate::{schema_registry, Address, LedgerClient, Schema, SchemaId, DID};

    pub(crate) async fn endorse_schema(
        client: &LedgerClient,
        did: &DID,
        signer: &BasicSigner,
    ) -> (SchemaId, Schema) {
        let identity = Address::try_from(did).unwrap();
        let (schema_id, schema) = schema(did, None);
        let transaction_endorsing_data =
            schema_registry::build_create_schema_endorsing_data(client, &schema_id, &schema)
                .await
                .unwrap();

        let endorsing_sign_bytes = transaction_endorsing_data.get_signing_bytes().unwrap();
        let signature = signer
            .sign(&endorsing_sign_bytes, &identity.to_string())
            .unwrap();

        let transaction = schema_registry::build_create_schema_signed_transaction(
            client,
            &TRUSTEE_ACC.clone(),
            &schema_id,
            &schema,
            &signature,
        )
        .await
        .unwrap();
        let _receipt = sign_and_submit_transaction(client, transaction, signer).await;
        (schema_id, schema)
    }

    #[async_std::test]
    async fn demo_create_schema() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID Document
        let did = super::did::did(&TRUSTEE_ACC.clone());

        // write
        let (schema_id, schema) = schema(&did, None);
        let transaction = schema_registry::build_create_schema_transaction(
            &client,
            &TRUSTEE_ACC.clone(),
            &schema_id,
            &schema,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Receipt: {}", receipt);

        // read
        let resolved_schema = schema_registry::resolve_schema(&client, &schema_id)
            .await
            .unwrap();
        assert_eq!(schema, resolved_schema);

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_schema() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        // create DID Document
        let did = super::did::did(&identity);

        // endorse schema
        let (schema_id, schema) = endorse_schema(&client, &did, &signer).await;

        // read
        let resolved_schema = schema_registry::resolve_schema(&client, &schema_id)
            .await
            .unwrap();
        assert_eq!(schema, resolved_schema);

        Ok(())
    }
}

mod credential_definition {
    use super::*;
    use crate::{credential_definition_registry, schema_registry};

    #[async_std::test]
    async fn demo_create_credential_definition() -> VdrResult<()> {
        let signer = basic_signer();
        let client = client();

        // create DID
        let did = super::did::did(&TRUSTEE_ACC.clone());

        // create Schema
        let (schema_id, schema) = schema(&did, None);
        let transaction = schema_registry::build_create_schema_transaction(
            &client,
            &TRUSTEE_ACC,
            &schema_id,
            &schema,
        )
        .await
        .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("Schema Receipt: {}", receipt);

        // write
        let (credential_definition_id, credential_definition) =
            credential_definition(&did, &schema_id, None);
        let transaction =
            credential_definition_registry::build_create_credential_definition_transaction(
                &client,
                &TRUSTEE_ACC,
                &credential_definition_id,
                &credential_definition,
            )
            .await
            .unwrap();
        let receipt = sign_and_submit_transaction(&client, transaction, &signer).await;
        println!("CredDef Receipt: {}", receipt);

        // read
        let resolved_credential_definition =
            credential_definition_registry::resolve_credential_definition(
                &client,
                &credential_definition_id,
            )
            .await
            .unwrap();
        assert_eq!(credential_definition, resolved_credential_definition);

        Ok(())
    }

    #[async_std::test]
    async fn demo_endorse_credential_definition() -> VdrResult<()> {
        let mut signer = basic_signer();
        let client = client();
        let (identity, _) = signer.create_key(None)?;

        // create DID Document
        let did = super::did::did(&identity);

        // create Schema
        let (schema_id, _) = super::schema::endorse_schema(&client, &did, &signer).await;

        // write
        let (credential_definition_id, credential_definition) =
            credential_definition(&did, &schema_id, None);
        let transaction_endorsing_data =
            credential_definition_registry::build_create_credential_definition_endorsing_data(
                &client,
                &credential_definition_id,
                &credential_definition,
            )
            .await
            .unwrap();

        let endorsing_sign_bytes = transaction_endorsing_data.get_signing_bytes()?;
        let signature = signer
            .sign(&endorsing_sign_bytes, &identity.to_string())
            .unwrap();

        let transaction =
            credential_definition_registry::build_create_credential_definition_signed_transaction(
                &client,
                &TRUSTEE_ACC.clone(),
                &credential_definition_id,
                &credential_definition,
                &signature,
            )
            .await
            .unwrap();
        let _receipt = sign_and_submit_transaction(&client, transaction, &signer).await;

        // read
        let resolved_credential_definition =
            credential_definition_registry::resolve_credential_definition(
                &client,
                &credential_definition_id,
            )
            .await
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
