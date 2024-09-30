// Copyright (c) 2024 DSR Corporation, Denver, Colorado.
// https://www.dsr-corporation.com
// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::module_inception)]
mod ffi;

pub use ffi::*;

pub type JsonValue = serde_json::Value;

impl UniffiCustomTypeConverter for JsonValue {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val).expect("unable wrap json value"))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serde_json::to_string(&obj).expect("unable unwrap json value")
    }
}

uniffi::custom_type!(JsonValue, String);
uniffi::include_scaffolding!("indy_besu_vdr");
