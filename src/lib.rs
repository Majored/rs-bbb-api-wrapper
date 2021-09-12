// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

pub mod error;
pub mod structs;
pub mod throttler;

use error::APIError;
use structs::alerts::Alert;
use structs::conversations::Conversation;
use structs::members::Member;
use structs::metrics::MetricsSnapshot;
use structs::resources::Resource;
use throttler::RateLimitStore;

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use log::debug;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;

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

/// A structure representing a parsed response from the API.
#[derive(Deserialize)]
pub struct APIResponse<D> {
    result: String,
    data: Option<D>,
    error: Option<APIError>,
}

impl<D> APIResponse<D> {
    /// Returns whether or not the response was successful.
    ///
    /// If true, a call to get_data() will not panic.
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }

    /// Returns whether or not the response was errored.
    ///
    /// If true, a call to get_error() will not panic.
    pub fn is_error(&self) -> bool {
        self.result == "error"
    }

    /// Returns the containing data within the response.
    ///
    /// Will panic if the response was not successful.
    pub fn get_data(self) -> D {
        self.data.unwrap()
    }

    /// Returns the containing error within the response.
    ///
    /// Will panic if the response was successful.
    pub fn get_error(self) -> APIError {
        self.error.unwrap()
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
    pub async fn build(token: APIToken) -> Result<APIWrapper, APIError> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Authorization", token.as_header().parse().unwrap());

        let http_client = ClientBuilder::new()
            .https_only(false)
            .default_headers(default_headers)
            .build()
            .unwrap();

        let wrapper = APIWrapper {
            http_client,
            rate_limit_store: RateLimitStore::new(),
        };
        wrapper.health().await?;

        Ok(wrapper)
    }

    /// A raw function which makes a GET request to a specific endpoint.
    async fn get<D: DeserializeOwned>(&self, endpoint: String) -> Result<D, APIError> {
        // As we need to be able to resend the request if we hit a rate limit, we need to either:
        // - use a loop
        // - use as async recursive function
        //
        // The latter would require the addition of an indirection via a boxed future due to infinitely-sized types.
        // This approach lacks consistency with the rest of this wrapper and is harder to maintain. We've gone with the
        // former where the outer loop controls the request retry, and the inner loop controls the stalling retry.

        loop {
            loop {
                match throttler::stall_for(&self.rate_limit_store, throttler::RequestType::READ) {
                    0 => break,
                    stall_for => {
                        debug!("Stalling request for {}ms to stay within rate limit.", stall_for);
                        tokio::time::sleep(Duration::from_millis(stall_for)).await;
                    }
                };
            }

            let response = match self.http_client.get(&endpoint).send().await {
                Ok(response) => response,
                Err(error) => {
                    return Err(APIError::from_raw(
                        "HttpClientError".to_string(),
                        format!("Unable to parse successful response: {}", error),
                    ));
                }
            };

            match response.status() {
                StatusCode::TOO_MANY_REQUESTS => {
                    let retry: u64 = response
                        .headers()
                        .get("Retry-After")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .parse()
                        .unwrap();

                    self.rate_limit_store.read_last_retry.store(retry, Ordering::Release);
                    self.rate_limit_store
                        .read_last_request
                        .store(throttler::unix_timestamp(), Ordering::Release);
                }
                _ => {
                    self.rate_limit_store.read_last_retry.store(0, Ordering::Release);
                    self.rate_limit_store
                        .read_last_request
                        .store(throttler::unix_timestamp(), Ordering::Release);

                    return APIWrapper::handle_response(response).await;
                }
            };
        }
    }

    async fn handle_response<D: DeserializeOwned>(response: Response) -> Result<D, APIError> {
        let status_code = response.status();
        let response: APIResponse<D> = match response.json().await {
            Ok(response) => response,
            Err(error) => {
                if status_code == StatusCode::OK {
                    return Err(APIError::from_raw(
                        "SuccessResponseParseError".to_string(),
                        format!("Unable to parse successful response: {}", error),
                    ));
                } else {
                    return Err(APIError::from_raw(
                        "ErrorResponseParseError".to_string(),
                        format!("Unable to parse error response: {}", error),
                    ));
                }
            }
        };

        if response.is_success() {
            Ok(response.get_data())
        } else {
            Err(response.get_error())
        }
    }

    /// Schedule a plain request which we expect to always succeed under nominal conditions.
    ///
    /// # Example
    /// ```
    /// wrapper.health().await?;
    /// println!("Received a successful response from the API.");
    /// ```
    pub async fn health(&self) -> Result<(), APIError> {
        let data: String = self.get(format!("{}/health", BASE_URL)).await?;

        if data != "ok" {
            return Err(APIError::from_raw(
                "HealthEndpointError".to_string(),
                format!("{} != \"ok\"", data),
            ));
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
    pub async fn ping(&self) -> Result<Duration, APIError> {
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
    pub async fn fetch_metrics(&self) -> Result<MetricsSnapshot, APIError> {
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
    pub async fn fetch_member(&self, member_id: u64) -> Result<Member, APIError> {
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
    pub async fn fetch_self(&self) -> Result<Member, APIError> {
        self.get(format!("{}/members/self", BASE_URL)).await
    }

    /// Fetch a list of unread alerts.
    ///
    /// # Example
    /// ```
    /// let tagged_in = wrapper.fetch_alerts().await?.iter().filter(|alert| alert.alert_type() == "tag");
    /// ```
    pub async fn fetch_alerts(&self) -> Result<Vec<Alert>, APIError> {
        self.get(format!("{}/alerts", BASE_URL)).await
    }

    /// Fetch a list of unread conversations.
    ///
    /// # Example
    /// ```
    /// let open_unread = wrapper.fetch_conversations().await?.iter().filter(|conversation| conversation.open());
    /// ```
    pub async fn fetch_conversations(&self) -> Result<Vec<Conversation>, APIError> {
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
    pub async fn fetch_resource(&self, resource_id: u64) -> Result<Resource<'_>, APIError> {
        Resource::from_raw_fetch_data(self, resource_id).await
    }
}
