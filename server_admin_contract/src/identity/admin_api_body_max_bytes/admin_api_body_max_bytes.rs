pub(crate) const ADMIN_API_BODY_MAX_BYTES_VALUE: usize = 65_536usize;

#[must_use]
pub fn admin_api_body_max_bytes() -> super::super::AdminApiBodyMaxBytes {
    super::super::AdminApiBodyMaxBytes::from(ADMIN_API_BODY_MAX_BYTES_VALUE)
}
