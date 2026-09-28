#[allow(
    clippy::single_call_fn,
    reason = "named stderr boundary remains directly unit tested because console writes cannot be asserted without capturing process output"
)]
pub(crate) fn memusage_program_text(
    clean_ansi_text: &crate::clean_ansi_text::CleanAnsiText,
) -> crate::stderr_text_ref::StderrTextRef<'_> {
    let text = clean_ansi_text.as_ref();
    crate::stderr_text_ref::StderrTextRef::from(
        text.rsplit_once(constants_str::MEMORY_USAGE_SUMMARY)
            .map_or(text, |(program_text, _summary_text)| program_text),
    )
}
