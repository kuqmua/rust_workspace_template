pub(super) fn debug_to_string<T>(v: &T) -> crate::domain_types::ErrorText
where
    T: std::fmt::Debug,
{
    crate::domain_types::ErrorText::try_from(format!("{v:?}"))
        .unwrap_or_else(crate::domain_types::ErrorText::from)
}
