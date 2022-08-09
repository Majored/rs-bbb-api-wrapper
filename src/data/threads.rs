// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct BasicThreadData {
    thread_id: u64,
    title: String,
    reply_count: u64,
    view_count: u64,
    creation_date: u64,
    last_message_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    thread_id: u64,
    forum_name: String,
    title: String,
    reply_count: u64,
    view_count: u64,
    post_date: u64,
    thread_type: String,
    thread_open: bool,
    last_post_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ReplyData {
    reply_id: u64,
    author_id: u64,
    post_date: u64,
    message: String,
}

#[derive(Serialize)]
pub(crate) struct ReplyBody<'a> {
    pub message: &'a str,
}
