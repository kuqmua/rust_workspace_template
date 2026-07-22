#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdPermitWaitTimeout(std::time::Duration);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecs(u64);
impl TryFrom<u64> for RetryAfterSecs {
    type Error = RetryAfterSecsTryFromU64Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(RetryAfterSecsTryFromU64Error)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecsTryFromU64Error;
impl std::fmt::Display for RetryAfterSecsTryFromU64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::RETRY_AFTER_SECONDS_MUST_BE_GREATER_THAN_ZERO)
    }
}
impl std::error::Error for RetryAfterSecsTryFromU64Error {}
impl TryFrom<RetryAfterSecs> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: RetryAfterSecs) -> Result<Self, Self::Error> {
        Self::from_str(value.0.to_string().as_str())
    }
}
#[derive(Clone, Debug, newtype::FromInner)]
pub struct StdArcTokioSemaphore(std::sync::Arc<tokio::sync::Semaphore>);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdSemaphorePermitCount(std::num::NonZeroUsize);

impl StdArcTokioSemaphore {
    #[must_use]
    pub fn new(permit_count: StdSemaphorePermitCount) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            permit_count.0.get(),
        )))
    }
    #[must_use]
    pub fn try_acquire(&self) -> Option<TokioOwnedSemaphorePermit> {
        std::sync::Arc::clone(&self.0)
            .try_acquire_owned()
            .ok()
            .map(TokioOwnedSemaphorePermit::from)
    }
}

#[derive(Debug, newtype::ErrorTransparent, newtype::FromInner, newtype::Display)]
pub struct TokioAcquireError(tokio::sync::AcquireError);
#[derive(Debug)]
pub enum AcquirePermitError {
    Closed(TokioAcquireError),
    Timeout(RetryAfterSecs),
}
#[derive(Debug, newtype::FromInner)]
pub struct TokioOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);

impl TokioOwnedSemaphorePermit {
    pub fn forget(self) {
        self.0.forget();
    }
}
impl std::fmt::Display for AcquirePermitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(error) => write!(f, "concurrency limiter is closed: {error}"),
            Self::Timeout(retry_after) => {
                write!(
                    f,
                    "concurrency limit reached; retry after {} seconds",
                    retry_after.0
                )
            }
        }
    }
}
impl std::error::Error for AcquirePermitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Closed(error) => Some(error),
            Self::Timeout(_) => None,
        }
    }
}
pub async fn acquire_permit(
    semaphore: StdArcTokioSemaphore,
    wait_timeout: StdPermitWaitTimeout,
    retry_after: RetryAfterSecs,
) -> Result<TokioOwnedSemaphorePermit, AcquirePermitError> {
    match tokio::time::timeout(wait_timeout.0, semaphore.0.acquire_owned()).await {
        Ok(Ok(permit)) => Ok(TokioOwnedSemaphorePermit::from(permit)),
        Ok(Err(error)) => Err(AcquirePermitError::Closed(TokioAcquireError::from(error))),
        Err(_elapsed) => Err(AcquirePermitError::Timeout(retry_after)),
    }
}
