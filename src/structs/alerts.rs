// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Alert {
    caused_member_id: u64,
    content_type: String,
    content_id: u64,
    alert_type: String,
    alert_date: u64,
}

impl Alert {
    /// Returns the identifier of the member who caused the alert to be sent.
    pub fn caused_member_id(&self) -> u64 {
        self.caused_member_id
    }

    /// Returns a reference to alert's content type.
    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    /// Returns the identifier of the content you were alerted about.
    pub fn content_id(&self) -> u64 {
        self.content_id
    }

    /// Returns a reference to the alert's type.
    pub fn alert_type(&self) -> &String {
        &self.alert_type
    }

    /// Returns the UNIX timestamp of when this alert occurred.
    pub fn alert_date(&self) -> u64 {
        self.alert_date
    }
}
