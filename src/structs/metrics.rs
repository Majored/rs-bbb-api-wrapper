// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MetricsSnapshot {
    interval: MetricsInterval,
    metrics: BTreeMap<String, u64>,
}

impl MetricsSnapshot {
    pub fn interval(&self) -> &MetricsInterval {
        &self.interval
    }

    pub fn get_metric(&self, name: &str) -> Option<&u64> {
        self.metrics.get(name)
    }
}

#[derive(Deserialize)]
pub struct MetricsInterval {
    time: u16,
    unit: String,
    last: u64,
}

impl MetricsInterval {
    pub fn time(&self) -> &u16 {
        &self.time
    }

    pub fn unit(&self) -> &String {
        &self.unit
    }

    pub fn last(&self) -> &u64 {
        &self.last
    }
}