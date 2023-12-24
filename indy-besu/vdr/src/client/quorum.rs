use std::sync::Arc;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use log::trace;
use web3::types::H256;

use crate::{Client, Transaction, TransactionType, VdrError, VdrResult};

use tokio::sync::mpsc;

use super::implementation::web3::client::Web3Client;

#[derive(Clone)]
pub struct QuorumConfig {
    nodes: Vec<String>,
    request_retries: u8,
    request_timeout_sec: u8,
    retry_interval_sec: u8,
}

pub struct QuorumHandler {
    clients: Vec<Arc<Box<dyn Client>>>,
    config: QuorumConfig,
}

impl QuorumHandler {
    pub fn new(config: QuorumConfig) -> VdrResult<QuorumHandler> {
        let clients = config
            .nodes
            .clone()
            .into_iter()
            .map(|node_address| {
                let client: Box<dyn Client> = Box::new(Web3Client::new(&node_address)?);
                Ok(Arc::new(client))
            })
            .collect::<Result<Vec<_>, VdrError>>()
            .map_err(|_| VdrError::ClientNodeUnreachable)?;

        Ok(QuorumHandler { clients, config })
    }

    async fn send_transaction_with_retries(
        sender: Sender<Vec<u8>>,
        client: Arc<Box<dyn Client>>,
        transaction: Transaction,
        transaction_hash: Vec<u8>,
        quorum_config: QuorumConfig,
    ) {
        trace!("Started eth_call task for transaction: {:?}", transaction);

        for _ in 1..=quorum_config.request_retries {
            match transaction.type_ {
                TransactionType::Write => {
                    if let Ok(Some(transaction)) = client
                        .get_transaction(H256::from_slice(&transaction_hash))
                        .await
                    {
                        if let Err(_) = sender.send(transaction.hash.0.to_vec()).await {
                            trace!("Receiver is closed for sender: {:?}", sender);
                        }
                        break;
                    } else {
                        trace!(
                            "eth_getTransaction not succeed for transaction_hash: {:?}. retry",
                            transaction_hash
                        );

                        tokio::time::sleep(Duration::from_millis(
                            quorum_config.retry_interval_sec.into(),
                        ))
                        .await;
                    }
                }
                TransactionType::Read => {
                    if let Ok(result) = client.call_transaction(&transaction).await {
                        if let Err(_) = sender.send(result).await {
                            trace!("Receiver is closed for sender: {:?}", sender);
                        }
                        break;
                    } else {
                        trace!(
                            "call_transaction not succeed for transaction: {:?}. retry",
                            transaction
                        );
                    }
                }
            };
        }

        trace!("Finished eth_call task for transaction: {:?}", transaction);
    }

    async fn wait_for_quorum(
        &self,
        mut receiver: Receiver<Vec<u8>>,
        expected_result: Vec<u8>,
    ) -> bool {
        let mut approvals_counter = 0;
        let approvals_needed = self.clients.len() / 3 + 1;
        let mut quorum_reached = false;

        while let Some(result) = tokio::select! {
            transaction = receiver.recv() => transaction,
            _ = tokio::time::sleep(Duration::from_secs(self.config.request_timeout_sec.into())) => {
                trace!("Quorum timeout reached");
                None
            }
        } {
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
        expected_result: &Vec<u8>,
    ) -> VdrResult<bool> {
        trace!("Started quorum check for transaction: {:?}", transaction);

        let clients_num = self.clients.len();

        let (sender, receiver) = mpsc::channel::<Vec<u8>>(clients_num);

        self.clients.clone().into_iter().for_each(|client| {
            tokio::spawn(QuorumHandler::send_transaction_with_retries(
                sender.clone(),
                client,
                transaction.clone(),
                expected_result.clone(),
                self.config.clone(),
            ));
        });

        drop(sender);

        let quorum_reached = self
            .wait_for_quorum(receiver, expected_result.clone())
            .await;

        if quorum_reached {
            trace!("Quorum succeed for transaction: {:?}", transaction);

            Ok(quorum_reached)
        } else {
            trace!("Quorum failed for transaction: {:?}", transaction);

            Err(VdrError::QuorumNotReached(format!(
                "Quorum not reached for transaction: {:?}",
                transaction,
            )))
        }
    }
}

#[cfg(test)]
pub mod write_quorum_test {
    use std::{thread, time};

