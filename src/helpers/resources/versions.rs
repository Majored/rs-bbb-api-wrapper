// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::VersionData;
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct VersionHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> VersionHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<VersionData>> {
        self.wrapper.get(&format!("{}/resources/{}/versions", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn latest(&self, resource_id: u64) -> Result<VersionData> {
        self.wrapper.get(&format!("{}/resources/{}/versions/latest", crate::BASE_URL, resource_id), None).await
    }

    pub async fn fetch(&self, resource_id: u64, version_id: u64) -> Result<VersionData> {
        self.wrapper.get(&format!("{}/resources/{}/versions/{}", crate::BASE_URL, resource_id, version_id), None).await
    }

    pub async fn delete(&self, resource_id: u64, version_id: u64) -> Result<()> {
        self.wrapper.delete(&format!("{}/resources/{}/versions/{}", crate::BASE_URL, resource_id, version_id)).await
    }
}
