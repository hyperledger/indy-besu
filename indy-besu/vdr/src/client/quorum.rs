use std::sync::Arc;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use log::trace;
use web3::types::H256;

use crate::{Client, Transaction, VdrError, VdrResult};

use tokio::sync::mpsc;
use web3::types::Transaction as Web3Transaction;

pub struct QuorumConfig {
    pub quorum_needed: bool,
    max_retries_num: u8,
    max_quorum_time_sec: u8,
}

impl Default for QuorumConfig {
    fn default() -> Self {
        QuorumConfig {
            quorum_needed: true,
            max_retries_num: 5,
            max_quorum_time_sec: 200,
        }
    }
}

pub mod write_quorum {
    use super::*;

    async fn send_eth_get_transaction_with_retries(
        sender: Sender<Web3Transaction>,
        client: Arc<Box<dyn Client>>,
        transaction_hash: H256,
        max_retries_num: u8,
    ) {
        trace!(
            "Started eth_getTransaction task for transaction_hash: {:?}",
            transaction_hash
        );

        for _ in 1..=max_retries_num {
            if let Ok(Some(transaction)) = client.get_transaction(transaction_hash).await {
                if let Err(_) = sender.send(transaction).await {
                    trace!("Receiver is closed for sender: {:?}", sender);
                }
                break;
            } else {
                trace!(
                    "eth_getTransaction not succeed for transaction_hash: {:?}. retry",
                    transaction_hash
                );
            }
        }

        drop(sender);

        trace!(
            "Finished eth_getTransaction task for transaction_hash: {}",
            transaction_hash
        );
    }

    async fn wait_for_quorum(
        mut receiver: Receiver<Web3Transaction>,
        required_transaction_hash: H256,
        clients_num: u16,
        max_quorum_time_sec: u8,
    ) -> bool {
        let mut approvals_counter = 0;
        let approvals_needed = clients_num / 3 + 1;
        let mut quorum_reached = false;

        while let Some(transaction) = tokio::select! {
            transaction = receiver.recv() => transaction,
            _ = tokio::time::sleep(Duration::from_secs(max_quorum_time_sec.into())) => {
                trace!("Quorum timeout reached");
                None
            }
        } {
            if required_transaction_hash == transaction.hash {
                approvals_counter += 1;

                quorum_reached = approvals_counter >= approvals_needed;
                if quorum_reached {
                    break;
                }
            }
        }

        quorum_reached
    }

    pub async fn quorum_check(
        clients: &Vec<Arc<Box<dyn Client>>>,
        transaction_hash: H256,
        config: &QuorumConfig,
    ) -> VdrResult<bool> {
        trace!(
            "Started quorum check for transaction_hash: {}",
            transaction_hash
        );

        let clients_num = clients.len();
        let (sender, receiver) = mpsc::channel::<Web3Transaction>(clients_num);

        clients.into_iter().for_each(|client| {
            tokio::spawn(send_eth_get_transaction_with_retries(
                sender.clone(),
                client.clone(),
                transaction_hash,
                config.max_retries_num,
            ));
        });

        drop(sender);

        let quorum_reached = wait_for_quorum(
            receiver,
            transaction_hash,
            clients_num as u16,
            config.max_quorum_time_sec,
        )
        .await;

        if quorum_reached {
            trace!("Quorum succeed for transaction_hash: {}", transaction_hash);

            Ok(quorum_reached)
        } else {
            trace!("Quorum failed for transaction_hash: {}", transaction_hash);

            Err(VdrError::QuorumNotReached(format!(
                "Quorum not reached for transaction_hash: {}",
                transaction_hash,
            )))
        }
    }
}

pub mod read_quorum {
    use super::*;

    async fn call_eth_transaction_with_retries(
        sender: Sender<Vec<u8>>,
        client: Arc<Box<dyn Client>>,
        transaction: Transaction,
        max_retries_num: u8,
    ) {
        trace!("Started eth_call task for transaction: {:?}", transaction);

        for _ in 1..=max_retries_num {
            if let Ok(output) = client.call_transaction(&transaction).await {
                if let Err(_) = sender.send(output).await {
                    trace!("Receiver is closed for sender: {:?}", sender);
                }
                break;
            } else {
                trace!(
                    "eth_call not succeed for transaction: {:?}. retry",
                    transaction
                );
            }
        }

        drop(sender);

        trace!("Finished eth_call task for transaction: {:?}", transaction);
    }

