#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct SignedCursor(String);

impl SignedCursor {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for SignedCursor {
    type Error = crate::signed_cursor_error::SignedCursorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > Self::MAXIMUM_LENGTH {
            Err(crate::signed_cursor_error::SignedCursorError::Empty)
        } else {
            Ok(Self(value))
        }
    }
}
