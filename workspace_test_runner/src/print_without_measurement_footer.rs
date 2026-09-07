pub(crate) fn print_without_measurement_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) -> Result<(), macro_helpers::tool_console_write_error::ToolConsoleWriteError> {
    stderr_text_ref
        .get()
        .lines()
        .filter(|line| {
            !line
                .trim()
                .starts_with(constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
        })
        .filter(|line| {
            !line
                .trim()
                .starts_with(constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
        })
        .filter(|line| {
            !line
                .trim()
                .starts_with(constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
        })
        .try_for_each(|line| {
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardError.write(
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{line}{}",
                    constants_str::NEWLINE
                )),
            )
        })
}
