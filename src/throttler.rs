use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

pub(crate) enum RequestType { READ, WRITE }

/// A strucutre for storing the relevant atomic values in order to track our compliance with the API's rate limits.
pub(crate) struct RateLimitStore {
    pub(crate) read_last_retry: AtomicU64,
    pub(crate) read_last_request: AtomicU64,

    pub(crate) write_last_retry: AtomicU64,
    pub(crate) write_last_request: AtomicU64,
}

impl RateLimitStore {
    pub fn new() -> Self {
        RateLimitStore {
            read_last_retry: AtomicU64::new(0),
            read_last_request: AtomicU64::new(unix_timestamp()),
        
            write_last_retry: AtomicU64::new(0),
            write_last_request: AtomicU64::new(unix_timestamp()),
        }
    }
}

/// Compute how long, if at all, we should stall the next request in order to be compliant with rate limiting.
/// 
/// Returned value is in milliseconds. A value of 0 indiciates that there's no need to stall the calling request.
pub(crate) fn stall_for(store: &RateLimitStore, request_type: RequestType) -> u64 {
    let time = unix_timestamp();
    let mut stall_for = 0;

    if let RequestType::READ = request_type {
        stall_for = stall_for_helper(&store.read_last_retry, &store.read_last_request, time);
    }
    if let RequestType::WRITE = request_type {
        stall_for = stall_for_helper(&store.write_last_retry, &store.write_last_request, time);
    }

    stall_for
}

/// A helper function for `stall_for` which computes over a generic set of rate limiting parameters.
fn stall_for_helper(a_last_retry: &AtomicU64, a_last_request: &AtomicU64, time: u64) -> u64 {
    let mut stall_for = 0;
    let last_retry = a_last_retry.load(Ordering::Acquire);
    let last_request = a_last_request.load(Ordering::Acquire);

    if last_retry > 0 && (time - last_request) < last_retry {
        stall_for = last_retry - (time - last_request);
    }

    stall_for
}

/// Return the current time as a UNIX millisecond timestamp.
pub(crate) fn unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().try_into().unwrap()
}
