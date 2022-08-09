// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-bbb-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct AlertData {
    caused_member_id: u64,
    content_type: String,
    content_id: u64,
    alert_type: String,
    alert_date: u64,
}

#[derive(Serialize)]
pub(crate) struct AlertReadBody {
    pub read: bool,
}