    async fn wait_for_quorum(
        mut receiver: Receiver<Vec<u8>>,
        required_result: Vec<u8>,
        clients_num: u16,
        max_quorum_time_sec: u8,
    ) -> bool {
        let mut approvals_counter = 0;
        let approvals_needed = clients_num / 3 + 1;
        let mut quorum_reached = false;

        while let Some(call_result) = tokio::select! {
            call_result = receiver.recv() => call_result,
            _ = tokio::time::sleep(Duration::from_secs(max_quorum_time_sec.into())) => {
                trace!("Quorum timeout reached");
                None
            }
        } {
            if required_result == call_result {
                approvals_counter += 1;

                quorum_reached = approvals_counter >= approvals_needed;
                if quorum_reached {
                    break;
                }
            }
        }

        quorum_reached
    }

    pub async fn quorum_check(
        clients: &Vec<Arc<Box<dyn Client>>>,
        transaction: &Transaction,
        required_result: &Vec<u8>,
        config: &QuorumConfig,
    ) -> VdrResult<bool> {
        trace!("Started quorum check for transaction {:?}", transaction);

        let clients_num = clients.len();
        let (sender, receiver) = mpsc::channel::<Vec<u8>>(clients_num);

        clients.into_iter().for_each(|client| {
            tokio::spawn(call_eth_transaction_with_retries(
                sender.clone(),
                client.clone(),
                transaction.clone(),
                config.max_retries_num,
            ));
        });

        drop(sender);

        let quorum_reached = wait_for_quorum(
            receiver,
            required_result.clone(),
            clients_num as u16,
            config.max_quorum_time_sec,
        )
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

    use crate::{client::MockClient, utils::init_env_logger};

    use super::*;
    use mockall::predicate::eq;
    use web3::types::Transaction as Web3Transaction;

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
        let client1 = mock_client(H256::from([1; 32]));
        let client2 = mock_client(H256::from([1; 32]));
        let clients = vec![client1, client2];

        let result =
            write_quorum::quorum_check(&clients, H256::from([1; 32]), &QuorumConfig::default())
                .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_quorum_check_failed_with_timeout() {
        init_env_logger();
        let timeout_time = 1;
        let client1 = mock_client_custom_output(H256::from([1; 32]), None);
        let client2 = mock_client_sleep_before_return(H256::from([1; 32]), None, timeout_time + 3);
        let clients = vec![client1, client2];

        let result = write_quorum::quorum_check(
            &clients,
            H256::from([1; 32]),
            &QuorumConfig {
                max_quorum_time_sec: timeout_time,
                ..Default::default()
            },
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_not_reached() {
        init_env_logger();
        let client1 = mock_client(H256::from([1; 32]));
        let client2 = mock_client_custom_output(
            H256::from([1; 32]),
            Some(Web3Transaction {
                hash: H256::from([2; 32]),
                ..Default::default()
            }),
        );
        let client3 = mock_client_custom_output(
            H256::from([1; 32]),
            Some(Web3Transaction {
                hash: H256::from([3; 32]),
                ..Default::default()
            }),
        );
        let clients = vec![client1, client2, client3];

        let result =
            write_quorum::quorum_check(&clients, H256::from([1; 32]), &QuorumConfig::default())
                .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quorum_check_got_transaction_after_retries() {
        init_env_logger();
        let retries_num = 5;
        let client1 = mock_client_retries(H256::from([1; 32]), retries_num);
        let client2 = mock_client(H256::from([1; 32]));
        let clients = vec![client1, client2];

        let result = write_quorum::quorum_check(
            &clients,
            H256::from([1; 32]),
            &QuorumConfig {
                max_retries_num: retries_num as u8,
                ..Default::default()
            },
        )
        .await;

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

        let result = read_quorum::quorum_check(
            &clients,
            &transaction,
            &expected_output,
            &QuorumConfig::default(),
        )
        .await;

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

        let result = read_quorum::quorum_check(
            &clients,
            &transaction,
            &expected_output,
            &QuorumConfig {
                max_quorum_time_sec: timeout_time,
                ..Default::default()
            },
        )
        .await;

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

        let result = read_quorum::quorum_check(
            &clients,
            &transaction,
            &expected_output,
            &QuorumConfig::default(),
        )
        .await;

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

        let result = read_quorum::quorum_check(
            &clients,
            &transaction,
            &expected_output,
            &QuorumConfig {
                max_retries_num: retries_num as u8,
                ..Default::default()
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
