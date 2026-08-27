pub(super) fn static_str_to_owned(
    v: super::StaticStrToOwnedInput,
) -> crate::domain_types::ErrorText {
    crate::domain_types::ErrorText::try_from(v.0.to_owned())
        .unwrap_or_else(crate::domain_types::ErrorText::from)
}
