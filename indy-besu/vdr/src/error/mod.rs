use serde_json::json;

#[cfg(not(feature = "wasm"))]
use web3::{
    Error as Web3Error,
    ethabi::Error as Web3EthabiError
};
#[cfg(feature = "wasm")]
use web3_wasm::{
    Error as Web3Error,
    ethabi::Error as Web3EthabiError
};

#[derive(thiserror::Error, Debug, PartialEq)]
#[cfg_attr(feature = "uni_ffi", derive(uniffi::Error))]
pub enum VdrError {
    #[error("Ledger Client: Node is unreachable")]
    ClientNodeUnreachable,

    #[error("Ledger Client: Invalid transaction: {}", msg)]
    ClientInvalidTransaction { msg: String },

    #[error("Ledger Client: Got invalid response: {}", msg)]
    ClientInvalidResponse { msg: String },

    #[error("Ledger Client: Transaction reverted: {}", msg)]
    ClientTransactionReverted { msg: String },

    #[error("Ledger Client: Unexpected error occurred: {}", msg)]
    ClientUnexpectedError { msg: String },

    #[error("Ledger Client: Invalid state {}", msg)]
    ClientInvalidState { msg: String },

    #[error("Contract: Invalid name: {}", msg)]
    ContractInvalidName { msg: String },

    #[error("Contract: Invalid specification: {}", msg)]
    ContractInvalidSpec { msg: String },

    #[error("Contract: Invalid data")]
    ContractInvalidInputData,

    #[error("Contract: Invalid response data: {}", msg)]
    ContractInvalidResponseData { msg: String },

    #[error("Signer: Invalid private key")]
    SignerInvalidPrivateKey,

    #[error("Signer: Invalid message")]
    SignerInvalidMessage,

    #[error("Signer: Key is missing: {}", msg)]
    SignerMissingKey { msg: String },

    #[error("Signer: Unexpected error occurred: {}", msg)]
    SignerUnexpectedError { msg: String },

    #[error("Invalid data: {}", msg)]
    CommonInvalidData { msg: String },
}

pub type VdrResult<T> = Result<T, VdrError>;

impl From<Web3Error> for VdrError {
    fn from(value: Web3Error) -> Self {
        match value {
            Web3Error::Unreachable => VdrError::ClientNodeUnreachable,
            Web3Error::InvalidResponse(err) => VdrError::ClientInvalidResponse { msg: err },
            Web3Error::Rpc(err) => VdrError::ClientTransactionReverted {
                msg: json!(err).to_string(),
            },
            _ => VdrError::ClientUnexpectedError {
                msg: value.to_string(),
            },
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
        match value {
            Web3EthabiError::InvalidName(name) => VdrError::ContractInvalidName(name),
            _ => VdrError::ContractInvalidInputData,
        }
    }
}

#[cfg(feature = "basic_signer")]
impl From<secp256k1::Error> for VdrError {
    fn from(value: secp256k1::Error) -> Self {
        match value {
            secp256k1::Error::InvalidSecretKey => VdrError::SignerInvalidPrivateKey,
            secp256k1::Error::InvalidMessage => VdrError::SignerInvalidMessage,
            err => VdrError::SignerUnexpectedError {
                msg: err.to_string(),
            },
        }
    }
}
