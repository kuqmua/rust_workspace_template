#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::IntoInnerFrom,
)]
pub(crate) struct AdminJoinedText(String);

impl TryFrom<String> for AdminJoinedText {
    type Error = crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().checked_sub(constants_usize::VALUE_16_777_216) {
            Some(excess) if excess > constants_usize::ZERO => {
                Err(crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong)
            }
            _within_limit => Ok(Self(value)),
        }
    }
}

impl From<crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError>
    for AdminJoinedText
{
    fn from(
        value: crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
