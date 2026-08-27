#[path = "http_header_policy/build_attachment_content_disposition.rs"]
mod build_attachment_content_disposition;
#[path = "http_header_policy/content_disposition_percent_encode_set.rs"]
mod content_disposition_percent_encode_set;
#[path = "http_header_policy/http_attachment_file_name_ref.rs"]
mod http_attachment_file_name_ref;
#[path = "http_header_policy/http_content_disposition.rs"]
mod http_content_disposition;
#[path = "http_header_policy/http_content_disposition_error.rs"]
mod http_content_disposition_error;
#[path = "http_header_policy/http_content_length.rs"]
mod http_content_length;
#[path = "http_header_policy/http_content_length_error.rs"]
mod http_content_length_error;

pub use build_attachment_content_disposition::build_attachment_content_disposition;
use content_disposition_percent_encode_set::CONTENT_DISPOSITION_PERCENT_ENCODE_SET;
pub use http_attachment_file_name_ref::HttpAttachmentFileNameRef;
pub use http_content_disposition::HttpContentDisposition;
pub use http_content_disposition_error::HttpContentDispositionError;
pub use http_content_length::HttpContentLength;
pub use http_content_length_error::HttpContentLengthError;

#[cfg(test)]
mod tests {
    #[test]
    fn content_disposition_sanitizes_and_encodes_file_name() {
        let value = super::build_attachment_content_disposition(
            super::HttpAttachmentFileNameRef::from(
                constants_str::TEST_UNSAFE_UNICODE_ATTACHMENT_FILE_NAME,
            ),
        )
        .expect("ec78ce2e content_disposition_sanitizes_and_encodes_file_name invariant must hold");
        let header = http::HeaderValue::from(value);
        assert_eq!(
            header,
            http::HeaderValue::from_static(
                constants_str::TEST_SAFE_UNICODE_ATTACHMENT_CONTENT_DISPOSITION
            )
        );
    }

    #[test]
    fn content_length_accepts_u64_maximum() {
        let value =
            super::HttpContentLength::try_from(constants_str::TEST_U64_MAXIMUM_TEXT.to_owned())
                .expect("f87ab266 content_length_accepts_u64_maximum invariant must hold");
        assert_eq!(u64::try_from(value), Ok(u64::MAX));
    }
}
