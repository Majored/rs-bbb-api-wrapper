// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Conversation {
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

impl Conversation {
    /// Returns the conversation's identifier.
    pub fn conversation_id(&self) -> u64 {
        self.conversation_id
    }

    /// Returns a reference to the conversation's title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Returns the conversation creator's identifier.
    pub fn creator_id(&self) -> u64 {
        self.creator_id
    }

    /// Returns the UNIX timestamp of when the conversation was created.
    pub fn start_date(&self) -> u64 {
        self.start_date
    }

    /// Returns the UNIX timestamp of when the last message was sent in the conversation.
    pub fn last_message_date(&self) -> u64 {
        self.last_message_date
    }

    /// Returns the UNIX timestamp of the latest message you've read in the conversation.
    pub fn last_read_date(&self) -> u64 {
        self.last_read_date
    }

    /// Returns whether the conversation is currently open.
    pub fn open(&self) -> bool {
        self.open
    }

    /// Returns the number of replies within the conversation.
    pub fn reply_count(&self) -> u64 {
        self.reply_count
    }

    /// Returns a reference to a list of the conversation's recipients.
    /// Does not include conversation starter.
    pub fn recipient_ids(&self) -> &Vec<u64> {
        &self.recipient_ids
    }
}
