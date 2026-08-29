pub use super::bounded_bytes::BoundedBytes;
pub use super::bounded_json_read_error::BoundedJsonReadError;
pub use super::bounded_json_text::BoundedJsonText;
pub use super::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore;
pub use super::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize;
pub use super::bounded_read_error::BoundedReadError;
pub use super::bounded_read_from_utf8_error::BoundedReadFromUtf8Error;
pub use super::bounded_read_io_error::BoundedReadIoError;
pub use super::bounded_read_maximum_bytes::BoundedReadMaximumBytes;
use super::bounded_read_observed_bytes::BoundedReadObservedBytes;
pub use super::bounded_text::BoundedText;
pub use super::classify_not_found_io_error::classify_not_found_io_error;
use super::ensure_size_within_limit::ensure_size_within_limit;
pub use super::io_error_presence_disposition::IoErrorPresenceDisposition;
pub use super::parse_bounded_json::parse_bounded_json;
use super::parse_bounded_json_owned::parse_bounded_json_owned;
pub use super::path_ref::PathRef;
pub use super::read_bounded_file::read_bounded_file;
pub use super::read_bounded_file_async::read_bounded_file_async;
pub use super::read_bounded_http_response::read_bounded_http_response;
pub use super::read_bounded_json_file_async::read_bounded_json_file_async;
pub use super::read_bounded_json_http_response::read_bounded_json_http_response;
pub use super::reqwest_error::ReqwestError;
pub use super::reqwest_response::ReqwestResponse;
pub use super::serde_json_error::SerdeJsonError;
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
            super::PathRef::from(path.as_path()),
            super::BoundedReadMaximumBytes::from(4usize),
        )
        .expect("28fce6c8 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
        assert_eq!(exact.into_inner(), b"abcd");
        let over = super::read_bounded_file(
            super::PathRef::from(path.as_path()),
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
        let maximum_bytes = super::BoundedReadMaximumBytes::from(constants_usize::ONE);
        let result = (|| {
            let metadata = std::fs::metadata(path.as_path()).map_err(|source| {
                super::BoundedReadError::Io {
                    source: super::BoundedReadIoError::from(source),
                }
            })?;
            if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
                return Err(super::BoundedReadError::ExceedsMaximum { maximum_bytes });
            }
            std::fs::write(&path, b"ab")
                .expect("d34a7bc1 file_growth_after_metadata_is_rechecked invariant must hold");
            let bytes =
                std::fs::read(path.as_path()).map_err(|source| super::BoundedReadError::Io {
                    source: super::BoundedReadIoError::from(source),
                })?;
            super::ensure_size_within_limit(
                super::BoundedReadObservedBytes::from(bytes.len()),
                maximum_bytes,
            )?;
            Ok(super::BoundedBytes::from(bytes))
        })();
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
            super::PathRef::from(path.as_path()),
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
            super::BoundedReadConcurrencyArcSemaphore::new(super::BoundedReadConcurrencyMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .await
        .expect("26fc4688 http_response_stream_obeys_limit_without_external_network invariant must hold");
        assert_eq!(bytes.into_inner(), b"abcd");
    }
}

// Root-owned module compatibility wrappers.
mod bounded_bytes {
    pub use super::super::bounded_bytes::*;
}
mod bounded_json_read_error {
    pub use super::super::bounded_json_read_error::*;
}
mod bounded_json_text {
    pub use super::super::bounded_json_text::*;
}
mod bounded_read_concurrency_arc_semaphore {
    pub use super::super::bounded_read_concurrency_arc_semaphore::*;
}
mod bounded_read_concurrency_maximum_non_zero_usize {
    pub use super::super::bounded_read_concurrency_maximum_non_zero_usize::*;
}
mod bounded_read_error {
    pub use super::super::bounded_read_error::*;
}
mod bounded_read_from_utf8_error {
    pub use super::super::bounded_read_from_utf8_error::*;
}
mod bounded_read_io_error {
    pub use super::super::bounded_read_io_error::*;
}
mod bounded_read_maximum_bytes {
    pub use super::super::bounded_read_maximum_bytes::*;
}
mod bounded_read_observed_bytes {
    pub use super::super::bounded_read_observed_bytes::*;
}
mod bounded_text {
    pub use super::super::bounded_text::*;
}
mod classify_not_found_io_error {
    pub use super::super::classify_not_found_io_error::*;
}
mod ensure_size_within_limit {
    pub use super::super::ensure_size_within_limit::*;
}
mod io_error_presence_disposition {
    pub use super::super::io_error_presence_disposition::*;
}
mod parse_bounded_json {
    pub use super::super::parse_bounded_json::*;
}
mod parse_bounded_json_owned {
    pub use super::super::parse_bounded_json_owned::*;
}
mod path_ref {
    pub use super::super::path_ref::*;
}
mod read_bounded_file {
    pub use super::super::read_bounded_file::*;
}
mod read_bounded_file_async {
    pub use super::super::read_bounded_file_async::*;
}
mod read_bounded_http_response {
    pub use super::super::read_bounded_http_response::*;
}
mod read_bounded_json_file_async {
    pub use super::super::read_bounded_json_file_async::*;
}
mod read_bounded_json_http_response {
    pub use super::super::read_bounded_json_http_response::*;
}
mod reqwest_error {
    pub use super::super::reqwest_error::*;
}
mod reqwest_response {
    pub use super::super::reqwest_response::*;
}
mod serde_json_error {
    pub use super::super::serde_json_error::*;
}
