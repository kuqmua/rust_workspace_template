#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct OutboundAllowedHost(pub(super) String);

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
        Ok(Self(value.to_ascii_lowercase()))
    }
}
