// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use crate::data::resources::{ReviewData, ReviewRespondData};
use crate::error::Result;
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct ReviewHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ReviewHelper<'a> {
    pub async fn list(&self, resource_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<ReviewData>> {
        self.wrapper.get(&format!("{}/resources/{}/reviews", crate::BASE_URL, resource_id), sort).await
    }

    pub async fn fetch_by_member(&self, resource_id: u64, member_id: u64) -> Result<ReviewData> {
        self.wrapper.get(&format!("{}/resources/{}/reviews/members/{}", crate::BASE_URL, resource_id, member_id), None).await
    }

    pub async fn respond(&self, resource_id: u64, review_id: u64, message: &str) -> Result<()> {
        let body = ReviewRespondData { message };
        self.wrapper.patch(&format!("{}/resources/{}/reviews/{}", crate::BASE_URL, resource_id, review_id), &body).await
    }
}
