use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
};

use ethereum_types::Address as Address_;
use log::trace;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address(String);

const PREFIX: &str = "0x";

impl From<&str> for Address {
    fn from(address: &str) -> Self {
        if address.starts_with(PREFIX) {
            Address(address.to_string())
        } else {
            Address(format!("{}{}", PREFIX, address))
        }
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryInto<ContractParam> for &Address {
    type Error = VdrError;

    fn try_into(self) -> Result<ContractParam, Self::Error> {
        trace!("Address: {:?} convert into ContractParam has started", self);

        let acc_address = Address_::from_str(self.as_ref()).map_err(|err| {
            VdrError::CommonInvalidData(format!(
                "Unable to parse account address. Err: {:?}",
                err.to_string()
            ))
        })?;

        let acc_address_contract_param = ContractParam::Address(acc_address);

        trace!(
            "Address: {:?} convert into ContractParam has finished. Result: {:?}",
            self,
            acc_address_contract_param
        );

        Ok(ContractParam::Address(acc_address))
    }
}

impl TryFrom<ContractOutput> for Address {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        trace!(
            "Address convert from ContractOutput: {:?} has started",
            value
        );

        let acc_address = Address(value.get_string(0)?);

        trace!(
            "Address convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            acc_address
        );

        Ok(acc_address)
    }
}
