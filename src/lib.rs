// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

pub mod data;
pub mod error;
pub mod helpers;
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

use std::time::{Duration, Instant};

use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{de::DeserializeOwned, Serialize};

/// The base API URL and version which will be prepended to all endpoints.
pub const BASE_URL: &str = "https://api.mc-market.org/v1";

/// An enum representing the two possible API token types.
pub enum APIToken {
    Private(String),
    Shared(String),
}

impl APIToken {
    pub(crate) fn as_header(&self) -> String {
        match self {
            APIToken::Private(value) => format!("Private {}", value),
            APIToken::Shared(value) => format!("Shared {}", value),
        }
    }
}

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
    /// fail so we conclude that a build failure has occured.
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
        default_headers.insert("Authorization", token.as_header().parse().unwrap());

        let http_client = ClientBuilder::new().https_only(true).default_headers(default_headers).build().unwrap();

        let wrapper = APIWrapper { http_client, rate_limit_store: RateLimitStore::new() };
        wrapper.health().await?;

        Ok(wrapper)
    }

    /// A raw function which makes a GET request to a specific endpoint.
    async fn get<D>(&self, endpoint: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        http::get(self, endpoint).await?.as_result()
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

    /// Schedule a plain request which we expect to always succeed under nominal conditions.
    ///
    /// # Example
    /// ```
    /// wrapper.health().await?;
    /// println!("Received a successful response from the API.");
    /// ```
    pub async fn health(&self) -> Result<()> {
        let data: String = self.get(&format!("{}/health", BASE_URL)).await?;

        if data != "ok" {
            return Err(APIError::from_raw("HealthEndpointError".to_string(), format!("{} != \"ok\"", data)));
        }

        Ok(())
    }

    /// Schedule a plain request and measure how long the API took to respond.
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
    /// picture of the API's current load. As a result of its purpose, the relevant endpoint (and thus, this function)
    /// is only accessible to staff members.
    ///
    /// # Example
    /// ```
    /// let mut connections: HashMap<u64, u64> = HashMap::new();
    /// let mut interval = tokio::time::interval(Duration::from_milis(1000));
    ///
    /// loop {
    ///     interval.tick().await;
    ///
    ///     let metrics = wrapper.fetch_metrics().await?;
    ///     connections.insert(metrics.interval().last(), metrics.get_metric("connections").unwrap());
    /// }
    /// ```
    pub async fn fetch_metrics(&self) -> Result<MetricsSnapshot> {
        self.get(&format!("{}/metrics", BASE_URL)).await
    }

    pub fn resources(&self) -> ResourceHelper<'_> {
        ResourceHelper { wrapper: self }
    }

    pub fn alerts(&self) -> AlertsHelper<'_> {
        AlertsHelper { wrapper: self }
    }

    pub fn conversations(&self) -> ConversationsHelper<'_> {
        ConversationsHelper { wrapper: self }
    }

    pub fn threads(&self) -> ThreadsHelper<'_> {
        ThreadsHelper { wrapper: self }
    }

    pub fn members(&self) -> MembersHelper<'_> {
        MembersHelper { wrapper: self }
    }
}
