#[path = "redacted_url/redact_rtsp_url_userinfo.rs"]
mod redact_rtsp_url_userinfo;
#[path = "redacted_url/redact_url_userinfo.rs"]
mod redact_url_userinfo;
#[path = "redacted_url/redacted_url.rs"]
mod redacted_url;
#[path = "redacted_url/redacted_url_text_ref.rs"]
mod redacted_url_text_ref;

pub use redact_rtsp_url_userinfo::redact_rtsp_url_userinfo;
pub use redact_url_userinfo::redact_url_userinfo;
pub use redacted_url::RedactedUrl;
pub use redacted_url_text_ref::RedactedUrlTextRef;

#[cfg(test)]
mod tests {
    #[test]
    fn urls_without_credentials_are_preserved() {
        let input = constants_str::VALUE_DABFAFF0;
        let redacted = super::redact_url_userinfo(input.into());
        assert_eq!(redacted.as_ref(), input);
        assert_eq!(redacted.to_string(), input);
        assert!(format!("{redacted:?}").contains(input));
    }

    #[test]
    fn malformed_urls_do_not_reflect_unstructured_input() {
        let malformed = super::redact_url_userinfo(constants_str::VALUE_D8B5BF9B.into());
        assert_eq!(malformed.as_ref(), constants_str::REDACTED_ALT_3);
        let nul = super::redact_url_userinfo(constants_str::VALUE_7C8CC910.into());
        assert_eq!(nul.as_ref(), constants_str::REDACTED_ALT_3);
    }

    #[test]
    fn fallback_parser_redacts_userinfo_for_unknown_schemes() {
        let secret = constants_str::SECRET;
        let input = format!("1invalid://user:{secret}@example.com/path?query=value");
        let redacted = super::redact_url_userinfo(input.as_str().into());
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
    fn credentials_are_removed_while_non_secret_parts_remain() {
        let redacted = super::redact_url_userinfo(constants_str::TEST_URL_WITH_CREDENTIALS.into());
        assert!(!redacted.as_ref().contains(constants_str::TEST_URL_PASSWORD));
        assert!(redacted.as_ref().contains(constants_str::LOCALHOST));
        assert!(redacted.as_ref().contains(constants_str::REDACTED_ALT));
    }

    #[test]
    fn rtsp_credentials_are_removed() {
        let redacted =
            super::redact_rtsp_url_userinfo(constants_str::TEST_RTSP_URL_WITH_CREDENTIALS.into());
        assert!(!redacted.as_ref().contains(constants_str::TEST_URL_PASSWORD));
        assert!(
            redacted
                .as_ref()
                .starts_with(constants_str::RTSP_SCHEME_PREFIX)
        );
    }
}
