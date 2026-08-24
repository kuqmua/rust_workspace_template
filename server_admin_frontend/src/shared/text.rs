const ADMIN_JOINED_TEXT_MAX_BYTES: usize = 16_777_216usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub(crate) enum AdminJoinedTextTryFromStringError {
    #[error("joined administrator frontend text exceeds the size limit")]
    TooLong,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::IntoInnerFrom,
)]
pub(crate) struct AdminJoinedText(String);

impl TryFrom<String> for AdminJoinedText {
    type Error = AdminJoinedTextTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().checked_sub(ADMIN_JOINED_TEXT_MAX_BYTES) {
            Some(excess) if excess > 0usize => Err(AdminJoinedTextTryFromStringError::TooLong),
            _within_limit => Ok(Self(value)),
        }
    }
}

impl From<AdminJoinedTextTryFromStringError> for AdminJoinedText {
    fn from(value: AdminJoinedTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

pub(crate) fn join_txt<'value_lt, Values>(values: Values) -> AdminJoinedText
where
    Values: IntoIterator<Item = &'value_lt str>,
{
    let value_iter = values.into_iter();
    let mut text = String::with_capacity(value_iter.size_hint().0.saturating_mul(16usize));
    value_iter.enumerate().for_each(|(index, value)| {
        if index > 0usize {
            text.push_str(str_constants::COMMA_SPACE);
        }
        text.push_str(value);
    });
    AdminJoinedText::try_from(text).unwrap_or_else(AdminJoinedText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn joins_borrowed_text_without_an_intermediate_collection() {
        assert_eq!(
            super::join_txt(["reader", "editor"]).as_ref(),
            "reader, editor"
        );
        assert!(
            super::join_txt(std::iter::empty::<&str>())
                .as_ref()
                .is_empty()
        );
    }
}
