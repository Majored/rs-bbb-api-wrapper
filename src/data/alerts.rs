// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::{Serialize, Deserialize};
use derive_getters::Getters;

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    caused_member_id: u64,
    content_type: String,
    content_id: u64,
    alert_type: String,
    alert_date: u64,
}