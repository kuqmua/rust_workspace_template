#[cfg(test)]
mod tests {
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
