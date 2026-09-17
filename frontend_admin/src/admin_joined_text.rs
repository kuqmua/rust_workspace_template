#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct AdminJoinedText(
    bounded_types::bounded_string::BoundedString<0usize, 16_777_216usize, false>,
);

impl TryFrom<String> for AdminJoinedText {
    type Error = crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().checked_sub(constants_usize::VALUE_16_777_216) {
            Some(excess) if excess > constants_usize::ZERO => {
                Err(crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong)
            }
            _within_limit => bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { .. }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum { .. } => Self::Error::TooLong,
                }),
        }
    }
}

impl From<crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError>
    for AdminJoinedText
{
    fn from(
        value: crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError,
    ) -> Self {
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
    }
}
