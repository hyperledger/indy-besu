use serde::{Deserialize, Serialize};

use crate::{ContractEvent, VdrError};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RevocationRegistryEvents {
    RevocationRegistryEntryCreatedEvent(RevRegEntryCreated),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevRegEntryCreated {
    pub revocation_registry_definition_id: Vec<u8>,
    pub timestamp: u64,
    pub rev_reg_entry: RevRegEntry,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevRegEntry {
    pub current_accumulator: String,
    pub prev_accumulator: String,
    pub issued: Vec<u32>,
    pub revoked: Vec<u32>,
    pub timestamp: u64,
}

impl TryFrom<ContractEvent> for RevRegEntryCreated {
    type Error = VdrError;

    fn try_from(log: ContractEvent) -> Result<Self, Self::Error> {
        let revocation_registry_definition_id = log.get_fixed_bytes(0)?;
        let timestamp = log.get_uint(1)?;
        let rev_reg_entry_tuple = log.get_tuple(2)?;
        let current_accumulator = String::from_utf8(rev_reg_entry_tuple.get_bytes(0)?)
            .map_err(|e| VdrError::InvalidRevocationRegistryEntry(e.to_string()))?;
        let prev_accumulator = String::from_utf8(rev_reg_entry_tuple.get_bytes(1)?)
            .map_err(|e| VdrError::InvalidRevocationRegistryEntry(e.to_string()))?;
        let issued = rev_reg_entry_tuple.get_uint32_array(2)?;
        let revoked = rev_reg_entry_tuple.get_uint32_array(3)?;
        let entry_timestamp = rev_reg_entry_tuple.get_u64(4)?;

        Ok(RevRegEntryCreated {
            revocation_registry_definition_id,
            timestamp,
            rev_reg_entry: RevRegEntry {
                current_accumulator,
                prev_accumulator,
                issued,
                revoked,
                timestamp: entry_timestamp,
            },
        })
    }
}
