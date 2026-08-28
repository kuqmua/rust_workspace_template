#[must_use]
pub fn redact_url_userinfo(value: super::RedactedUrlTextRef<'_>) -> super::RedactedUrl {
    let input = value.0;
    if let Ok(mut url) = reqwest::Url::parse(input) {
        if url.username().is_empty() && url.password().is_none() {
            return super::RedactedUrl::from(
                crate::domain_types::RequiredNulFreeBoundedText::try_from(input.to_owned()).ok(),
            );
        }
        if url.set_username(constants_str::REDACTED_ALT).is_ok() && url.set_password(None).is_ok() {
            return super::RedactedUrl::from(
                crate::domain_types::RequiredNulFreeBoundedText::try_from(url.to_string()).ok(),
            );
        }
    }
    let Some((scheme, remainder)) = input.split_once(constants_str::TEXT_ALT_10) else {
        return super::RedactedUrl::from(
            crate::domain_types::RequiredNulFreeBoundedText::try_from(
                constants_str::REDACTED_ALT_3.to_owned(),
            )
            .ok(),
        );
    };
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let Some(authority) = remainder.get(..authority_end) else {
        return super::RedactedUrl::from(
            crate::domain_types::RequiredNulFreeBoundedText::try_from(
                constants_str::REDACTED_ALT_3.to_owned(),
            )
            .ok(),
        );
    };
    let Some(userinfo_end) = authority.rfind('@') else {
        return super::RedactedUrl::from(
            crate::domain_types::RequiredNulFreeBoundedText::try_from(input.to_owned()).ok(),
        );
    };
    let host = authority
        .get(userinfo_end.saturating_add(constants_usize::ONE)..)
        .unwrap_or(constants_str::REDACTED_ALT_3);
    let suffix = remainder.get(authority_end..).unwrap_or_default();
    let mut output = String::with_capacity(input.len());
    output.push_str(scheme);
    output.push_str(constants_str::TEXT_ALT_10);
    output.push_str(constants_str::REDACTED_ALT);
    output.push('@');
    output.push_str(host);
    output.push_str(suffix);
    super::RedactedUrl::from(crate::domain_types::RequiredNulFreeBoundedText::try_from(output).ok())
}
