pub const ADMIN_DISPLAY_NAME_MAX_CHARS: usize = 256usize;
pub const ADMIN_DISPLAY_NAME_MIN_CHARS: usize = 1usize;
pub const ADMIN_LOGIN_MAX_CHARS: usize = 128usize;
pub const ADMIN_LOGIN_MIN_CHARS: usize = 3usize;
pub const ADMIN_PASSWORD_MAX_CHARS: usize = 1024usize;
pub const ADMIN_PASSWORD_MIN_CHARS: usize = 1usize;
pub const ADMIN_NEW_PASSWORD_MIN_CHARS: usize = 12usize;
pub const ADMIN_ROLE_NAME_MAX_CHARS: usize = 128usize;
pub const ADMIN_ROLE_NAME_MIN_CHARS: usize = 1usize;

pub(crate) const ADMIN_DISPLAY_NAME_IS_VALID: fn(&str) -> bool = |value| value.trim() == value;
pub(crate) const ADMIN_LOGIN_IS_VALID: fn(&str) -> bool = |value| {
    value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'.' | b'-')
    })
};
pub(crate) const ADMIN_NEW_PASSWORD_IS_VALID: fn(&str) -> bool = |value| {
    text_policy::validate_password_policy::validate_password_policy(
        text_policy::password_text_ref::PasswordTextRef::from(value),
        text_policy::password_length_range::PasswordLengthRange::from_prevalidated(
            text_policy::password_length::PasswordLength::from(ADMIN_NEW_PASSWORD_MIN_CHARS),
            text_policy::password_length::PasswordLength::from(ADMIN_PASSWORD_MAX_CHARS),
        ),
    )
    .is_ok()
};
#[cfg(test)]
mod tests {
    #[test]
    fn positive_identifier_value_rejects_zero() {
        assert_eq!(
            crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(constants_i64::ZERO),
            Err(crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error),
        );
    }
}
