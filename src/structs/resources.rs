// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

use crate::data::resources::{ResourceData, DownloadData, ReviewData, UpdateData, VersionData, LicenseData, PurchaseData};
use crate::error::Result;
use crate::APIWrapper;

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
        Self { wrapper, identifier, data: None }
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
    pub async fn from_raw_fetch_data(wrapper: &'a APIWrapper, identifier: u64) -> Result<Resource<'a>> {
        let mut resource = Resource::from_raw(wrapper, identifier);
        resource.fetch_data().await?;
        Ok(resource)
    }

    pub async fn fetch_data(&mut self) -> Result<()> {
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

    pub async fn list_reviews(&self) -> Result<Vec<ReviewData>> {
        self.wrapper.get(format!("{}/resources/{}/reviews", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_downloads(&self) -> Result<Vec<DownloadData>> {
        self.wrapper.get(format!("{}/resources/{}/downloads", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_licenses(&self) -> Result<Vec<LicenseData>> {
        self.wrapper.get(format!("{}/resources/{}/licenses", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_purchases(&self) -> Result<Vec<PurchaseData>> {
        self.wrapper.get(format!("{}/resources/{}/purchases", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_versions(&self) -> Result<Vec<VersionData>> {
        self.wrapper.get(format!("{}/resources/{}/versions", crate::BASE_URL, self.identifier)).await
    }

    pub async fn list_updates(&self) -> Result<Vec<UpdateData>> {
        self.wrapper.get(format!("{}/resources/{}/updates", crate::BASE_URL, self.identifier)).await
    }
}
