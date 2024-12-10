pub trait RateLimitService {}

pub trait TryAcquire: RateLimitService {
    fn try_acquire(&self) -> bool;
}
