use log::trace;
use serde_json::json;

#[cfg(not(feature = "wasm"))]
use web3::{ethabi::Error as Web3EthabiError, Error as Web3Error};
#[cfg(feature = "wasm")]
use web3_wasm::{ethabi::Error as Web3EthabiError, Error as Web3Error};

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum VdrError {
    #[error("Ledger: Quorum not reached: {}", _0)]
    QuorumNotReached(String),

    #[error("Ledger Client: Node is unreachable")]
    ClientNodeUnreachable,

    #[error("Ledger Client: Invalid transaction: {}", _0)]
    ClientInvalidTransaction(String),

    #[error("Ledger Client: Got invalid response: {}", _0)]
    ClientInvalidResponse(String),

    #[error("Ledger Client: Transaction reverted: {}", _0)]
    ClientTransactionReverted(String),

    #[error("Ledger Client: Unexpected error occurred: {}", _0)]
    ClientUnexpectedError(String),

    #[error("Ledger Client: Invalid state {}", _0)]
    ClientInvalidState(String),

    #[error("Contract: Invalid name: {}", _0)]
    ContractInvalidName(String),

    #[error("Contract: Invalid specification: {}", _0)]
    ContractInvalidSpec(String),

    #[error("Contract: Invalid data")]
    ContractInvalidInputData,

    #[error("Contract: Invalid response data: {}", _0)]
    ContractInvalidResponseData(String),

    #[error("Signer: Invalid private key")]
    SignerInvalidPrivateKey,

    #[error("Signer: Invalid message")]
    SignerInvalidMessage,

    #[error("Signer: Key is missing: {}", _0)]
    SignerMissingKey(String),

    #[error("Signer: Unexpected error occurred: {}", _0)]
    SignerUnexpectedError(String),

    #[error("Invalid data: {}", _0)]
    CommonInvalidData(String),

    #[error("Could not get transaction: {}", _0)]
    GetTransactionError(String),
}

pub type VdrResult<T> = Result<T, VdrError>;

impl From<Web3Error> for VdrError {
    fn from(value: Web3Error) -> Self {
        let vdr_error = match value {
            Web3Error::Unreachable => VdrError::ClientNodeUnreachable,
            Web3Error::InvalidResponse(err) => VdrError::ClientInvalidResponse(err),
            Web3Error::Rpc(err) => VdrError::ClientTransactionReverted(json!(err).to_string()),
            _ => VdrError::ClientUnexpectedError(value.to_string()),
        };

        trace!(
            "VdrError convert from web3::Error has finished. Result: {:?}",
            vdr_error
        );

        vdr_error
    }
}

impl From<Web3EthabiError> for VdrError {
    fn from(value: Web3EthabiError) -> Self {
        let vdr_error = match value {
            Web3EthabiError::InvalidName(name) => VdrError::ContractInvalidName(name),
            _ => VdrError::ContractInvalidInputData,
        };

        trace!(
            "VdrError convert from web3::ethabi::Error has finished. Result: {:?}",
            vdr_error
        );

        vdr_error
    }
}

#[cfg(feature = "basic_signer")]
impl From<secp256k1::Error> for VdrError {
    fn from(value: secp256k1::Error) -> Self {
        match value {
            secp256k1::Error::InvalidSecretKey => VdrError::SignerInvalidPrivateKey,
            secp256k1::Error::InvalidMessage => VdrError::SignerInvalidMessage,
            err => VdrError::SignerUnexpectedError(err.to_string()),
        }
    }
}
