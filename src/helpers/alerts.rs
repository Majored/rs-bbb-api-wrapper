// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::alerts::AlertReadBody;
use crate::error::Result;
use crate::APIWrapper;

pub struct AlertsHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> AlertsHelper<'a> {
    pub async fn mark_as_read(&self) -> Result<()> {
        self.wrapper.patch(&format!("{}/alerts", crate::BASE_URL), &AlertReadBody { read: true }).await
    }
}
