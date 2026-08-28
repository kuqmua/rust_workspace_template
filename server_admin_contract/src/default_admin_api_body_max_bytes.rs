use super::{ADMIN_API_BODY_MAX_BYTES_VALUE, AdminApiBodyMaxBytes};

#[must_use]
pub fn default_admin_api_body_max_bytes() -> AdminApiBodyMaxBytes {
    AdminApiBodyMaxBytes::from(ADMIN_API_BODY_MAX_BYTES_VALUE)
}
