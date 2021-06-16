// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Member {
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

impl Member {
    pub fn member_id(&self) -> &u64 {
        &self.member_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn join_date(&self) -> &u64 {
        &self.join_date
    }

    pub fn last_activity_date(&self) -> Option<&u64> {
        self.last_activity_date.as_ref()
    }

    pub fn gender(&self) -> Option<&String> {
        self.gender.as_ref()
    }

    pub fn timezone(&self) -> Option<&String> {
        self.timezone.as_ref()
    }

    pub fn banned(&self) -> &bool {
        &self.banned
    }

    pub fn suspended(&self) -> &bool {
        &self.suspended
    }

    pub fn restricted(&self) -> &bool {
        &self.restricted
    }

    pub fn disabled(&self) -> &bool {
        &self.disabled
    }

    pub fn post_count(&self) -> &u64 {
        &self.post_count
    }

    pub fn resource_count(&self) -> &u64 {
        &self.resource_count
    }

    pub fn purchase_count(&self) -> &u64 {
        &self.purchase_count
    }

    pub fn feedback_positive(&self) -> &u64 {
        &self.feedback_positive
    }

    pub fn feedback_neutral(&self) -> &u64 {
        &self.feedback_neutral
    }

    pub fn feedback_negative(&self) -> &u64 {
        &self.feedback_negative
    }

    pub fn feedback_total(&self) -> u64 {
        self.feedback_positive - self.feedback_negative
    }
}
