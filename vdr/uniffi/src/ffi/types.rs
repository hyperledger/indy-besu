use crate::JsonValue;
use indy2_vdr::{
    ContractConfig as ContractConfig_, ContractSpec as ContractSpec_, PingStatus as PingStatus_,
    QuorumConfig as QuorumConfig_, SignatureData as SignatureData_, Status as Status_,
    TransactionSignature as TransactionSignature_, TransactionType as TransactionType_,
};

#[derive(uniffi::Record)]
pub struct PingStatus {
    pub status: Status,
}

#[derive(uniffi::Enum)]
pub enum Status {
    Ok,
    Err { msg: String },
}

#[derive(uniffi::Record)]
pub struct ContractConfig {
    pub address: String,
    pub spec_path: Option<String>,
    pub spec: Option<ContractSpec>,
}

#[derive(uniffi::Record)]
pub struct ContractSpec {
    pub name: String,
    pub abi: JsonValue,
}

#[derive(uniffi::Enum)]
pub enum TransactionType {
    Read,
    Write,
}

#[derive(uniffi::Record)]
pub struct SignatureData {
    pub recovery_id: u64,
    pub signature: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct TransactionSignature {
    v: u64,
    r: Vec<u8>,
    s: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct QuorumConfig {
    pub nodes: Vec<String>,
    pub request_retries: Option<u8>,
    pub request_timeout: Option<u64>,
    pub retry_interval: Option<u64>,
}

impl From<PingStatus_> for PingStatus {
    fn from(status: PingStatus_) -> Self {
        PingStatus {
            status: Status::from(status.status),
        }
    }
}

impl From<Status_> for Status {
    fn from(status: Status_) -> Self {
        match status {
            Status_::Ok => Status::Ok,
            Status_::Err { msg } => Status::Err { msg },
        }
    }
}

impl Into<ContractConfig_> for ContractConfig {
    fn into(self) -> ContractConfig_ {
        ContractConfig_ {
            address: self.address,
            spec_path: self.spec_path,
            spec: self.spec.map(ContractSpec::into),
        }
    }
}

impl Into<ContractSpec_> for ContractSpec {
    fn into(self) -> ContractSpec_ {
        ContractSpec_ {
            name: self.name,
            abi: self.abi,
        }
    }
}

impl Into<TransactionType_> for TransactionType {
    fn into(self) -> TransactionType_ {
        match self {
            TransactionType::Read => TransactionType_::Read,
            TransactionType::Write => TransactionType_::Write,
        }
    }
}

impl Into<SignatureData_> for SignatureData {
    fn into(self) -> SignatureData_ {
        SignatureData_ {
            recovery_id: self.recovery_id,
            signature: self.signature,
        }
    }
}

impl Into<TransactionSignature_> for TransactionSignature {
    fn into(self) -> TransactionSignature_ {
        TransactionSignature_ {
            v: self.v,
            r: self.r,
            s: self.s,
        }
    }
}

impl Into<QuorumConfig_> for QuorumConfig {
    fn into(self) -> QuorumConfig_ {
        QuorumConfig_ {
            nodes: self.nodes,
            request_retries: self.request_retries,
            request_timeout: self.request_timeout,
            retry_interval: self.retry_interval,
        }
    }
}