    use crate::{
        client::{test::CLIENT_NODE_ADDRESSES, MockClient},
        utils::init_env_logger,
    };

    use super::*;
    use mockall::predicate::eq;
    use web3::types::Transaction as Web3Transaction;

    impl Default for QuorumConfig {
        fn default() -> Self {
            QuorumConfig {
                nodes: CLIENT_NODE_ADDRESSES
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                request_retries: 5,
                request_timeout_sec: 200,
                retry_interval_sec: 1,
            }
        }
    }

    fn mock_client(txn_hash: H256) -> Arc<Box<dyn Client>> {
        let expected_output = Some(Web3Transaction {
            hash: txn_hash,
            ..Default::default()
        });

        let mut mock_client = MockClient::new();
        mock_client
            .expect_get_transaction()
            .with(eq(txn_hash))
            .returning(move |_| Ok(expected_output.clone()));

        Arc::new(Box::new(mock_client))
    }

    fn mock_client_custom_output(
        txn_hash: H256,
        expected_output: Option<Web3Transaction>,
    ) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();
        mock_client
            .expect_get_transaction()
            .with(eq(txn_hash))
            .returning(move |_| Ok(expected_output.clone()));

        Arc::new(Box::new(mock_client))
    }

    fn mock_client_sleep_before_return(
        txn_hash: H256,
        expected_output: Option<Web3Transaction>,
        sleep_time_sec: u8,
    ) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();
        mock_client
            .expect_get_transaction()
            .with(eq(txn_hash))
            .returning(move |_| {
                thread::sleep(time::Duration::from_secs(sleep_time_sec.into()));
                Ok(expected_output.clone())
            });

        Arc::new(Box::new(mock_client))
    }

    fn mock_client_retries(txn_hash: H256, retries_num: usize) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();
        let expected_output = Some(Web3Transaction {
            hash: txn_hash,
            ..Default::default()
        });

        mock_client
            .expect_get_transaction()
            .with(eq(txn_hash))
            .times(retries_num - 1)
            .returning(move |_| Ok(None));

        mock_client
            .expect_get_transaction()
            .with(eq(txn_hash))
            .returning(move |_| Ok(expected_output.clone()));

        Arc::new(Box::new(mock_client))
    }

    #[tokio::test]
    async fn test_quorum_check_positive_case() {
        init_env_logger();
        let txn_hash = vec![1; 32];
        let transaction = Transaction {
            type_: TransactionType::Write,
            ..Transaction::default()
        };
        let client1 = mock_client(H256::from_slice(&txn_hash));
        let client2 = mock_client(H256::from_slice(&txn_hash));
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig::default(),
        };

        let result = quorum.check(&transaction, &txn_hash).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_quorum_check_failed_with_timeout() {
        init_env_logger();
        let timeout_time = 1;
        let transaction = Transaction {
            type_: TransactionType::Write,
            ..Transaction::default()
        };
        let txn_hash = vec![1; 32];
        let client1 = mock_client_custom_output(H256::from_slice(&txn_hash), None);
        let client2 =
            mock_client_sleep_before_return(H256::from_slice(&txn_hash), None, timeout_time + 3);
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig {
                request_timeout_sec: timeout_time,
                ..Default::default()
            },
        };

        let result = quorum.check(&transaction, &txn_hash).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_not_reached() {
        init_env_logger();
        let transaction = Transaction {
            type_: TransactionType::Write,
            ..Transaction::default()
        };
        let txn_hash = vec![1; 32];
        let client1: Arc<Box<dyn Client>> = mock_client(H256::from_slice(&txn_hash));
        let client2 = mock_client_custom_output(
            H256::from_slice(&txn_hash),
            Some(Web3Transaction {
                hash: H256::from([2; 32]),
                ..Default::default()
            }),
        );
        let client3 = mock_client_custom_output(
            H256::from_slice(&txn_hash),
            Some(Web3Transaction {
                hash: H256::from([3; 32]),
                ..Default::default()
            }),
        );
        let clients = vec![client1, client2, client3];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig::default(),
        };

        let result = quorum.check(&transaction, &txn_hash).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_got_transaction_after_retries() {
        init_env_logger();
        let retries_num = 5;
        let transaction = Transaction {
            type_: TransactionType::Write,
            ..Transaction::default()
        };
        let txn_hash = vec![1; 32];
        let client1 = mock_client_retries(H256::from_slice(&txn_hash), retries_num);
        let client2 = mock_client(H256::from_slice(&txn_hash));
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig {
                request_retries: retries_num as u8,
                ..Default::default()
            },
        };

        let result = quorum.check(&transaction, &txn_hash).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}

