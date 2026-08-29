pub(super) fn debug_to_string<T>(v: &T) -> crate::error_text::ErrorText
where
    T: std::fmt::Debug,
{
    crate::error_text::ErrorText::try_from(format!("{v:?}"))
        .unwrap_or_else(crate::error_text::ErrorText::from)
}
