use crate::{
    error::VdrError,
    types::{ContractOutput, ContractParam},
};
use std::{fmt, ops::Deref};

use ethereum_types::Address as Address_;
use log::trace;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Address {
    value: String,
}

impl From<&str> for Address {
    fn from(address: &str) -> Self {
        if address.starts_with("0x") {
            Address {
                value: address.to_string(),
            }
        } else {
            Address {
                value: format!("0x{}", address),
            }
        }
    }
}

impl Deref for Address {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

struct IAddressVisitor;

impl<'de> Visitor<'de> for IAddressVisitor {
    type Value = Address;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string expected")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Address::from(v))
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IAddressVisitor)
    }
}

impl TryInto<ContractParam> for Address {
    type Error = VdrError;

    fn try_into(self) -> Result<ContractParam, Self::Error> {
        trace!("Address: {:?} convert into ContractParam has started", self);

        let acc_address = Address_::from_str(&self).map_err(|err| VdrError::CommonInvalidData {
            msg: format!(
                "Unable to parse account address. Err: {:?}",
                err.to_string()
            ),
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

        let acc_address = Address {
            value: value.get_string(0)?,
        };

        trace!(
            "Address convert from ContractOutput: {:?} has finished. Result: {:?}",
            value,
            acc_address
        );

        Ok(acc_address)
    }
}
