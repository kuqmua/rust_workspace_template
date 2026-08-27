pub(super) fn as_ref_str_to_owned<T>(v: &T) -> crate::domain_types::ErrorText
where
    T: ?Sized + AsRef<str>,
{
    crate::domain_types::ErrorText::try_from(v.as_ref().to_owned())
        .unwrap_or_else(crate::domain_types::ErrorText::from)
}
