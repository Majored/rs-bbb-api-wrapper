// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::{Serialize, Deserialize};
use derive_getters::Getters;

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct MemberData {
    member_id: u64,
    username: String,
    join_date: u64,
    last_activity_date: Option<u64>,
    gender: Option<String>,
    timezone: Option<String>,
    banned: bool,
    suspended: bool,
    restricted: bool,
    disabled: bool,
    post_count: u64,
    resource_count: u64,
    purchase_count: u64,
    feedback_positive: u64,
    feedback_neutral: u64,
    feedback_negative: u64,
}
