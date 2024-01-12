use crate::{
    client::LedgerClient,
    contracts::auth::{HasRole, Role},
    error::VdrResult,
    types::{Address, Transaction, TransactionBuilder, TransactionParser, TransactionType},
};
use log::{debug, info};

const CONTRACT_NAME: &str = "RoleControl";
const METHOD_ASSIGN_ROLE: &str = "assignRole";
const METHOD_REVOKE_ROLE: &str = "revokeRole";
const METHOD_HAS_ROLE: &str = "hasRole";
const METHOD_GET_ROLE: &str = "getRole";

/// Build transaction to execute RoleControl.assignRole contract method to assign a role to an account
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `role` role to assign
/// - `account` assignee account
///
/// # Returns
/// Write transaction to sign and submit
pub async fn build_assign_role_transaction(
    client: &LedgerClient,
    from: &Address,
    role: &Role,
    account: &Address,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, assignee: {:?}, role: {:?}",
        METHOD_ASSIGN_ROLE, from, account, role
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_ASSIGN_ROLE)
        .add_param((*role).into())
        .add_param(account.try_into()?)
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_ASSIGN_ROLE, transaction
    );

    transaction
}

/// Build transaction to execute RoleControl.revokeRole contract method to revoke a role from an account
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `from` transaction sender account address
/// - `role` role to assign
/// - `account` revokee account
///
/// # Returns
/// Write transaction to sign and submit
pub async fn build_revoke_role_transaction(
    client: &LedgerClient,
    from: &Address,
    role: &Role,
    account: &Address,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Sender: {:?}, revokee: {:?}, role: {:?}",
        METHOD_REVOKE_ROLE, from, account, role
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_REVOKE_ROLE)
        .add_param((*role).into())
        .add_param(account.try_into()?)
        .set_type(TransactionType::Write)
        .set_from(from)
        .build(client)
        .await;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_REVOKE_ROLE, transaction
    );

    transaction
}

/// Build transaction to execute RoleControl.hasRole contract method to check an account has a role
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `role` role to check
/// - `account` account to check
///
/// # Returns
/// Read transaction to submit
pub async fn build_has_role_transaction(
    client: &LedgerClient,
    role: &Role,
    account: &Address,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Account to check: {:?}, role: {:?}",
        METHOD_HAS_ROLE, account, role
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_HAS_ROLE)
        .add_param((*role).into())
        .add_param(account.try_into()?)
        .set_type(TransactionType::Read)
        .build(client)
        .await;

    info!(
        "{} txn build has finished. Result {:?}",
        METHOD_HAS_ROLE, transaction
    );

    transaction
}

/// Build transaction to execute RoleControl.getRole contract method to get account's role
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `account` account address
///
/// # Returns
/// Read transaction to submit
pub async fn build_get_role_transaction(
    client: &LedgerClient,
    account: &Address,
) -> VdrResult<Transaction> {
    debug!(
        "{} txn build has started. Account to get: {:?}",
        METHOD_GET_ROLE, account,
    );

    let transaction = TransactionBuilder::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_GET_ROLE)
        .add_param(account.try_into()?)
        .set_type(TransactionType::Read)
        .build(client)
        .await;

    info!(
        "{} txn build has finished. Result: {:?}",
        METHOD_GET_ROLE, transaction
    );

    transaction
}

/// Parse the result of execution RoleControl.HasRole contract method to check an account has a role
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
/// Account has role result
pub fn parse_has_role_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<bool> {
    debug!(
        "{} result parse has started. Bytes to parse: {:?}",
        METHOD_HAS_ROLE, bytes
    );

    let parse_result = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_HAS_ROLE)
        .parse::<HasRole>(client, bytes);

    info!(
        "{} result parse has finished. Result: {:?}",
        METHOD_HAS_ROLE, parse_result
    );

    parse_result
}

