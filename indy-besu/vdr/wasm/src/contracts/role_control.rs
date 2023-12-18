use indy2_vdr::{RoleControl, Address, Role};
use wasm_bindgen::prelude::*;

use crate::transaction::TransactionWrapper;
use crate::client::LedgerClientWrapper;
use crate::error::{JsResult, Result};

#[wasm_bindgen(js_name = RoleControl)]
pub struct RoleControlWrapper(pub(crate) RoleControl);

#[wasm_bindgen(js_class = RoleControl)]
impl RoleControlWrapper {
    #[wasm_bindgen(js_name = buildAssignRoleTransaction)]
    pub async fn build_assign_role_transaction(client: &LedgerClientWrapper,
                                               from: &str,
                                               role: u8,
                                               account: &str) -> Result<TransactionWrapper> {
        let role = Role::try_from(role).as_js()?;
        let from = Address::new(from);
        let account = Address::new(account);
        let transaction = RoleControl::build_assign_role_transaction(&client.0, &from, &role, &account).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = buildRevokeRoleTransaction)]
    pub async fn build_revoke_role_transaction(client: &LedgerClientWrapper,
                                               from: &str,
                                               role: u8,
                                               account: &str) -> Result<TransactionWrapper> {
        let role = Role::try_from(role).as_js()?;
        let from = Address::new(from);
        let account = Address::new(account);
        let transaction = RoleControl::build_revoke_role_transaction(&client.0, &from, &role, &account).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = buildHasRoleTransaction)]
    pub async fn build_has_role_transaction(client: &LedgerClientWrapper,
                                            role: u8,
                                            account: &str) -> Result<TransactionWrapper> {
        let role = Role::try_from(role).as_js()?;
        let account = Address::new(account);
        let transaction = RoleControl::build_has_role_transaction(&client.0, &role, &account).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = buildGetRoleTransaction)]
    pub async fn build_get_role_transaction(client: &LedgerClientWrapper,
                                            account: &str) -> Result<TransactionWrapper> {
        let account = Address::new(account);
        let transaction = RoleControl::build_get_role_transaction(&client.0, &account).await.as_js()?;
        Ok(TransactionWrapper(transaction))
    }

    #[wasm_bindgen(js_name = parseHasRoleResult)]
    pub fn parse_has_role_result(client: &LedgerClientWrapper,
                                 bytes: Vec<u8>) -> Result<bool> {
        let has_role = RoleControl::parse_has_role_result(&client.0, &bytes).as_js()?;
        Ok(has_role)
    }

    #[wasm_bindgen(js_name = parseGetRoleResult)]
    pub fn parse_get_role_result(client: &LedgerClientWrapper,
                                 bytes: Vec<u8>) -> Result<u8> {
        let role = RoleControl::parse_get_role_result(&client.0, &bytes).as_js()?;
        Ok(role.into())
    }
}