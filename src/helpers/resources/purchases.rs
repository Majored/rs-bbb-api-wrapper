// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::PurchaseData;
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct PurchaseHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> PurchaseHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<PurchaseData>> {
        self.wrapper.get(&format!("{}/resources/{}/purchases", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn fetch(&self, resource_id: u64, purchase_id: u64) -> Result<PurchaseData> {
        self.wrapper.get(&format!("{}/resources/{}/purchases/{}", crate::BASE_URL, resource_id, purchase_id), None).await
    }
}
