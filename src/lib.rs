// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

//! # mcm_api_wrapper
//!
//! An asynchronous Rust wrapper for MC-Market's [HTTP API](https://www.mc-market.org/wiki/ultimate-api/).
//!
//! ## Features
//! - Built on reqwest/hyper - a fast and correct HTTP implementation.
//! - Full coverage of the API with a fully asynchronous design using the tokio runtime.
//! - Requests are queued and may be dynamically delayed to stay within rate limiting rules.
//!
//! [Read more.](https://github.com/Majored/rs-mcm-api-wrapper)

pub mod data;
pub mod error;
pub mod helpers;
pub mod sort;
pub(crate) mod http;
pub(crate) mod throttler;

use data::metrics::MetricsSnapshot;
use error::{APIError, Result};
use helpers::alerts::AlertsHelper;
use helpers::resources::ResourceHelper;
use helpers::conversations::ConversationsHelper;
use helpers::members::MembersHelper;
use helpers::threads::ThreadsHelper;
use throttler::RateLimitStore;
use sort::SortOptions;

use std::time::{Duration, Instant};

use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{de::DeserializeOwned, Serialize, Deserialize};

/// The base API URL and version which will be prepended to all endpoints.
pub(crate) const BASE_URL: &str = "https://api.mc-market.org/v1";

/// An enum representing the two possible API token types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIToken {
    Private(String),
    Shared(String),
}

impl APIToken {
    /// Returns the token as an expected 'Authorization' header value.
    pub(crate) fn as_header(&self) -> String {
        match self {
            APIToken::Private(value) => format!("Private {}", value),
            APIToken::Shared(value) => format!("Shared {}", value),
        }
    }
}

/// The primary wrapping type for interactions with MC-Market's API.
pub struct APIWrapper {
    pub(crate) http_client: Client,
    pub(crate) rate_limit_store: RateLimitStore,
}

impl APIWrapper {
    /// Construct a new API wrapper instance.
    ///
    /// # Note
    /// During the construction process, we make a request to the `health` endpoint which we expect to always succeed
    /// under nominal conditions. If the request does fail, we expect subsequent requests to other endpoint to also
    /// fail so we conclude that a construction failure has occured.
    ///
    /// # Example
    /// ```
    /// let token = APIToken::Private(String::from("y6xWrGkAzh8Gp4qBWFMG7tDyB+zB+Lub"));
    /// let wrapper = APIWrapper::build(token).await.unwrap();
    ///
    /// println!("Successfully connected to the API.");
    /// ```
    pub async fn new(token: APIToken) -> Result<APIWrapper> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Authorization", token.as_header().parse().expect("token not a valid HeaderValue"));

        let http_client = ClientBuilder::new().https_only(true).default_headers(default_headers).build().expect("http client build failed");

        let wrapper = APIWrapper { http_client, rate_limit_store: RateLimitStore::new() };
        wrapper.health().await?;

        Ok(wrapper)
    }

    /// A raw function which makes a GET request to a specific endpoint.
    async fn get<D>(&self, endpoint: &str, sort: Option<&SortOptions<'_>>) -> Result<D>
    where
        D: DeserializeOwned,
    {
        if sort.is_some() {
            let endpoint = format!("{}?{}", endpoint, &sort.unwrap().to_query_string()?);
            http::get(self, &endpoint).await?.as_result()
        } else {
            http::get(self, endpoint).await?.as_result()
        }
    }

    /// A raw function which makes a POST request to a specific endpoint.
    async fn post<D, B>(&self, endpoint: &str, body: &B) -> Result<D>
    where
        D: DeserializeOwned,
        B: Serialize,
    {
        http::post(self, endpoint, body).await?.as_result()
    }

    /// A raw function which makes a PATCH request to a specific endpoint.
    async fn patch<D, B>(&self, endpoint: &str, body: &B) -> Result<D>
    where
        D: DeserializeOwned,
        B: Serialize,
    {
        http::patch(self, endpoint, body).await?.as_result()
    }

    /// A raw function which makes a DELETE request to a specific endpoint.
    async fn delete<D>(&self, endpoint: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        http::delete(self, endpoint).await?.as_result()
    }

    /// Schedule an empty request which we expect to always succeed under nominal conditions.
    ///
    /// # Example
    /// ```
    /// wrapper.health().await?;
    /// println!("Received a successful response from the API.");
    /// ```
    pub async fn health(&self) -> Result<()> {
        let data: String = self.get(&format!("{}/health", BASE_URL), None).await?;

        if data != "ok" {
            return Err(APIError::from_raw("HealthEndpointError".to_string(), format!("{} != \"ok\"", data)));
        }

        Ok(())
    }

    /// Schedule an empty request and measure how long the API took to respond.
    ///
    /// # Note
    /// This duration may not be representative of the raw request latency due to the fact that requests may be stalled
    /// locally within this wrapper to ensure compliance with rate limiting rules. Whilst this is a trade-off, it can
    /// be argued that the returned duration will be more representative of the true latencies experienced.
    ///
    /// # Example
    /// ```
    /// println!("Took {}ms for the API to respond.", wrapper.ping().await?.as_millis());
    /// ```
    pub async fn ping(&self) -> Result<Duration> {
        let time = Instant::now();
        self.health().await?;
        Ok(time.elapsed())
    }

    /// Fetch a snapshot of metrics values from the prior minute along with refresh interval metadata.
    ///
    /// # Note
    /// This function is intended to be polled once a minute and the values averaged to provide a clear and accurate
    /// picture of the API's current load. As a result of its purpose, the relevant endpoint (and thus, this method)
    /// is only accessible to staff members.
    pub async fn metrics(&self) -> Result<MetricsSnapshot> {
        self.get(&format!("{}/metrics", BASE_URL), None).await
    }

    /// Construct and return a resource helper type wrapping this instance.
    pub fn resources(&self) -> ResourceHelper<'_> {
        ResourceHelper { wrapper: self }
    }

    /// Construct and return an alert helper type wrapping this instance.
    pub fn alerts(&self) -> AlertsHelper<'_> {
        AlertsHelper { wrapper: self }
    }

    /// Construct and return a conversation helper type wrapping this instance.
    pub fn conversations(&self) -> ConversationsHelper<'_> {
        ConversationsHelper { wrapper: self }
    }

    /// Construct and return a thread helper type wrapping this instance.
    pub fn threads(&self) -> ThreadsHelper<'_> {
        ThreadsHelper { wrapper: self }
    }

    /// Construct and return a member helper type wrapping this instance.
    pub fn members(&self) -> MembersHelper<'_> {
        MembersHelper { wrapper: self }
    }
}
