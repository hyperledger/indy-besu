use ethabi::Uint;
use ethereum::{
    EnvelopedEncodable, LegacyTransaction, LegacyTransactionMessage, TransactionAction,
    TransactionSignature as EthTransactionSignature,
};
use ethereum_types::{H160, H256, U256};
use log::warn;
use log_derive::{logfn, logfn_inputs};
use serde_derive::{Deserialize, Serialize};
use sha3::Digest;
use std::{fmt::Debug, str::FromStr, sync::RwLock};

use crate::{
    client::{GAS_LIMIT, GAS_PRICE},
    error::{VdrError, VdrResult},
    types::{
        contract::UintBytesParam, signature::SignatureData, Address, ContractOutput, ContractParam,
    },
    LedgerClient,
};

/// Type of transaction: write/read
/// depending on the transaction type different client methods will be executed to submit transaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    #[default]
    Read,
    Write,
}

/// Transaction object
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    /// type of transaction: write/read
    /// depending on the transaction type different client methods will be executed to submit transaction
    #[serde(rename = "type")]
    pub type_: TransactionType,
    /// transaction sender account address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    /// transaction recipient address
    pub to: Address,
    /// nonce - count of transaction sent by account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    /// chain id of the ledger
    pub chain_id: u64,
    /// transaction payload
    pub data: Vec<u8>,
    /// transaction signature
    pub signature: RwLock<Option<TransactionSignature>>,
    /// transaction hash
    pub hash: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl Transaction {
    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub fn new(
        type_: TransactionType,
        from: Option<Address>,
        to: Address,
        chain_id: u64,
        data: Vec<u8>,
        nonce: Option<u64>,
        signature: Option<TransactionSignature>,
    ) -> Transaction {
        Transaction {
            type_,
            from,
            to,
            chain_id,
            data,
            nonce,
            signature: RwLock::new(signature),
            hash: None,
        }
    }

    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        let eth_transaction: LegacyTransactionMessage = LegacyTransactionMessage {
            nonce: self.get_nonce()?,
            gas_price: *GAS_PRICE,
            gas_limit: *GAS_LIMIT,
            action: TransactionAction::Call(self.get_to()?),
            value: Default::default(),
            input: self.data.clone(),
            chain_id: Some(self.chain_id),
        };
        let hash = eth_transaction.hash();
        Ok(hash.as_bytes().to_vec())
    }

    #[logfn(Info)]
    #[logfn_inputs(Debug)]
    pub fn set_signature(&self, signature_data: SignatureData) {
        let mut signature = self.signature.write().unwrap();
        *signature = Some(TransactionSignature {
            v: signature_data.v().0 + 35 + self.chain_id * 2,
            r: signature_data.r().0,
            s: signature_data.s().0,
        })
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn encode(&self) -> VdrResult<Vec<u8>> {
        let transaction = LegacyTransaction {
            nonce: self.get_nonce()?,
            gas_price: *GAS_PRICE,
            gas_limit: *GAS_LIMIT,
            action: TransactionAction::Call(self.get_to()?),
            value: Default::default(),
            input: self.data.clone(),
            signature: self.get_transaction_signature()?,
        };
        Ok(transaction.encode().to_vec())
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_to(&self) -> VdrResult<H160> {
        H160::from_str(self.to.as_ref()).map_err(|_| {
            VdrError::ClientInvalidTransaction(format!(
                "Invalid transaction target address {:?}",
                self.to
            ))
        })
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_nonce(&self) -> VdrResult<U256> {
        let nonce = self.nonce.ok_or_else(|| {
            VdrError::ClientInvalidTransaction("Transaction `nonce` is not set".to_string())
        })?;
        Ok(U256::from(nonce))
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_transaction_signature(&self) -> VdrResult<EthTransactionSignature> {
        let signature = self.signature.read().unwrap();
        let signature = signature
            .as_ref()
            .ok_or_else(|| VdrError::ClientInvalidTransaction("Missing signature".to_string()))?
            .clone();

        let signature = EthTransactionSignature::new(
            signature.v,
            H256::from_slice(&signature.r),
            H256::from_slice(&signature.s),
        )
        .ok_or_else(|| {
            VdrError::ClientInvalidTransaction("Transaction `nonce` is not set".to_string())
        })?;
        Ok(signature)
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        let self_signature = self.signature.read().unwrap();
        let other_signature = other.signature.read().unwrap();
        self.type_ == other.type_
            && self.from == other.from
            && self.to == other.to
            && self.nonce == other.nonce
            && self.chain_id == other.chain_id
            && self.data == other.data
            && *self_signature == *other_signature
    }
}

#[cfg(test)]
impl Clone for Transaction {
    fn clone(&self) -> Self {
        Transaction {
            type_: self.type_.clone(),
            from: self.from.clone(),
            to: self.to.clone(),
            nonce: self.nonce.clone(),
            chain_id: self.chain_id.clone(),
            data: self.data.clone(),
            signature: RwLock::new(self.signature.read().unwrap().clone()),
            hash: self.hash.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TransactionBuilder {
    contract: String,
    method: String,
    from: Option<Address>,
    nonce: Option<[u64; 4]>,
    params: Vec<ContractParam>,
    type_: TransactionType,
}

impl TransactionBuilder {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn new() -> TransactionBuilder {
        TransactionBuilder::default()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_contract(mut self, contract: &str) -> TransactionBuilder {
        self.contract = contract.to_string();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_method(mut self, method: &str) -> TransactionBuilder {
        self.method = method.to_string();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_param<T: TryInto<ContractParam, Error = VdrError> + Debug>(
        mut self,
        param: T,
    ) -> VdrResult<TransactionBuilder> {
        self.params.push(param.try_into()?);
        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_type(mut self, type_: TransactionType) -> TransactionBuilder {
        self.type_ = type_;
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_from(mut self, from: &Address) -> TransactionBuilder {
        self.from = Some(from.clone());
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub async fn build(self, client: &LedgerClient) -> VdrResult<Transaction> {
        let contract = client.contract(&self.contract)?;

        let data = contract
            .function(&self.method)?
            .encode_input(&self.params)?;

        let nonce = match self.type_ {
            TransactionType::Write => {
                let from = self.from.as_ref().ok_or_else(|| {
                    VdrError::ClientInvalidTransaction(
                        "Transaction `sender` is not set".to_string(),
                    )
                })?;
                Some(client.get_transaction_count(from).await?)
            }
            TransactionType::Read => None,
        };

        let transaction = Transaction {
            type_: self.type_,
            from: self.from,
            to: contract.address().clone(),
            chain_id: client.chain_id(),
            data,
            nonce,
            signature: RwLock::new(None),
            hash: None,
        };
        Ok(transaction)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TransactionParser {
    contract: String,
    method: String,
}

impl TransactionParser {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn new() -> TransactionParser {
        TransactionParser::default()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_contract(mut self, contract: &str) -> TransactionParser {
        self.contract = contract.to_string();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_method(mut self, method: &str) -> TransactionParser {
        self.method = method.to_string();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn parse<T: TryFrom<ContractOutput, Error = VdrError> + Debug>(
        self,
        client: &LedgerClient,
        bytes: &[u8],
    ) -> VdrResult<T> {
        if bytes.is_empty() {
            let vdr_error =
                VdrError::ContractInvalidResponseData("Empty response bytes".to_string());

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }
        let contract = client.contract(&self.contract)?;
        let output = contract
            .function(&self.method)?
            .decode_output(bytes)
            .map(ContractOutput::from)?;

        if output.is_empty() {
            let vdr_error =
                VdrError::ContractInvalidResponseData("Unable to parse response".to_string());

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }

        T::try_from(output)
    }
}

/// Transaction Endorsing object
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionEndorsingData {
    pub to: Address,
    pub from: Address,
    pub params: Vec<ContractParam>,
}

impl TransactionEndorsingData {
    const PREFIX: u8 = 0x19;
    const VERSION: u8 = 0x0;

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        let mut tokens = vec![
            ContractParam::Uint(Uint::from(Self::PREFIX)),
            ContractParam::FixedBytes(vec![Self::VERSION]),
            (&self.to).try_into()?,
        ];
        tokens.extend_from_slice(self.params.as_slice());

        let encoded = ethers_core::abi::encode_packed(&tokens).unwrap();
        let hash = sha3::Keccak256::digest(encoded).to_vec();
        Ok(hash)
    }
}

#[derive(Debug, Default)]
pub(crate) struct TransactionEndorsingDataBuilder {
    contract: String,
    identity: Address,
    params: Vec<ContractParam>,
}

impl TransactionEndorsingDataBuilder {
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn new() -> TransactionEndorsingDataBuilder {
        TransactionEndorsingDataBuilder::default()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_contract(mut self, contract: &str) -> TransactionEndorsingDataBuilder {
        self.contract = contract.to_string();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn set_identity(mut self, identity: &Address) -> TransactionEndorsingDataBuilder {
        self.identity = identity.to_owned();
        self
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn add_param<T: TryInto<ContractParam, Error = VdrError> + Debug>(
        mut self,
        param: T,
    ) -> VdrResult<TransactionEndorsingDataBuilder> {
        self.params.push(param.try_into()?);
        Ok(self)
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub async fn build(self, client: &LedgerClient) -> VdrResult<TransactionEndorsingData> {
        let contract = client.contract(&self.contract)?;
        Ok(TransactionEndorsingData {
            to: contract.address().to_owned(),
            from: self.identity.to_owned(),
            params: self.params,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Block(u64);

impl Block {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for Block {
    fn from(value: u64) -> Self {
        Block(value)
    }
}

impl TryFrom<ContractOutput> for Block {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        Ok(Block::from(value.get_u64(0)?))
    }
}

impl From<&Block> for ContractParam {
    fn from(value: &Block) -> Self {
        ContractParam::Uint(Uint::from(value.0))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Nonce(u64);

impl Nonce {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Nonce(value)
    }
}

impl TryFrom<ContractOutput> for Nonce {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        Ok(Nonce::from(value.get_u64(0)?))
    }
}

impl TryFrom<&Nonce> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &Nonce) -> Result<Self, Self::Error> {
        UintBytesParam::from(value.0).try_into()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlockDetails {
    pub number: u64,
    pub timestamp: u64,
}


#[cfg(test)]
pub mod test {

    use super::*;
    use crate::{
        client::client::test::{
            client, mock_client, write_transaction, CONTRACT_METHOD_EXAMPLE, CONTRACT_NAME_EXAMPLE,
            VALIDATOR_LIST_BYTES,
        },
        contracts::{
            cl::types::credential_definition::test::_CREDENTIAL_DEFINITION_ID,
            network::ValidatorAddresses,
        },
    };

    #[cfg(test)]
    pub mod txn_test {
        use super::*;

        #[async_std::test]
        async fn get_to_invalid() {
            let transaction = Transaction {
                to: Address::from("123"),
                ..write_transaction()
            };

            let get_to_err = transaction.get_to().unwrap_err();

            assert!(matches!(
                get_to_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn get_nonce_not_set() {
            let transaction = Transaction {
                nonce: None,
                ..write_transaction()
            };

            let get_nonce_err = transaction.get_nonce().unwrap_err();

            assert!(matches!(
                get_nonce_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn get_nonce_invalid() {
            let transaction = Transaction {
                nonce: Some(vec![1, 1]),
                ..write_transaction()
            };

            let get_nonce_err = transaction.get_nonce().unwrap_err();

            assert!(matches!(
                get_nonce_err,  | VdrError::CommonInvalidData { .. }
            ));
        }

        #[async_std::test]
        async fn get_transaction_signature_not_set() {
            let transaction = Transaction {
                signature: RwLock::new(None),
                ..write_transaction()
            };

            let get_sig_err = transaction.get_transaction_signature().unwrap_err();

            assert!(matches!(
                get_sig_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn get_transaction_signature_invalid() {
            let transaction = Transaction {
                signature: RwLock::new(Some(TransactionSignature {
                    v: 1,
                    r: vec![1; 32],
                    s: vec![1; 32],
                })),
                ..write_transaction()
            };

            let get_sig_err = transaction.get_transaction_signature().unwrap_err();

            assert!(matches!(
                get_sig_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }
    }

    #[cfg(test)]
    pub mod txn_builder_test {

        use super::*;

        #[async_std::test]
        async fn build_txn_contract_not_set() {
            let client = client();

            let build_txn_err = TransactionBuilder::new()
                .set_method(CONTRACT_NAME_EXAMPLE)
                .add_param(ContractParam::String(String::from(
                    _CREDENTIAL_DEFINITION_ID,
                )))
                .set_type(TransactionType::Read)
                .build(&client)
                .await
                .unwrap_err();

            assert!(matches!(
                build_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }

        #[async_std::test]
        async fn build_txn_contract_method_does_not_exist() {
            let client = client();

            let build_txn_err = TransactionBuilder::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method("123")
                .add_param(ContractParam::String(String::from(
                    _CREDENTIAL_DEFINITION_ID,
                )))
                .set_type(TransactionType::Read)
                .build(&client)
                .await
                .unwrap_err();

            assert!(matches!(
                build_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }

        #[async_std::test]
        async fn build_txn_write_sender_does_not_set() {
            let validator_address = &Address::from("0x93917cadbace5dfce132b991732c6cda9bcc5b8a");
            let client = client();

            let build_txn_err = TransactionBuilder::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .add_param(validator_address.try_into().unwrap())
                .set_type(TransactionType::Write)
                .build(&client)
                .await
                .unwrap_err();

            assert!(matches!(
                build_txn_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn build_txn_invalid_account_address() {
            let validator_address = &Address::from("0x93917cadbace5dfce132b991732c6cda9bcc5b8a");
            let client = client();

            let build_txn_err = TransactionBuilder::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .add_param(validator_address.try_into().unwrap())
                .set_type(TransactionType::Write)
                .set_from(&Address::default())
                .build(&client)
                .await
                .unwrap_err();

            assert!(matches!(
                build_txn_err,  | VdrError::ClientInvalidTransaction { .. }
            ));
        }

        #[async_std::test]
        async fn build_txn_address_with_0x_prefix() {
            let validator_address = &Address::from("0x93917cadbace5dfce132b991732c6cda9bcc5b8a");
            let client = mock_client();

            TransactionBuilder::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .add_param(validator_address.try_into().unwrap())
                .set_type(TransactionType::Write)
                .set_from(&Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"))
                .build(&client)
                .await
                .unwrap();
        }

        #[async_std::test]
        async fn build_txn_address_without_0x_prefix() {
            let validator_address = &Address::from("93917cadbace5dfce132b991732c6cda9bcc5b8a");
            let client = mock_client();

            TransactionBuilder::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .add_param(validator_address.try_into().unwrap())
                .set_type(TransactionType::Write)
                .set_from(&Address::from("0xf0e2db6c8dc6c681bb5d6ad121a107f300e9b2b5"))
                .build(&client)
                .await
                .unwrap();
        }
    }

    #[cfg(test)]
    pub mod txn_parser_test {

        use super::*;

        #[async_std::test]
        async fn parse_txn_empty_response_bytes() {
            let response = [];
            let client = mock_client();

            let parse_txn_err = TransactionParser::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .parse::<ValidatorAddresses>(&client, &response)
                .unwrap_err();

            assert!(matches!(
                parse_txn_err,  | VdrError::ContractInvalidResponseData { .. }
            ));
        }

        #[async_std::test]
        async fn parse_txn_contract_not_set() {
            let client = mock_client();

            let parse_txn_err = TransactionParser::new()
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .parse::<ValidatorAddresses>(&client, &VALIDATOR_LIST_BYTES)
                .unwrap_err();

            assert!(matches!(
                parse_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }

        #[async_std::test]
        async fn parse_txn_contract_does_not_exist() {
            let client = mock_client();

            let parse_txn_err = TransactionParser::new()
                .set_contract("123")
                .set_method(CONTRACT_METHOD_EXAMPLE)
                .parse::<ValidatorAddresses>(&client, &VALIDATOR_LIST_BYTES)
                .unwrap_err();

            assert!(matches!(
                parse_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }

        #[async_std::test]
        async fn parse_txn_contract_method_not_set() {
            let client = mock_client();

            let parse_txn_err = TransactionParser::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .parse::<ValidatorAddresses>(&client, &VALIDATOR_LIST_BYTES)
                .unwrap_err();

            assert!(matches!(
                parse_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }

        #[async_std::test]
        async fn parse_txn_contract_method_does_not_exist() {
            let client = mock_client();

            let parse_txn_err = TransactionParser::new()
                .set_contract(CONTRACT_NAME_EXAMPLE)
                .set_method("123")
                .parse::<ValidatorAddresses>(&client, &VALIDATOR_LIST_BYTES)
                .unwrap_err();

            assert!(matches!(
                parse_txn_err,  | VdrError::ContractInvalidName { .. }
            ));
        }
    }
}