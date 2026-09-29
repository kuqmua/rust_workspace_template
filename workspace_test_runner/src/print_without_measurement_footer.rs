pub(crate) fn print_without_measurement_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) -> Result<(), macro_helpers::tool_console_write_error::ToolConsoleWriteError> {
    crate::parse_cargo_measurement_footer::parse_cargo_measurement_footer(stderr_text_ref)
        .map_or(stderr_text_ref, |footer| footer.get_program_text())
        .get()
        .lines()
        .try_for_each(|line| {
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardError.write(
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{line}{}",
                    constants_str::NEWLINE
                )),
            )
        })
}
