use log::trace;
use serde_json::json;

#[derive(thiserror::Error, uniffi::Error, Debug, PartialEq)]
pub enum VdrError {
    #[error("Ledger Client: Node is unreachable")]
    ClientNodeUnreachable,

    #[error("Ledger Client: Invalid transaction: {}", msg)]
    ClientInvalidTransaction {
        msg: String
    },

    #[error("Ledger Client: Got invalid response: {}", msg)]
    ClientInvalidResponse {
        msg: String
    },

    #[error("Ledger Client: Transaction reverted: {}", msg)]
    ClientTransactionReverted {
        msg: String
    },

    #[error("Ledger Client: Unexpected error occurred: {}", msg)]
    ClientUnexpectedError {
        msg: String
    },

    #[error("Ledger Client: Invalid state {}", msg)]
    ClientInvalidState {
        msg: String
    },

    #[error("Contract: Invalid name: {}", msg)]
    ContractInvalidName {
        msg: String
    },

    #[error("Contract: Invalid specification: {}", msg)]
    ContractInvalidSpec {
        msg: String
    },

    #[error("Contract: Invalid data")]
    ContractInvalidInputData,

    #[error("Contract: Invalid response data: {}", msg)]
    ContractInvalidResponseData {
        msg: String
    },

    #[error("Signer: Invalid private key")]
    SignerInvalidPrivateKey,

    #[error("Signer: Invalid message")]
    SignerInvalidMessage,

    #[error("Signer: Key is missing: {}", msg)]
    SignerMissingKey {
        msg: String
    },

    #[error("Signer: Unexpected error occurred: {}", msg)]
    SignerUnexpectedError {
        msg: String
    },

    #[error("Invalid data: {}", msg)]
    CommonInvalidData {
        msg: String
    },
}

pub type VdrResult<T> = Result<T, VdrError>;

impl From<web3::Error> for VdrError {
    fn from(value: web3::Error) -> Self {
        let vdr_error = match value {
            web3::Error::Unreachable => VdrError::ClientNodeUnreachable,
            web3::Error::InvalidResponse(err) => VdrError::ClientInvalidResponse { msg: err },
            web3::Error::Rpc(err) => VdrError::ClientTransactionReverted { msg: json!(err).to_string() },
            _ => VdrError::ClientUnexpectedError { msg: value.to_string() },
        };

        trace!(
            "VdrError convert from web3::Error has finished. Result: {:?}",
            vdr_error
        );

        vdr_error
    }
}

impl From<web3::ethabi::Error> for VdrError {
    fn from(value: web3::ethabi::Error) -> Self {
        let vdr_error = match value {
            web3::ethabi::Error::InvalidName(name) => VdrError::ContractInvalidName { msg: name },
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
        let vdr_error = match value {
            secp256k1::Error::InvalidSecretKey => VdrError::SignerInvalidPrivateKey,
            secp256k1::Error::InvalidMessage => VdrError::SignerInvalidMessage,
            err => VdrError::SignerUnexpectedError { msg: err.to_string() },
        };

        trace!(
            "VdrError convert from secp256k1::Error has finished. Result: {:?}",
            vdr_error
        );

        vdr_error
    }
}
