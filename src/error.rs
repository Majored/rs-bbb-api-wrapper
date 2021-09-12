// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use serde::Deserialize;

pub type Result<V> = std::result::Result<V, APIError>;

#[derive(Deserialize, Debug)]
pub struct APIError {
    code: String,
    message: String,
}

impl APIError {
    pub fn from_raw(code: String, message: String) -> APIError {
        Self { code, message }
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}
