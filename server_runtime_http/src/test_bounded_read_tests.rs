#[cfg(test)]
mod tests {
    fn unique_path(str: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rust-workspace-template-bounded-read-{}-{str}",
            uuid::Uuid::new_v4()
        ))
    }
    #[test]
    fn test_exact_limit_and_one_byte_over_are_distinguished() {
        let path = unique_path(constants_str::LIMIT);
        std::fs::write(&path, b"abcd").expect(constants_str::DIAGNOSTIC_11DDBA38);
        let exact = crate::read_bounded_file::read_bounded_file(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(4usize),
        )
        .expect(constants_str::DIAGNOSTIC_28FCE6C8);
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
        std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_30B575C6);
    }
    #[test]
    fn test_file_growth_after_metadata_is_rechecked() {
        let path = unique_path(constants_str::GROWTH);
        std::fs::write(&path, b"a").expect(constants_str::DIAGNOSTIC_C0745B58);
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
            std::fs::write(&path, b"ab").expect(constants_str::DIAGNOSTIC_D34A7BC1);
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
        std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_385EED61);
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
            .expect(constants_str::DIAGNOSTIC_712A0EA9);
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
        .expect(constants_str::DIAGNOSTIC_D2D69400);
        let pretty = json.pretty().expect(constants_str::DIAGNOSTIC_35493DB4);
        assert!(pretty.as_ref().contains('\n'));
        assert_eq!(
            pretty
                .compact()
                .expect(constants_str::DIAGNOSTIC_08123A26)
                .as_ref(),
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY
        );
    }
    #[tokio::test]
    async fn test_asynchronous_file_read_obeys_limit() {
        let path = unique_path(constants_str::ASYNC);
        tokio::fs::write(&path, b"abc")
            .await
            .expect(constants_str::DIAGNOSTIC_F68E33F3);
        let bytes = crate::read_bounded_file_async::read_bounded_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(3usize),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_51D66E2C);
        assert_eq!(bytes.into_inner(), b"abc");
        tokio::fs::remove_file(path)
            .await
            .expect(constants_str::DIAGNOSTIC_9D5A2DB0);
    }
    #[tokio::test]
    async fn test_http_response_stream_obeys_limit_without_external_network() {
        let response = http::Response::builder()
            .header(http::header::CONTENT_LENGTH, constants_str::VALUE_4)
            .body(constants_str::ABCD_ALT)
            .expect(constants_str::DIAGNOSTIC_2306B26A);
        let bytes = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(response)),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(4usize),
            crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::new(crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            )),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_26FC4688);
        assert_eq!(bytes.into_inner(), b"abcd");
    }
}
