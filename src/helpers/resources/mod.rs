// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::ResourceData;
use crate::error::Result;
use crate::APIWrapper;

pub struct ResourceHelper<'a> {
    pub(crate) wrapper: &'a APIWrapper,
}

impl<'a> ResourceHelper<'a> {
    pub async fn fetch(&self, resource_id: u64) -> Result<ResourceData> {
        self.wrapper.get(format!("{}/resources/{}", crate::BASE_URL, resource_id)).await
    }
}
