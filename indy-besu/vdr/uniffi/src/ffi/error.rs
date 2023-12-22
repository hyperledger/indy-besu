use indy2_vdr::VdrError as VdrError_;

#[derive(thiserror::Error, Debug)]
#[derive(uniffi::Error)]
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

impl From<VdrError_> for VdrError {
    fn from(error: VdrError_) -> Self {
        match error {
            VdrError_::ClientNodeUnreachable => VdrError::ClientNodeUnreachable,
            VdrError_::ClientInvalidTransaction { msg } => VdrError::ClientInvalidTransaction { msg },
            VdrError_::ClientInvalidResponse { msg } => VdrError::ClientInvalidResponse { msg },
            VdrError_::ClientTransactionReverted { msg } => VdrError::ClientTransactionReverted { msg },
            VdrError_::ClientUnexpectedError { msg } => VdrError::ClientUnexpectedError { msg },
            VdrError_::ClientInvalidState { msg } => VdrError::ClientInvalidState { msg },
            VdrError_::ContractInvalidName { msg } => VdrError::ContractInvalidName { msg },
            VdrError_::ContractInvalidSpec { msg } => VdrError::ContractInvalidSpec { msg },
            VdrError_::ContractInvalidInputData => VdrError::ContractInvalidInputData,
            VdrError_::ContractInvalidResponseData { msg } => VdrError::ContractInvalidResponseData { msg },
            VdrError_::SignerInvalidPrivateKey => VdrError::SignerInvalidPrivateKey,
            VdrError_::SignerInvalidMessage => VdrError::SignerInvalidMessage,
            VdrError_::SignerMissingKey { msg } => VdrError::SignerMissingKey { msg },
            VdrError_::SignerUnexpectedError { msg } => VdrError::SignerUnexpectedError { msg },
            VdrError_::CommonInvalidData { msg } => VdrError::CommonInvalidData { msg },
        }
    }
}