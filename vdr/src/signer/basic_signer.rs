use crate::{
    error::{VdrError, VdrResult},
    signer::Signer,
};

use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;

use crate::{client::Address, signer::SignatureData};
use std::str::FromStr;
use web3::signing::keccak256;

pub struct KeyPair {
    #[allow(unused)]
    public_key: PublicKey,
    private_key: SecretKey,
}

pub struct BasicSigner {
    secp: Secp256k1<All>,
    keys: HashMap<String, KeyPair>,
}

impl BasicSigner {
    pub fn new() -> VdrResult<BasicSigner> {
        Ok(BasicSigner {
            secp: Secp256k1::new(),
            keys: HashMap::new(),
        })
    }

    pub fn create_key(&mut self, private_key: Option<&str>) -> VdrResult<(Address, Vec<u8>)> {
        let (account, key_pair) = self.create_account(private_key)?;
        let public_key_bytes = key_pair.public_key.serialize_uncompressed().to_vec();
        self.keys.insert(account.value().to_string(), key_pair);
        Ok((account, public_key_bytes))
    }

    fn key_for_account(&self, account: &str) -> VdrResult<&KeyPair> {
        self.keys
            .get(account)
            .ok_or(VdrError::SignerMissingKey(account.to_string()))
    }

    fn account_from_key(&self, public_key: &PublicKey) -> String {
        let hash = keccak256(&public_key.serialize_uncompressed()[1..]);
        format!("0x{}", hex::encode(&hash[12..]))
    }

    pub fn create_account(&self, private_key: Option<&str>) -> VdrResult<(Address, KeyPair)> {
        let private_key = match private_key {
            Some(private_key) => SecretKey::from_str(private_key)?,
            None => SecretKey::new(&mut rand::thread_rng()),
        };
        let public_key = PublicKey::from_secret_key(&self.secp, &private_key);
        let address = Address::new(&self.account_from_key(&public_key));
        let key_pair = KeyPair {
            public_key,
            private_key,
        };
        Ok((address, key_pair))
    }
}

impl Signer for BasicSigner {
    fn sign(&self, message: &[u8], account: &str) -> VdrResult<SignatureData> {
        let key = self.key_for_account(account)?;
        let message = Message::from_digest_slice(message)?;
        let (recovery_id, signature) = self
            .secp
            .sign_ecdsa_recoverable(&message, &key.private_key)
            .serialize_compact();
        Ok(SignatureData {
            recovery_id: recovery_id.to_i32() as u64,
            signature: signature.to_vec(),
        })
    }
}

pub mod test {
    use super::*;
    use once_cell::sync::Lazy;

    pub static ACCOUNT: Lazy<Address> =
        Lazy::new(|| Address::new("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"));

    pub const PRIVATE_KEY: &'static str =
        "8bbbb1b345af56b560a5b20bd4b0ed1cd8cc9958a16262bc75118453cb546df7";

    pub fn basic_signer() -> BasicSigner {
        let mut signer = BasicSigner::new().unwrap();
        signer.create_key(Some(PRIVATE_KEY)).unwrap();
        signer
    }

    #[test]
    fn add_key_test() {
        let basic_signer = basic_signer();
        basic_signer.key_for_account(ACCOUNT.value()).unwrap();
    }
}
