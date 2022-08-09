// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

pub mod downloads;
pub mod licenses;
pub mod purchases;
pub mod reviews;
pub mod updates;
pub mod versions;

use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

use crate::data::resources::BasicResourceData;
use crate::data::resources::ResourceData;
use crate::data::resources::ResourceModifyData;

use downloads::DownloadHelper;
use licenses::LicenseHelper;
use purchases::PurchaseHelper;
use reviews::ReviewHelper;
use updates::UpdateHelper;
use versions::VersionHelper;

pub struct ResourceHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ResourceHelper<'a> {
    pub async fn list(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<BasicResourceData>> {
        self.wrapper.get(&format!("{}/resources", crate::BASE_URL), sort).await
    }

    pub async fn list_owned(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<BasicResourceData>> {
        self.wrapper.get(&format!("{}/resources/owned", crate::BASE_URL), sort).await
    }

    pub async fn list_collaborated(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<BasicResourceData>> {
        self.wrapper.get(&format!("{}/resources/collaborated", crate::BASE_URL), sort).await
    }

    pub async fn fetch(&self, resource_id: u64) -> Result<ResourceData> {
        self.wrapper.get(&format!("{}/resources/{}", crate::BASE_URL, resource_id), None).await
    }

    pub async fn modify(&self, resource_id: u64, fields: &ResourceModifyData<'_>) -> Result<ResourceData> {
        self.wrapper.patch(&format!("{}/resources/{}", crate::BASE_URL, resource_id), fields).await
    }

    pub fn downloads(&self) -> DownloadHelper<'_> {
        DownloadHelper { wrapper: self.wrapper }
    }
    
    pub fn licenses(&self) -> LicenseHelper<'_> {
        LicenseHelper { wrapper: self.wrapper }
    }

    pub fn purchases(&self) -> PurchaseHelper<'_> {
        PurchaseHelper { wrapper: self.wrapper }
    }

    pub fn reviews(&self) -> ReviewHelper<'_> {
        ReviewHelper { wrapper: self.wrapper }
    }

    pub fn updates(&self) -> UpdateHelper<'_> {
        UpdateHelper { wrapper: self.wrapper }
    }

    pub fn versions(&self) -> VersionHelper<'_> {
        VersionHelper { wrapper: self.wrapper }
    }
}
