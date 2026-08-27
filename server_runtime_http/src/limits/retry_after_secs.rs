#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecs(pub(super) super::RetryAfterSecsNonZeroU64);

impl TryFrom<u64> for RetryAfterSecs {
    type Error = super::RetryAfterSecsTryFromU64Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(super::RetryAfterSecsTryFromU64Error)
    }
}

impl From<std::num::NonZeroU64> for RetryAfterSecs {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(super::RetryAfterSecsNonZeroU64::from(value))
    }
}

impl TryFrom<RetryAfterSecs> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;

    fn try_from(value: RetryAfterSecs) -> Result<Self, Self::Error> {
        Self::from_str(value.0.0.get().to_string().as_str())
    }
}
