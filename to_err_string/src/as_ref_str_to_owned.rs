pub(super) fn as_ref_str_to_owned<T>(t: &T) -> crate::error_text::ErrorText
where
    T: ?Sized + AsRef<str>,
{
    crate::error_text::ErrorText::try_from(t.as_ref().to_owned())
        .unwrap_or_else(crate::error_text::ErrorText::from)
}
