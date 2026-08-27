#[must_use]
pub fn redact_rtsp_url_userinfo(value: super::RedactedUrlTextRef<'_>) -> super::RedactedUrl {
    super::redact_url_userinfo(value)
}
