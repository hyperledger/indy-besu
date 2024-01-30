use crate::ffi::{
    error::{VdrError, VdrResult},
    types::{SignatureData, TransactionSignature, TransactionType},
};
use indy_besu_vdr::{
    Address, Transaction as Transaction_, TransactionEndorsingData as TransactionEndorsingData_,
};

#[derive(uniffi::Object)]
pub struct Transaction {
    pub transaction: Transaction_,
}

#[uniffi::export]
impl Transaction {
    #[uniffi::constructor]
    pub fn new(
        type_: TransactionType,
        from: Option<String>,
        to: String,
        chain_id: u64,
        data: Vec<u8>,
        nonce: Option<u64>,
        signature: Option<TransactionSignature>,
    ) -> Transaction {
        Transaction {
            transaction: Transaction_::new(
                type_.into(),
                from.map(|from| Address::from(from.as_str())),
                Address::from(to.as_str()),
                chain_id,
                data,
                nonce,
                signature.map(TransactionSignature::into),
            ),
        }
    }

    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        self.transaction.get_signing_bytes().map_err(VdrError::from)
    }

    pub fn set_signature(&self, signature_data: SignatureData) {
        self.transaction.set_signature(signature_data.into())
    }
}

impl From<Transaction_> for Transaction {
    fn from(transaction: Transaction_) -> Self {
        Transaction { transaction }
    }
}

#[derive(uniffi::Object)]
pub struct TransactionEndorsingData {
    pub data: TransactionEndorsingData_,
}

#[uniffi::export]
impl TransactionEndorsingData {
    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        self.data.get_signing_bytes().map_err(VdrError::from)
    }
}

impl From<TransactionEndorsingData_> for TransactionEndorsingData {
    fn from(data: TransactionEndorsingData_) -> Self {
        TransactionEndorsingData { data }
    }
}
