// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::{
    DownloadData, LicenseData, PurchaseData, ResourceData, ReviewData, UpdateData, VersionData,
};
use crate::error::Result;
use crate::APIWrapper;

pub struct ResourceHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ResourceHelper<'a> {
    pub async fn fetch(&self, resource_id: u64) -> Result<ResourceData> {
        self.wrapper.get(&format!("{}/resources/{}", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_reviews(&self, resource_id: u64) -> Result<Vec<ReviewData>> {
        self.wrapper.get(&format!("{}/resources/{}/reviews", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_downloads(&self, resource_id: u64) -> Result<Vec<DownloadData>> {
        self.wrapper.get(&format!("{}/resources/{}/downloads", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_licenses(&self, resource_id: u64) -> Result<Vec<LicenseData>> {
        self.wrapper.get(&format!("{}/resources/{}/licenses", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_purchases(&self, resource_id: u64) -> Result<Vec<PurchaseData>> {
        self.wrapper.get(&format!("{}/resources/{}/purchases", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_versions(&self, resource_id: u64) -> Result<Vec<VersionData>> {
        self.wrapper.get(&format!("{}/resources/{}/versions", crate::BASE_URL, resource_id)).await
    }

    pub async fn list_updates(&self, resource_id: u64) -> Result<Vec<UpdateData>> {
        self.wrapper.get(&format!("{}/resources/{}/updates", crate::BASE_URL, resource_id)).await
    }
}
