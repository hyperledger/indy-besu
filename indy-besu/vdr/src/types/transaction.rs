use ethereum::{LegacyTransactionMessage, TransactionAction};
use ethereum_types::{H160, U256};
use log::{trace, warn};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    client::GAS,
    error::{VdrError, VdrResult},
    types::{Address, ContractOutput, ContractParam},
    LedgerClient,
};

/// Type of transaction: write/read
/// depending on the transaction type different client methods will be executed to submit transaction
#[derive(Clone, Debug, PartialEq, uniffi::Enum)]
pub enum TransactionType {
    Read,
    Write,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Read
    }
}

/// Transaction object
#[derive(Clone, Debug, Default, PartialEq, uniffi::Object)]
pub struct Transaction {
    /// type of transaction: write/read
    /// depending on the transaction type different client methods will be executed to submit transaction
    pub type_: TransactionType,
    /// transaction sender account address
    pub from: Option<Address>,
    /// transaction recipient address
    pub to: String,
    /// nonce - count of transaction sent by account
    pub nonce: Option<Vec<u64>>,
    /// chain id of the ledger
    pub chain_id: u64,
    /// transaction payload
    pub data: Vec<u8>,
    /// transaction signature
    pub signature: Option<TransactionSignature>,
}

#[derive(Clone, Debug, Default, PartialEq, uniffi::Record)]
pub struct TransactionSignature {
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct SignatureData {
    /// recovery ID using for public key recovery
    pub recovery_id: u64,
    /// ECDSA signature
    pub signature: Vec<u8>,
}

#[uniffi::export]
impl Transaction {
    #[uniffi::constructor]
    pub fn new(type_: TransactionType,
               from: Option<Address>,
               to: String,
               chain_id: u64,
               data: Vec<u8>,
               nonce: Option<Vec<u64>>,
               signature: Option<TransactionSignature>,
    ) -> Transaction {
        Transaction {
            type_,
            from,
            to,
            chain_id,
            data,
            nonce,
            signature,
        }
    }

    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        let nonce = self.nonce.as_ref().ok_or_else(|| {
            VdrError::ClientInvalidTransaction {
                msg: "Transaction `nonce` is not set".to_string()
            }
        })?;
        let to = H160::from_str(&self.to).map_err(|_| {
            VdrError::ClientInvalidTransaction {
                msg: format!(
                    "Invalid transaction target address {}",
                    self.to
                )
            }
        })?;

        let nonce: [u64; 4] = nonce.clone().try_into()
            .map_err(|_| VdrError::CommonInvalidData {
                msg: "Invalid nonce provided".to_string()
            })?;

        let eth_transaction = LegacyTransactionMessage {
            nonce: U256(nonce),
            gas_price: U256([0, 0, 0, 0]),
            gas_limit: U256([GAS, 0, 0, 0]),
            action: TransactionAction::Call(to),
            value: Default::default(),
            input: self.data.clone(),
            chain_id: Some(self.chain_id),
        };

        let hash = eth_transaction.hash();
        Ok(hash.as_bytes().to_vec())
    }
}

impl Transaction {
    /// set signature for the transaction
    pub fn set_signature(&mut self, signature_data: SignatureData) {
        let v = signature_data.recovery_id + 35 + self.chain_id * 2;
        self.signature = Some(TransactionSignature {
            v,
            r: signature_data.signature[..32].to_vec(),
            s: signature_data.signature[32..].to_vec(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransactionBuilder {
    contract: String,
    method: String,
    from: Option<Address>,
    nonce: Option<[u64; 4]>,
    params: Vec<ContractParam>,
    type_: TransactionType,
}

impl TransactionBuilder {
    pub fn new() -> TransactionBuilder {
        TransactionBuilder::default()
    }

    pub fn set_contract(mut self, contract: &str) -> TransactionBuilder {
        self.contract = contract.to_string();

        trace!(
            "Set contract: {} to TransactionBuilder: {:?}",
            contract,
            self
        );

        self
    }

    pub fn set_method(mut self, method: &str) -> TransactionBuilder {
        self.method = method.to_string();

        trace!("Set method: {} to TransactionBuilder: {:?}", method, self);

        self
    }

    pub fn add_param(mut self, param: ContractParam) -> TransactionBuilder {
        self.params.push(param.clone());

        trace!(
            "Added ContractParam: {:?} to TransactionBuilder: {:?}",
            param,
            self
        );

        self
    }

    pub fn set_type(mut self, type_: TransactionType) -> TransactionBuilder {
        self.type_ = type_.clone();

        trace!(
            "Set TransactionType: {:?} to TransactionBuilder: {:?}",
            type_,
            self
        );

        self
    }

    pub fn set_from(mut self, from: &Address) -> TransactionBuilder {
        self.from = Some(from.clone());

        trace!("Set from: {:?} to TransactionBuilder: {:?}", from, self);

        self
    }

    pub async fn build(self, client: &LedgerClient) -> VdrResult<Transaction> {
        let contract = client.contract(&self.contract)?;
        let data = contract.encode_input(&self.method, &self.params)?;
        let nonce = match self.type_ {
            TransactionType::Write => {
                let from = self.from.as_ref().ok_or_else(|| {
                    VdrError::ClientInvalidTransaction {
                        msg: "Transaction `sender` is not set".to_string()
                    }
                })?;

                let nonce = client.get_transaction_count(from).await?;
                Some(nonce.to_vec())
            }
            TransactionType::Read => None,
        };

        let transaction = Transaction {
            type_: self.type_,
            from: self.from,
            to: contract.address(),
            chain_id: client.chain_id(),
            data,
            nonce,
            signature: None,
        };

        trace!("Built transaction: {:?}", transaction);

        Ok(transaction)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransactionParser {
    contract: String,
    method: String,
}

impl TransactionParser {
    pub fn new() -> TransactionParser {
        TransactionParser::default()
    }

    pub fn set_contract(mut self, contract: &str) -> TransactionParser {
        self.contract = contract.to_string();

        trace!(
            "Set contract: {} to TransactionParser: {:?}",
            contract,
            self
        );

        self
    }

    pub fn set_method(mut self, method: &str) -> TransactionParser {
        self.method = method.to_string();

        trace!("Set method: {} to TransactionParser: {:?}", method, self);

        self
    }

    pub fn parse<T: TryFrom<ContractOutput, Error=VdrError>>(
        self,
        client: &LedgerClient,
        bytes: &[u8],
    ) -> VdrResult<T> {
        if bytes.is_empty() {
            let vdr_error =
                VdrError::ContractInvalidResponseData {
                    msg: "Empty response bytes".to_string()
                };

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }
        let contract = client.contract(&self.contract)?;
        let output = contract.decode_output(&self.method, bytes)?;

        if output.is_empty() {
            let vdr_error =
                VdrError::ContractInvalidResponseData {
                    msg: "Unable to parse response".to_string()
                };

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }

        trace!("Decoded transaction output: {:?}", output);

        T::try_from(output)
    }
}
