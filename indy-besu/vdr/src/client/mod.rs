pub mod client;
pub mod constants;
pub mod implementation;
pub mod quorum;

use crate::{
    error::VdrResult,
    types::{Address, ContractOutput, ContractParam, PingStatus},
    Transaction,
};
use async_trait::async_trait;

pub use client::LedgerClient;
pub use constants::*;
pub use quorum::{QuorumConfig, QuorumHandler};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[cfg_attr(not(feature = "wasm"), async_trait)]
#[cfg_attr(feature = "wasm", async_trait(?Send))]
pub trait Client: Sync + Send {
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
    /// - `transaction` transaction to submit
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// hash of a block in which transaction included
    async fn submit_transaction(&self, transaction: &[u8]) -> VdrResult<Vec<u8>>;

    /// Submit read transaction to the ledger
    ///
    /// # Params
    /// - `transaction` prepared transaction to submit
    ///
    /// # Returns
    /// result data of transaction execution
    async fn call_transaction(&self, to: &str, transaction: &[u8]) -> VdrResult<Vec<u8>>;

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

    /// Get the transaction for the given transaction hash
    ///
    /// # Params
    /// - `hash` hash of a transaction to get
    ///
    /// # Returns
    /// transaction for the requested hash
    async fn get_transaction(&self, hash: &[u8]) -> VdrResult<Option<Transaction>>;
}

pub trait Contract: Sync + Send {
    /// Get the address of deployed contract
    ///
    /// # Returns
    /// address of the deployed contract. Should be used to execute contract methods
    fn address(&self) -> &Address;

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
