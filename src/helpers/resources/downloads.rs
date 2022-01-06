// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::DownloadData; 
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct DownloadHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> DownloadHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<DownloadData>> {
        self.wrapper.get(&format!("{}/resources/{}/downloads", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn list_by_member(&self, resource_id: u64, member_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<DownloadData>> {
        self.wrapper.get(&format!("{}/resources/{}/downloads/members/{}", crate::BASE_URL, resource_id, member_id), sort).await
    }

    pub async fn list_by_version(&self, resource_id: u64, version_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<DownloadData>> {
        self.wrapper.get(&format!("{}/resources/{}/downloads/versions/{}", crate::BASE_URL, resource_id, version_id), sort).await
    }
}