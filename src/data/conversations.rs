// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    conversation_id: u64,
    title: String,
    creator_id: u64,
    start_date: u64,
    last_message_date: u64,
    last_read_date: u64,
    open: bool,
    reply_count: u64,
    recipient_ids: Vec<u64>,
}
