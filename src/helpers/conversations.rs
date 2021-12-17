// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::error::Result;
use crate::data::conversations::{ConversationData, ReplyData, ConversationStartBody, ConversationReplyBody};
use crate::sort::SortOptions;
use crate::APIWrapper;

pub struct ConversationsHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ConversationsHelper<'a> {
    pub async fn list_unread(&self, sort: Option<&SortOptions<'_>>) -> Result<Vec<ConversationData>> {
        self.wrapper.get(&format!("{}/conversations", crate::BASE_URL), sort).await
    }

    pub async fn list_replies(&self, conversation_id: u64, sort: Option<&SortOptions<'_>>) -> Result<Vec<ReplyData>> {
        self.wrapper.get(&format!("{}/conversations/{}/replies", crate::BASE_URL, conversation_id), sort).await
    }

    pub async fn start(&self, title: &str, message: &str, recipient_ids: &[u64]) -> Result<u64> {
        let data = ConversationStartBody { title, message, recipient_ids };
        self.wrapper.post(&format!("{}/conversations", crate::BASE_URL), &data).await
    }

    pub async fn reply(&self, conversation_id: u64, message: &str) -> Result<u64> {
        let data = ConversationReplyBody { message };
        self.wrapper.post(&format!("{}/conversations/{}/replies", crate::BASE_URL, conversation_id), &data).await
    }
}
