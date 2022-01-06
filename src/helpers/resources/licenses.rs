// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::LicenseData;
use crate::data::resources::{LicenseIssuePermData, LicenseIssueTempData, LicenseModifyPermData, LicenseModifyTempData};
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct LicenseHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> LicenseHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<LicenseData>> {
        self.wrapper.get(&format!("{}/resources/{}/licenses", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn fetch(&self, resource_id: u64, license_id: u64) -> Result<LicenseData> {
        self.wrapper.get(&format!("{}/resources/{}/licenses/{}", crate::BASE_URL, resource_id, license_id), None).await
    }

    pub async fn fetch_by_member(&self, resource_id: u64, member_id: u64) -> Result<LicenseData> {
        self.wrapper.get(&format!("{}/resources/{}/licenses/members/{}", crate::BASE_URL, resource_id, member_id), None).await
    }

    pub async fn issue_permanent(&self, resource_id: u64, fields: &LicenseIssuePermData) -> Result<u64> {
        self.wrapper.post(&format!("{}/resources/{}/licenses", crate::BASE_URL, resource_id), &fields).await
    }

    pub async fn issue_temporary(&self, resource_id: u64, fields: &LicenseIssueTempData) -> Result<u64> {
        self.wrapper.post(&format!("{}/resources/{}/licenses", crate::BASE_URL, resource_id), &fields).await
    }

    pub async fn modify_permanent(&self, resource_id: u64, license_id: u64, fields: &LicenseModifyPermData) -> Result<()> {
        self.wrapper.patch(&format!("{}/resources/{}/licenses/{}", crate::BASE_URL, resource_id, license_id), &fields).await
    }

    pub async fn modify_temporary(&self, resource_id: u64, license_id: u64, fields: &LicenseModifyTempData) -> Result<()> {
        self.wrapper.patch(&format!("{}/resources/{}/licenses/{}", crate::BASE_URL, resource_id, license_id), &fields).await
    }
}
