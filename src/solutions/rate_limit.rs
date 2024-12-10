use std::time::Duration;

use crate::core::services::{RateLimitService, TryAcquire};

pub struct RateLimiter {
    pub inner: leaky_bucket::RateLimiter,
}

impl RateLimitService for RateLimiter {}

impl TryAcquire for RateLimiter {
    fn try_acquire(&self) -> bool {
        self.inner.try_acquire(1)
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            inner: leaky_bucket::RateLimiter::builder()
                .initial(5)
                .max(5)
                .interval(Duration::from_secs(1))
                .build(),
        }
    }
}
