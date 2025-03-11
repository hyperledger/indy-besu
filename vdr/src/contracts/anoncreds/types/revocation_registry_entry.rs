// Copyright (c) 2024 DSR Corporation, Denver, Colorado.
// https://www.dsr-corporation.com
// SPDX-License-Identifier: Apache-2.0

use crate::{error::VdrError, types::ContractParam, RevocationRegistryDefinitionId, VdrResult};

use crate::contracts::did::types::did::DID;

use ethabi::{Bytes, Uint};
use serde_derive::{Deserialize, Serialize};

/// Wrapper structure for DID
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Accumulator(String);

impl Accumulator {
    pub(crate) fn validate(&self) -> VdrResult<()> {
        if self.0.is_empty() {
            return Err(VdrError::InvalidRevocationRegistryEntry(format!(
                "Incorrect Accumulator: {}",
                &self.0
            )));
        }

        Ok(())
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }
}

impl From<&str> for Accumulator {
    fn from(acc: &str) -> Self {
        Accumulator(acc.to_string())
    }
}

impl AsRef<str> for Accumulator {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&Accumulator> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &Accumulator) -> Result<Self, Self::Error> {
        Ok(ContractParam::Bytes(Bytes::from(value.as_bytes())))
    }
}

/// Definition of AnonCreds Revocation Registry Definition object matching to the specification - `<https://hyperledger.github.io/anoncreds-spec/#term:revocation-registry-entry>`
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryEntry {
    #[serde(rename = "revRegDefId")]
    pub rev_reg_def_id: RevocationRegistryDefinitionId,
    #[serde(rename = "issuerId")]
    pub issuer_id: DID,
    pub rev_reg_entry_data: RevocationRegistryEntryData,
}

/// Revocation Registry Entry Data stored in the Registry
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryEntryData {
    #[serde(rename = "currentAccumulator")]
    pub current_accumulator: Accumulator,
    #[serde(rename = "prevAccumulator")]
    pub prev_accumulator: Accumulator,
    pub issued: Vec<u32>,
    pub revoked: Vec<u32>,
}

impl RevocationRegistryEntry {
    pub fn new(
        rev_reg_def_id: RevocationRegistryDefinitionId,
        issuer_id: DID,
        current_accumulator: Accumulator,
        prev_accumulator: Accumulator,
        issued: Vec<u32>,
        revoked: Vec<u32>,
    ) -> RevocationRegistryEntry {
        RevocationRegistryEntry {
            rev_reg_def_id,
            issuer_id,
            rev_reg_entry_data: RevocationRegistryEntryData {
                prev_accumulator,
                current_accumulator,
                issued,
                revoked,
            },
        }
    }

    //TODO:
    pub(crate) fn validate(&self) -> VdrResult<()> {
        self.rev_reg_entry_data.current_accumulator.validate()?;
        self.rev_reg_entry_data.prev_accumulator.validate()?;

        Ok(())
    }

    //TODO:
    pub fn to_string(&self) -> VdrResult<String> {
        serde_json::to_string(self).map_err(|err| {
            VdrError::InvalidRevocationRegistryEntry(format!(
                "Unable to serialize Revocation RegistryE Entry as JSON. Err: {:?}",
                err
            ))
        })
    }

    //TODO:
    pub fn from_string(value: &str) -> VdrResult<RevocationRegistryEntry> {
        serde_json::from_str(value).map_err(|err| {
            VdrError::InvalidRevocationRegistryEntry(format!(
                "Unable to parse Revocation Registry Entry from JSON. Err: {:?}",
                err.to_string()
            ))
        })
    }
}

impl TryFrom<&RevocationRegistryEntryData> for ContractParam {
    type Error = VdrError;

    fn try_from(value: &RevocationRegistryEntryData) -> Result<Self, Self::Error> {
        Ok(ContractParam::Tuple(vec![
            ContractParam::Bytes(Bytes::from(value.current_accumulator.as_bytes())),
            ContractParam::Array(
                value
                    .issued
                    .iter()
                    .map(|&x| ContractParam::Uint(Uint::from(x)))
                    .collect(),
            ),
            ContractParam::Array(
                value
                    .revoked
                    .iter()
                    .map(|&x| ContractParam::Uint(Uint::from(x)))
                    .collect(),
            ),
        ]))
    }
}

//TODO:
#[cfg(test)]
pub mod test {
    use super::*;

    pub fn revocation_registry_entry_data() -> RevocationRegistryEntryData {
        RevocationRegistryEntryData {
            current_accumulator: Accumulator::from("currentAccum"),
            prev_accumulator: Accumulator::from("prevAccum"),
            issued: vec![0, 1, 2, 3, 4],
            revoked: vec![],
        }
    }

    pub fn revocation_registry_entry(
        issuer_id: &DID,
        rev_reg_def_id: &RevocationRegistryDefinitionId,
    ) -> RevocationRegistryEntry {
        RevocationRegistryEntry {
            issuer_id: issuer_id.clone(),
            rev_reg_def_id: rev_reg_def_id.clone(),
            rev_reg_entry_data: revocation_registry_entry_data(),
        }
    }
}
