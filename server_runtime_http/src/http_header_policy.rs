#[cfg(test)]
mod tests {
    #[test]
    fn content_disposition_sanitizes_and_encodes_file_name() {
        let value =
            crate::build_attachment_content_disposition::build_attachment_content_disposition(
                crate::http_attachment_file_name_ref::HttpAttachmentFileNameRef::from(
                    constants_str::test_fixtures::TEST_UNSAFE_UNICODE_ATTACHMENT_FILE_NAME,
                ),
            )
            .expect(
                "ec78ce2e content_disposition_sanitizes_and_encodes_file_name invariant must hold",
            );
        let header = http::HeaderValue::from(value);
        assert_eq!(
            header,
            http::HeaderValue::from_static(
                constants_str::test_fixtures::TEST_SAFE_UNICODE_ATTACHMENT_CONTENT_DISPOSITION
            )
        );
    }

    #[test]
    fn content_length_accepts_u64_maximum() {
        let value = crate::http_content_length::HttpContentLength::try_from(
            constants_str::test_fixtures::TEST_U64_MAXIMUM_TEXT.to_owned(),
        )
        .expect("f87ab266 content_length_accepts_u64_maximum invariant must hold");
        assert_eq!(u64::try_from(value), Ok(u64::MAX));
    }
}

// Root-owned module compatibility wrappers.
mod build_attachment_content_disposition {}
mod content_disposition_percent_encode_set {}
mod http_attachment_file_name_ref {}
mod http_content_disposition {}
mod http_content_disposition_error {}
mod http_content_length {}
mod http_content_length_error {}
