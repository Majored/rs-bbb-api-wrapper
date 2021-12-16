// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    conversation_id: u64,
    title: String,
    creation_date: u64,
    creator_id: u64,
    last_message_date: u64,
    last_read_date: u64,
    open: bool,
    reply_count: u64,
    recipient_ids: Vec<u64>,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ReplyData {
    message_id: u64,
    message_date: u64,
    author_id: u64,
    message: String,
}

#[derive(Serialize)]
pub(crate) struct ConversationStartBody<'a> {
    pub(crate) title: &'a str,
    pub(crate) message: &'a str,
    pub(crate) recipient_ids: &'a [u64],
}

#[derive(Serialize)]
pub(crate) struct ConversationReplyBody<'a> {
    pub(crate) message: &'a str,
}
