pub(crate) fn print_without_memusage_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) {
    let clean = crate::strip_ansi_codes::strip_ansi_codes(crate::ansi_text_ref::AnsiTextRef::from(
        stderr_text_ref.get(),
    ));
    clean
        .as_ref()
        .lines()
        .take_while(|line| !line.contains(constants_str::MEMORY_USAGE_SUMMARY))
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| eprintln!("{line}"));
}
