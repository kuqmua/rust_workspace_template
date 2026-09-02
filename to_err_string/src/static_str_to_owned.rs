pub(super) fn static_str_to_owned(
    static_str_to_owned_input: crate::static_str_to_owned_input::StaticStrToOwnedInput,
) -> crate::error_text::ErrorText {
    crate::error_text::ErrorText::try_from(static_str_to_owned_input.get().to_owned())
        .unwrap_or_else(crate::error_text::ErrorText::from)
}
