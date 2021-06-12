// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

pub mod error;

use error::APIError;

use reqwest::{Response, Client, ClientBuilder, StatusCode};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://api.mc-market.org/v1";

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

    pub async fn health(&self) -> Result<(), APIError> {
        let data: String = self.get("/health").await?;

        if data == "ok" {
            Ok(())
        } else {
            Err(APIError::from_raw("HealthEndpointError".to_string(), format!("{} != \"ok\"", data)))
        }
    }
}