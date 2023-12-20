mod client;
mod constants;
mod implementation;
mod quorum;

use crate::{
    error::VdrResult,
    types::{Address, ContractOutput, ContractParam, PingStatus, Transaction},
};

pub use client::*;
pub use constants::*;
pub use quorum::*;

use web3::types::{Transaction as Web3Transaction, H256};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait Client: Send + Sync {
    /// Retrieve count of transaction for the given account
    ///
    /// # Params
    /// - `address` address of an account to get number of written transactions
    ///
    /// # Returns
    /// number of transactions
    async fn get_transaction_count(&self, address: &Address) -> VdrResult<[u64; 4]>;

    /// Submit transaction to the ledger
    ///
    /// # Params
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// hash of a block in which transaction included
    async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>>;

    /// Submit read transaction to the ledger
    ///
    /// # Params
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// result data of transaction execution
    async fn call_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>>;

    /// Get the receipt for the given block hash
    ///
    /// # Params
    /// - `hash` hash of a block to get the receipt
    ///
    /// # Returns
    /// receipt as JSON string for the requested block
    async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String>;

    /// Check client connection (passed node is alive and return valid ledger data)
    ///
    /// # Returns
    /// ledger status
    async fn ping(&self) -> VdrResult<PingStatus>;

    async fn get_transaction(&self, hash: H256) -> VdrResult<Option<Web3Transaction>>;
}

pub trait Contract {
    /// Get the address of deployed contract
    ///
    /// # Returns
    /// address of the deployed contract. Should be used to execute contract methods
    fn address(&self) -> String;

    /// Encode data required for the execution of a contract method
    ///
    /// # Params
    /// - `method` method to execute
    /// - `params` data to pass/encode for contract execution
    ///
    /// # Returns
    /// encoded data to set into transaction
    fn encode_input(&self, method: &str, params: &[ContractParam]) -> VdrResult<Vec<u8>>;

    /// Decode the value (bytes) returned as the result of the execution of a contract method
    ///
    /// # Params
    /// - `method` method to execute
    /// - `output` data to decode (returned as result of sending call transaction)
    ///
    /// # Returns
    /// contract execution result in decoded form
    fn decode_output(&self, method: &str, output: &[u8]) -> VdrResult<ContractOutput>;
}
