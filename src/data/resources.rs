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
    price: f64,
    currency: String,
    purchase_count: u64,
    download_count: u64,
    review_count: u64,
    review_average: f64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct DownloadData {
    download_id: u64,
    version_id: u64,
    downloader_id: u64,
    download_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct ReviewData {
    review_id: u64,
    reviewer_id: u64,
    review_date: u64,
    rating: u8,
    message: String,
    response: String,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateData {
    update_id: u64,
    title: String,
    message: String,
    update_date: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    version_id: u64,
    name: String,
    release_date: u64,
    download_count: u64,
}

#[derive(Getters, Debug, Clone, Serialize, Deserialize)]
pub struct LicenseData {
    license_id: u64,
    purchaser_id: u64,
    validated: bool,
    active: bool,
    permanent: bool,
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

#[derive(Debug, Default, Serialize)]
pub struct LicenseIssuePermData {
    pub purchaser_id: u64,
    pub permanent: bool,
    pub active: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct LicenseIssueTempData {
    pub purchaser_id: u64,
    pub permanent: bool,
    pub start_date: u64,
    pub end_date: u64,
}

#[derive(Debug, Default, Serialize)]
pub struct LicenseModifyPermData {
    pub permanent: bool,
    pub active: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct LicenseModifyTempData {
    pub permanent: bool,
    pub start_date: u64,
    pub end_date: u64,
}

#[derive(Serialize)]
pub(crate) struct ReviewRespondData<'a> {
    pub message: &'a str,
}


#[derive(Debug, Default, Serialize)]
pub struct ResourceModifyData<'a> {
    pub title: Option<&'a str>,
    pub tag_line: Option<&'a str>,
    pub description: Option<&'a str>,
}