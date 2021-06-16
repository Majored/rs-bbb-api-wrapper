// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::error::APIError;
use crate::APIWrapper;
use serde::Deserialize;

pub struct Resource<'a> {
    wrapper: &'a APIWrapper,
    identifier: u64,
    data: Option<ResourceData>,
}

impl<'a> Resource<'a> {
    /// Construct a resource from its raw parts (an immutable borrow to the wrapper and an identifier).
    /// 
    /// # Note
    /// This construction does not make any requests to the API, thus also doesn't validate if the resource associated
    /// with the given identifier exists or is viewable by the requesting member. You may want to use this construction
    /// when calling a child function/endpoint (such as listing reviews) without needing underlying data about the
    /// resource (thus, we avoid an additional API request).
    /// 
    /// # Example
    /// ```
    /// let resource = Resource::from_raw(&wrapper, 16682);
    /// let reviews = resource.list_reviews().await.unwrap();
    /// 
    /// assert!(reviews.len() != 0);
    /// ```
    pub fn from_raw(wrapper: &'a APIWrapper, identifier: u64) -> Resource<'a> {
        Self {
            wrapper,
            identifier,
            data: None,
        }
    }

    /// Construct a resource from its raw parts (an immutable borrow to the wrapper and an identifier), and fetch its
    /// underlying data.
    /// 
    /// # Note
    /// As this operaion is fallible, we return a Result rather than the raw Resource type. This allows any potential
    /// request error to be unwound higher up the call stack.
    /// 
    /// This is an alternate function for `APIWrapper::fetch_resource()`.
    /// 
    /// # Example
    /// ```
    /// let resource = match Resource::from_raw_fetch_data(&wrapper, 16682) {
    ///     Ok(resource) => resource,
    ///     Err(error) => {
    ///         if error.code() == "ContentNotFoundError" {
    ///             error!("Resource does not exist.");
    ///             return;
    ///         } else {
    ///             panic!("An unexpected error occured: {:?}", error);
    ///         }
    ///     }
    /// };
    /// 
    /// assert_eq!(resource.data_unchecked().resource_id(), 16682);
    /// ```
    pub async fn from_raw_fetch_data(wrapper: &'a APIWrapper, identifier: u64) -> Result<Resource<'a>, APIError> {
        let mut resource = Resource::from_raw(wrapper, identifier);
        resource.fetch_data().await?;
        Ok(resource)
    }

    pub async fn fetch_data(&mut self) -> Result<(), APIError> {
        self.data = Some(self.wrapper.get(format!("{}/resources/{}", crate::BASE_URL, self.identifier)).await?);
        Ok(())
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    pub fn data(&self) -> Option<&ResourceData> {
        self.data.as_ref()
    }

    pub fn data_unchecked(&self) -> &ResourceData {
        self.data.as_ref().unwrap()
    }

    pub async fn list_reviews(&self) -> Result<Vec<Review>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/reviews", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_downloads(&self) -> Result<Vec<Download>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/downloads", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_licenses(&self) -> Result<Vec<License>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/licenses", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_purchases(&self) -> Result<Vec<Purchase>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/purchases", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_versions(&self) -> Result<Vec<Version>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/versions", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_updates(&self) -> Result<Vec<Update>, APIError> {
        self.wrapper.get(format!("{}/resources/{}/updates", crate::BASE_URL, self.identifier)).await
    }
}

#[derive(Deserialize)]
pub struct ResourceData {
    resource_id: u64,
    author_id: u64,
    title: String,
    tag_line: String,
    description: String,
    release_date: u64,
    last_update_date: u64,
    deleted: bool,
    moderated: bool,
    category_id: u64,
    category_title: String,
    current_version_id: u64,
    discussion_thread_id: u64,
    price: f64,
    currency: String,
    download_count: u64,
    review_count: u64,
    review_average: f64,
}

impl ResourceData {
    pub fn resource_id(&self) -> &u64 {
        &self.resource_id
    }

    pub fn author_id(&self) -> &u64 {
        &self.author_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn tag_line(&self) -> &String {
        &self.tag_line
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn release_date(&self) -> &u64 {
        &self.release_date
    }

    pub fn last_update_date(&self) -> &u64 {
        &self.last_update_date
    }

    pub fn deleted(&self) -> &bool {
        &self.deleted
    }

    pub fn moderated(&self) -> &bool {
        &self.moderated
    }

    pub fn category_id(&self) -> &u64 {
        &self.category_id
    }

    pub fn category_title(&self) -> &String {
        &self.category_title
    }

    pub fn current_version_id(&self) -> &u64 {
        &self.current_version_id
    }

    pub fn discussion_thread_id(&self) -> &u64 {
        &self.discussion_thread_id
    }

    pub fn price(&self) -> &f64 {
        &self.price
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn download_count(&self) -> &u64 {
        &self.download_count
    }

    pub fn review_count(&self) -> &u64 {
        &self.review_count
    }

    pub fn review_average(&self) -> &f64 {
        &self.review_average
    }
}

#[derive(Deserialize)]
pub struct Download {
    resource_id: u64,
    version_id: u64,
    downloader_id: u64,
    download_date: u64,
}

#[derive(Deserialize)]
pub struct Review {
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

#[derive(Deserialize)]
pub struct Update {
    update_id: u64,
    title: String,
    message: String,
    deleted: Option<bool>,
    update_date: u64,
    likes: u64,
}

#[derive(Deserialize)]
pub struct Version {
    version_id: u64,
    update_id: u64,
    name: String,
    deleted: Option<bool>,
    release_date: u64,
    download_count: u64,
}

#[derive(Deserialize)]
pub struct License {
    license_id: u64,
    purchaser_id: u64,
    validated: bool,
    active: bool,
    start_date: u64,
    end_date: u64,
    previous_end_date: u64,
}

#[derive(Deserialize)]
pub struct Purchase {
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
