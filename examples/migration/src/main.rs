mod holder;
mod issuer;
mod ledger;
mod trustee;
mod verifier;
mod wallet;

use indy_besu_vdr::{
    CredentialDefinition, DidDocAttribute, PublicKeyAttribute, PublicKeyPurpose, PublicKeyType,
    Role, Schema, ServiceAttribute, ServiceEndpoint,
};

use crate::{holder::Holder, issuer::Issuer, trustee::Trustee, verifier::Verifier};

#[async_std::main]
async fn main() {
    /*
     * Step 1: Setup actors: Trustee, Issuer, Holder, Verifier
     */
    println!("1. Setup actors");
    println!("  1.1 Setup Trustee");
    let mut trustee = Trustee::setup().await;
    println!("  1.2 Setup Holder");
    let mut holder = Holder::setup().await;
    println!("  1.3 Setup Verifier");
    let mut verifier = Verifier::setup().await;
    println!("  1.4 Setup Issuer");
    let mut issuer = Issuer::setup().await;

    /*
     * Set actor to use Indy Ledger
     */
    trustee.use_indy_ledger();
    issuer.use_indy_ledger();
    holder.use_indy_ledger();
    verifier.use_indy_ledger();

    /*
     * Step 2: Before Ledger migration (use Indy) setup Issuer and Credential Data: DID, Schema, Credential Definition
     */
    // Publish Schema
    println!("2. Prepare Issuer/Credential data");
    println!("  2.1 Trustee publish DID");
    trustee
        .publish_indy_did(&issuer.indy_did, &issuer.edkey)
        .await;
    println!("  2.2 Issuer publish Endpoint");
    issuer
        .publish_service_endpoint_to_indy(&issuer.service)
        .await;
    println!("  2.3 Issuer publish Schema");
    let (schema_id, schema) = issuer.create_schema().await;
    issuer.publish_schema_to_indy(&schema).await;
    println!("  2.4 Issuer publish Cred Def");
    let (_, cred_def) = issuer.create_cred_def(&schema_id).await;
    issuer.publish_cred_def_to_indy(&cred_def).await;
    println!("  DID: {}", issuer.indy_did);
    println!("  Schema: {:?}", schema);
    println!("  Credential Definition: {:?}", cred_def);

    /*
     * Step 3: Before Ledger migration (use Indy) issue credential to Holder and verify Proof using Indy Ledger
     */
    println!("3. Issue Credential and Verity Proof");
    println!("  3.1 Issuer create Credential Offer");
    let cred_offer = issuer.create_credential_offer(&schema_id, &cred_def);
    println!("  3.2 Holder create Credential Request");
    let cred_request = holder.create_credential_request(&cred_offer).await;
    println!("  3.3 Issuer sign Credential");
    let mut credential = issuer.sign_credential(&cred_def, &cred_offer, &cred_request);
    println!("  3.4 Holder store Credential");
    holder.store_credential(&mut credential).await;

    // Make sure verification works
    println!("  3.5 Verifier create Proof Request");
    let proof_request = Verifier::request();
    println!("  3.6 Holder create Proof");
    let proof = holder.make_proof(&proof_request, &credential).await;
    println!("  3.7 Verifier verifies Proof");
    let valid = verifier.verify_proof(&proof_request, &proof).await;
    println!("  Verification Result: {}", valid);

    /*
     * Trustee assign role to Issuer in Besu Ledger
     */
    trustee.use_besu_ledger();
    trustee
        .assign_besu_role(&Role::Trustee, &issuer.account)
        .await;

    /*
     * Step 4: Issuer does data migration to Besu Ledger
     */
    println!("4. Issuer migrate data to Besu Ledger");
    println!(
        "  4.1 Issuer publish Besu Ledger key to Indy Ledger to prove DID ownership for Besu key"
    );
    issuer
        .publish_besu_ledger_account_to_indy(&issuer.secpkey)
        .await;

    /*
     * Set actor to use Besu Ledger
     */
    trustee.use_besu_ledger();
    issuer.use_besu_ledger();
    holder.use_besu_ledger();
    verifier.use_besu_ledger();

    println!("  4.2 Issuer publish DID Service and Public Key");
    let service = DidDocAttribute::Service(ServiceAttribute {
        type_: "IndyService".to_string(),
        service_endpoint: ServiceEndpoint::String(issuer.service.to_string()),
    });
    issuer.publish_did_attribute_to_besu(&service).await;

    let key = DidDocAttribute::PublicKey(PublicKeyAttribute {
        purpose: PublicKeyPurpose::VeriKey,
        type_: PublicKeyType::Ed25519VerificationKey2020,
        public_key_hex: None,
        public_key_base64: None,
        public_key_base58: Some(issuer.edkey.to_string()),
        public_key_pem: None,
    });
    issuer.publish_did_attribute_to_besu(&key).await;

    println!("  4.3 Issuer publish Schema");
    let schema = Schema::from_indy_format(&schema, &issuer.besu_did).unwrap();
    let schema_id = issuer.publish_schema_to_besu(&schema).await;

    println!("  4.4 Issuer publish Credential Definition");
    let cred_def =
        CredentialDefinition::from_indy_format(&cred_def, &issuer.besu_did, schema_id.as_ref())
            .unwrap();
    issuer.publish_cred_def_to_besu(&cred_def).await;

    /*
     * Step 5: Verify existing credential using Besu Ledger
     */
    println!("5. Verify existing credential using Besu Ledger");
    println!("  5.1 Verifier create Proof Request");
    let proof_request = Verifier::request();
    println!("  5.2 Holder create Proof");
    let proof = holder.make_proof(&proof_request, &credential).await;
    println!("  5.3 Verifier verifies Proof");
    let valid = verifier.verify_proof(&proof_request, &proof).await;
    println!("  Verification Result: {}", valid);
}
