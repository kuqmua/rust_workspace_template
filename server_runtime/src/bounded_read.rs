#[derive(Clone, Copy, Debug)]
pub struct StdPathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> From<&'path_lt std::path::Path> for StdPathRef<'path_lt> {
    fn from(value: &'path_lt std::path::Path) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedReadMaximumBytes(usize);
impl From<usize> for BoundedReadMaximumBytes {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for BoundedReadMaximumBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedBytes(Vec<u8>);
impl BoundedBytes {
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedText(String);
impl BoundedText {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl AsRef<str> for BoundedText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for BoundedText {
    type Error = BoundedReadError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 16_777_216usize {
            return Err(BoundedReadError::ExceedsMaximum {
                maximum_bytes: BoundedReadMaximumBytes(16_777_216usize),
            });
        }
        Ok(Self(value))
    }
}
impl TryFrom<BoundedBytes> for BoundedText {
    type Error = BoundedReadError;
    fn try_from(value: BoundedBytes) -> Result<Self, Self::Error> {
        let text = String::from_utf8(value.0).map_err(|source| BoundedReadError::Utf8 {
            source: StdFromUtf8Error(source),
        })?;
        Self::try_from(text)
    }
}
#[derive(Clone, Debug)]
pub struct StdBoundedReadConcurrency(std::sync::Arc<tokio::sync::Semaphore>);
#[derive(Clone, Copy, Debug)]
pub struct StdBoundedReadConcurrencyMaximum(std::num::NonZeroUsize);
impl From<std::num::NonZeroUsize> for StdBoundedReadConcurrencyMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value)
    }
}
impl StdBoundedReadConcurrency {
    #[must_use]
    pub fn new(maximum_concurrent_reads: StdBoundedReadConcurrencyMaximum) -> Self {
        Self(std::sync::Arc::new(tokio::sync::Semaphore::new(
            maximum_concurrent_reads.0.get(),
        )))
    }
}
#[derive(Debug)]
pub struct StdIoError(std::io::Error);
impl std::fmt::Display for StdIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub struct ReqwestError(reqwest::Error);
impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ReqwestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub struct StdFromUtf8Error(std::string::FromUtf8Error);
impl std::fmt::Display for StdFromUtf8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdFromUtf8Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub struct ReqwestResponse(reqwest::Response);
impl From<reqwest::Response> for ReqwestResponse {
    fn from(value: reqwest::Response) -> Self {
        Self(value)
    }
}
#[derive(Debug, thiserror::Error)]
pub enum BoundedReadError {
    #[error("content exceeds maximum size of {maximum_bytes} bytes")]
    ExceedsMaximum {
        maximum_bytes: BoundedReadMaximumBytes,
    },
    #[error("HTTP response body read failed")]
    Http {
        #[source]
        source: ReqwestError,
    },
    #[error("file read failed")]
    Io {
        #[source]
        source: StdIoError,
    },
    #[error("bounded read concurrency limiter is closed")]
    LimiterClosed,
    #[error("text content must be valid UTF-8")]
    Utf8 {
        #[source]
        source: StdFromUtf8Error,
    },
}
#[derive(Clone, Copy, Debug)]
struct BoundedReadObservedBytes(usize);
const fn ensure_size_within_limit(
    size: BoundedReadObservedBytes,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<(), BoundedReadError> {
    if size.0 > maximum_bytes.0 {
        Err(BoundedReadError::ExceedsMaximum { maximum_bytes })
    } else {
        Ok(())
    }
}
#[cfg(test)]
#[allow(
    clippy::single_call_fn,
    reason = "test seam simulates file growth between metadata and content reads"
)]
fn read_bounded_file_with_after_metadata(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
    after_metadata: impl FnOnce(),
) -> Result<BoundedBytes, BoundedReadError> {
    let metadata = std::fs::metadata(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError(source),
    })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    after_metadata();
    let bytes = std::fs::read(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError(source),
    })?;
    ensure_size_within_limit(BoundedReadObservedBytes(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes(bytes))
}
pub fn read_bounded_file(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<BoundedBytes, BoundedReadError> {
    let metadata = std::fs::metadata(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError(source),
    })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let bytes = std::fs::read(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError(source),
    })?;
    ensure_size_within_limit(BoundedReadObservedBytes(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes(bytes))
}
pub async fn read_bounded_file_async(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<BoundedBytes, BoundedReadError> {
    let metadata = tokio::fs::metadata(path.0)
        .await
        .map_err(|source| BoundedReadError::Io {
            source: StdIoError(source),
        })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let bytes = tokio::fs::read(path.0)
        .await
        .map_err(|source| BoundedReadError::Io {
            source: StdIoError(source),
        })?;
    ensure_size_within_limit(BoundedReadObservedBytes(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes(bytes))
}
pub async fn read_bounded_http_response(
    response: ReqwestResponse,
    maximum_bytes: BoundedReadMaximumBytes,
    concurrency: StdBoundedReadConcurrency,
) -> Result<BoundedBytes, BoundedReadError> {
    let _permit = concurrency
        .0
        .acquire_owned()
        .await
        .map_err(|_error| BoundedReadError::LimiterClosed)?;
    let mut inner_response = response.0;
    if let Some(content_length) = inner_response.content_length()
        && content_length > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX)
    {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let mut bytes = Vec::new();
    while let Some(chunk) =
        inner_response
            .chunk()
            .await
            .map_err(|source| BoundedReadError::Http {
                source: ReqwestError(source),
            })?
    {
        let next_len = bytes.len().saturating_add(chunk.len());
        ensure_size_within_limit(BoundedReadObservedBytes(next_len), maximum_bytes)?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(BoundedBytes(bytes))
}
#[cfg(test)]
mod tests {
    fn unique_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rust-workspace-template-bounded-read-{}-{name}",
            uuid::Uuid::new_v4()
        ))
    }
    #[test]
    fn exact_limit_and_one_byte_over_are_distinguished() {
        let path = unique_path(str_constants::LIMIT);
        std::fs::write(&path, b"abcd").expect("11ddba38");
        let exact = super::read_bounded_file(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(4usize),
        )
        .expect("28fce6c8");
        assert_eq!(exact.into_inner(), b"abcd");
        let over = super::read_bounded_file(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(3usize),
        );
        assert!(matches!(
            over,
            Err(super::BoundedReadError::ExceedsMaximum {
                maximum_bytes: super::BoundedReadMaximumBytes(3usize)
            })
        ));
        std::fs::remove_file(path).expect("30b575c6");
    }
    #[test]
    fn file_growth_after_metadata_is_rechecked() {
        let path = unique_path(str_constants::GROWTH);
        std::fs::write(&path, b"a").expect("c0745b58");
        let result = super::read_bounded_file_with_after_metadata(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(1usize),
            || std::fs::write(&path, b"ab").expect("d34a7bc1"),
        );
        assert!(matches!(
            result,
            Err(super::BoundedReadError::ExceedsMaximum {
                maximum_bytes: super::BoundedReadMaximumBytes(1usize)
            })
        ));
        std::fs::remove_file(path).expect("385eed61");
    }
    #[test]
    fn invalid_utf8_is_not_lossily_converted() {
        let result = super::BoundedText::try_from(super::BoundedBytes(vec![0xffu8]));
        assert!(matches!(result, Err(super::BoundedReadError::Utf8 { .. })));
    }
    #[tokio::test]
    async fn asynchronous_file_read_obeys_limit() {
        let path = unique_path(str_constants::ASYNC);
        tokio::fs::write(&path, b"abc").await.expect("f68e33f3");
        let bytes = super::read_bounded_file_async(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(3usize),
        )
        .await
        .expect("51d66e2c");
        assert_eq!(bytes.into_inner(), b"abc");
        tokio::fs::remove_file(path).await.expect("9d5a2db0");
    }
    #[tokio::test]
    async fn http_response_stream_obeys_limit_without_external_network() {
        let response = http::Response::builder()
            .header(http::header::CONTENT_LENGTH, str_constants::VALUE_4)
            .body(str_constants::ABCD_ALT)
            .expect("2306b26a");
        let bytes = super::read_bounded_http_response(
            super::ReqwestResponse::from(reqwest::Response::from(response)),
            super::BoundedReadMaximumBytes::from(4usize),
            super::StdBoundedReadConcurrency::new(super::StdBoundedReadConcurrencyMaximum::from(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .await
        .expect("26fc4688");
        assert_eq!(bytes.into_inner(), b"abcd");
    }
}
