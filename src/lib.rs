// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

pub mod error;
pub(crate) mod http;
pub mod structs;
pub(crate) mod throttler;

use error::{APIError, Result};
use structs::alerts::Alert;
use structs::conversations::Conversation;
use structs::members::Member;
use structs::metrics::MetricsSnapshot;
use structs::resources::Resource;
use throttler::RateLimitStore;

use std::time::{Duration, Instant};

use reqwest::{Client, ClientBuilder, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};

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
    /// Build an API wrapper instance from a raw API token.
    ///
    /// # Note
    /// During the build process, we make a request to the `health` endpoint which we expect to always succeed under
    /// nominal conditions. If the request does fail, we expect subsequent requests to other endpoint to also fail so
    /// we conclude that a build failure has occured.
    ///
    /// # Example
    /// ```
    /// let token = APIToken::Private(String::from("y6xWrGkAzh8Gp4qBWFMG7tDyB+zB+Lub"));
    /// let wrapper = APIWrapper::build(token).await.unwrap();
    ///
    /// println!("Successfully connected to the API.");
    /// ```
    pub async fn build(token: APIToken) -> Result<APIWrapper> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Authorization", token.as_header().parse().unwrap());

        let http_client = ClientBuilder::new().https_only(false).default_headers(default_headers).build().unwrap();

        let wrapper = APIWrapper { http_client, rate_limit_store: RateLimitStore::new() };
        wrapper.health().await?;

        Ok(wrapper)
    }

    /// A raw function which makes a GET request to a specific endpoint.
    async fn get<D>(&self, endpoint: &str) -> Result<D> where D: DeserializeOwned {
        http::read(self, endpoint).await?.as_result()
    }

    /// A raw function which makes a POST request to a specific endpoint.
    async fn post<D, B>(&self, endpoint: &str, body: &B) -> Result<D> where D: DeserializeOwned, B: Serialize {
        http::write(self, endpoint, body, true).await?.as_result()
    }

    /// A raw function which makes a PATCH request to a specific endpoint.
    async fn patch<D, B>(&self, endpoint: &str, body: &B) -> Result<D> where D: DeserializeOwned, B: Serialize {
        http::write(self, endpoint, body, false).await?.as_result()
    }

    /// Schedule a plain request which we expect to always succeed under nominal conditions.
    ///
    /// # Example
    /// ```
    /// wrapper.health().await?;
    /// println!("Received a successful response from the API.");
    /// ```
    pub async fn health(&self) -> Result<()> {
        let data: String = self.get(format!("{}/health", BASE_URL)).await?;

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
        self.get(format!("{}/metrics", BASE_URL)).await
    }

    /// Fetch detailed information about a member.
    ///
    /// # Note
    /// The Member structure contains three Option fields. The values of these may be None if the member has chosen to
    /// hide these fields from public view, or if they've disabled their account.
    ///
    /// # Example
    /// ```
    /// let member = wrapper.fetch_member(87939).await?;
    /// assert_eq!("Harry", member.username());
    /// ```
    pub async fn fetch_member(&self, member_id: u64) -> Result<Member> {
        self.get(format!("{}/members/{}", BASE_URL, member_id)).await
    }

    /// Fetch detailed information about yourself.
    ///
    /// # Note
    /// The Member structure contains three Option fields. However, when fetching information about yourself, only the
    /// `gender` field may be None if you've selected your gender as 'unspecified'.
    ///
    /// # Example
    /// ```
    /// let member = wrapper.fetch_self().await?;
    /// assert!(!member.banned());
    /// ```
    pub async fn fetch_self(&self) -> Result<Member> {
        self.get(format!("{}/members/self", BASE_URL)).await
    }

    /// Fetch a list of unread alerts.
    ///
    /// # Example
    /// ```
    /// let tagged_in = wrapper.fetch_alerts().await?.iter().filter(|alert| alert.alert_type() == "tag");
    /// ```
    pub async fn fetch_alerts(&self) -> Result<Vec<Alert>> {
        self.get(format!("{}/alerts", BASE_URL)).await
    }

    /// Fetch a list of unread conversations.
    ///
    /// # Example
    /// ```
    /// let open_unread = wrapper.fetch_conversations().await?.iter().filter(|conversation| conversation.open());
    /// ```
    pub async fn fetch_conversations(&self) -> Result<Vec<Conversation>> {
        self.get(format!("{}/conversations", BASE_URL)).await
    }

    /// Construct a Resource and fetch detailed information about it.
    ///
    /// # Note
    /// This is a helper function and is equivalent to:
    /// ```
    /// Resource::from_raw_fetch_data(wrapper, identifier)
    /// ```
    ///
    /// # Example
    /// ```
    /// let resource = wrapper.fetch_resource(16682).await?;
    /// assert!(resource.has_data());
    /// ```
    pub async fn fetch_resource(&self, resource_id: u64) -> Result<Resource<'_>> {
        Resource::from_raw_fetch_data(self, resource_id).await
    }
}
