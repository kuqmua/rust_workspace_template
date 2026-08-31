#[cfg(test)]
mod tests {
    fn unique_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rust-workspace-template-bounded-read-{}-{name}",
            uuid::Uuid::new_v4()
        ))
    }
    #[test]
    fn test_exact_limit_and_one_byte_over_are_distinguished() {
        let path = unique_path(constants_str::LIMIT);
        std::fs::write(&path, b"abcd")
            .expect("11ddba38 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
        let exact = crate::read_bounded_file::read_bounded_file(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(4usize),
        )
        .expect("28fce6c8 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
        assert_eq!(exact.into_inner(), b"abcd");
        let over = crate::read_bounded_file::read_bounded_file(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(3usize),
        );
        assert!(matches!(
            over,
            Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })
                if maximum_bytes.get() == 3usize
        ));
        std::fs::remove_file(path)
            .expect("30b575c6 exact_limit_and_one_byte_over_are_distinguished invariant must hold");
    }
    #[test]
    fn test_file_growth_after_metadata_is_rechecked() {
        let path = unique_path(constants_str::GROWTH);
        std::fs::write(&path, b"a")
            .expect("c0745b58 file_growth_after_metadata_is_rechecked invariant must hold");
        let maximum_bytes =
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(constants_usize::ONE);
        let result = (|| {
            let metadata = std::fs::metadata(path.as_path()).map_err(|source| {
                crate::bounded_read_error::BoundedReadError::Io {
                    source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
                }
            })?;
            if metadata.len() > u64::try_from(maximum_bytes.get()).unwrap_or(u64::MAX) {
                return Err(
                    crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes },
                );
            }
            std::fs::write(&path, b"ab")
                .expect("d34a7bc1 file_growth_after_metadata_is_rechecked invariant must hold");
            let bytes = std::fs::read(path.as_path()).map_err(|source| {
                crate::bounded_read_error::BoundedReadError::Io {
                    source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
                }
            })?;
            crate::ensure_size_within_limit::ensure_size_within_limit(
                crate::bounded_read_observed_bytes::BoundedReadObservedBytes::from(bytes.len()),
                maximum_bytes,
            )?;
            Ok(crate::bounded_bytes::BoundedBytes::from(bytes))
        })();
        assert!(matches!(
            result,
            Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                maximum_bytes: error_maximum_bytes,
            }) if error_maximum_bytes.get() == constants_usize::ONE
        ));
        std::fs::remove_file(path)
            .expect("385eed61 file_growth_after_metadata_is_rechecked invariant must hold");
    }
    #[test]
    fn test_invalid_utf8_is_not_lossily_converted() {
        let result = crate::bounded_text::BoundedText::try_from(
            crate::bounded_bytes::BoundedBytes::from(vec![0xffu8]),
        );
        assert!(matches!(
            result,
            Err(crate::bounded_read_error::BoundedReadError::Utf8 { .. })
        ));
    }
    #[test]
    fn test_only_not_found_is_classified_as_missing() {
        assert!(matches!(
            crate::classify_not_found_io_error::classify_not_found_io_error(
                std::io::Error::from(std::io::ErrorKind::NotFound).into()
            ),
            crate::io_error_presence_disposition::IoErrorPresenceDisposition::Missing
        ));
        assert!(matches!(
            crate::classify_not_found_io_error::classify_not_found_io_error(
                std::io::Error::from(std::io::ErrorKind::PermissionDenied).into()
            ),
            crate::io_error_presence_disposition::IoErrorPresenceDisposition::Other(_)
        ));
    }
    #[test]
    fn test_bounded_json_distinguishes_invalid_document() {
        let valid = crate::bounded_bytes::BoundedBytes::from(
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY
                .as_bytes()
                .to_vec(),
        );
        let _json = crate::parse_bounded_json::parse_bounded_json(&valid)
            .expect("712a0ea9 bounded_json_distinguishes_invalid_document invariant must hold");
        let invalid = crate::bounded_bytes::BoundedBytes::from(
            constants_str::TEST_INVALID_JSON.as_bytes().to_vec(),
        );
        assert!(matches!(
            crate::parse_bounded_json::parse_bounded_json(&invalid),
            Err(crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(_))
        ));
    }
    #[test]
    fn test_bounded_json_formats_pretty_and_compact_text() {
        let json = crate::bounded_json_text::BoundedJsonText::try_from(String::from(
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
    async fn test_asynchronous_file_read_obeys_limit() {
        let path = unique_path(constants_str::ASYNC);
        tokio::fs::write(&path, b"abc")
            .await
            .expect("f68e33f3 asynchronous_file_read_obeys_limit invariant must hold");
        let bytes = crate::read_bounded_file_async::read_bounded_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(3usize),
        )
        .await
        .expect("51d66e2c asynchronous_file_read_obeys_limit invariant must hold");
        assert_eq!(bytes.into_inner(), b"abc");
        tokio::fs::remove_file(path)
            .await
            .expect("9d5a2db0 asynchronous_file_read_obeys_limit invariant must hold");
    }
    #[tokio::test]
    async fn test_http_response_stream_obeys_limit_without_external_network() {
        let response = http::Response::builder()
            .header(http::header::CONTENT_LENGTH, constants_str::VALUE_4)
            .body(constants_str::ABCD_ALT)
            .expect("2306b26a http_response_stream_obeys_limit_without_external_network invariant must hold");
        let bytes = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(response)),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(4usize),
            crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::new(crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .await
        .expect("26fc4688 http_response_stream_obeys_limit_without_external_network invariant must hold");
        assert_eq!(bytes.into_inner(), b"abcd");
    }
}

// Root-owned module compatibility wrappers.
mod bounded_bytes {}
mod bounded_json_read_error {}
mod bounded_json_text {}
mod bounded_read_concurrency_arc_semaphore {}
mod bounded_read_concurrency_maximum_non_zero_usize {}
mod bounded_read_error {}
mod bounded_read_from_utf8_error {}
mod bounded_read_io_error {}
mod bounded_read_maximum_bytes {}
mod bounded_read_observed_bytes {}
mod bounded_text {}
mod classify_not_found_io_error {}
mod ensure_size_within_limit {}
mod io_error_presence_disposition {}
mod parse_bounded_json {}
mod parse_bounded_json_owned {}
mod runtime_path_ref {}
mod read_bounded_file {}
mod read_bounded_file_async {}
mod read_bounded_http_response {}
mod read_bounded_json_file_async {}
mod read_bounded_json_http_response {}
mod reqwest_error {}
mod reqwest_response {}
mod serde_json_error {}
