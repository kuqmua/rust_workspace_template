#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct OutboundAllowedHost(
    bounded_types::bounded_string::BoundedString<1usize, 253usize, false>,
);

impl OutboundAllowedHost {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for OutboundAllowedHost {
    type Error = crate::outbound_host_allowlist_error::OutboundHostAllowlistError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 253usize
            || value.bytes().any(|byte| {
                !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b':' | b'[' | b']'))
            })
        {
            return Err(
                crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost,
            );
        }
        bounded_types::bounded_string::BoundedString::try_from(value.to_ascii_lowercase())
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::InvalidHost,
            })
    }
}
