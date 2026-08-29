pub use super::build_attachment_content_disposition::build_attachment_content_disposition;
use super::content_disposition_percent_encode_set::CONTENT_DISPOSITION_PERCENT_ENCODE_SET;
pub use super::http_attachment_file_name_ref::HttpAttachmentFileNameRef;
pub use super::http_content_disposition::HttpContentDisposition;
pub use super::http_content_disposition_error::HttpContentDispositionError;
pub use super::http_content_length::HttpContentLength;
pub use super::http_content_length_error::HttpContentLengthError;
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

// Root-owned module compatibility wrappers.
mod build_attachment_content_disposition {
    pub use super::super::build_attachment_content_disposition::*;
}
mod content_disposition_percent_encode_set {
    pub use super::super::content_disposition_percent_encode_set::*;
}
mod http_attachment_file_name_ref {
    pub use super::super::http_attachment_file_name_ref::*;
}
mod http_content_disposition {
    pub use super::super::http_content_disposition::*;
}
mod http_content_disposition_error {
    pub use super::super::http_content_disposition_error::*;
}
mod http_content_length {
    pub use super::super::http_content_length::*;
}
mod http_content_length_error {
    pub use super::super::http_content_length_error::*;
}
