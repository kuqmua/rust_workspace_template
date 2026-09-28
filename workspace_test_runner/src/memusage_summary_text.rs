pub(crate) fn memusage_summary_text(
    clean_ansi_text: &crate::clean_ansi_text::CleanAnsiText,
) -> Option<crate::stderr_text_ref::StderrTextRef<'_>> {
    clean_ansi_text
        .as_ref()
        .rsplit_once(constants_str::MEMORY_USAGE_SUMMARY)
        .map(|(_preceding_text, summary)| crate::stderr_text_ref::StderrTextRef::from(summary))
}
