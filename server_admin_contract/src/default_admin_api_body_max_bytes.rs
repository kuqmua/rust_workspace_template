#[must_use]
pub fn default_admin_api_body_max_bytes() -> crate::admin_api_body_max_bytes::AdminApiBodyMaxBytes {
    crate::admin_api_body_max_bytes::AdminApiBodyMaxBytes::from(
        crate::admin_api_body_max_bytes::ADMIN_API_BODY_MAX_BYTES_VALUE,
    )
}
