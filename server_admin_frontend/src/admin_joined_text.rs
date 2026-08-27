#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::IntoInnerFrom,
)]
pub(crate) struct AdminJoinedText(String);

impl TryFrom<String> for AdminJoinedText {
    type Error = crate::domain_types::shared::text::AdminJoinedTextTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().checked_sub(constants_usize::VALUE_16_777_216) {
            Some(excess) if excess > constants_usize::ZERO => {
                Err(crate::domain_types::shared::text::AdminJoinedTextTryFromStringError::TooLong)
            }
            _within_limit => Ok(Self(value)),
        }
    }
}

impl From<crate::domain_types::shared::text::AdminJoinedTextTryFromStringError>
    for AdminJoinedText
{
    fn from(value: crate::domain_types::shared::text::AdminJoinedTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
