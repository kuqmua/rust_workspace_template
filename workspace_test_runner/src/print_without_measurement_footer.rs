pub(crate) fn print_without_measurement_footer(stderr: crate::domain_types::StderrTextRef<'_>) {
    stderr
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
