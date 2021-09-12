// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicThread {
    thread_id: u64,
    title: String,
    reply_count: u64,
    view_count: u64,
    creation_date: u64,
    last_message_date: u64,
}

impl BasicThread {
    /// Returns the identifier of the thread.
    pub fn thread_id(&self) -> u64 {
        self.thread_id
    }

    /// Returns the title of the thread.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the reply count of the thread.
    pub fn reply_count(&self) -> u64 {
        self.reply_count
    }

    /// Returns the view count of the thread.
    pub fn view_count(&self) -> u64 {
        self.view_count
    }

    /// Returns the creation date of the thread.
    pub fn creation_date(&self) -> u64 {
        self.creation_date
    }

    /// Returns the last message date of the thread.
    pub fn last_message_date(&self) -> u64 {
        self.last_message_date
    }
}

#[derive(Deserialize)]
pub struct Thread {
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

impl Thread {
    /// Returns the identifier of the thread.
    pub fn thread_id(&self) -> u64 {
        self.thread_id
    }

    /// Returns the title of the thread.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the name of the forum where the thread is located.
    pub fn forum_name(&self) -> &str {
        &self.forum_name
    }

    /// Returns the reply count of the thread.
    pub fn reply_count(&self) -> u64 {
        self.reply_count
    }

    /// Returns the view count of the thread.
    pub fn view_count(&self) -> u64 {
        self.view_count
    }

    /// Returns the post date of the thread.
    pub fn post_date(&self) -> u64 {
        self.post_date
    }

    /// Returns the type of the thread.
    pub fn thread_type(&self) -> &str {
        &self.thread_type
    }

    /// Returns whether or not the thread is open for replies.
    pub fn thread_open(&self) -> bool {
        self.thread_open
    }

    /// Returns the last post date of the thread.
    pub fn last_post_date(&self) -> u64 {
        self.last_post_date
    }
}

#[derive(Deserialize)]
pub struct Reply {
    reply_id: u64,
    author_id: u64,
    post_date: u64,
    message: String,
    like_count: u64,
}

impl Reply {
    /// Returns the identifier of the reply.
    pub fn reply_id(&self) -> u64 {
        self.reply_id
    }

    /// Returns the identifier of the reply's author.
    pub fn author_id(&self) -> u64 {
        self.author_id
    }

    /// Returns the post date of the reply.
    pub fn post_date(&self) -> u64 {
        self.post_date
    }

    /// Returns the message content of the reply.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the like count of the reply.
    pub fn like_count(&self) -> u64 {
        self.like_count
    }
}
