pub(crate) fn print_without_measurement_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) {
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
        .for_each(|line| eprintln!("{line}"));
}
