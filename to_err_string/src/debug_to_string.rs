pub(super) fn debug_to_string<T>(t: &T) -> crate::error_text::ErrorText
where
    T: std::fmt::Debug,
{
    crate::error_text::ErrorText::try_from(format!("{t:?}"))
        .unwrap_or_else(crate::error_text::ErrorText::from)
}
