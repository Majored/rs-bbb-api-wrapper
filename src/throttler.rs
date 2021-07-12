use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

// Rate limits are not currently disclosed - default to 0 for the moment.
const READ_BURST_LIMIT: u8 = 0;
const READ_NORMAL_LIMIT: u8 = 0;

const WRITE_BURST_LIMIT: u8 = 0;
const WRITE_NORMAL_LIMIT: u8 = 0;

pub enum RequestType { READ, WRITE }

/// A strucutre for storing the relevant atomic values in order to track our compliance with the API's rate limits.
pub struct RateLimitStore {
    pub(crate) read_burst_count: AtomicU8,
    pub(crate) read_burst_refresh: AtomicU64,
    pub(crate) read_normal_count: AtomicU8,
    pub(crate) read_normal_refresh: AtomicU64,

    pub(crate) write_burst_count: AtomicU8,
    pub(crate) write_burst_refresh: AtomicU64,
    pub(crate) write_normal_count: AtomicU8,
    pub(crate) write_normal_refresh: AtomicU64,
}

impl RateLimitStore {
    pub fn new() -> Self {
        RateLimitStore {
            read_burst_count: AtomicU8::new(0),
            read_burst_refresh: AtomicU64::new(unix_timestamp()),
            read_normal_count: AtomicU8::new(0),
            read_normal_refresh: AtomicU64::new(unix_timestamp()),
        
            write_burst_count: AtomicU8::new(0),
            write_burst_refresh: AtomicU64::new(unix_timestamp()),
            write_normal_count: AtomicU8::new(0),
            write_normal_refresh: AtomicU64::new(unix_timestamp()),
        }
    }
}

/// Compute how long, if at all, we should stall the next request in order to be compliant with rate limiting.
/// 
/// Returned value is in milliseconds. A value of 0 indiciates that there's no need to stall the calling request.
pub fn stall_for(store: &RateLimitStore, request_type: RequestType) -> u64 {
    let time = unix_timestamp();
    let mut stall_for = 0;

    if let RequestType::READ = request_type {
        // Throttle burst
        stall_for = std::cmp::max(
            stall_for,
            stall_for_helper(&store.read_burst_count, &store.read_burst_refresh, time, 1000, READ_BURST_LIMIT),
        );

        // Throttle normal
        stall_for = std::cmp::max(
            stall_for,
            stall_for_helper(&store.read_normal_count, &store.read_normal_refresh, time, 60000, READ_NORMAL_LIMIT),
        );
    }

    if let RequestType::WRITE = request_type {
        // Throttle burst
        stall_for = std::cmp::max(
            stall_for,
            stall_for_helper(&store.write_burst_count, &store.write_burst_refresh, time, 1000, WRITE_BURST_LIMIT),
        );

        // Throttle normal
        stall_for = std::cmp::max(
            stall_for,
            stall_for_helper(&store.write_normal_count, &store.write_normal_refresh, time, 60000, WRITE_NORMAL_LIMIT),
        );
    }

    stall_for
}

/// A helper function for `stall_for` which computes over a generic set of rate limiting parameters.
fn stall_for_helper(a_count: &AtomicU8, a_refresh: &AtomicU64, time: u64, interval: u64, max: u8) -> u64 {
    let mut stall_for = 0;
    let count = a_count.load(Ordering::Acquire);
    let refresh = a_refresh.load(Ordering::Acquire);

    if (refresh + interval) >= time {
        if count >= max {
            stall_for = std::cmp::max(stall_for, (refresh + interval) - time);
        }
    } else {
        a_count.store(0, Ordering::Release);
        a_refresh.store(time, Ordering::Release);
    }

    stall_for
}

/// Return the current time as a UNIX millisecond timestamp.
pub fn unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().try_into().unwrap()
}
