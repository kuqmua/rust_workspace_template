#[path = "identity/admin_api_body_max_bytes.rs"]
mod admin_api_body_max_bytes;
#[path = "identity/admin_display_name.rs"]
mod admin_display_name;
#[path = "identity/admin_login.rs"]
mod admin_login;
#[path = "identity/admin_new_password.rs"]
mod admin_new_password;
#[path = "identity/admin_password.rs"]
mod admin_password;
#[path = "identity/admin_role_name.rs"]
mod admin_role_name;
#[path = "identity/admin_text.rs"]
mod admin_text;
#[path = "identity/positive_non_zero_i64.rs"]
mod positive_non_zero_i64;

pub const ADMIN_DISPLAY_NAME_MAX_CHARS: usize = 256usize;
pub const ADMIN_DISPLAY_NAME_MIN_CHARS: usize = 1usize;
pub const ADMIN_LOGIN_MAX_CHARS: usize = 128usize;
pub const ADMIN_LOGIN_MIN_CHARS: usize = 3usize;
pub const ADMIN_PASSWORD_MAX_CHARS: usize = 1024usize;
pub const ADMIN_PASSWORD_MIN_CHARS: usize = 1usize;
pub const ADMIN_NEW_PASSWORD_MIN_CHARS: usize = 12usize;
pub const ADMIN_ROLE_NAME_MAX_CHARS: usize = 128usize;
pub const ADMIN_ROLE_NAME_MIN_CHARS: usize = 1usize;

const ADMIN_DISPLAY_NAME_IS_VALID: fn(&str) -> bool = |value| value.trim() == value;
const ADMIN_LOGIN_IS_VALID: fn(&str) -> bool = |value| {
    value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'.' | b'-')
    })
};
const ADMIN_NEW_PASSWORD_IS_VALID: fn(&str) -> bool = |value| {
    text_policy::domain_types::validate_password_policy(
        text_policy::domain_types::PasswordTextRef::from(value),
        text_policy::domain_types::PasswordLengthRange::from_prevalidated(
            text_policy::domain_types::PasswordLength::from(ADMIN_NEW_PASSWORD_MIN_CHARS),
            text_policy::domain_types::PasswordLength::from(ADMIN_PASSWORD_MAX_CHARS),
        ),
    )
    .is_ok()
};

pub(crate) use admin_api_body_max_bytes::ADMIN_API_BODY_MAX_BYTES_VALUE;
pub use admin_api_body_max_bytes::{AdminApiBodyMaxBytes, admin_api_body_max_bytes};
pub use admin_display_name::*;
pub use admin_login::*;
pub use admin_new_password::*;
pub use admin_password::*;
pub use admin_role_name::*;
pub use admin_text::*;
pub use positive_non_zero_i64::PositiveNonZeroI64;

#[cfg(test)]
mod tests {
    #[test]
    fn positive_identifier_value_rejects_zero() {
        assert_eq!(
            super::PositiveNonZeroI64::try_from(constants_i64::ZERO),
            Err(crate::domain_types::AdminIdTryFromI64Error),
        );
    }
}