#[cfg(test)]
pub mod read_quorum_test {
    use std::{thread, time};

    use crate::{client::MockClient, utils::init_env_logger};

    use super::*;
    use mockall::predicate::eq;

    fn mock_client_custom_output(
        transaction: Transaction,
        expected_output: VdrResult<Vec<u8>>,
    ) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();
        mock_client
            .expect_call_transaction()
            .with(eq(transaction))
            .returning(move |_| expected_output.clone());

        Arc::new(Box::new(mock_client))
    }

    fn mock_client_sleep_before_return(
        transaction: Transaction,
        expected_output: VdrResult<Vec<u8>>,
        sleep_time_sec: u8,
    ) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();
        mock_client
            .expect_call_transaction()
            .with(eq(transaction))
            .returning(move |_| {
                thread::sleep(time::Duration::from_secs(sleep_time_sec.into()));
                expected_output.clone()
            });

        Arc::new(Box::new(mock_client))
    }

    fn mock_client_retries(
        transaction: Transaction,
        expected_output: VdrResult<Vec<u8>>,
        retries_num: usize,
    ) -> Arc<Box<dyn Client>> {
        let mut mock_client = MockClient::new();

        mock_client
            .expect_call_transaction()
            .with(eq(transaction.clone()))
            .times(retries_num - 1)
            .returning(move |_| Err(VdrError::ContractInvalidResponseData("".to_string())));

        mock_client
            .expect_call_transaction()
            .with(eq(transaction))
            .returning(move |_| expected_output.clone());

        Arc::new(Box::new(mock_client))
    }

    #[tokio::test]
    async fn test_quorum_check_positive_case() {
        init_env_logger();
        let transaction = Transaction::default();
        let expected_output = vec![1, 1, 1, 1];
        let client1 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
        let client2 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig::default(),
        };

        let result = quorum.check(&transaction, &expected_output).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_quorum_check_failed_with_timeout() {
        init_env_logger();
        let timeout_time = 1;
        let err = Err(VdrError::ClientTransactionReverted(
            "Transaction reverted".to_string(),
        ));
        let transaction = Transaction::default();
        let expected_output = vec![1, 1, 1, 1];
        let client1 = mock_client_custom_output(transaction.clone(), err.clone());
        let client2 = mock_client_sleep_before_return(transaction.clone(), err, timeout_time + 3);
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig {
                request_timeout_sec: timeout_time,
                ..Default::default()
            },
        };

        let result = quorum.check(&transaction, &expected_output).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_not_reached() {
        init_env_logger();
        let transaction = Transaction::default();
        let expected_output = vec![1, 1, 1, 1];
        let client1 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
        let client2 = mock_client_custom_output(transaction.clone(), Ok(vec![1, 1, 1, 2]));
        let client3 = mock_client_custom_output(transaction.clone(), Ok(vec![1, 1, 1, 3]));
        let clients = vec![client1, client2, client3];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig::default(),
        };

        let result = quorum.check(&transaction, &expected_output).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_got_transaction_after_retries() {
        init_env_logger();
        let retries_num = 5;
        let transaction = Transaction::default();
        let expected_output = vec![1, 1, 1, 1];
        let client1 = mock_client_retries(
            transaction.clone(),
            Ok(expected_output.clone()),
            retries_num,
        );
        let client2 = mock_client_custom_output(transaction.clone(), Ok(expected_output.clone()));
        let clients = vec![client1, client2];
        let quorum = QuorumHandler {
            clients,
            config: QuorumConfig {
                request_retries: retries_num as u8,
                ..Default::default()
            },
        };

        let result = quorum.check(&transaction, &expected_output).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
