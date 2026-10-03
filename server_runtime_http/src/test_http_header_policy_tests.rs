#[cfg(test)]
mod tests {
    #[test]
    fn test_attachment_filename_exact_input_size_limits() {
        assert!([0usize, 1usize, 4095usize, 4096usize, 4097usize].into_iter().all(|length| {
            let text = constants_str::X.repeat(length);
            let result = crate::build_attachment_content_disposition::build_attachment_content_disposition(
                crate::http_attachment_file_name_ref::HttpAttachmentFileNameRef::from(text.as_str()),
            );
            if length == 0usize {
                matches!(result, Err(crate::http_content_disposition_error::HttpContentDispositionError::Empty))
            } else if length > 4096usize {
                matches!(result, Err(crate::http_content_disposition_error::HttpContentDispositionError::TooLong))
            } else {
                let expected = format!("{}{text}{}{text}", constants_str::CONTENT_DISPOSITION_ATTACHMENT_PREFIX, constants_str::CONTENT_DISPOSITION_UTF8_DELIMITER);
                result.is_ok_and(|http_content_disposition| {
                    let header = http::HeaderValue::from(http_content_disposition);
                    header.as_bytes() == expected.as_bytes()
                })
            }
        }));
    }

    #[test]
    fn test_attachment_filename_sanitizes_each_path_quote_and_control_character() {
        assert!(['"', '/', '\\', '\0', '\r', '\n', '\t', '\u{85}'].into_iter().all(|character| {
            let text = format!("{}{character}{}", constants_str::X, constants_str::X);
            let sanitized = format!("{}_{}", constants_str::X, constants_str::X);
            let expected = format!("{}{sanitized}{}{sanitized}", constants_str::CONTENT_DISPOSITION_ATTACHMENT_PREFIX, constants_str::CONTENT_DISPOSITION_UTF8_DELIMITER);
            crate::build_attachment_content_disposition::build_attachment_content_disposition(
                crate::http_attachment_file_name_ref::HttpAttachmentFileNameRef::from(text.as_str()),
            ).is_ok_and(|http_content_disposition| http::HeaderValue::from(http_content_disposition).as_bytes() == expected.as_bytes())
        }));
    }

    #[test]
    fn test_content_length_rejects_each_invalid_category_with_size_precedence() {
        assert!(
            [
                (
                    String::default(),
                    crate::http_content_length_error::HttpContentLengthError::Empty
                ),
                (
                    constants_str::X.to_owned(),
                    crate::http_content_length_error::HttpContentLengthError::InvalidSymbol
                ),
                (
                    '\u{661}'.to_string(),
                    crate::http_content_length_error::HttpContentLengthError::InvalidSymbol
                ),
                (
                    format!("+{}", 1u8),
                    crate::http_content_length_error::HttpContentLengthError::InvalidSymbol
                ),
                (
                    (u128::from(u64::MAX) + 1u128).to_string(),
                    crate::http_content_length_error::HttpContentLengthError::OutOfRange
                ),
                (
                    '0'.to_string().repeat(21usize),
                    crate::http_content_length_error::HttpContentLengthError::TooLong
                ),
                (
                    constants_str::X.repeat(21usize),
                    crate::http_content_length_error::HttpContentLengthError::TooLong
                ),
            ]
            .into_iter()
            .all(|(text, error)| {
                crate::http_content_length::HttpContentLength::try_from(text) == Err(error)
            })
        );
    }

    #[test]
    fn test_content_length_preserves_leading_zeroes_and_numeric_value() {
        assert!([0u64, 1u64, u64::MAX].into_iter().all(|number| {
            let text = format!("{number:020}");
            crate::http_content_length::HttpContentLength::try_from(text.clone()).is_ok_and(
                |http_content_length| {
                    http_content_length.as_ref() == text
                        && u64::try_from(http_content_length) == Ok(number)
                },
            )
        }));
    }

    #[test]
    fn test_content_disposition_sanitizes_and_encodes_file_name() {
        let value =
            crate::build_attachment_content_disposition::build_attachment_content_disposition(
                crate::http_attachment_file_name_ref::HttpAttachmentFileNameRef::from(
                    constants_str::TEST_UNSAFE_UNICODE_ATTACHMENT_FILE_NAME,
                ),
            )
            .expect(constants_str::DIAGNOSTIC_EC78CE2E);
        let header = http::HeaderValue::from(value);
        assert_eq!(
            header,
            http::HeaderValue::from_static(
                constants_str::TEST_SAFE_UNICODE_ATTACHMENT_CONTENT_DISPOSITION
            )
        );
    }

    #[test]
    fn test_content_length_accepts_u64_maximum() {
        let value = crate::http_content_length::HttpContentLength::try_from(
            constants_str::TEST_U64_MAXIMUM_TEXT.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_F87AB266);
        assert_eq!(u64::try_from(value), Ok(u64::MAX));
    }
}
