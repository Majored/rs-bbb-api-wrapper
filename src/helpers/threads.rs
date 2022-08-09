// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use crate::error::Result;
use crate::data::threads::{BasicThreadData, ThreadData, ReplyData, ReplyBody};
use crate::APIWrapper;
use crate::sort::SortOptions;

pub struct ThreadsHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ThreadsHelper<'a> {
    pub async fn list_threads(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<BasicThreadData>> {
        self.wrapper.get(&format!("{}/threads", crate::BASE_URL), sort).await
    }

    pub async fn fetch_thread(&self, thread_id: u64) -> Result<ThreadData> {
        self.wrapper.get(&format!("{}/threads/{}", crate::BASE_URL, thread_id), None).await
    }

    pub async fn list_replies(&self, thread_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<ReplyData>> {
        self.wrapper.get(&format!("{}/threads/{}/replies", crate::BASE_URL, thread_id), sort).await
    }

    pub async fn reply(&self, thread_id: u64, message: &str) -> Result<u64> {
        self.wrapper.post(&format!("{}/threads/{}/replies", crate::BASE_URL, thread_id), &ReplyBody { message }).await
    }
}
