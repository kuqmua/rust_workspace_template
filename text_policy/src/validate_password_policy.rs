pub fn validate_password_policy(
    password: crate::password_text_ref::PasswordTextRef<'_>,
    range: crate::password_length_range::PasswordLengthRange,
) -> Result<(), crate::password_policy_violation::PasswordPolicyViolation> {
    if password.0.len() < range.minimum.0 {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::TooShort);
    }
    if password.0.len() > range.maximum.0 {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::TooLong);
    }
    if password.0.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::ContainsWhitespace);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_digit()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingDigit);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_lowercase()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingLowercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingUppercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_punctuation()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingSpecial);
    }
    Ok(())
}
