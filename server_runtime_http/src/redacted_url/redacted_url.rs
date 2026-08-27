#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq, newtype::FromInner)]
pub struct RedactedUrl(Option<crate::domain_types::RequiredNulFreeBoundedText>);

impl AsRef<str> for RedactedUrl {
    fn as_ref(&self) -> &str {
        self.0
            .as_ref()
            .map_or(constants_str::REDACTED_ALT_3, AsRef::as_ref)
    }
}

impl std::fmt::Display for RedactedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl std::fmt::Debug for RedactedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::REDACTED_URL)
            .field(&self.as_ref())
            .finish()
    }
}
