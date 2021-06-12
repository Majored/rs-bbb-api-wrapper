// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

pub mod error;
pub mod structs;

use std::time::{Instant, Duration};

use error::APIError;
use structs::members::Member;
use structs::metrics::MetricsSnapshot;

use reqwest::{Response, Client, ClientBuilder, StatusCode};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub const BASE_URL: &str = "https://api.mc-market.org/v1";

pub enum APIToken {
    Private(String),
    Shared(String),
}

impl APIToken {
    pub fn as_header(&self) -> String {
        match self {
            APIToken::Private(value) => format!("Private {}", value),
            APIToken::Shared(value) => format!("Shared {}", value),
        }
    }
}

#[derive(Deserialize)]
pub struct APIResponse<D> {
    result: String,
    data: Option<D>,
    error: Option<APIError>
}

impl<D> APIResponse<D> {
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }

    pub fn is_error(&self) -> bool {
        self.result == "error"
    }

    pub fn get_data(self) -> D {
        self.data.unwrap()
    }

    pub fn get_error(self) -> APIError {
        self.error.unwrap()
    }
}

pub struct APIWrapper {
    http_client: Client,
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

        let http_client = ClientBuilder::new().https_only(true).default_headers(default_headers).build().unwrap();

        let wrapper = APIWrapper { http_client };
        wrapper.health().await?;

        Ok(wrapper)
    }

    async fn get<D: DeserializeOwned>(&self, endpoint: &str) -> Result<D, APIError> {
        let response = match self.http_client.get(format!("{}{}", BASE_URL, endpoint)).send().await {
            Ok(response) => response,
            Err(error) => {
                return Err(APIError::from_raw(
                    "HttpClientError".to_string(),
                    format!("Unable to parse successful response: {}", error)
                ));
            }
        };

        APIWrapper::handle_response(response).await
    }

    async fn handle_response<D: DeserializeOwned>(response: Response) -> Result<D, APIError> {
        let status_code = response.status();
        let response: APIResponse<D> = match response.json().await {
            Ok(response) => response,
            Err(error) => {
                if status_code == StatusCode::OK {
                    return Err(APIError::from_raw(
                        "SuccessResponseParseError".to_string(),
                        format!("Unable to parse successful response: {}", error)
                    ));
                } else {
                    return Err(APIError::from_raw(
                        "ErrorResponseParseError".to_string(),
                        format!("Unable to parse error response: {}", error)
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
        let data: String = self.get("/health").await?;

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
        self.get("/metrics").await
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
        self.get(&format!("/members/{}", member_id)[..]).await
    }
}