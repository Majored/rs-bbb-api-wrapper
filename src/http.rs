// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/mcm-rust-api-wrapper/blob/main/LICENSE)

//! As we need to be able to resend the request if we hit a rate limit, we need to either:
//! - use a loop
//! - use as async recursive function
//!
//! The latter would require the addition of an indirection via a boxed future due to infinitely-sized types. This
//! approach lacks consistency with the rest of this wrapper and is harder to maintain. We've gone with the former
//! where the outer loop controls the request retry, and the inner loop controls the stalling retry.

use crate::error::APIError;
use crate::error::Result;
use crate::throttler::{RateLimitStore, RequestType};
use crate::APIWrapper;

use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::time::Duration;

/// A structure representing a parsed response from the API.
#[derive(Deserialize)]
pub struct APIResponse<D> {
    pub result: String,
    pub data: Option<D>,
    pub error: Option<APIError>,
}

impl<D> APIResponse<D> {
    /// Returns whether or not the response was successful.
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }

    /// Returns whether or not the response was errored.
    pub fn is_error(&self) -> bool {
        self.result == "error"
    }

    /// Returns the containing data within the response.
    ///
    /// Will panic if the response was not successful.
    pub fn data(self) -> D {
        self.data.expect("no data present")
    }

    /// Returns the containing data within the response.
    ///
    /// Will panic if the response was not successful.
    pub fn data_ref(&self) -> &D {
        self.data.as_ref().expect("no data present")
    }

    /// Returns the containing error within the response.
    ///
    /// Will panic if the response was successful.
    pub fn error(self) -> APIError {
        self.error.expect("no error present")
    }

    /// Returns the containing error within the response.
    ///
    /// Will panic if the response was successful.
    pub fn error_ref(&self) -> &APIError {
        self.error.as_ref().expect("no error present")
    }

    pub fn as_result(self) -> Result<D> {
        if self.is_success() {
            Ok(self.data())
        } else {
            Err(self.error())
        }
    }
}

pub async fn get<D>(wrapper: &APIWrapper, endpoint: &str) -> Result<APIResponse<D>> where D: DeserializeOwned {
    loop {
        loop {
            match crate::throttler::stall_for(&wrapper.rate_limit_store, RequestType::READ) {
                0 => break,
                stall_for => tokio::time::sleep(Duration::from_millis(stall_for)).await,
            };
        }

        let response = wrapper.http_client.get(endpoint).send().await?;

        if !did_hit_limit(&wrapper.rate_limit_store, &response, RequestType::READ) {
            return response.json().await?;
        }
    }
}

pub async fn post<D, B>(wrapper: &APIWrapper, endpoint: &str, body: &B) -> Result<APIResponse<D>>
where
    D: DeserializeOwned,
    B: Serialize,
{
    loop {
        loop {
            match crate::throttler::stall_for(&wrapper.rate_limit_store, RequestType::WRITE) {
                0 => break,
                stall_for => tokio::time::sleep(Duration::from_millis(stall_for)).await,
            };
        }

        let response = wrapper.http_client.post(endpoint).json(body).send().await?;

        if !did_hit_limit(&wrapper.rate_limit_store, &response, RequestType::WRITE) {
            return response.json().await?;
        }
    }
}

pub async fn patch<D, B>(wrapper: &APIWrapper, endpoint: &str, body: &B) -> Result<APIResponse<D>>
where
    D: DeserializeOwned,
    B: Serialize,
{
    loop {
        loop {
            match crate::throttler::stall_for(&wrapper.rate_limit_store, RequestType::WRITE) {
                0 => break,
                stall_for => tokio::time::sleep(Duration::from_millis(stall_for)).await,
            };
        }

        let response = wrapper.http_client.post(endpoint).json(body).send().await?;

        if !did_hit_limit(&wrapper.rate_limit_store, &response, RequestType::WRITE) {
            return response.json().await?;
        }
    }
}

pub async fn delete<D>(wrapper: &APIWrapper, endpoint: &str) -> Result<APIResponse<D>> where D: DeserializeOwned {
    loop {
        loop {
            match crate::throttler::stall_for(&wrapper.rate_limit_store, RequestType::WRITE) {
                0 => break,
                stall_for => tokio::time::sleep(Duration::from_millis(stall_for)).await,
            };
        }

        let response = wrapper.http_client.delete(endpoint).send().await?;

        if !did_hit_limit(&wrapper.rate_limit_store, &response, RequestType::WRITE) {
            return response.json().await?;
        }
    }
}

fn did_hit_limit(store: &RateLimitStore, response: &Response, request_type: RequestType) -> bool {
    if response.status() != StatusCode::TOO_MANY_REQUESTS {
        match &request_type {
            RequestType::READ => store.reset_read(),
            RequestType::WRITE => store.reset_write(),
        };

        return false;
    }

    let retry = response.headers().get("Retry-After").expect("no retry-after header present");
    let retry: u64 = retry.to_str().expect("non-ascii characters present").parse().expect("not a valid u64 int");

    match &request_type {
        RequestType::READ => store.store_read(retry),
        RequestType::WRITE => store.store_write(retry),
    };

    true
}
