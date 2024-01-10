use crate::{
    client::Client,
    error::{VdrError, VdrResult},
    types::PingStatus,
    Address, Transaction,
};

use async_trait::async_trait;
use log::{trace, warn};
use serde_json::json;
use std::{str::FromStr, time::Duration};

#[cfg(not(feature = "wasm"))]
use web3::{
    api::Eth,
    transports::Http,
    types::{Address as EthAddress, Bytes, CallRequest, TransactionId, H256},
    Web3,
};

#[cfg(feature = "wasm")]
use web3_wasm::{
    api::Eth,
    transports::Http,
    types::{Address as EthAddress, Bytes, CallRequest, TransactionId, H256},
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

#[cfg_attr(not(feature = "wasm"), async_trait)]
#[cfg_attr(feature = "wasm", async_trait(? Send))]
impl Client for Web3Client {
    async fn get_transaction_count(&self, address: &crate::Address) -> VdrResult<[u64; 4]> {
        let account_address =
            EthAddress::from_str(address).map_err(|_| VdrError::ClientInvalidTransaction {
                msg: format!("Invalid transaction sender address {:?}", address),
            })?;

        let nonce = self
            .client
            .eth()
            .transaction_count(account_address, None)
            .await
            .unwrap();

        Ok(nonce.0)
    }

    async fn submit_transaction(&self, transaction: &[u8]) -> VdrResult<Vec<u8>> {
        trace!(
            "Submit transaction process has started. Transaction: {:?}",
            transaction
        );

        let receipt = self
            .client
            .send_raw_transaction_with_confirmation(
                Bytes::from(transaction),
                Duration::from_millis(POLL_INTERVAL),
                NUMBER_TX_CONFIRMATIONS,
            )
            .await?;

        trace!("Submitted transaction: {:?}", transaction);

        Ok(receipt.transaction_hash.0.to_vec())
    }

    async fn call_transaction(&self, to: &str, transaction: &[u8]) -> VdrResult<Vec<u8>> {
        trace!(
            "Call transaction process has started. Transaction: {:?}",
            transaction
        );

        let address = EthAddress::from_str(to).map_err(|_| {
            let vdr_error = VdrError::ClientInvalidTransaction {
                msg: format!("Invalid transaction target address {:?}", to),
            };

            warn!(
                "Error: {} during calling transaction: {:?}",
                vdr_error, transaction
            );

            vdr_error
        })?;
        let request = CallRequest::builder()
            .to(address)
            .data(Bytes(transaction.to_vec()))
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
                let vdr_error = VdrError::ClientInvalidResponse {
                    msg: "Missing transaction receipt".to_string(),
                };

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

    async fn get_transaction(&self, transaction_hash: &[u8]) -> VdrResult<Option<Transaction>> {
        let transaction_id = TransactionId::Hash(H256::from_slice(transaction_hash));
        let transaction = self
            .client
            .eth()
            .transaction(transaction_id)
            .await
            .map_err(|_| VdrError::GetTransactionError {
                msg: "Could not get transaction by hash".to_string(),
            })?;

        let transaction = transaction.map(|transaction| Transaction {
            type_: Default::default(),
            from: transaction
                .from
                .map(|from| Address::from(from.to_string().as_str())),
            to: transaction
                .to
                .map(|from| Address::from(from.to_string().as_str()))
                .unwrap_or_default(),
            nonce: Some(transaction.nonce.0.to_vec()),
            chain_id: 0,
            data: transaction.input.0.to_vec(),
            signature: Default::default(),
            hash: Some(transaction.hash.as_bytes().to_vec()),
        });
        Ok(transaction)
    }
}
