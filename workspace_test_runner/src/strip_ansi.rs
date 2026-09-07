#[allow(
    clippy::single_call_fn,
    reason = "strip ansi remains a named owner because its boundary role is clearer and directly testable"
)]
pub(super) fn strip_ansi(
    tool_ansi_chars: macro_helpers::tool_ansi_chars::ToolAnsiChars<'_>,
) -> crate::command_text::CommandText {
    crate::command_text::CommandText::try_from(String::from(tool_ansi_chars))
        .unwrap_or_else(crate::command_text::CommandText::from)
}
