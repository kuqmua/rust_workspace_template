pub(super) fn static_str_to_owned(
    v: crate::static_str_to_owned_input::StaticStrToOwnedInput,
) -> crate::error_text::ErrorText {
    crate::error_text::ErrorText::try_from(v.get().to_owned())
        .unwrap_or_else(crate::error_text::ErrorText::from)
}
