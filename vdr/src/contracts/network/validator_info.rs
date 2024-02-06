use crate::{error::VdrError, types::ContractOutput, Address};

pub type ValidatorAddresses = Vec<Address>;

impl TryFrom<ContractOutput> for ValidatorAddresses {
    type Error = VdrError;

    fn try_from(value: ContractOutput) -> Result<Self, Self::Error> {
        value.get_address_array(0)
    }
}
