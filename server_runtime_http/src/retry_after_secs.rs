#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct RetryAfterSecs(std::num::NonZeroU64);

impl RetryAfterSecs {
    pub(crate) const fn get(self) -> u64 {
        self.0.get()
    }
}

impl TryFrom<u64> for RetryAfterSecs {
    type Error = crate::retry_after_secs_try_from_u64_error::RetryAfterSecsTryFromU64Error;

    fn try_from(u64: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(u64)
            .map(Self::from)
            .ok_or(crate::retry_after_secs_try_from_u64_error::RetryAfterSecsTryFromU64Error::Zero)
    }
}

impl TryFrom<RetryAfterSecs> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;

    fn try_from(retry_after_secs: RetryAfterSecs) -> Result<Self, Self::Error> {
        Self::from_str(retry_after_secs.get().to_string().as_str())
    }
}
