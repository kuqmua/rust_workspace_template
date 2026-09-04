pub fn validate_password_policy(
    password_text_ref: crate::password_text_ref::PasswordTextRef<'_>,
    password_length_range: crate::password_length_range::PasswordLengthRange,
) -> Result<(), crate::password_policy_violation::PasswordPolicyViolation> {
    let password_text: &str = password_text_ref.into();
    if password_text.len() < usize::from(password_length_range.minimum()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::TooShort);
    }
    if password_text.len() > usize::from(password_length_range.maximum()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::TooLong);
    }
    if password_text.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::ContainsWhitespace);
    }
    if !password_text.bytes().any(|byte| byte.is_ascii_digit()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingDigit);
    }
    if !password_text.bytes().any(|byte| byte.is_ascii_lowercase()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingLowercase);
    }
    if !password_text.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingUppercase);
    }
    if !password_text
        .bytes()
        .any(|byte| byte.is_ascii_punctuation())
    {
        return Err(crate::password_policy_violation::PasswordPolicyViolation::MissingSpecial);
    }
    Ok(())
}
