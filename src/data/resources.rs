// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct BasicResourceData {
    resource_id: u64,
    author_id: u64,
    title: String,
    tag_line: String,
    price: f64,
    currency: String,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceData {
    resource_id: u64,
    author_id: u64,
    title: String,
    tag_line: String,
    description: String,
    release_date: u64,
    last_update_date: u64,
    category_title: String,
    current_version_id: u64,
    discussion_thread_id: u64,
    price: f64,
    currency: String,
    purchase_count: u64,
    download_count: u64,
    review_count: u64,
    review_average: f64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct DownloadData {
    resource_id: u64,
    version_id: u64,
    downloader_id: u64,
    download_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ReviewData {
    review_id: u64,
    resource_id: u64,
    version_id: u64,
    version_name: String,
    reviewer_id: u64,
    review_date: u64,
    deleted: Option<bool>,
    rating: u8,
    message: String,
    author_response: String,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateData {
    update_id: u64,
    title: String,
    message: String,
    deleted: Option<bool>,
    update_date: u64,
    likes: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    version_id: u64,
    update_id: u64,
    name: String,
    deleted: Option<bool>,
    release_date: u64,
    download_count: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct LicenseData {
    license_id: u64,
    purchaser_id: u64,
    validated: bool,
    active: bool,
    start_date: u64,
    end_date: u64,
    previous_end_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseData {
    purchase_id: u64,
    purchaser_id: u64,
    license_id: u64,
    renewal: bool,
    status: String,
    price: f64,
    currency: String,
    purchase_date: u64,
    validation_date: u64,
}
