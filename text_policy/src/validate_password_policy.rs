use crate::domain_types::{PasswordLengthRange, PasswordPolicyViolation, PasswordTextRef};

pub fn validate_password_policy(
    password: PasswordTextRef<'_>,
    range: PasswordLengthRange,
) -> Result<(), PasswordPolicyViolation> {
    if password.0.len() < range.minimum.0 {
        return Err(PasswordPolicyViolation::TooShort);
    }
    if password.0.len() > range.maximum.0 {
        return Err(PasswordPolicyViolation::TooLong);
    }
    if password.0.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(PasswordPolicyViolation::ContainsWhitespace);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_digit()) {
        return Err(PasswordPolicyViolation::MissingDigit);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_lowercase()) {
        return Err(PasswordPolicyViolation::MissingLowercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(PasswordPolicyViolation::MissingUppercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_punctuation()) {
        return Err(PasswordPolicyViolation::MissingSpecial);
    }
    Ok(())
}