/// Parse the result of execution RoleControl.GetRole contract method to get account's role
///
/// # Params
/// - `client` client connected to the network where contract will be executed
/// - `bytes` result bytes returned from the ledger
///
/// # Returns
/// Account's role
pub fn parse_get_role_result(client: &LedgerClient, bytes: &[u8]) -> VdrResult<Role> {
    debug!(
        "{} result parse has started. Bytes to parse: {:?}",
        METHOD_GET_ROLE, bytes
    );

    let parse_result = TransactionParser::new()
        .set_contract(CONTRACT_NAME)
        .set_method(METHOD_GET_ROLE)
        .parse::<Role>(client, bytes);

    info!(
        "{} result parse has finished. Result: {:?}",
        METHOD_GET_ROLE, parse_result
    );

    parse_result
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        client::client::test::{
            mock_client, CHAIN_ID, DEFAULT_NONCE, ROLE_CONTROL_ADDRESS, TRUSTEE_ACC,
        },
        utils::init_env_logger,
    };
    use std::sync::RwLock;

    pub const NEW_ACCOUNT: &str = "0x0886328869e4e1f401e1052a5f4aae8b45f42610";

    fn account() -> Address {
        Address::from(NEW_ACCOUNT)
    }

    mod build_assign_role_transaction {
        use super::*;

        #[async_std::test]
        async fn build_assign_role_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let expected_data = vec![
                136, 165, 191, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 50,
                136, 105, 228, 225, 244, 1, 225, 5, 42, 95, 74, 174, 139, 69, 244, 38, 16,
            ];

            let transaction =
                build_assign_role_transaction(&client, &TRUSTEE_ACC, &Role::Trustee, &account())
                    .await
                    .unwrap();

            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: ROLE_CONTROL_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: expected_data,
                signature: RwLock::new(None),
                hash: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_revoke_role_transaction {
        use super::*;

        #[async_std::test]
        async fn build_revoke_role_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let expected_data = vec![
                76, 187, 135, 211, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 50,
                136, 105, 228, 225, 244, 1, 225, 5, 42, 95, 74, 174, 139, 69, 244, 38, 16,
            ];

            let transaction =
                build_revoke_role_transaction(&client, &TRUSTEE_ACC, &Role::Trustee, &account())
                    .await
                    .unwrap();

            let expected_transaction = Transaction {
                type_: TransactionType::Write,
                from: Some(TRUSTEE_ACC.clone()),
                to: ROLE_CONTROL_ADDRESS.clone(),
                nonce: Some(DEFAULT_NONCE.clone()),
                chain_id: CHAIN_ID,
                data: expected_data,
                signature: RwLock::new(None),
                hash: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod build_get_role_transaction {
        use super::*;

        #[async_std::test]
        async fn build_get_role_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let expected_data = vec![
                68, 39, 103, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 50, 136, 105, 228,
                225, 244, 1, 225, 5, 42, 95, 74, 174, 139, 69, 244, 38, 16,
            ];

            let transaction = build_get_role_transaction(&client, &account())
                .await
                .unwrap();

            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: ROLE_CONTROL_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: expected_data,
                signature: RwLock::new(None),
                hash: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_get_role_result {
        use super::*;

        #[test]
        fn parse_get_role_result_test() {
            init_env_logger();
            let client = mock_client();
            let result = vec![0; 32];
            let expected_role = Role::Empty;

            let role = parse_get_role_result(&client, &result).unwrap();

            assert_eq!(expected_role, role);
        }
    }

    mod build_has_role_transaction {
        use super::*;

        #[async_std::test]
        async fn build_has_role_transaction_test() {
            init_env_logger();
            let client = mock_client();
            let expected_data = vec![
                158, 151, 184, 246, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 50,
                136, 105, 228, 225, 244, 1, 225, 5, 42, 95, 74, 174, 139, 69, 244, 38, 16,
            ];

            let transaction = build_has_role_transaction(&client, &Role::Trustee, &account())
                .await
                .unwrap();

            let expected_transaction = Transaction {
                type_: TransactionType::Read,
                from: None,
                to: ROLE_CONTROL_ADDRESS.clone(),
                nonce: None,
                chain_id: CHAIN_ID,
                data: expected_data,
                signature: RwLock::new(None),
                hash: None,
            };

            assert_eq!(expected_transaction, transaction);
        }
    }

    mod parse_has_role_result {
        use super::*;

        #[test]
        fn parse_has_role_result_test() {
            init_env_logger();
            let client = mock_client();
            let result = vec![0; 32];
            let expected_has_role = false;

            let has_role = parse_has_role_result(&client, &result).unwrap();

            assert_eq!(expected_has_role, has_role);
        }
    }
}
