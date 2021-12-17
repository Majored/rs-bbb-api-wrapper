// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::error::Result;
use crate::data::members::{MemberData, ProfilePostData, ProfilePostEditBody, BanData};
use crate::APIWrapper;
use crate::sort::SortOptions;

pub struct MembersHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> MembersHelper<'a> {
    pub async fn fetch_self(&self) -> Result<MemberData> {
        self.wrapper.get(&format!("{}/members/self", crate::BASE_URL), None).await
    }

    pub async fn fetch_by_id(&self, member_id: u64) -> Result<MemberData> {
        self.wrapper.get(&format!("{}/members/{}", crate::BASE_URL, member_id), None).await
    }

    pub async fn fetch_by_name(&self, member_name: &str) -> Result<MemberData> {
        self.wrapper.get(&format!("{}/members/username/{}", crate::BASE_URL, member_name), None).await
    }

    pub async fn list_recent_bans(&self) -> Result<BanData> {
        self.wrapper.get(&format!("{}/members/bans", crate::BASE_URL), None).await
    }

    pub async fn list_profile_posts(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<ProfilePostData>> {
        self.wrapper.get(&format!("{}/members/profile-posts", crate::BASE_URL), sort).await
    }

    pub async fn fetch_profile_post(&self, profile_post_id: u64) -> Result<ProfilePostData> {
        self.wrapper.get(&format!("{}/members/profile-posts/{}", crate::BASE_URL, profile_post_id), None).await
    }

    pub async fn edit_profile_post(&self, profile_post_id: u64, message: &str) -> Result<()> {
        let data = ProfilePostEditBody { message };
        self.wrapper.patch(&format!("{}/members/profile-posts/{}", crate::BASE_URL, profile_post_id), &data).await
    }

    pub async fn delete_profile_post(&self, profile_post_id: u64) -> Result<()> {
        self.wrapper.delete(&format!("{}/members/profile-posts/{}", crate::BASE_URL, profile_post_id)).await
    }
}
