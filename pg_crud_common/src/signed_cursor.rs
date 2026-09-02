#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
)]
pub struct SignedCursor(String);

impl SignedCursor {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for SignedCursor {
    type Error = crate::signed_cursor_error::SignedCursorError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > Self::MAXIMUM_LENGTH {
            Err(crate::signed_cursor_error::SignedCursorError::Empty)
        } else {
            Ok(Self(string))
        }
    }
}
