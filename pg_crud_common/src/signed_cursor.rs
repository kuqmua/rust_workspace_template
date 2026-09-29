#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct SignedCursor(bounded_types::bounded_string::BoundedString<1usize, 65_536usize, false>);

impl SignedCursor {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for SignedCursor {
    type Error = crate::signed_cursor_error::SignedCursorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if value.len() > Self::MAXIMUM_LENGTH {
            return Err(Self::Error::TooLong);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                } => Self::Error::TooLong,
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::Empty,
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signed_cursor_distinguishes_empty_and_oversized_values() {
        assert_eq!(
            crate::signed_cursor::SignedCursor::try_from(String::new()),
            Err(crate::signed_cursor_error::SignedCursorError::Empty)
        );
        let oversized = constants_str::X.repeat(super::SignedCursor::MAXIMUM_LENGTH + 1usize);
        assert_eq!(
            crate::signed_cursor::SignedCursor::try_from(oversized)
                .map_err(|error| error.to_string()),
            Err(String::from(constants_str::CURSOR_EXCEEDS_MAXIMUM_LENGTH))
        );
    }
}
