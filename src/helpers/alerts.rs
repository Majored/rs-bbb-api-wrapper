// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use crate::data::alerts::{AlertData, AlertReadBody};
use crate::sort::SortOptions;
use crate::error::Result;
use crate::APIWrapper;

pub struct AlertsHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> AlertsHelper<'a> {
    pub async fn list_unread(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<AlertData>> {
        self.wrapper.get(&format!("{}/alerts", crate::BASE_URL), sort).await
    }

    pub async fn mark_as_read(&self) -> Result<()> {
        self.wrapper.patch(&format!("{}/alerts", crate::BASE_URL), &AlertReadBody { read: true }).await
    }
}
