use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Rate limits are not currently disclosed - default to 0.
const READ_BURST_LIMIT: u8 = 0;
const READ_NORMAL_LIMIT: u8 = 0;

const WRITE_BURST_LIMIT: u8 = 0;
const WRITE_NORMAL_LIMIT: u8 = 0;

pub enum RequestType { READ, WRITE }

/// A strucutre for storing the relevant atomic values in order to track rate limiting values.
/// 
/// # Note
/// We cannot currently store atomic u128s as the API is not stable yet; plan to switch over once it is so we can
/// provide sub-second stall durations.
#[derive(Default)]
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

/// Compute how long, if at all, we should stall the next request in order to stay within rate limiting rules.
pub fn stall_for(store: &RateLimitStore, request_type: RequestType) -> u64 {
    let time = unix_timestamp();
    let mut stall_for = 0;

    if let RequestType::READ = request_type {
        // Throttle burst
        let count = store.read_burst_count.load(Ordering::Acquire);
        let refresh = store.read_burst_refresh.load(Ordering::Acquire);

        if refresh == time {
            if count >= READ_BURST_LIMIT {
                stall_for = std::cmp::max(time, 1);
            }
        } else {
            store.read_burst_count.store(0, Ordering::Release);
            store.read_burst_refresh.store(time, Ordering::Release);
        }

        // Thorttle normal
        let count = store.read_normal_count.load(Ordering::Acquire);
        let refresh = store.read_normal_refresh.load(Ordering::Acquire);

        if (refresh + 60) >= time {
            if count >= READ_NORMAL_LIMIT {
                stall_for = std::cmp::max(time, (refresh + 60) - time);
            }
        } else {
            store.read_normal_count.store(0, Ordering::Release);
            store.read_normal_refresh.store(time, Ordering::Release);
        }
    }

    if let RequestType::WRITE = request_type {
        // Throttle burst
        let count = store.write_burst_count.load(Ordering::Acquire);
        let refresh = store.write_burst_refresh.load(Ordering::Acquire);

        if refresh == time {
            if count >= WRITE_BURST_LIMIT {
                stall_for = std::cmp::max(time, 1);
            }
        } else {
            store.write_burst_count.store(0, Ordering::Release);
            store.write_burst_refresh.store(time, Ordering::Release);
        }

        // Thorttle normal
        let count = store.write_normal_count.load(Ordering::Acquire);
        let refresh = store.write_normal_refresh.load(Ordering::Acquire);

        if (refresh + 60) >= time {
            if count >= WRITE_NORMAL_LIMIT {
                stall_for = std::cmp::max(time, (refresh + 60) - time);
            }
        } else {
            store.write_normal_count.store(0, Ordering::Release);
            store.write_normal_refresh.store(time, Ordering::Release);
        }
    }

    stall_for
}

/// Return the current time as a UNIX second timestamp.
pub fn unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
