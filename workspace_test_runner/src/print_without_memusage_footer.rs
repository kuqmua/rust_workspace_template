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
