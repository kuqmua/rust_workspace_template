pub(crate) fn print_without_memusage_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) -> Result<(), macro_helpers::tool_console_write_error::ToolConsoleWriteError> {
    let clean = crate::strip_ansi_codes::strip_ansi_codes(
        macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(stderr_text_ref.get()),
        ),
    );
    clean
        .as_ref()
        .lines()
        .take_while(|line| !line.contains(constants_str::MEMORY_USAGE_SUMMARY))
        .filter(|line| !line.trim().is_empty())
        .try_for_each(|line| {
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardError.write(
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{line}{}",
                    constants_str::NEWLINE
                )),
            )
        })
}
