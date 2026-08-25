#![allow(clippy::single_call_fn)] // reporting stays independent from command execution so diagnostics have one owner
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

pub(crate) fn print_without_memusage_footer(stderr: crate::domain_types::StderrTextRef<'_>) {
    let clean =
        crate::domain_types::strip_ansi_codes(crate::domain_types::AnsiTextRef::from(stderr.get()));
    clean
        .as_ref()
        .lines()
        .take_while(|line| !line.contains(constants_str::MEMORY_USAGE_SUMMARY))
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| eprintln!("{line}"));
}
