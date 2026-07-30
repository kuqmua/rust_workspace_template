#[derive(Clone, Eq, PartialEq, newtype::FromInner)]
pub struct RedactedUrl(Option<crate::RequiredNulFreeBoundedText>);

impl AsRef<str> for RedactedUrl {
    fn as_ref(&self) -> &str {
        self.0
            .as_ref()
            .map_or(str_constants::REDACTED_ALT_3, AsRef::as_ref)
    }
}
impl std::fmt::Display for RedactedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl std::fmt::Debug for RedactedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(str_constants::REDACTED_URL)
            .field(&self.as_ref())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct RedactedUrlTextRef<'value_lt>(&'value_lt str);

#[must_use]
pub fn redact_url_userinfo(value: RedactedUrlTextRef<'_>) -> RedactedUrl {
    let input = value.0;
    if let Ok(mut url) = reqwest::Url::parse(input) {
        if url.username().is_empty() && url.password().is_none() {
            return RedactedUrl::from(
                crate::RequiredNulFreeBoundedText::try_from(input.to_owned()).ok(),
            );
        }
        if url.set_username(str_constants::REDACTED_ALT).is_ok() && url.set_password(None).is_ok() {
            return RedactedUrl::from(
                crate::RequiredNulFreeBoundedText::try_from(url.to_string()).ok(),
            );
        }
    }
    let Some((scheme, remainder)) = input.split_once(str_constants::TEXT_ALT_10) else {
        return RedactedUrl::from(
            crate::RequiredNulFreeBoundedText::try_from(str_constants::REDACTED_ALT_3.to_owned())
                .ok(),
        );
    };
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let Some(authority) = remainder.get(..authority_end) else {
        return RedactedUrl::from(
            crate::RequiredNulFreeBoundedText::try_from(str_constants::REDACTED_ALT_3.to_owned())
                .ok(),
        );
    };
    let Some(userinfo_end) = authority.rfind('@') else {
        return RedactedUrl::from(
            crate::RequiredNulFreeBoundedText::try_from(input.to_owned()).ok(),
        );
    };
    let host = authority
        .get(userinfo_end.saturating_add(1usize)..)
        .unwrap_or(str_constants::REDACTED_ALT_3);
    let suffix = remainder.get(authority_end..).unwrap_or_default();
    let mut output = String::with_capacity(input.len());
    output.push_str(scheme);
    output.push_str(str_constants::TEXT_ALT_10);
    output.push_str(str_constants::REDACTED_ALT);
    output.push('@');
    output.push_str(host);
    output.push_str(suffix);
    RedactedUrl::from(crate::RequiredNulFreeBoundedText::try_from(output).ok())
}

#[must_use]
pub fn redact_rtsp_url_userinfo(value: RedactedUrlTextRef<'_>) -> RedactedUrl {
    redact_url_userinfo(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn urls_without_credentials_are_preserved() {
        let input = "https://example.com/path?query=value#fragment";
        let redacted = super::redact_url_userinfo(input.into());
        assert_eq!(redacted.as_ref(), input);
        assert_eq!(redacted.to_string(), input);
        assert!(format!("{redacted:?}").contains(input));
    }

    #[test]
    fn malformed_urls_do_not_reflect_unstructured_input() {
        let malformed = super::redact_url_userinfo("not a url".into());
        assert_eq!(malformed.as_ref(), str_constants::REDACTED_ALT_3);
        let nul = super::redact_url_userinfo("\0secret".into());
        assert_eq!(nul.as_ref(), str_constants::REDACTED_ALT_3);
    }

    #[test]
    fn fallback_parser_redacts_userinfo_for_unknown_schemes() {
        let secret = "secret";
        let input = format!("1invalid://user:{secret}@example.com/path?query=value");
        let redacted = super::redact_url_userinfo(input.as_str().into());
        assert_eq!(
            redacted.as_ref(),
            format!(
                "1invalid://{}@example.com/path?query=value",
                str_constants::REDACTED_ALT
            )
        );
        assert!(!redacted.as_ref().contains(secret));
    }

    #[test]
    fn credentials_are_removed_while_non_secret_parts_remain() {
        let redacted = super::redact_url_userinfo(str_constants::TEST_URL_WITH_CREDENTIALS.into());
        assert!(!redacted.as_ref().contains(str_constants::TEST_URL_PASSWORD));
        assert!(redacted.as_ref().contains(str_constants::LOCALHOST));
        assert!(redacted.as_ref().contains(str_constants::REDACTED_ALT));
    }

    #[test]
    fn rtsp_credentials_are_removed() {
        let redacted =
            super::redact_rtsp_url_userinfo(str_constants::TEST_RTSP_URL_WITH_CREDENTIALS.into());
        assert!(!redacted.as_ref().contains(str_constants::TEST_URL_PASSWORD));
        assert!(
            redacted
                .as_ref()
                .starts_with(str_constants::RTSP_SCHEME_PREFIX)
        );
    }
}
