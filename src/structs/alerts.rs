// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::error::Result;
use crate::APIWrapper;

use serde::Serialize;

#[derive(Serialize)]
struct AlertReadBody {
    read: bool,
}

pub async fn mark_as_read(wrapper: &APIWrapper) -> Result<()> {
    wrapper.patch(format!("{}/alerts", crate::BASE_URL), &AlertReadBody { read: true }).await
}
