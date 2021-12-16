// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

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

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePostData {
    profile_post_id: u64,
    author_id: u64,
    post_date: u64,
    message: String,
    like_count: u64,
    comment_count: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct BanData {
    member_id: u64,
    banned_by_id: u64,
    ban_date: u64,
    reason: String,
}

#[derive(Serialize)]
pub(crate) struct ProfilePostEditBody<'a> {
    pub message: &'a str,
}
