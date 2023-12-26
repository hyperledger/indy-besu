use std::{ops::Deref, sync::Arc, time::Duration};

use futures::{
    channel::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    StreamExt,
};

use log::trace;

use crate::{
    client::implementation::web3::client::Web3Client, Client, Transaction, TransactionType,
    VdrError, VdrResult,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct QuorumConfig {
    pub nodes: Vec<String>,
    pub request_retries: Option<u8>,
    pub request_timeout: Option<u64>,
    pub retry_interval: Option<u64>,
}

const DEFAULT_REQUEST_RETRIES: u8 = 4;
const DEFAULT_REQUEST_TIMEOUT: u64 = 2000;
const DEFAULT_RETRY_INTERVAL: u64 = 500;

pub struct QuorumHandler {
    clients: Vec<Arc<Box<dyn Client>>>,
    request_retries: u8,
    request_timeout: Duration,
    retry_interval: Duration,
}

impl QuorumHandler {
    pub fn new(config: QuorumConfig) -> VdrResult<QuorumHandler> {
        let clients = config
            .nodes
            .iter()
            .map(|node_address| {
                let client: Box<dyn Client> = Box::new(Web3Client::new(node_address)?);
                Ok(Arc::new(client))
            })
            .collect::<Result<Vec<_>, VdrError>>()?;

        Ok(QuorumHandler {
            clients,
            request_retries: config.request_retries.unwrap_or(DEFAULT_REQUEST_RETRIES),
            request_timeout: Duration::from_millis(
                config.request_timeout.unwrap_or(DEFAULT_REQUEST_TIMEOUT),
            ),
            retry_interval: Duration::from_millis(
                config.retry_interval.unwrap_or(DEFAULT_RETRY_INTERVAL),
            ),
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_transaction_with_retries(
        mut sender: Sender<Vec<u8>>,
        client: Arc<Box<dyn Client>>,
        type_: TransactionType,
        to: String,
        data: Vec<u8>,
        request_retries: u8,
        request_timeout: Duration,
        retry_interval: Duration,
    ) {
        trace!("Started eth_call task for transaction: {:?}", data);

        for _ in 1..request_retries {
            match type_ {
                TransactionType::Write => {
                    let future = client.get_transaction(&data);
                    match async_std::future::timeout(request_timeout, future).await {
                        Ok(Ok(Some(transaction))) if transaction.hash.is_some() => {
                            let hash = transaction.hash.unwrap();
                            if sender.try_send(hash).is_err() {
                                trace!("Receiver is closed for sender: {:?}", sender);
                            }
                            break;
                        }
                        _ => {
                            trace!(
                                "eth_getTransaction not succeed for transaction_hash: {:?}. retry",
                                data
                            );
                            async_std::task::sleep(retry_interval).await;
                        }
                    }
                }
                TransactionType::Read => {
                    let future = client.call_transaction(&to, &data);
                    match async_std::future::timeout(request_timeout, future).await {
                        Ok(Ok(transaction)) => {
                            if sender.try_send(transaction).is_err() {
                                trace!("Receiver is closed for sender: {:?}", sender);
                            }
                            break;
                        }
                        _ => {
                            trace!(
                                "call_transaction not succeed for transaction_hash: {:?}. retry",
                                data
                            );
                            async_std::task::sleep(retry_interval).await;
                        }
                    }
                }
            };
        }

        trace!("Finished eth_call task for transaction: {:?}", data);
    }

    async fn wait_for_quorum(
        &self,
        mut receiver: Receiver<Vec<u8>>,
        expected_result: &[u8],
    ) -> bool {
        let approvals_needed = self.clients.len() / 3 + 1;
        let mut approvals_counter = 0;
        let mut quorum_reached = false;

        while let Some(result) = receiver.next().await {
            if result == expected_result {
                approvals_counter += 1;

                quorum_reached = approvals_counter >= approvals_needed;
                if quorum_reached {
                    break;
                }
            }
        }

        quorum_reached
    }

    pub async fn check(
        &self,
        transaction: &Transaction,
        expected_result: &[u8],
    ) -> VdrResult<bool> {
        trace!("Started quorum check for transaction: {:?}", transaction);

        let clients_count = self.clients.len();
        let (sender, receiver) = mpsc::channel::<Vec<u8>>(clients_count);

        for client in self.clients.iter() {
            let type_ = transaction.type_.clone();
            let to = transaction.to.clone();
            let transaction_data = match transaction.type_ {
                TransactionType::Write => expected_result.to_vec(),
                TransactionType::Read => transaction.data.to_vec(),
            };

            #[cfg(feature = "wasm")]
            {
                let sender = sender.clone();
                let client = client.clone();
                let request_retries = self.request_retries;
                let request_timeout = self.request_timeout;
                let retry_interval = self.retry_interval;
                async_std::task::block_on(QuorumHandler::send_transaction_with_retries(
                    sender,
                    client,
                    type_,
                    to.deref().to_string(),
                    transaction_data,
                    request_retries,
                    request_timeout,
                    retry_interval,
                ));
            }

            #[cfg(not(feature = "wasm"))]
            {
                async_std::task::spawn(QuorumHandler::send_transaction_with_retries(
                    sender.clone(),
                    client.clone(),
                    type_,
                    to.deref().to_string(),
                    transaction_data,
                    self.request_retries,
                    self.request_timeout,
                    self.retry_interval,
                ));
            }
        }

        let quorum_reached = self.wait_for_quorum(receiver, expected_result).await;
        if quorum_reached {
            trace!("Quorum succeed for transaction: {:?}", transaction);
            Ok(quorum_reached)
        } else {
            trace!("Quorum failed for transaction: {:?}", transaction);
            Err(VdrError::QuorumNotReached {
                msg: format!("Quorum not reached for transaction: {:?}", transaction),
            })
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::client::client::test::CLIENT_NODE_ADDRESSES;

    //
    //     use std::{thread, time};
    //     use web3::types::H256;
    //     use mockall::predicate::eq;
    //     use web3::types::Transaction as Web3Transaction;
    //
    //     use crate::client::client::test::{
    //         CLIENT_NODE_ADDRESSES, MockClient,
    //     };
    //     use crate::utils::init_env_logger;
    //
    //     mod write_quorum_test {
    //         use super::*;
    //
    impl Default for QuorumConfig {
        fn default() -> Self {
            QuorumConfig {
                nodes: CLIENT_NODE_ADDRESSES
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                request_retries: Some(DEFAULT_REQUEST_RETRIES),
                request_timeout: Some(DEFAULT_REQUEST_TIMEOUT),
                retry_interval: Some(DEFAULT_RETRY_INTERVAL),
            }
        }
    }
    //
    //         fn mock_client(txn_hash: &[u8]) -> Arc<Box<dyn Client>> {
    //             let expected_output = Some(Transaction {
    //                 hash: Some(txn_hash.to_vec()),
    //                 ..Default::default()
    //             });
    //
    //             let mut mock_client = MockClient::new();
    //             mock_client
    //                 .expect_get_transaction()
    //                 .with(eq(txn_hash))
    //                 .returning(move |_| Ok(expected_output));
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         fn mock_client_custom_output(
    //             txn_hash: H256,
    //             expected_output: Option<Web3Transaction>,
    //         ) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //             mock_client
    //                 .expect_get_transaction()
    //                 .with(eq(txn_hash))
    //                 .returning(move |_| Ok(expected_output.clone()));
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         fn mock_client_sleep_before_return(
    //             txn_hash: H256,
    //             expected_output: Option<Web3Transaction>,
    //             sleep_time_sec: u64,
    //         ) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //             mock_client
    //                 .expect_get_transaction()
    //                 .with(eq(txn_hash))
    //                 .returning(move |_| {
    //                     thread::sleep(time::Duration::from_millis(sleep_time_sec.into()));
    //                     Ok(expected_output.clone())
    //                 });
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         fn mock_client_retries(txn_hash: H256, retries_num: usize) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //             let expected_output = Some(Web3Transaction {
    //                 hash: txn_hash,
    //                 ..Default::default()
    //             });
    //
    //             mock_client
    //                 .expect_get_transaction()
    //                 .with(eq(txn_hash))
    //                 .times(retries_num - 1)
    //                 .returning(move |_| Ok(None));
    //
    //             mock_client
    //                 .expect_get_transaction()
    //                 .with(eq(txn_hash))
    //                 .returning(move |_| Ok(expected_output.clone()));
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_positive_case() {
    //             init_env_logger();
    //             let txn_hash = vec![1; 32];
    //             let transaction = Transaction {
    //                 type_: TransactionType::Write,
    //                 ..Transaction::default()
    //             };
    //             let client1 = mock_client(&txn_hash);
    //             let client2 = mock_client(&txn_hash);
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &txn_hash).await;
    //
    //             assert!(result.is_ok());
    //             assert_eq!(result.unwrap(), true);
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_failed_with_timeout() {
    //             init_env_logger();
    //             let timeout_time: u64 = 1000;
    //             let transaction = Transaction {
    //                 type_: TransactionType::Write,
    //                 ..Transaction::default()
    //             };
    //             let txn_hash = vec![1; 32];
    //             let client1 = mock_client_custom_output(H256::from_slice(&txn_hash), None);
    //             let client2 =
    //                 mock_client_sleep_before_return(H256::from_slice(&txn_hash), None, timeout_time + 3000);
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(timeout_time),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &txn_hash).await;
    //
    //             assert!(result.is_err());
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_not_reached() {
    //             init_env_logger();
    //             let transaction = Transaction {
    //                 type_: TransactionType::Write,
    //                 ..Transaction::default()
    //             };
    //             let txn_hash = vec![1; 32];
    //             let client1: Arc<Box<dyn Client>> = mock_client(&txn_hash);
    //             let client2 = mock_client_custom_output(
    //                 H256::from_slice(&txn_hash),
    //                 Some(Web3Transaction {
    //                     hash: H256::from([2; 32]),
    //                     ..Default::default()
    //                 }),
    //             );
    //             let client3 = mock_client_custom_output(
    //                 H256::from_slice(&txn_hash),
    //                 Some(Web3Transaction {
    //                     hash: H256::from([3; 32]),
    //                     ..Default::default()
    //                 }),
    //             );
    //             let clients = vec![client1, client2, client3];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &txn_hash).await;
    //
    //             assert!(result.is_err());
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_got_transaction_after_retries() {
    //             init_env_logger();
    //             let retries_num = 5;
    //             let transaction = Transaction {
    //                 type_: TransactionType::Write,
    //                 ..Transaction::default()
    //             };
    //             let txn_hash = vec![1; 32];
    //             let client1 = mock_client_retries(H256::from_slice(&txn_hash), retries_num);
    //             let client2 = mock_client(&txn_hash);
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: retries_num as u8,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &txn_hash).await;
    //
    //             assert!(result.is_ok());
    //             assert_eq!(result.unwrap(), true);
    //         }
    //     }
    //
    //     mod read_quorum_test {
    //         use super::*;
    //
    //         fn mock_client_custom_output(
    //             transaction: Transaction,
    //             expected_output: VdrResult<Vec<u8>>,
    //         ) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //             mock_client
    //                 .expect_call_transaction()
    //                 .with(eq(transaction))
    //                 .returning(move |_| expected_output.clone());
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         fn mock_client_sleep_before_return(
    //             transaction: Transaction,
    //             expected_output: VdrResult<Vec<u8>>,
    //             sleep_time_sec: u8,
    //         ) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //             mock_client
    //                 .expect_call_transaction()
    //                 .with(eq(transaction))
    //                 .returning(move |_| {
    //                     thread::sleep(time::Duration::from_secs(sleep_time_sec.into()));
    //                     expected_output.clone()
    //                 });
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         fn mock_client_retries(
    //             transaction: Transaction,
    //             expected_output: VdrResult<Vec<u8>>,
    //             retries_num: usize,
    //         ) -> Arc<Box<dyn Client>> {
    //             let mut mock_client = MockClient::new();
    //
    //             mock_client
    //                 .expect_call_transaction()
    //                 .with(eq(transaction))
    //                 .times(retries_num - 1)
    //                 .returning(move |_| Err(VdrError::ContractInvalidResponseData {
    //                     msg: "".to_string()
    //                 }));
    //
    //             mock_client
    //                 .expect_call_transaction()
    //                 .with(eq(transaction))
    //                 .returning(move |_| expected_output.clone());
    //
    //             Arc::new(Box::new(mock_client))
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_positive_case() {
    //             init_env_logger();
    //             let transaction = Transaction::default();
    //             let expected_output = vec![1, 1, 1, 1];
    //             let client1 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
    //             let client2 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &expected_output).await;
    //
    //             assert!(result.is_ok());
    //             assert_eq!(result.unwrap(), true);
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_failed_with_timeout() {
    //             init_env_logger();
    //             let timeout_time: u64 = 1000;
    //             let err = Err(VdrError::ClientTransactionReverted {
    //                 msg: "Transaction reverted".to_string()
    //             });
    //             let transaction = Transaction::default();
    //             let expected_output = vec![1, 1, 1, 1];
    //             let client1 = mock_client_custom_output(transaction.clone(), err.clone());
    //             let client2 = mock_client_sleep_before_return(transaction.clone(), err, 4);
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(timeout_time),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &expected_output).await;
    //
    //             assert!(result.is_err());
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_not_reached() {
    //             init_env_logger();
    //             let transaction = Transaction::default();
    //             let expected_output = vec![1, 1, 1, 1];
    //             let client1 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
    //             let client2 = mock_client_custom_output(transaction.clone(), Ok(vec![1, 1, 1, 2]));
    //             let client3 = mock_client_custom_output(transaction.clone(), Ok(vec![1, 1, 1, 3]));
    //             let clients = vec![client1, client2, client3];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: DEFAULT_REQUEST_RETRIES,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //             };
    //
    //             let result = quorum.check(&transaction, &expected_output).await;
    //
    //             assert!(result.is_err());
    //         }
    //
    //         #[tokio::test]
    //         async fn test_quorum_check_got_transaction_after_retries() {
    //             init_env_logger();
    //             let retries_num = 5;
    //             let transaction = Transaction::default();
    //             let expected_output = vec![1, 1, 1, 1];
    //             let client1 = mock_client_retries(
    //                 transaction.clone(),
    //                 Ok(expected_output.clone()),
    //                 retries_num,
    //             );
    //             let client2 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
    //             let clients = vec![client1, client2];
    //             let quorum = QuorumHandler {
    //                 clients,
    //                 request_retries: retries_num as u8,
    //                 request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT),
    //                 retry_interval: Duration::from_millis(DEFAULT_RETRY_INTERVAL),
    //
    //             };
    //
    //             let result = quorum.check(&transaction, &expected_output).await;
    //
    //             assert!(result.is_ok());
    //             assert_eq!(result.unwrap(), true);
    //         }
    //     }
}
