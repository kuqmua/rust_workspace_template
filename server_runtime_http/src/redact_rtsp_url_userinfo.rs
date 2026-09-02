#[must_use]
pub fn redact_rtsp_url_userinfo(
    redacted_url_text_ref: crate::redacted_url_text_ref::RedactedUrlTextRef<'_>,
) -> crate::redacted_url::RedactedUrl {
    crate::redact_url_userinfo::redact_url_userinfo(redacted_url_text_ref)
}
