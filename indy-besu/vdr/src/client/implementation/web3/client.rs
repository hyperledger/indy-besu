use crate::{
    client::{constants::GAS, Client},
    error::{VdrError, VdrResult},
    types::{PingStatus, Transaction, TransactionType},
};

use ethereum::{EnvelopedEncodable, LegacyTransaction, TransactionAction};
use log::{trace, warn};
use serde_json::json;
use std::{str::FromStr, time::Duration};
use web3::{
    api::Eth,
    transports::Http,
    types::{Address, Bytes, CallRequest, H256, U256},
    Web3,
};

pub struct Web3Client {
    client: Web3<Http>,
}

const POLL_INTERVAL: u64 = 200;
const NUMBER_TX_CONFIRMATIONS: usize = 1; // FIXME: what number of confirmation events should we wait? 2n+1?

impl Web3Client {
    pub fn new(node_address: &str) -> VdrResult<Web3Client> {
        trace!(
            "Started creating new Web3Client. Node address: {}",
            node_address
        );

        let transport = Http::new(node_address).map_err(|_| VdrError::ClientNodeUnreachable)?;
        let web3 = Web3::new(transport);
        let web3_client = Web3Client { client: web3 };

        trace!("Created new Web3Client. Node address: {}", node_address);

        Ok(web3_client)
    }

    pub fn eth(&self) -> Eth<Http> {
        self.client.eth()
    }
}

#[async_trait::async_trait]
impl Client for Web3Client {
    async fn get_transaction_count(&self, address: &crate::Address) -> VdrResult<[u64; 4]> {
        let account_address = Address::from_str(address.value()).map_err(|_| {
            VdrError::ClientInvalidTransaction(format!(
                "Invalid transaction sender address {}",
                address.value()
            ))
        })?;

        let nonce = self
            .client
            .eth()
            .transaction_count(account_address, None)
            .await
            .unwrap();

        Ok(nonce.0)
    }

    async fn submit_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        trace!(
            "Submit transaction process has started. Transaction: {:?}",
            transaction
        );

        if transaction.type_ != TransactionType::Write {
            let vdr_error =
                VdrError::ClientInvalidTransaction("Write transaction expected".to_string());

            warn!(
                "Error: {} during submitting transaction: {:?}",
                vdr_error, transaction
            );

            return Err(vdr_error);
        }

        let to = Address::from_str(&transaction.to).map_err(|_| {
            VdrError::ClientInvalidTransaction(format!(
                "Invalid transaction target address {}",
                transaction.to
            ))
        })?;

        let signature = transaction
            .signature
            .as_ref()
            .ok_or_else(|| VdrError::ClientInvalidTransaction("Missing signature".to_string()))?;

        let nonce = transaction.nonce.ok_or_else(|| {
            VdrError::ClientInvalidTransaction("Transaction `nonce` is not set".to_string())
        })?;

        let eth_transaction = LegacyTransaction {
            nonce: U256(nonce),
            gas_price: U256([0, 0, 0, 0]),
            gas_limit: U256([GAS, 0, 0, 0]),
            action: TransactionAction::Call(to),
            value: Default::default(),
            input: transaction.data.clone(),
            signature: signature.clone(),
        };

        let receipt = self
            .client
            .send_raw_transaction_with_confirmation(
                Bytes::from(eth_transaction.encode()),
                Duration::from_millis(POLL_INTERVAL),
                NUMBER_TX_CONFIRMATIONS,
            )
            .await?;

        trace!("Submitted transaction: {:?}", transaction);

        Ok(receipt.transaction_hash.0.to_vec())
    }

    async fn call_transaction(&self, transaction: &Transaction) -> VdrResult<Vec<u8>> {
        trace!(
            "Call transaction process has started. Transaction: {:?}",
            transaction
        );

        if transaction.type_ != TransactionType::Read {
            let vdr_error =
                VdrError::ClientInvalidTransaction("Read transaction expected".to_string());

            warn!(
                "Error: {} during calling transaction: {:?}",
                vdr_error, transaction
            );

            return Err(vdr_error);
        }
        let address = Address::from_str(&transaction.to).map_err(|_| {
            let vdr_error = VdrError::ClientInvalidTransaction(format!(
                "Invalid transaction target address {}",
                transaction.to
            ));

            warn!(
                "Error: {} during calling transaction: {:?}",
                vdr_error, transaction
            );

            vdr_error
        })?;
        let request = CallRequest::builder()
            .to(address)
            .data(Bytes(transaction.data.clone()))
            .build();
        let response = self.client.eth().call(request, None).await?;

        trace!("Called transaction: {:?}", transaction);

        Ok(response.0.to_vec())
    }

    async fn get_receipt(&self, hash: &[u8]) -> VdrResult<String> {
        let receipt = self
            .client
            .eth()
            .transaction_receipt(H256::from_slice(hash))
            .await?
            .ok_or_else(|| {
                let vdr_error =
                    VdrError::ClientInvalidResponse("Missing transaction receipt".to_string());

                warn!("Error: {} getting receipt", vdr_error,);

                vdr_error
            })
            .map(|receipt| json!(receipt).to_string());

        trace!("Got receipt: {:?}", receipt);

        receipt
    }

    async fn ping(&self) -> VdrResult<PingStatus> {
        let ping_result = match self.client.eth().block_number().await {
            Ok(_current_block) => Ok(PingStatus::ok()),
            Err(_) => Ok(PingStatus::err("Could not get current network block")),
        };

        trace!("Ping result: {:?}", ping_result);

        ping_result
    }
}
