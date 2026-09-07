pub(crate) fn strip_ansi_codes(
    tool_ansi_chars: macro_helpers::tool_ansi_chars::ToolAnsiChars<'_>,
) -> crate::clean_ansi_text::CleanAnsiText {
    crate::clean_ansi_text::CleanAnsiText::try_from(String::from(tool_ansi_chars))
        .unwrap_or_else(crate::clean_ansi_text::CleanAnsiText::from)
}
