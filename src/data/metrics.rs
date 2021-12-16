// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use std::collections::BTreeMap;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    interval: MetricsInterval,
    metrics: BTreeMap<String, u64>,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct MetricsInterval {
    time: u16,
    unit: String,
    last: u64,
}
