#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct CursorPayload(bounded_types::bounded_string::BoundedString<1usize, 65_536usize, false>);

#[cfg(test)]
impl CursorPayload {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for CursorPayload {
    type Error = crate::cursor_payload_error::CursorPayloadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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
    fn test_cursor_payload_accepts_exact_byte_bounds() {
        assert!(
            [constants_usize::ONE, super::CursorPayload::MAXIMUM_LENGTH]
                .into_iter()
                .all(|length| {
                    super::CursorPayload::try_from(constants_str::X.repeat(length))
                        .is_ok_and(|cursor| cursor.as_ref().len() == length)
                })
        );
    }

    #[test]
    fn test_cursor_payload_distinguishes_empty_and_oversized_values() {
        assert_eq!(
            crate::cursor_payload::CursorPayload::try_from(String::new()),
            Err(crate::cursor_payload_error::CursorPayloadError::Empty)
        );
        let oversized = constants_str::X.repeat(super::CursorPayload::MAXIMUM_LENGTH + 1usize);
        assert_eq!(
            crate::cursor_payload::CursorPayload::try_from(oversized)
                .map_err(|error| error.to_string()),
            Err(String::from(constants_str::CURSOR_EXCEEDS_MAXIMUM_LENGTH))
        );
    }
}
