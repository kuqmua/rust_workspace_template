#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct AdminJoinedText(String);

impl TryFrom<String> for AdminJoinedText {
    type Error = crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        match string.len().checked_sub(constants_usize::VALUE_16_777_216) {
            Some(excess) if excess > constants_usize::ZERO => {
                Err(crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong)
            }
            _within_limit => Ok(Self(string)),
        }
    }
}

impl From<crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError>
    for AdminJoinedText
{
    fn from(
        admin_joined_text_try_from_string_error: crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError,
    ) -> Self {
        Self(admin_joined_text_try_from_string_error.to_string())
    }
}
