#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct StdPathRef<'path_lt>(&'path_lt std::path::Path);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::Display,
)]
pub struct BoundedReadMaximumBytes(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInner,
)]
pub struct BoundedBytes(Vec<u8>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::IntoInner,
)]
pub struct BoundedText(String);
impl TryFrom<String> for BoundedText {
    type Error = BoundedReadError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(BoundedReadError::ExceedsMaximum {
                maximum_bytes: BoundedReadMaximumBytes(constants_usize::VALUE_16_777_216),
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub struct StdBoundedReadConcurrency(std::sync::Arc<tokio::sync::Semaphore>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct StdBoundedReadConcurrencyMaximum(std::num::NonZeroUsize);

impl StdBoundedReadConcurrency {
    #[must_use]
    pub fn new(maximum_concurrent_reads: StdBoundedReadConcurrencyMaximum) -> Self {
        Self::from(std::sync::Arc::new(tokio::sync::Semaphore::new(
            maximum_concurrent_reads.0.get(),
        )))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct StdIoError(std::io::Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum IoErrorPresenceDisposition {
    Missing,
    Other(StdIoError),
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct ReqwestError(reqwest::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct StdFromUtf8Error(std::string::FromUtf8Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ReqwestResponse(reqwest::Response);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct SerdeJsonError(serde_json::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct BoundedJsonText(String);
impl BoundedJsonText {
    pub fn compact(&self) -> Result<Self, BoundedJsonReadError> {
        let value = serde_json::from_str::<serde_json::Value>(self.0.as_str())
            .map_err(|error| BoundedJsonReadError::SerdeJson(SerdeJsonError::from(error)))?;
        let text = serde_json::to_string(&value)
            .map_err(|error| BoundedJsonReadError::SerdeJson(SerdeJsonError::from(error)))?;
        Self::try_from(text)
    }

    pub fn pretty(&self) -> Result<Self, BoundedJsonReadError> {
        let value = serde_json::from_str::<serde_json::Value>(self.0.as_str())
            .map_err(|error| BoundedJsonReadError::SerdeJson(SerdeJsonError::from(error)))?;
        let text = serde_json::to_string_pretty(&value)
            .map_err(|error| BoundedJsonReadError::SerdeJson(SerdeJsonError::from(error)))?;
        Self::try_from(text)
    }
}
impl TryFrom<String> for BoundedJsonText {
    type Error = BoundedJsonReadError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(BoundedJsonReadError::Read(
                BoundedReadError::ExceedsMaximum {
                    maximum_bytes: BoundedReadMaximumBytes(constants_usize::VALUE_16_777_216),
                },
            ));
        }
        let _validated_value = serde_json::from_str::<serde_json::Value>(value.as_str())
            .map_err(|error| BoundedJsonReadError::SerdeJson(SerdeJsonError(error)))?;
        Ok(Self(value))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedJsonReadError {
    #[error("bounded content read failed")]
    Read(#[source] BoundedReadError),
    #[error("bounded content is not valid JSON")]
    SerdeJson(#[source] SerdeJsonError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct BoundedReadObservedBytes(usize);

#[must_use]
pub fn classify_not_found_io_error(error: StdIoError) -> IoErrorPresenceDisposition {
    if error.0.kind() == std::io::ErrorKind::NotFound {
        IoErrorPresenceDisposition::Missing
    } else {
        IoErrorPresenceDisposition::Other(error)
    }
}
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
        source: StdIoError::from(source),
    })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    after_metadata();
    let bytes = std::fs::read(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError::from(source),
    })?;
    ensure_size_within_limit(BoundedReadObservedBytes::from(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes::from(bytes))
}
pub fn read_bounded_file(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<BoundedBytes, BoundedReadError> {
    let metadata = std::fs::metadata(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError::from(source),
    })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let bytes = std::fs::read(path.0).map_err(|source| BoundedReadError::Io {
        source: StdIoError::from(source),
    })?;
    ensure_size_within_limit(BoundedReadObservedBytes::from(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes::from(bytes))
}
pub async fn read_bounded_file_async(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<BoundedBytes, BoundedReadError> {
    let metadata = tokio::fs::metadata(path.0)
        .await
        .map_err(|source| BoundedReadError::Io {
            source: StdIoError::from(source),
        })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let bytes = tokio::fs::read(path.0)
        .await
        .map_err(|source| BoundedReadError::Io {
            source: StdIoError::from(source),
        })?;
    ensure_size_within_limit(BoundedReadObservedBytes::from(bytes.len()), maximum_bytes)?;
    Ok(BoundedBytes::from(bytes))
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
    let initial_capacity = inner_response
        .content_length()
        .and_then(|length| usize::try_from(length).ok())
        .map_or(constants_usize::ZERO, |length| length.min(maximum_bytes.0));
    let mut bytes = Vec::with_capacity(initial_capacity);
    while let Some(chunk) =
        inner_response
            .chunk()
            .await
            .map_err(|source| BoundedReadError::Http {
                source: ReqwestError::from(source),
            })?
    {
        let next_len = bytes.len().saturating_add(chunk.len());
        ensure_size_within_limit(BoundedReadObservedBytes::from(next_len), maximum_bytes)?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(BoundedBytes::from(bytes))
}
pub fn parse_bounded_json(bytes: &BoundedBytes) -> Result<BoundedJsonText, BoundedJsonReadError> {
    parse_bounded_json_owned(bytes.clone())
}
fn parse_bounded_json_owned(bytes: BoundedBytes) -> Result<BoundedJsonText, BoundedJsonReadError> {
    let text = String::from_utf8(bytes.0).map_err(|error| {
        BoundedJsonReadError::Read(BoundedReadError::Utf8 {
            source: StdFromUtf8Error::from(error),
        })
    })?;
    BoundedJsonText::try_from(text)
}
pub async fn read_bounded_json_file_async(
    path: StdPathRef<'_>,
    maximum_bytes: BoundedReadMaximumBytes,
) -> Result<BoundedJsonText, BoundedJsonReadError> {
    let bytes = read_bounded_file_async(path, maximum_bytes)
        .await
        .map_err(BoundedJsonReadError::Read)?;
    parse_bounded_json_owned(bytes)
}
pub async fn read_bounded_json_http_response(
    response: ReqwestResponse,
    maximum_bytes: BoundedReadMaximumBytes,
    concurrency: StdBoundedReadConcurrency,
) -> Result<BoundedJsonText, BoundedJsonReadError> {
    let bytes = read_bounded_http_response(response, maximum_bytes, concurrency)
        .await
        .map_err(BoundedJsonReadError::Read)?;
    parse_bounded_json_owned(bytes)
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
        let path = unique_path(constants_str::LIMIT);
        std::fs::write(&path, b"abcd")
            .expect("11ddba38 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
        let exact = super::read_bounded_file(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(4usize),
        )
        .expect("28fce6c8 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
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
        std::fs::remove_file(path)
            .expect("30b575c6 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
    }
    #[test]
    fn file_growth_after_metadata_is_rechecked() {
        let path = unique_path(constants_str::GROWTH);
        std::fs::write(&path, b"a")
            .expect("c0745b58 file_growth_after_metadata_is_rechecked invariant must hold");
        let result = super::read_bounded_file_with_after_metadata(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(constants_usize::ONE),
            || {
                std::fs::write(&path, b"ab")
                    .expect("d34a7bc1 file_growth_after_metadata_is_rechecked invariant must hold");
            },
        );
        assert!(matches!(
            result,
            Err(super::BoundedReadError::ExceedsMaximum {
                maximum_bytes: super::BoundedReadMaximumBytes(constants_usize::ONE)
            })
        ));
        std::fs::remove_file(path)
            .expect("385eed61 file_growth_after_metadata_is_rechecked invariant must hold");
    }
    #[test]
    fn invalid_utf8_is_not_lossily_converted() {
        let result = super::BoundedText::try_from(super::BoundedBytes::from(vec![0xffu8]));
        assert!(matches!(result, Err(super::BoundedReadError::Utf8 { .. })));
    }
    #[test]
    fn only_not_found_is_classified_as_missing() {
        assert!(matches!(
            super::classify_not_found_io_error(
                std::io::Error::from(std::io::ErrorKind::NotFound).into()
            ),
            super::IoErrorPresenceDisposition::Missing
        ));
        assert!(matches!(
            super::classify_not_found_io_error(
                std::io::Error::from(std::io::ErrorKind::PermissionDenied).into()
            ),
            super::IoErrorPresenceDisposition::Other(_)
        ));
    }
    #[test]
    fn bounded_json_distinguishes_invalid_document() {
        let valid = super::BoundedBytes::from(
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY
                .as_bytes()
                .to_vec(),
        );
        let _json = super::parse_bounded_json(&valid)
            .expect("712a0ea9 bounded_json_distinguishes_invalid_document invariant must hold");
        let invalid =
            super::BoundedBytes::from(constants_str::TEST_INVALID_JSON.as_bytes().to_vec());
        assert!(matches!(
            super::parse_bounded_json(&invalid),
            Err(super::BoundedJsonReadError::SerdeJson(_))
        ));
    }
    #[test]
    fn bounded_json_formats_pretty_and_compact_text() {
        let json = super::BoundedJsonText::try_from(String::from(
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY,
        ))
        .expect("d2d69400 bounded_json_formats_pretty_and_compact_text invariant must hold");
        let pretty = json
            .pretty()
            .expect("35493db4 bounded_json_formats_pretty_and_compact_text invariant must hold");
        assert!(pretty.as_ref().contains('\n'));
        assert_eq!(
            pretty
                .compact()
                .expect("08123a26 bounded_json_formats_pretty_and_compact_text invariant must hold")
                .as_ref(),
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY
        );
    }
    #[tokio::test]
    async fn asynchronous_file_read_obeys_limit() {
        let path = unique_path(constants_str::ASYNC);
        tokio::fs::write(&path, b"abc")
            .await
            .expect("f68e33f3 asynchronous_file_read_obeys_limit invariant must hold");
        let bytes = super::read_bounded_file_async(
            super::StdPathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(3usize),
        )
        .await
        .expect("51d66e2c asynchronous_file_read_obeys_limit invariant must hold");
        assert_eq!(bytes.into_inner(), b"abc");
        tokio::fs::remove_file(path)
            .await
            .expect("9d5a2db0 asynchronous_file_read_obeys_limit invariant must hold");
    }
    #[tokio::test]
    async fn http_response_stream_obeys_limit_without_external_network() {
        let response = http::Response::builder()
            .header(http::header::CONTENT_LENGTH, constants_str::VALUE_4)
            .body(constants_str::ABCD_ALT)
            .expect("2306b26a http_response_stream_obeys_limit_without_external_network invariant must hold");
        let bytes = super::read_bounded_http_response(
            super::ReqwestResponse::from(reqwest::Response::from(response)),
            super::BoundedReadMaximumBytes::from(4usize),
            super::StdBoundedReadConcurrency::new(super::StdBoundedReadConcurrencyMaximum::from(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .await
        .expect("26fc4688 http_response_stream_obeys_limit_without_external_network invariant must hold");
        assert_eq!(bytes.into_inner(), b"abcd");
    }
}
