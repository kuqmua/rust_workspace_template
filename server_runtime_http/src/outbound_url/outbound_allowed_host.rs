#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct OutboundAllowedHost(pub(super) String);

impl TryFrom<String> for OutboundAllowedHost {
    type Error = super::OutboundHostAllowlistError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 253usize
            || value.bytes().any(|byte| {
                !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b':' | b'[' | b']'))
            })
        {
            return Err(super::OutboundHostAllowlistError::InvalidHost);
        }
        Ok(Self(value.to_ascii_lowercase()))
    }
}
