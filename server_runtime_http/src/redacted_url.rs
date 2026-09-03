#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RedactedUrl(
    Option<text_policy::required_nul_free_bounded_text::RequiredNulFreeBoundedText>,
);

impl AsRef<str> for RedactedUrl {
    fn as_ref(&self) -> &str {
        self.0
            .as_ref()
            .map_or(constants_str::REDACTED_ALT_3, AsRef::as_ref)
    }
}

impl std::fmt::Display for RedactedUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_ref())
    }
}

impl std::fmt::Debug for RedactedUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::REDACTED_URL)
            .field(&self.as_ref())
            .finish()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_urls_without_credentials_are_preserved() {
        let input = constants_str::VALUE_DABFAFF0;
        let redacted = crate::redact_url_userinfo::redact_url_userinfo(input.into());
        assert_eq!(redacted.as_ref(), input);
        assert_eq!(redacted.to_string(), input);
        assert!(format!("{redacted:?}").contains(input));
    }

    #[test]
    fn test_malformed_urls_do_not_reflect_unstructured_input() {
        let malformed =
            crate::redact_url_userinfo::redact_url_userinfo(constants_str::VALUE_D8B5BF9B.into());
        assert_eq!(malformed.as_ref(), constants_str::REDACTED_ALT_3);
        let nul =
            crate::redact_url_userinfo::redact_url_userinfo(constants_str::VALUE_7C8CC910.into());
        assert_eq!(nul.as_ref(), constants_str::REDACTED_ALT_3);
    }

    #[test]
    fn test_fallback_parser_redacts_userinfo_for_unknown_schemes() {
        let secret = constants_str::SECRET;
        let input = format!("1invalid://user:{secret}@example.com/path?query=value");
        let redacted = crate::redact_url_userinfo::redact_url_userinfo(input.as_str().into());
        assert_eq!(
            redacted.as_ref(),
            format!(
                "1invalid://{}@example.com/path?query=value",
                constants_str::REDACTED_ALT
            )
        );
        assert!(!redacted.as_ref().contains(secret));
    }

    #[test]
    fn test_credentials_are_removed_while_non_secret_parts_remain() {
        let redacted = crate::redact_url_userinfo::redact_url_userinfo(
            constants_str::TEST_URL_WITH_CREDENTIALS.into(),
        );
        assert!(!redacted.as_ref().contains(constants_str::TEST_URL_PASSWORD));
        assert!(redacted.as_ref().contains(constants_str::LOCALHOST));
        assert!(redacted.as_ref().contains(constants_str::REDACTED_ALT));
    }

    #[test]
    fn test_rtsp_credentials_are_removed() {
        let redacted = crate::redact_rtsp_url_userinfo::redact_rtsp_url_userinfo(
            constants_str::TEST_RTSP_URL_WITH_CREDENTIALS.into(),
        );
        assert!(!redacted.as_ref().contains(constants_str::TEST_URL_PASSWORD));
        assert!(
            redacted
                .as_ref()
                .starts_with(constants_str::RTSP_SCHEME_PREFIX)
        );
    }
}
