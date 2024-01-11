mod common;

#[cfg(test)]
pub use common::{init_env_logger, rand_bytes, rand_string};

use crate::{ContractParam, VdrResult};

pub fn encode_contract_param<T: Into<ContractParam>>(obj: T) -> VdrResult<Vec<u8>> {
    let contract_param: ContractParam = obj.into();
    let encoded = ethabi::encode(&[contract_param]);
    Ok(encoded)
}
