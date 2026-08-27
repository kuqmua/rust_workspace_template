#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct SignedCursor(String);

impl SignedCursor {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for SignedCursor {
    type Error = crate::domain_types::SignedCursorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > Self::MAXIMUM_LENGTH {
            Err(crate::domain_types::SignedCursorError)
        } else {
            Ok(Self(value))
        }
    }
}
