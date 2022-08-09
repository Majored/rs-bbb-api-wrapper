// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use crate::data::resources::UpdateData;
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct UpdateHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> UpdateHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<UpdateData>> {
        self.wrapper.get(&format!("{}/resources/{}/updates", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn latest(&self, resource_id: u64) -> Result<UpdateData> {
        self.wrapper.get(&format!("{}/resources/{}/updates/latest", crate::BASE_URL, resource_id), None).await
    }

    pub async fn fetch(&self, resource_id: u64, update_id: u64) -> Result<UpdateData> {
        self.wrapper.get(&format!("{}/resources/{}/updates/{}", crate::BASE_URL, resource_id, update_id), None).await
    }

    pub async fn delete(&self, resource_id: u64, update_id: u64) -> Result<()> {
        self.wrapper.delete(&format!("{}/resources/{}/updates/{}", crate::BASE_URL, resource_id, update_id)).await
    }
}
