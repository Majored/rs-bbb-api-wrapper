// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

//! Represents the sorting options made available by the API.

use crate::error::Result;

use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SortOptions<'a> {
    pub sort: Option<&'a str>,
    pub order: Option<&'a str>,
    pub page: Option<u64>,
}

impl<'a> SortOptions<'a> {
    pub fn sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn order(mut self, order: &'a str) -> Self {
        self.order = Some(order);
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn to_query_string(&self) -> Result<String> {
        Ok(serde_qs::to_string(self)?)
    }    
}