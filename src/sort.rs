// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

#[derive(Default)]
pub struct SortOptions<'a> {
    pub sort: Option<&'a str>,
    pub order: Option<&'a str>,
    pub page: Option<u64>,
}