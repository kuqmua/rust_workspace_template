#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdPermitWaitTimeout(std::time::Duration);
impl From<std::time::Duration> for StdPermitWaitTimeout {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecs(u64);
impl TryFrom<u64> for RetryAfterSecs {
    type Error = RetryAfterSecsTryFromU64Er;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(RetryAfterSecsTryFromU64Er)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecsTryFromU64Er;
impl std::fmt::Display for RetryAfterSecsTryFromU64Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("retry-after seconds must be greater than zero")
    }
}
impl std::error::Error for RetryAfterSecsTryFromU64Er {}
impl TryFrom<RetryAfterSecs> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: RetryAfterSecs) -> Result<Self, Self::Error> {
        Self::from_str(value.0.to_string().as_str())
    }
}
#[derive(Clone, Debug)]
pub struct StdArcTokioSemaphore(std::sync::Arc<tokio::sync::Semaphore>);
impl From<std::sync::Arc<tokio::sync::Semaphore>> for StdArcTokioSemaphore {
    fn from(value: std::sync::Arc<tokio::sync::Semaphore>) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct TokioAcquireEr(tokio::sync::AcquireError);
impl std::fmt::Display for TokioAcquireEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for TokioAcquireEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum AcquirePermitEr {
    Closed(TokioAcquireEr),
    Timeout(RetryAfterSecs),
}
#[derive(Debug)]
pub struct TokioOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);
impl From<tokio::sync::OwnedSemaphorePermit> for TokioOwnedSemaphorePermit {
    fn from(value: tokio::sync::OwnedSemaphorePermit) -> Self {
        Self(value)
    }
}
impl TokioOwnedSemaphorePermit {
    pub fn forget(self) {
        self.0.forget();
    }
}
impl std::fmt::Display for AcquirePermitEr {
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
impl std::error::Error for AcquirePermitEr {
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
) -> Result<TokioOwnedSemaphorePermit, AcquirePermitEr> {
    match tokio::time::timeout(wait_timeout.0, semaphore.0.acquire_owned()).await {
        Ok(Ok(permit)) => Ok(TokioOwnedSemaphorePermit::from(permit)),
        Ok(Err(error)) => Err(AcquirePermitEr::Closed(TokioAcquireEr(error))),
        Err(_elapsed) => Err(AcquirePermitEr::Timeout(retry_after)),
    }
}
