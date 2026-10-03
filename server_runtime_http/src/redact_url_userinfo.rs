#[must_use]
pub fn redact_url_userinfo(
    redacted_url_text_ref: crate::redacted_url_text_ref::RedactedUrlTextRef<'_>,
) -> crate::redacted_url::RedactedUrl {
    let input = redacted_url_text_ref.get();
    if let Ok(mut url) = reqwest::Url::parse(input) {
        if url.username().is_empty() && url.password().is_none() {
            return crate::redacted_url::RedactedUrl::from(
                text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                    input.to_owned(),
                )
                .ok(),
            );
        }
        if url.set_username(constants_str::REDACTED_ALT).is_ok() && url.set_password(None).is_ok() {
            return crate::redacted_url::RedactedUrl::from(
                text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                    url.to_string(),
                )
                .ok(),
            );
        }
    }
    let Some((scheme, remainder)) = input.split_once(constants_str::TEXT_ALT_10) else {
        return crate::redacted_url::RedactedUrl::from(
            text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::REDACTED_ALT_3.to_owned(),
            )
            .ok(),
        );
    };
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let Some(authority) = remainder.get(..authority_end) else {
        return crate::redacted_url::RedactedUrl::from(
            text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::REDACTED_ALT_3.to_owned(),
            )
            .ok(),
        );
    };
    let Some(userinfo_end) = authority.rfind('@') else {
        return crate::redacted_url::RedactedUrl::from(
            text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                input.to_owned(),
            )
            .ok(),
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
    crate::redacted_url::RedactedUrl::from(
        text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(output)
            .ok(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fallback_redaction_uses_last_authority_separator() {
        assert!(['/', '?', '#'].into_iter().all(|delimiter| {
            let input = format!(
                "1{}://{}@{}:{}@{}{delimiter}{}@{}",
                constants_str::X,
                constants_str::X,
                constants_str::X,
                constants_str::SECRET,
                constants_str::LOCALHOST,
                constants_str::X,
                constants_str::LOCALHOST
            );
            let expected = format!(
                "1{}://{}@{}{delimiter}{}@{}",
                constants_str::X,
                constants_str::REDACTED_ALT,
                constants_str::LOCALHOST,
                constants_str::X,
                constants_str::LOCALHOST
            );
            let redacted = crate::redact_url_userinfo::redact_url_userinfo(input.as_str().into());
            redacted.as_ref() == expected
                && redacted.to_string() == expected
                && !format!("{redacted:?}").contains(constants_str::SECRET)
        }));
    }

    #[test]
    fn test_fallback_preserves_authorities_without_userinfo() {
        assert!(['/', '?', '#'].into_iter().all(|delimiter| {
            let input = format!(
                "1{}://{}{delimiter}{}@{}",
                constants_str::X,
                constants_str::LOCALHOST,
                constants_str::X,
                constants_str::SECRET
            );
            crate::redact_url_userinfo::redact_url_userinfo(input.as_str().into()).as_ref() == input
        }));
        let input = format!("1{}://{}", constants_str::X, constants_str::LOCALHOST);
        assert_eq!(
            crate::redact_url_userinfo::redact_url_userinfo(input.as_str().into()).as_ref(),
            input
        );
    }
}
